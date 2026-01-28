use crate::config::{AgentConfig, RoutingConfig, RoutingRule};
use anyhow::{Result, bail};

pub struct AgentRouter {
    routing_config: RoutingConfig,
}

impl AgentRouter {
    pub fn new(routing_config: RoutingConfig) -> Self {
        Self { routing_config }
    }

    /// Select the best agent for a given task type and prompt
    pub fn select_agent<'a>(
        &self,
        task_type: &str,
        prompt: &str,
        available_agents: &'a [&'a AgentConfig],
    ) -> Result<&'a AgentConfig> {
        // Find matching routing rules
        let matching_rules: Vec<&RoutingRule> = self.routing_config
            .rules
            .iter()
            .filter(|rule| {
                // Check if task_type matches
                if rule.task_type != task_type {
                    return false;
                }

                // Check if any keyword appears in prompt
                rule.keywords.is_empty() || 
                rule.keywords.iter().any(|keyword| {
                    prompt.to_lowercase().contains(&keyword.to_lowercase())
                })
            })
            .collect();

        // If we have matching rules, use preferred agents
        if !matching_rules.is_empty() {
            for rule in matching_rules {
                for preferred_agent_id in &rule.preferred_agents {
                    if let Some(agent) = available_agents
                        .iter()
                        .find(|a| &a.id == preferred_agent_id)
                    {
                        tracing::debug!(
                            "Selected agent {} via routing rule for task_type: {}",
                            agent.id,
                            task_type
                        );
                        return Ok(agent);
                    }
                }
            }
        }

        // Fallback: select by priority
        available_agents
            .first()
            .copied()
            .ok_or_else(|| anyhow::anyhow!("No available agents for task"))
    }

    /// Get agents that match task requirements
    pub fn filter_capable_agents<'a>(
        &self,
        task_type: &str,
        all_agents: &'a [&'a AgentConfig],
    ) -> Vec<&'a AgentConfig> {
        // Check routing rules for capability hints
        let required_capabilities: Vec<String> = self.routing_config
            .rules
            .iter()
            .filter(|rule| rule.task_type == task_type)
            .flat_map(|rule| {
                // Extract capabilities from preferred agents
                rule.preferred_agents.iter().filter_map(|agent_id| {
                    all_agents
                        .iter()
                        .find(|a| &a.id == agent_id)
                        .and_then(|a| a.capabilities.first().cloned())
                })
            })
            .collect();

        if required_capabilities.is_empty() {
            // No specific requirements, return all
            return all_agents.to_vec();
        }

        // Filter agents that have any of the required capabilities
        all_agents
            .iter()
            .filter(|agent| {
                agent.capabilities.iter().any(|cap| {
                    required_capabilities.contains(cap)
                })
            })
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RateLimit;

    fn create_test_agent(id: &str, capabilities: Vec<&str>, priority: u8) -> AgentConfig {
        AgentConfig {
            id: id.to_string(),
            name: id.to_string(),
            agent_type: "cli".to_string(),
            command: Some("test".to_string()),
            args: None,
            extension_name: None,
            rate_limit: RateLimit {
                requests_per_minute: 60,
                requests_per_day: 2000,
            },
            capabilities: capabilities.iter().map(|s| s.to_string()).collect(),
            priority,
        }
    }

    #[test]
    fn test_agent_selection_by_rule() {
        let routing_config = RoutingConfig {
            rules: vec![
                RoutingRule {
                    task_type: "code-generation".to_string(),
                    keywords: vec!["write code".to_string()],
                    preferred_agents: vec!["qwen".to_string()],
                },
            ],
        };

        let router = AgentRouter::new(routing_config);

        let agents = vec![
            create_test_agent("qwen", vec!["code-generation"], 1),
            create_test_agent("codex", vec!["code-generation"], 2),
        ];

        let agent_refs: Vec<&AgentConfig> = agents.iter().collect();

        let selected = router
            .select_agent("code-generation", "please write code for me", &agent_refs)
            .unwrap();

        assert_eq!(selected.id, "qwen");
    }

    #[test]
    fn test_fallback_to_priority() {
        let routing_config = RoutingConfig { rules: vec![] };
        let router = AgentRouter::new(routing_config);

        let agents = vec![
            create_test_agent("low", vec!["test"], 1),
            create_test_agent("high", vec!["test"], 2),
        ];

        // Sort by priority (higher first)
        let mut agent_refs: Vec<&AgentConfig> = agents.iter().collect();
        agent_refs.sort_by(|a, b| b.priority.cmp(&a.priority));

        let selected = router
            .select_agent("test", "any prompt", &agent_refs)
            .unwrap();

        assert_eq!(selected.id, "high");
    }
}
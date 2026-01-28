use crate::config::{AgentConfig, RoutingConfig, RoutingRule};
use anyhow::{Result, bail};

pub struct AgentRouter {
    routing_config: RoutingConfig,
}

impl AgentRouter {
    pub fn new(routing_config: RoutingConfig) -> Self {
        Self { routing_config }
    }

    /// Select the best agent for a given task type and prompt using a tiered priority system.
    pub fn select_agent<'a>(
        &self,
        task_type: &str,
        prompt: &str,
        available_agents: &'a [&'a AgentConfig],
    ) -> Result<&'a AgentConfig> {
        
        // --- 1. คำนวณ Final Priority สำหรับแต่ละ Rule ที่ Match ---
        let mut candidate_rules: Vec<(f64, &RoutingRule)> = self.routing_config
            .rules
            .iter()
            .filter(|rule| {
                // ตรวจสอบว่า rule match กับ task หรือไม่ (เหมือนเดิม)
                rule.task_type == task_type &&
                (rule.keywords.is_empty() || 
                 rule.keywords.iter().any(|kw| prompt.to_lowercase().contains(&kw.to_lowercase())))
            })
            .map(|rule| {
                // --- คำนวณ Final Priority ตามสูตรของ Gemini CLI ---
                let tier_base = match self.routing_config.tier.as_str() {
                    "admin" => 3.0,
                    "user" => 2.0,
                    _ => 1.0, // default
                };
                // สูตร: final_priority = tier_base + (rule_priority / 1000)
                let final_priority = tier_base + (f64::from(rule.priority) / 1000.0);
                (final_priority, rule)
            })
            .collect();

        // --- 2. จัดลำดับ Rule ที่ Match ทั้งหมดตาม Final Priority (สูงไปต่ำ) ---
        candidate_rules.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        // --- 3. วนลูปตาม Rule ที่มี Priority สูงสุดก่อน ---
        if !candidate_rules.is_empty() {
            tracing::debug!("Found {} candidate routing rules. Evaluating in order of priority...", candidate_rules.len());
            for (priority_score, rule) in candidate_rules {
                tracing::trace!("Evaluating rule with priority score {:.3}: preferred_agents: {:?}", priority_score, rule.preferred_agents);
                // วนลูปตาม preferred_agents ใน rule นั้นๆ
                for preferred_agent_id in &rule.preferred_agents {
                    // หา agent ที่พร้อมใช้งาน (available)
                    if let Some(agent) = available_agents
                        .iter()
                        .find(|a| &a.id == preferred_agent_id)
                    {
                        tracing::debug!(
                            "Selected agent '{}' via rule with priority score {:.3} for task_type: '{}'",
                            agent.id,
                            priority_score,
                            task_type
                        );
                        // เจอแล้ว! คืนค่า agent ตัวแรกที่เจอใน rule ที่มี priority สูงสุด
                        return Ok(agent);
                    } else {
                        tracing::trace!("Preferred agent '{}' is not in the list of available agents.", preferred_agent_id);
                    }
                }
            }
        }

        // --- 4. Fallback: ถ้าไม่มี Rule ไหนเลยที่หา Agent ที่พร้อมใช้งานเจอ ---
        // ให้กลับไปใช้วิธีเดิม คือเลือกจาก Agent ที่มี Priority สูงสุดในบรรดาตัวที่พร้อมใช้งานทั้งหมด
        tracing::debug!("No matching rule found an available agent. Falling back to agent priority.");
        available_agents
            .first() // available_agents ควรจะถูกเรียงตาม priority มาจากข้างนอกแล้ว
            .copied()
            .ok_or_else(|| anyhow::anyhow!("No available agents for task"))
    }

    /// Get agents that match task requirements (no changes needed here)
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
            rate_limit: RateLimit::default(),
            capabilities: capabilities.iter().map(|s| s.to_string()).collect(),
            priority,
        }
    }

    #[test]
    fn test_select_agent_based_on_rule_priority() {
        let routing_config = RoutingConfig {
            tier: "user".to_string(),
            rules: vec![
                // Rule ทั่วไป, priority ต่ำ
                RoutingRule {
                    task_type: "code-generation".to_string(),
                    keywords: vec!["write code".to_string()],
                    preferred_agents: vec!["codex".to_string()],
                    priority: 100,
                },
                // Rule เฉพาะทาง, priority สูง
                RoutingRule {
                    task_type: "code-generation".to_string(),
                    keywords: vec!["rust".to_string()],
                    preferred_agents: vec!["qwen".to_string()],
                    priority: 200,
                },
            ],
        };

        let router = AgentRouter::new(routing_config);

        let agents = vec![
            create_test_agent("qwen", vec!["code-generation"], 1),
            create_test_agent("codex", vec!["code-generation"], 2),
        ];
        let agent_refs: Vec<&AgentConfig> = agents.iter().collect();

        // Prompt ที่มีคำว่า "rust" ควรจะ match rule ที่มี priority สูงกว่า
        let selected = router
            .select_agent("code-generation", "write some rust code", &agent_refs)
            .unwrap();

        assert_eq!(selected.id, "qwen");
    }

    #[test]
    fn test_select_agent_from_preferred_list_order() {
        let routing_config = RoutingConfig {
            tier: "user".to_string(),
            rules: vec![
                RoutingRule {
                    task_type: "code-generation".to_string(),
                    keywords: vec![],
                    // qwen มาก่อน codex
                    preferred_agents: vec!["qwen".to_string(), "codex".to_string()],
                    priority: 100,
                },
            ],
        };
        let router = AgentRouter::new(routing_config);

        // Agent ทั้งสองตัวพร้อมใช้งาน
        let agents = vec![
            create_test_agent("qwen", vec!["code-generation"], 1),
            create_test_agent("codex", vec!["code-generation"], 2),
        ];
        let agent_refs: Vec<&AgentConfig> = agents.iter().collect();
        
        let selected = router.select_agent("code-generation", "any", &agent_refs).unwrap();
        assert_eq!(selected.id, "qwen"); // ควรเลือกตัวแรกใน list

        // ถ้า qwen ไม่พร้อมใช้งาน
        let available_agents = vec![&agents[1]]; // มีแค่ codex
        let selected_fallback = router.select_agent("code-generation", "any", &available_agents).unwrap();
        assert_eq!(selected_fallback.id, "codex"); // ควรเลือกตัวถัดไปใน list
    }

    #[test]
    fn test_fallback_to_agent_priority_when_no_rule_matches() {
        let routing_config = RoutingConfig { tier: "user".to_string(), rules: vec![] };
        let router = AgentRouter::new(routing_config);

        let agents = vec![
            create_test_agent("low", vec!["test"], 1),
            create_test_agent("high", vec!["test"], 10),
        ];

        // เรียงตาม priority ของ Agent เอง
        let mut agent_refs: Vec<&AgentConfig> = agents.iter().collect();
        agent_refs.sort_by(|a, b| b.priority.cmp(&a.priority));

        let selected = router.select_agent("test", "any prompt", &agent_refs).unwrap();
        assert_eq!(selected.id, "high");
    }
}

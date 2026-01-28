use crate::config::AgentConfig;
use std::collections::HashMap;
use anyhow::{Result, Context};

pub struct AgentRegistry {
    agents: HashMap<String, AgentConfig>,
    active_tasks: HashMap<String, TaskInfo>,
}

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub task_id: String,
    pub agent_id: String,
    pub task_type: String,
    pub status: TaskStatus,
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

impl AgentRegistry {
    pub fn new(agents: Vec<AgentConfig>) -> Self {
        let agents_map = agents
            .into_iter()
            .map(|agent| (agent.id.clone(), agent))
            .collect();

        Self {
            agents: agents_map,
            active_tasks: HashMap::new(),
        }
    }

    /// Get agent by ID
    pub fn get_agent(&self, id: &str) -> Option<&AgentConfig> {
        self.agents.get(id)
    }

    /// Get agents by capability
    pub fn get_agents_by_capability(&self, capability: &str) -> Vec<&AgentConfig> {
        self.agents
            .values()
            .filter(|agent| agent.capabilities.contains(&capability.to_string()))
            .collect()
    }

    /// Get all agent IDs
    pub fn list_agent_ids(&self) -> Vec<String> {
        self.agents.keys().cloned().collect()
    }

    /// Get all agents sorted by priority (higher first)
    pub fn get_agents_by_priority(&self) -> Vec<&AgentConfig> {
        let mut agents: Vec<&AgentConfig> = self.agents.values().collect();
        agents.sort_by(|a, b| b.priority.cmp(&a.priority));
        agents
    }

    /// Register a new task
    pub fn register_task(&mut self, task: TaskInfo) {
        self.active_tasks.insert(task.task_id.clone(), task);
    }

    /// Update task status
    pub fn update_task_status(&mut self, task_id: &str, status: TaskStatus) -> Result<()> {
        self.active_tasks
            .get_mut(task_id)
            .map(|task| task.status = status)
            .context("Task not found")
    }

    /// Get active task count
    pub fn active_task_count(&self) -> usize {
        self.active_tasks
            .values()
            .filter(|task| matches!(task.status, TaskStatus::Running | TaskStatus::Pending))
            .count()
    }

    /// Remove completed/failed tasks (cleanup)
    pub fn cleanup_finished_tasks(&mut self) {
        self.active_tasks.retain(|_, task| {
            matches!(task.status, TaskStatus::Running | TaskStatus::Pending)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::RateLimit;

    #[test]
    fn test_agent_registry() {
        let agents = vec![
            AgentConfig {
                id: "test1".to_string(),
                name: "Test 1".to_string(),
                agent_type: "cli".to_string(),
                command: Some("test".to_string()),
                args: None,
                extension_name: None,
                rate_limit: RateLimit {
                    requests_per_minute: 60,
                    requests_per_day: 2000,
                },
                capabilities: vec!["test".to_string()],
                priority: 1,
            }
        ];

        let registry = AgentRegistry::new(agents);
        assert_eq!(registry.list_agent_ids().len(), 1);
        assert!(registry.get_agent("test1").is_some());
    }
}
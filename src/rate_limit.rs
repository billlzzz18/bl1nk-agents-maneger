use crate::config::RateLimitingConfig;
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

pub struct RateLimitTracker {
    config: RateLimitingConfig,
    usage: HashMap<String, AgentUsage>,
}

#[derive(Debug, Clone)]
struct AgentUsage {
    requests_today: u32,
    requests_this_minute: u32,
    day_start: DateTime<Utc>,
    minute_start: DateTime<Utc>,
}

impl AgentUsage {
    fn new() -> Self {
        let now = Utc::now();
        Self {
            requests_today: 0,
            requests_this_minute: 0,
            day_start: now,
            minute_start: now,
        }
    }

    fn reset_if_needed(&mut self) {
        let now = Utc::now();

        // Reset daily counter
        if now.signed_duration_since(self.day_start) > Duration::days(1) {
            self.requests_today = 0;
            self.day_start = now;
        }

        // Reset minute counter
        if now.signed_duration_since(self.minute_start) > Duration::minutes(1) {
            self.requests_this_minute = 0;
            self.minute_start = now;
        }
    }
}

impl RateLimitTracker {
    pub fn new(config: RateLimitingConfig) -> Self {
        Self {
            config,
            usage: HashMap::new(),
        }
    }

    /// Check if agent can make a request and increment counter if yes
    pub async fn check_and_increment(&mut self, agent_id: &str) -> bool {
        let usage = self.usage
            .entry(agent_id.to_string())
            .or_insert_with(AgentUsage::new);

        usage.reset_if_needed();

        // Check limits (hardcoded for now, should come from agent config)
        let daily_limit = 2000u32;
        let minute_limit = 60u32;

        if usage.requests_today >= daily_limit {
            tracing::warn!("Agent {} hit daily rate limit", agent_id);
            return false;
        }

        if usage.requests_this_minute >= minute_limit {
            tracing::warn!("Agent {} hit per-minute rate limit", agent_id);
            return false;
        }

        // Increment counters
        usage.requests_today += 1;
        usage.requests_this_minute += 1;

        tracing::debug!(
            "Agent {} usage: {}/day, {}/min",
            agent_id,
            usage.requests_today,
            usage.requests_this_minute
        );

        true
    }

    /// Get current usage for an agent
    pub fn get_usage(&self, agent_id: &str) -> Option<(u32, u32)> {
        self.usage.get(agent_id).map(|usage| {
            (usage.requests_today, usage.requests_this_minute)
        })
    }

    /// Reset all usage counters (for testing)
    pub fn reset_all(&mut self) {
        self.usage.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiting() {
        let config = RateLimitingConfig {
            strategy: "round-robin".to_string(),
            track_usage: true,
            usage_db_path: "/tmp/test.db".to_string(),
        };

        let mut tracker = RateLimitTracker::new(config);

        // First request should succeed
        assert!(tracker.check_and_increment("test-agent").await);

        // Check usage
        let (daily, minute) = tracker.get_usage("test-agent").unwrap();
        assert_eq!(daily, 1);
        assert_eq!(minute, 1);
    }

    #[tokio::test]
    async fn test_minute_limit() {
        let config = RateLimitingConfig {
            strategy: "round-robin".to_string(),
            track_usage: true,
            usage_db_path: "/tmp/test.db".to_string(),
        };

        let mut tracker = RateLimitTracker::new(config);

        // Exhaust minute limit
        for _ in 0..60 {
            assert!(tracker.check_and_increment("test-agent").await);
        }

        // 61st request should fail
        assert!(!tracker.check_and_increment("test-agent").await);
    }
}
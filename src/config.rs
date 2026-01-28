use serde::Deserialize;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::fs;

// เพิ่ม dirs เข้ามาเพื่อหา default config path
use dirs;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub main_agent: MainAgentConfig,
    #[serde(default)]
    pub agents: Vec<AgentConfig>,
    #[serde(default)]
    pub routing: RoutingConfig,
    #[serde(default)]
    pub rate_limiting: RateLimitingConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default = "default_max_concurrent_tasks")]
    pub max_concurrent_tasks: usize,
}

fn default_max_concurrent_tasks() -> usize { 5 }

#[derive(Debug, Clone, Deserialize)]
pub struct MainAgentConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub agent_type: String,
    // เพิ่ม session_token_path เข้ามาตามไฟล์ config ตัวอย่าง
    pub session_token_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub agent_type: String,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub extension_name: Option<String>,
    #[serde(default)]
    pub rate_limit: RateLimit,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub priority: u8,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RateLimit {
    #[serde(default = "default_requests_per_minute")]
    pub requests_per_minute: u32,
    #[serde(default = "default_requests_per_day")]
    pub requests_per_day: u32,
}

fn default_requests_per_minute() -> u32 { 60 }
fn default_requests_per_day() -> u32 { 2000 }

// --- ส่วนที่แก้ไข: อัปเดต RoutingConfig และ RoutingRule ---

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RoutingConfig {
    #[serde(default = "default_routing_tier")]
    pub tier: String, // เพิ่มฟิลด์ tier
    #[serde(default)]
    pub rules: Vec<RoutingRule>,
}

// ฟังก์ชันสำหรับกำหนดค่าเริ่มต้นของ tier
fn default_routing_tier() -> String { "user".to_string() }

#[derive(Debug, Clone, Deserialize)]
pub struct RoutingRule {
    pub task_type: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub preferred_agents: Vec<String>,
    #[serde(default)] // priority เป็น optional, ค่าเริ่มต้นคือ 0
    pub priority: u16, // เพิ่มฟิลด์ priority (ใช้ u16 เพื่อรองรับ 0-999)
}


#[derive(Debug, Clone, Deserialize, Default)]
pub struct RateLimitingConfig {
    #[serde(default = "default_rate_limit_strategy")]
    pub strategy: String,
    #[serde(default = "default_track_usage")]
    pub track_usage: bool,
    pub usage_db_path: Option<String>,
}

fn default_rate_limit_strategy() -> String { "round-robin".to_string() }
fn default_track_usage() -> bool { true }

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LoggingConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_output")]
    pub output: String,
}

fn default_log_level() -> String { "info".to_string() }
fn default_log_output() -> String { "stdout".to_string() }


// --- ฟังก์ชันสำหรับโหลดและปรับปรุง Config (ยังคงเหมือนเดิม) ---

impl Config {
    /// Loads configuration from a given path, and injects bundled agents if the feature is enabled.
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or_else(default_config_path);

        tracing::info!("Loading configuration from: {}", path.display());

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {}", path.display()))?;

        let mut config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse TOML config")?;

        // ส่วนของการฉีด Bundled Agent ยังคงทำงานได้เหมือนเดิม
        #[cfg(feature = "bundle-pmat")]
        {
            // ตรวจสอบก่อนว่ามี agent ที่ id ซ้ำกันหรือไม่ เพื่อป้องกันการเพิ่มซ้ำซ้อน
            if !config.agents.iter().any(|a| a.id == "pmat-architect-internal") {
                tracing::info!("'bundle-pmat' feature is enabled. Injecting internal PMAT agent.");
                
                let pmat_agent = AgentConfig {
                    id: "pmat-architect-internal".to_string(),
                    name: "PMAT Code Architect (Bundled)".to_string(),
                    agent_type: "internal".to_string(),
                    command: Some("pmat-internal".to_string()),
                    args: None,
                    extension_name: None,
                    rate_limit: RateLimit::default(),
                    capabilities: vec![
                        "context-generation".to_string(),
                        "code-analysis".to_string(),
                        "technical-debt-grading".to_string(),
                    ],
                    priority: 255,
                };
                config.agents.push(pmat_agent);
            }
        }

        Ok(config)
    }
}

/// Returns the default path for the configuration file.
fn default_config_path() -> PathBuf {
    // ~/.config/gemini-mcp-proxy/config.toml
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("gemini-mcp-proxy")
        .join("config.toml")
}

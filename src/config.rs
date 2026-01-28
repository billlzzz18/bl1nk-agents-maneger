use serde::Deserialize;
use anyhow::{Result, Context};
use std::path::PathBuf;
use std::fs;

// --- โครงสร้าง Config ทั้งหมดจะยังคงเหมือนเดิม ---

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

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RoutingConfig {
    #[serde(default)]
    pub rules: Vec<RoutingRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RoutingRule {
    pub task_type: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub preferred_agents: Vec<String>,
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


// --- ฟังก์ชันสำหรับโหลดและปรับปรุง Config ---

impl Config {
    /// Loads configuration from a given path, and injects bundled agents if the feature is enabled.
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        let path = path.unwrap_or_else(default_config_path);
        
        tracing::info!("Loading configuration from: {}", path.display());

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file at {}", path.display()))?;
        
        let mut config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse TOML config")?;

        // --- ส่วนที่สำคัญที่สุด: การฉีด (Inject) Bundled Agent ---
        // โค้ดบล็อกนี้จะถูกคอมไพล์ก็ต่อเมื่อฟีเจอร์ `bundle-pmat` ถูกเปิดใช้งาน
        #[cfg(feature = "bundle-pmat")]
        {
            tracing::info!("'bundle-pmat' feature is enabled. Injecting internal PMAT agent.");
            
            // สร้าง AgentConfig สำหรับ pmat-internal ขึ้นมาในโค้ด
            let pmat_agent = AgentConfig {
                id: "pmat-architect-internal".to_string(),
                name: "PMAT Code Architect (Bundled)".to_string(),
                agent_type: "internal".to_string(), // ระบุ type เป็น "internal"
                command: Some("pmat-internal".to_string()), // ใช้ชื่อเฉพาะเพื่อการระบุตัวตน
                args: None,
                extension_name: None,
                rate_limit: RateLimit::default(),
                capabilities: vec![
                    "context-generation".to_string(),
                    "code-analysis".to_string(),
                    "technical-debt-grading".to_string(),
                    "test-quality-validation".to_string(),
                    "code-refactoring-analysis".to_string(),
                ],
                priority: 255, // ให้ priority สูงที่สุด (0-255) เพื่อให้เป็นตัวเลือกแรกเสมอ
            };

            // เพิ่ม pmat agent เข้าไปในลิสต์ของ agents
            config.agents.push(pmat_agent);
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

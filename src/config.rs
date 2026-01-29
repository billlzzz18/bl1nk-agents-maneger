use serde::Deserialize;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;

// =============================================================================
// ส่วนที่ 1: โครงสร้างสำหรับ Policy (`policy.toml`)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Policy {
    #[serde(default)]
    pub routing: RoutingPolicy,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RoutingPolicy {
    #[serde(default)]
    pub tier: String,
    #[serde(default)]
    pub rules: Vec<RoutingRule>,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct RoutingRule {
    pub task_type: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub preferred_agents: Vec<String>,
    #[serde(default)]
    pub priority: u16,
}

// =============================================================================
// ส่วนที่ 2: โครงสร้างสำหรับ Agent Registry (`registry.json`)
// =============================================================================

#[derive(Debug, Clone, Deserialize, Default)]
pub struct AgentRegistryConfig {
    #[serde(default)]
    pub agents: Vec<AgentTechnicalConfig>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
#[serde(default)]
pub struct AgentTechnicalConfig {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub agent_type: String,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub extension_name: Option<String>,
    pub capabilities: Vec<String>, // Capabilities ยังคงอยู่ที่นี่ เพราะมันเป็นส่วนหนึ่งของ "สิ่งที่ Agent ทำได้ทางเทคนิค"
    pub priority: u8, // Priority ยังอยู่ที่นี่ เพื่อใช้เป็น Fallback เมื่อไม่มี Rule ใน Policy
    pub rate_limit: RateLimit,
}

impl Default for AgentTechnicalConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            agent_type: "cli".to_string(),
            command: None,
            args: None,
            extension_name: None,
            capabilities: Vec::new(),
            priority: 0,
            rate_limit: RateLimit::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct RateLimit {
    pub requests_per_minute: u32,
    pub requests_per_day: u32,
}

impl Default for RateLimit {
    fn default() -> Self {
        Self { requests_per_minute: 60, requests_per_day: 2000 }
    }
}


// =============================================================================
// ส่วนที่ 3: โครงสร้าง Config สุดท้ายที่โปรแกรมจะใช้งาน
// =============================================================================

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub server: ServerConfig, // การตั้งค่า Server ยังคงอยู่ที่นี่
    pub logging: LoggingConfig,
    pub policy: Policy,
    pub agents: Vec<AgentTechnicalConfig>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}
impl Default for ServerConfig { /* ... */ }

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct LoggingConfig {
    pub level: String,
}
impl Default for LoggingConfig { /* ... */ }


// =============================================================================
// ตรรกะการโหลดที่เคารพใน Separation of Concerns
// =============================================================================

impl Config {
    pub fn load() -> Result<Self> {
        // 1. โหลด Policy
        let mut policy = Policy::default();
        if let Ok(policy_path) = get_policy_path() {
            if policy_path.exists() {
                tracing::info!("Loading policy from: {}", policy_path.display());
                policy = load_toml_file::<Policy>(&policy_path).unwrap_or_default();
            }
        }
        // (ในอนาคตสามารถเพิ่มการ merge project-level policy ที่นี่ได้)

        // 2. โหลด Agent Registry
        let mut agents = AgentRegistryConfig::default();
        if let Ok(registry_path) = get_registry_path() {
            if registry_path.exists() {
                tracing::info!("Loading agent registry from: {}", registry_path.display());
                agents = load_toml_file::<AgentRegistryConfig>(&registry_path).unwrap_or_default();
            }
        }

        // 3. ฉีด Bundled Agent (ถ้ามี)
        let mut all_agents = agents.agents;
        #[cfg(feature = "bundle-pmat")]
        {
            tracing::info!("'bundle-pmat' feature is enabled. Injecting internal PMAT agent.");
            let pmat_agent = AgentTechnicalConfig {
                id: "pmat-architect-internal".to_string(),
                name: "PMAT Code Architect (Bundled)".to_string(),
                agent_type: "internal".to_string(),
                command: Some("pmat-internal".to_string()),
                priority: 255,
                capabilities: vec!["code-analysis".to_string(), "context-generation".to_string()],
                ..Default::default()
            };
            all_agents.push(pmat_agent);
        }

        // 4. ประกอบร่างเป็น Config สุดท้าย
        let final_config = Config {
            server: ServerConfig::default(), // สามารถ override จากไฟล์อื่นได้ในอนาคต
            logging: LoggingConfig::default(),
            policy,
            agents: all_agents,
        };

        Ok(final_config)
    }
}

// --- ฟังก์ชัน Helper ---

fn load_toml_file<T: for<'de> Deserialize<'de> + Default>(path: &Path) -> Result<T> {
    let content = fs::read_to_string(path)?;
    toml::from_str(&content).with_context(|| format!("Failed to parse TOML from {}", path.display()))
}

fn get_policy_path() -> Result<PathBuf> {
    Ok(dirs::config_dir().context("Cannot find config dir")?.join("gemini-mcp-proxy").join("policy.toml"))
}

fn get_registry_path() -> Result<PathBuf> {
    Ok(dirs::config_dir().context("Cannot find config dir")?.join("gemini-mcp-proxy").join("registry.toml"))
}

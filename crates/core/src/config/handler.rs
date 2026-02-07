use crate::config::{AgentConfig, Config, OhMyOpenCodeConfig, CategoryConfig}; // ใช้ Config struct หลักของเรา
use crate::agents::{AgentRegistry};
use crate::agents::create_builtin_agents;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::{Result, Context};

// Mock shared utilities until we port shared/*
mod shared {
    use super::*;
    pub fn log(msg: &str, data: Option<Value>) {
        tracing::info!(msg, ?data);
    }
    
    pub async fn fetch_available_models(_client: &Option<Client>) -> Result<HashSet<String>> {
        Ok(HashSet::new()) // Placeholder
    }
    
    pub fn read_connected_providers_cache() -> Option<Value> {
        None
    }
}

// Type aliases for cleaner code
type JsonMap = serde_json::Map<String, Value>;
type Client = reqwest::Client; // We use reqwest in Cargo.toml

// Context struct - Context ของการรัน Config
#[derive(Clone)]
pub struct ConfigContext {
    pub directory: PathBuf,
    pub client: Option<Client>,
}

// Model Cache State - เก็บ State ของโมเดล (Port มาจาก TS)
#[derive(Default, Clone)]
pub struct ModelCacheState {
    pub anthropic_context_1m_enabled: bool,
    pub model_context_limits_cache: Arc<RwLock<HashMap<String, usize>>>,
}

// Dependencies injection struct
pub struct ConfigHandlerDeps {
    pub ctx: ConfigContext,
    pub plugin_config: OhMyOpenCodeConfig, // Use the real struct
    pub model_cache_state: ModelCacheState,
}

// Main Handler Structure
pub struct ConfigHandler {
    deps: ConfigHandlerDeps,
}

impl ConfigHandler {
    pub fn new(deps: ConfigHandlerDeps) -> Self {
        Self { deps }
    }

    /// Main logic to process and merge configuration
    pub async fn handle_config(&mut self, config: &mut Value) -> Result<()> {
        let deps = &mut self.deps;
        
        // 1. Handle Providers and Context Limits
        self.handle_providers(config).await?;

        // 2. Load Plugins
        let plugin_components = if deps.plugin_config.claude_code.as_ref().and_then(|c| c.plugins).unwrap_or(true) {
            // Placeholder for load_all_plugin_components
            self.load_all_plugin_components().await?
        } else {
            PluginComponents::default()
        };

        if !plugin_components.plugins.is_empty() {
            shared::log(
                &format!("Loaded {} Claude Code plugins", plugin_components.plugins.len()),
                Some(serde_json::json!({
                    "plugins": plugin_components.plugins.iter().map(|p| format!("{}@{}", p.name, p.version)).collect::<Vec<_>>()
                }))
            );
        }

        if !plugin_components.errors.is_empty() {
            shared::log("Plugin load errors", Some(serde_json::json!({ "errors": plugin_components.errors })));
        }

        // 3. Migrate Disabled Agents
        let migrated_disabled_agents = self.migrate_disabled_agents(&deps.plugin_config.disabled_agents);

        // 4. Discovery Skills (Parallel Loading simulation)
        let all_discovered_skills = self.discover_all_skills().await?;

        // 5. Create Builtin Agents (Mocked for now)
        let browser_provider = deps.plugin_config.browser_automation_engine
            .as_ref()
            .map(|b| b.provider.clone())
            .unwrap_or_else(|| "playwright".to_string());

        let model = config.get("model").and_then(|v| v.as_str()).map(String::from);

        let builtin_agents_configs = create_builtin_agents(
            &migrated_disabled_agents,
            &deps.plugin_config.agents,
            deps.ctx.directory.to_str(),
            model.as_deref(),
            None, // CategoriesConfig not yet available here
            None, // GitMasterConfig not yet available here
        ).await?;

        // Convert Map<String, AgentConfig> to Map<String, Value> for compatibility
        let mut builtin_agents = HashMap::new();
        for (k, v) in builtin_agents_configs {
            builtin_agents.insert(k, serde_json::to_value(v)?);
        }

        // 6. Load User & Project Agents
        let (user_agents, project_agents) = if deps.plugin_config.claude_code.as_ref().and_then(|c| c.agents).unwrap_or(true) {
            (self.load_user_agents().await?, self.load_project_agents().await?)
        } else {
            (HashMap::new(), HashMap::new())
        };

        // 7. Plugin Agents Migration
        let plugin_agents = self.migrate_plugin_agents(&plugin_components.agents)?;

        // 8. Sisyphus Logic
        let is_sisyphus_enabled = deps.plugin_config.sisyphus_agent.as_ref().map(|s| !s.disabled).unwrap_or(true);
        
        // Ensure 'agent' field exists in config
        if config.get("agent").is_none() {
            config["agent"] = serde_json::json!({});
        }
        
        if is_sisyphus_enabled && builtin_agents.contains_key("orchestrator") {
            self.configure_sisyphus_mode(config, &builtin_agents, &user_agents, &project_agents, &plugin_agents).await?;
        } else {
            // Merge all agents into config.agent
            let config_agent = config["agent"].as_object_mut().unwrap();
            
            // Merge helper
            fn merge_agents(target: &mut JsonMap, source: &HashMap<String, Value>) {
                for (k, v) in source {
                    target.insert(k.clone(), v.clone());
                }
            }

            merge_agents(config_agent, &builtin_agents);
            merge_agents(config_agent, &user_agents);
            merge_agents(config_agent, &project_agents);
            merge_agents(config_agent, &plugin_agents);
        }

        // 9. Permission Overrides
        self.apply_permission_overrides(config)?;

        // 10. Load MCP Configs
        let mcp_result = if deps.plugin_config.claude_code.as_ref().and_then(|c| c.mcp).unwrap_or(true) {
            self.load_mcp_configs().await?
        } else {
            McpConfigResult { servers: HashMap::new() }
        };

        // Merge MCPs
        if config.get("mcp").is_none() {
            config["mcp"] = serde_json::json!({});
        }
        // Add logic to merge builtin mcps and plugin mcps here...

        // 11. Load Commands (Parallel Loading)
        self.load_and_merge_commands(config, &plugin_components).await?;

        Ok(())
    }

    async fn handle_providers(&self, config: &Value) -> Result<()> {
        if let Some(providers) = config.get("provider").and_then(|p| p.as_object()) {
            // Check anthropic-beta header (Simplified)
            if let Some(anthropic) = providers.get("anthropic") {
                // Logic to check headers...
            }

            // Update model cache context limits
            let mut cache = self.deps.model_cache_state.model_context_limits_cache.write().await;
            for (provider_id, provider_config) in providers {
                if let Some(models) = provider_config.get("models").and_then(|m| m.as_object()) {
                    for (model_id, model_config) in models {
                        if let Some(limit) = model_config.get("limit") {
                            if let Some(context) = limit.get("context").and_then(|c| c.as_u64()) {
                                cache.insert(format!("{}/{}", provider_id, model_id), context as usize);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // --- Sisyphus Logic Extraction ---
    async fn configure_sisyphus_mode(
        &self, 
        config: &mut Value,
        builtin_agents: &HashMap<String, Value>,
        user_agents: &HashMap<String, Value>,
        project_agents: &HashMap<String, Value>,
        plugin_agents: &HashMap<String, Value>
    ) -> Result<()> {
        let deps = &self.deps;
        
        // Set default agent
        config["default_agent"] = Value::String("orchestrator".to_string());

        let mut final_agent_config = serde_json::Map::new();

        // Add Orchestrator
        if let Some(orchestrator) = builtin_agents.get("orchestrator") {
            final_agent_config.insert("orchestrator".to_string(), orchestrator.clone());
        }

        // Handle Builder Logic... (Placeholder)
        
        // Merge other agents (filtering out orchestrator to avoid dupes)
        for (k, v) in builtin_agents {
            if k != "orchestrator" { final_agent_config.insert(k.clone(), v.clone()); }
        }
        for (k, v) in user_agents { final_agent_config.insert(k.clone(), v.clone()); }
        for (k, v) in project_agents { final_agent_config.insert(k.clone(), v.clone()); }
        for (k, v) in plugin_agents { final_agent_config.insert(k.clone(), v.clone()); }

        // Update config
        config["agent"] = Value::Object(final_agent_config);

        Ok(())
    }

    fn apply_permission_overrides(&self, config: &mut Value) -> Result<()> {
        // Disable specific tools globally
        if let Some(tools) = config.get_mut("tools").and_then(|t| t.as_object_mut()) {
            tools.insert("grep_app_*".to_string(), Value::Bool(false));
            tools.insert("LspHover".to_string(), Value::Bool(false));
            tools.insert("LspCodeActions".to_string(), Value::Bool(false));
            tools.insert("LspCodeActionResolve".to_string(), Value::Bool(false));
        }

        // Helper to update agent permissions
        let mut update_agent_perm = |agent_name: &str, updates: Vec<(&str, &str)>| {
            if let Some(agent_config) = config.get_mut("agent")
                .and_then(|a| a.get_mut(agent_name))
                .and_then(|a| a.as_object_mut()) 
            {
                if !agent_config.contains_key("permission") {
                    agent_config.insert("permission".to_string(), serde_json::json!({}));
                }
                if let Some(perms) = agent_config.get_mut("permission").and_then(|p| p.as_object_mut()) {
                    for (k, v) in updates {
                        perms.insert(k.to_string(), Value::String(v.to_string()));
                    }
                }
            }
        };

        update_agent_perm("researcher", vec![("grep_app_*", "allow")]);
        update_agent_perm("observer", vec![("task", "deny"), ("look_at", "deny")]);
        update_agent_perm("manager", vec![("task", "deny"), ("call_omo_agent", "deny"), ("delegate_task", "allow")]);
        update_agent_perm("orchestrator", vec![("call_omo_agent", "deny"), ("delegate_task", "allow"), ("question", "allow")]);
        update_agent_perm("planner", vec![("call_omo_agent", "deny"), ("delegate_task", "allow"), ("question", "allow")]);
        update_agent_perm("orchestrator-junior", vec![("delegate_task", "allow")]);

        // Global permissions
        if config.get("permission").is_none() {
            config["permission"] = serde_json::json!({});
        }
        if let Some(global_perms) = config.get_mut("permission").and_then(|p| p.as_object_mut()) {
            global_perms.insert("webfetch".to_string(), Value::String("allow".to_string()));
            global_perms.insert("external_directory".to_string(), Value::String("allow".to_string()));
            global_perms.insert("delegate_task".to_string(), Value::String("deny".to_string()));
        }

        Ok(())
    }

    // --- Mock / Placeholder Methods for Loading ---
    
    async fn load_all_plugin_components(&self) -> Result<PluginComponents> {
        Ok(PluginComponents::default())
    }

    fn migrate_disabled_agents(&self, disabled: &Option<Vec<String>>) -> Vec<String> {
        disabled.as_ref().map(|list| {
            list.iter().map(|agent| {
                agent.clone() 
            }).collect()
        }).unwrap_or_default()
    }

    async fn discover_all_skills(&self) -> Result<Vec<Value>> {
        Ok(vec![])
    }

    // Placeholder for createBuiltinAgents
    async fn create_builtin_agents_mock(
        &self,
        _disabled: &[String],
        _agents: &HashMap<String, Value>,
        _model: Option<&str>,
        _skills: &[Value],
        _browser: &str
    ) -> Result<HashMap<String, Value>> {
        Ok(HashMap::new())
    }

    async fn load_user_agents(&self) -> Result<HashMap<String, Value>> { Ok(HashMap::new()) }
    async fn load_project_agents(&self) -> Result<HashMap<String, Value>> { Ok(HashMap::new()) }
    
    fn migrate_plugin_agents(&self, agents: &HashMap<String, Value>) -> Result<HashMap<String, Value>> {
        Ok(agents.clone())
    }

    async fn load_mcp_configs(&self) -> Result<McpConfigResult> {
        Ok(McpConfigResult { servers: HashMap::new() })
    }

    async fn load_and_merge_commands(&self, _config: &mut Value, _plugins: &PluginComponents) -> Result<()> {
        Ok(())
    }
}

// --- Data Structures ---

#[derive(Default)]
struct PluginComponents {
    commands: HashMap<String, Value>,
    skills: HashMap<String, Value>,
    agents: HashMap<String, Value>,
    mcpServers: HashMap<String, Value>, // Corrected camelCase mismatch
    plugins: Vec<PluginInfo>,
    errors: Vec<String>,
}

struct PluginInfo {
    name: String,
    version: String,
}

struct McpConfigResult {
    servers: HashMap<String, Value>,
}

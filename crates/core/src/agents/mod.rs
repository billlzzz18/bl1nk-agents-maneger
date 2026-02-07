pub mod register;
pub mod router;
pub mod extractor;
pub mod creator;
pub mod types;
pub mod utils;
pub mod prompt_builder;
pub mod orchestrator;
pub mod expert;
pub mod researcher;
pub mod explorer;
pub mod observer;
pub mod consultant;
pub mod auditor;
pub mod manager;
pub mod planner;
pub mod orchestrator_junior;

pub use register::AgentRegistry;
pub use router::AgentRouter;
pub use extractor::AgentExecutor;
pub use creator::AgentCreator;
pub use types::{AgentConfig, AgentName, BuiltinAgentName, AgentOverrideConfig, AgentOverrides,
    AgentPromptMetadata, AgentCategory, AgentCost, DelegationTrigger, RateLimit, ThinkingConfig,
    PartialAgentConfig, is_gpt_model};
pub use utils::{create_builtin_agents, build_agent, create_env_context};
pub use prompt_builder::{
    AvailableAgent, AvailableCategory, AvailableSkill, AvailableTool, ToolCategory, SkillLocation,
    categorize_tools, build_key_triggers_section, build_tool_selection_table, build_explorer_section,
    build_researcher_section, build_delegation_table, build_category_skills_delegation_guide,
    build_expert_section, build_hard_blocks_section, build_anti_patterns_section,
    build_ultrawork_section
};
pub use orchestrator::{create_orchestrator_agent};
pub use expert::{create_expert_agent, EXPERT_PROMPT_METADATA};
pub use researcher::{create_researcher_agent, RESEARCHER_PROMPT_METADATA};
pub use explorer::{create_explorer_agent, EXPLORER_PROMPT_METADATA};
pub use observer::{create_observer_agent, OBSERVER_PROMPT_METADATA};
pub use consultant::{create_consultant_agent, CONSULTANT_PROMPT_METADATA};
pub use auditor::{create_auditor_agent, AUDITOR_PROMPT_METADATA};
pub use manager::{create_manager_agent, MANAGER_PROMPT_METADATA};
pub use planner::{create_planner_agent, PLANNER_PROMPT_METADATA};
pub use orchestrator_junior::{create_orchestrator_junior_agent};
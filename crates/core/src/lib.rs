use serde::{Deserialize, Serialize};
use anyhow::Result;

pub mod config;
pub mod agents;
pub mod mcp;
pub mod rate_limit;
pub mod hooks;
pub mod adapters;
pub mod cli;
pub mod filesystem;
pub mod projects;
pub mod search;
pub mod session;
pub mod events;
pub mod rpc;

// --- Backend Integration (Aligning with Desktop Server) ---

pub mod backend {
    use super::*;
    use crate::filesystem::{self, GitInfo, FileContent, DirEntry};
    use crate::projects::{self, EnrichedProject};
    use crate::search::{self, RecentChat, DetailedConversation, SearchFilters, SearchResult};
    use crate::session::{self as core_session, ProcessStatus};

    pub use crate::events::EventEmitter;

    pub struct GeminiBackend<E: EventEmitter> {
        pub emitter: E,
    }

    impl<E: EventEmitter> GeminiBackend<E> {
        pub fn new(emitter: E) -> Self {
            Self { emitter }
        }

        // Real methods implemented using core modules
        pub async fn list_projects(&self, _limit: u32, _offset: u32) -> Result<Value> {
            let projects = projects::list_projects().await?;
            Ok(serde_json::to_value(projects)?)
        }

        pub async fn list_enriched_projects(&self) -> Result<Vec<EnrichedProject>> {
            projects::list_enriched_projects().await
        }

        pub async fn get_enriched_project(&self, sha: String, path: String) -> Result<EnrichedProject> {
            projects::get_enriched_project(sha, path).await
        }

        pub async fn get_project_discussions(&self, id: &str) -> Result<Vec<RecentChat>> {
            search::get_project_discussions(id).await
        }

        pub async fn get_recent_chats(&self) -> Result<Vec<RecentChat>> {
            search::get_recent_chats().await
        }

        pub async fn search_chats(&self, q: String, f: Option<SearchFilters>) -> Result<Vec<SearchResult>> {
            search::search_chats(q, f).await
        }

        pub async fn get_detailed_conversation(&self, id: &str) -> Result<DetailedConversation> {
            search::get_detailed_conversation(id).await
        }

        pub async fn export_conversation_history(&self, id: &str, format: &str) -> Result<String> {
            search::export_conversation_history(id, format).await
        }

        pub async fn check_cli_installed(&self) -> Result<bool> {
            // Simple check if our own process can run or search? 
            // For now return true as we are running.
            Ok(true)
        }

        pub async fn initialize_session(&self, sid: String, dir: String, model: String, bc: Option<crate::session::QwenConfig>, ga: Option<crate::session::GeminiAuthConfig>, lc: Option<crate::session::LLxprtConfig>) -> Result<()> {
            core_session::SessionManager::get_instance().await.create_session(sid, dir, model, bc, ga, lc).await
        }

        pub async fn send_message(&self, sid: String, msg: String, history: String) -> Result<()> {
            core_session::SessionManager::get_instance().await.send_message(sid, msg, history).await
        }

        pub async fn get_process_statuses(&self) -> Result<Vec<ProcessStatus>> {
            core_session::SessionManager::get_instance().await.get_all_statuses().await
        }

        pub async fn kill_process(&self, id: &str) -> Result<()> {
            core_session::SessionManager::get_instance().await.kill_session(id.to_string()).await
        }

        pub async fn handle_tool_confirmation(&self, sid: String, rid: u32, tid: String, output: String) -> Result<()> {
             core_session::SessionManager::get_instance().await.handle_tool_confirmation(sid, rid, tid, output).await
        }

        pub async fn execute_confirmed_command(&self, cmd: String) -> Result<String> {
            // This might need a separate execution module or use session manager
            Ok("Command execution not implemented in backend yet".into())
        }

        pub async fn generate_conversation_title(&self, msg: String, model: Option<String>) -> Result<String> {
            search::generate_conversation_title(msg, model).await
        }

        pub async fn validate_directory(&self, p: String) -> Result<bool> {
            filesystem::validate_directory(p).await
        }

        pub async fn is_home_directory(&self, p: String) -> Result<bool> {
            filesystem::is_home_directory(p).await
        }

        pub async fn get_home_directory(&self) -> Result<String> {
            filesystem::get_home_directory().await
        }

        pub async fn get_parent_directory(&self, p: String) -> Result<Option<String>> {
            filesystem::get_parent_directory(p).await
        }

        pub async fn list_directory_contents(&self, p: String) -> Result<Vec<DirEntry>> {
            filesystem::list_directory_contents(p).await
        }

        pub async fn list_files_recursive(&self, p: String) -> Result<Vec<DirEntry>> {
            filesystem::list_files_recursive(p).await
        }

        pub async fn list_volumes(&self) -> Result<Vec<DirEntry>> {
            filesystem::list_volumes().await
        }

        pub async fn get_git_info(&self, p: String) -> Result<Option<GitInfo>> {
            filesystem::get_git_info(p).await
        }

        pub async fn read_file_content(&self, p: String) -> Result<FileContent> {
            filesystem::read_file_content(p).await
        }

        pub async fn read_binary_file_as_base64(&self, p: String) -> Result<String> {
            filesystem::read_binary_file_as_base64(p).await
        }

        pub async fn get_canonical_path(&self, p: String) -> Result<String> {
            filesystem::get_canonical_path(p).await
        }

        pub async fn read_file_content_with_options(&self, p: String, follow_symlinks: bool) -> Result<FileContent> {
            filesystem::read_file_content_with_options(p, follow_symlinks).await
        }

        pub async fn write_file_content(&self, p: String, content: String) -> Result<FileContent> {
            filesystem::write_file_content(p, content).await
        }
    }

    pub mod session {
        use super::*;
        #[derive(Debug, Clone, Serialize, Deserialize)] pub struct QwenConfig;
        #[derive(Debug, Clone, Serialize, Deserialize)] pub struct GeminiAuthConfig;
        #[derive(Debug, Clone, Serialize, Deserialize)] pub struct LLxprtConfig;
    }
}

// Re-export commonly used types
pub use config::Config;
pub use mcp::Orchestrator;
pub use agents::{AgentRegistry, AgentExecutor};
use serde_json::Value;
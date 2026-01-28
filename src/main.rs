mod config;
mod mcp;
mod agents;
mod rate_limit;

use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    tracing::info!("Starting Gemini MCP/ACP Orchestrator");

    // Load configuration
    let config = config::Config::load_default()?;
    tracing::info!("Loaded configuration with {} agents", config.agents.len());

    // Initialize the orchestrator
    let orchestrator = mcp::Orchestrator::new(config).await?;

    // Run the MCP server (listens for requests from Gemini CLI)
    tracing::info!("Starting MCP server on stdio");
    orchestrator.run_stdio().await?;

    Ok(())
}
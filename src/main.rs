mod config;
mod mcp;
mod agents;
mod rate_limit;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

/// Gemini MCP/ACP Orchestrator: A dual-mode proxy for multi-agent systems.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the configuration file.
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// (ใหม่) Run as an HTTP server for A2A protocol instead of stdio.
    #[arg(long)]
    http: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info"))
        )
        .init();

    tracing::info!("Starting Gemini MCP/ACP Orchestrator...");

    // Parse command-line arguments
    let args = Args::parse();

    // Load configuration using the path from args, or None for default path
    let config = config::Config::load(args.config)?;

    // Log a confirmation message
    #[cfg(feature = "bundle-pmat")]
    tracing::info!(
        "Loaded configuration with {} agents (including bundled PMAT).",
        config.agents.len()
    );
    #[cfg(not(feature = "bundle-pmat"))]
    tracing::info!(
        "Loaded configuration with {} agents. (To include bundled PMAT, compile with --features bundle-pmat)",
        config.agents.len()
    );

    // Initialize the orchestrator
    let orchestrator = mcp::Orchestrator::new(config).await?;

    // --- ส่วนที่แก้ไข: เลือกว่าจะรันโหมดไหนตาม Flag ---
    if args.http {
        // รันในโหมด HTTP Server (สำหรับ A2A)
        tracing::info!("Starting in HTTP Server mode for A2A protocol...");
        orchestrator.run_http().await?;
    } else {
        // รันในโหมด Stdio (แบบเดิม, สำหรับ MCP)
        tracing::info!("Starting in Stdio mode for MCP protocol...");
        orchestrator.run_stdio().await?;
    }

    tracing::info!("Orchestrator shut down gracefully.");

    Ok(())
}

# Gemini MCP Proxy

**Dual-mode MCP/ACP orchestrator for multi-agent systems, with an intelligent routing engine and optional bundled PMAT.**

## Overview

This project implements an **Agent Community Protocol (ACP)** orchestrator that:

-   **Acts as MCP Server** - Exposes tools to Gemini CLI via Model Context Protocol.
-   **Acts as ACP Client** - Communicates with sub-agents using Agent Client Protocol.
-   **Routes Intelligently** - Selects the optimal agent using a powerful, tiered priority-based policy engine.
-   **Manages Rate Limits** - Tracks usage per agent with configurable limits.
-   **Runs Background Tasks** - Delegates long-running tasks asynchronously.
-   **Includes Bundled PMAT (Optional)** - Provides zero-configuration, high-performance code analysis out-of-the-box.

## Architecture

```
┌──────────────────────────────────────────────┐
│  Gemini CLI (Main Agent)                     │
│  - MCP Client/Server                         │
└────────────┬─────────────────────────────────┘
             │ MCP Protocol (JSON-RPC 2.0)
             ↓
┌──────────────────────────────────────────────┐
│  Gemini MCP Proxy (This Project)             │
│  ┌────────────────────────────────────────┐  │
│  │ MCP Server Interface                   │  │
│  │  - delegate_task                       │  │
│  │  - agent_status                        │  │
│  └────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────┐  │
│  │ Intelligent Routing Engine             │  │
│  │  - Tiered Priority System              │  │
│  └────────────────────────────────────────┘  │
│  ┌────────────────────────────────────────┐  │
│  │ ACP Client / Internal Agent Executor   │  │
│  │  - Spawn external agents via CLI       │  │
│  │  - Call bundled agents (PMAT) directly │  │
│  └────────────────────────────────────────┘  │
└────────────┬─────────────────────────────────┘
             │ ACP Protocol (JSON-RPC 2.0)
             ↓
┌──────────────────────────────────────────────┐
│  Sub-Agents (Each is MCP/ACP Server)        │
│  - qwencode, codex, jules, pmat, etc.       │
└──────────────────────────────────────────────┘
```

## Key Concepts

### Intelligent Routing Engine

Inspired by the Gemini CLI's policy engine, our router uses a sophisticated priority system to select the best agent for any given task.

-   **Tiered Rules**: Rules can be organized into `admin`, `user`, and `default` tiers, ensuring that more critical rules are always evaluated first.
-   **Rule Priority**: Within each tier, rules are assigned a priority from 0-999. The rule with the highest priority wins.
-   **Ordered Fallback**: Each rule can specify an ordered list of `preferred_agents`, creating a fallback chain (e.g., "try Agent A, if not available, try Agent B").
-   **Agent Priority Fallback**: If no rule finds an available agent, the system falls back to selecting the highest-priority available agent.

### Bundled PMAT Agent

This project can be compiled with an optional `bundle-pmat` feature flag. When enabled, it embeds the powerful **PMAT** code analysis engine directly into the binary.

-   **Zero-Configuration**: Provides instant access to code analysis, context generation, and technical debt grading without needing to install a separate `pmat` CLI.
-   **High Performance**: Executes as a direct library call, eliminating process spawning overhead for maximum speed.
-   **Seamless Integration**: The bundled agent is automatically registered and can be targeted by the routing engine just like any external agent.

## Installation

### Prerequisites

```bash
# Install Rust (1.82+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update

# Clone repository
git clone <your-repo-url>
cd gemini-mcp-proxy
```

### Build Options

Choose the version that best fits your needs.

**1. Standard Build (Lightweight)**

Compiles quickly and results in a smaller binary. Requires external agents (like `pmat` CLI) to be installed on the system.

```bash
cargo build --release
```

**2. Bundled PMAT Build (Recommended)**

Includes the PMAT code analysis engine directly in the binary for zero-configuration, high-performance analysis.

```bash
cargo build --release --features bundle-pmat
```

**3. Bundled PMAT Full Build**

Includes the full version of PMAT with support for all 17+ programming languages.

```bash
cargo build --release --features bundle-pmat-full
```

## Configuration

Create `~/.config/gemini-mcp-proxy/config.toml` with your agent definitions and routing rules.

```toml
# ~/.config/gemini-mcp-proxy/config.toml

[server]
host = "127.0.0.1"
port = 3000

# --- Agent Definitions ---
# The bundled PMAT agent is added automatically if the feature is enabled.
# You only need to define your external agents here.
[[agents]]
id = "qwen-coder"
name = "Qwen Code Assistant"
type = "cli"
command = "qwencode"
args = ["--mode", "agent"]
capabilities = ["code-generation", "refactoring"]
priority = 150 # Agent's own priority, used as a fallback

[[agents]]
id = "jules-agent"
name = "Jules Extension"
type = "gemini-extension"
extension_name = "jules"
capabilities = ["research", "analysis"]
priority = 120

# --- Intelligent Routing Engine Configuration ---
[routing]
# Set the tier for this config file ('user', 'admin', or 'default')
tier = "user"

# Rule for all code analysis tasks
[[routing.rules]]
task_type = "code-analysis"
keywords = ["analyze", "review", "grade", "understand"]
# Always prefer the high-performance bundled PMAT agent first.
preferred_agents = ["pmat-architect-internal", "jules-agent"]
priority = 900 # High priority to ensure this rule is matched first

# Rule for Rust-specific code generation
[[routing.rules]]
task_type = "code-generation"
keywords = ["rust", "cargo", "macro"]
preferred_agents = ["qwen-coder", "pmat-architect-internal"]
priority = 250 # Higher priority than the generic rule below

# Generic rule for code generation
[[routing.rules]]
task_type = "code-generation"
keywords = ["write code", "implement"]
preferred_agents = ["qwen-coder"]
priority = 200

# --- Other Settings ---
[rate_limiting]
track_usage = true
usage_db_path = "~/.config/gemini-mcp-proxy/usage.db"

[logging]
level = "info" # Use "debug" or "trace" for more details
```

## Usage

### Run as MCP Server

```bash
# Run with default config (~/.config/gemini-mcp-proxy/config.toml)
# Use the binary that matches your build choice (standard or bundled)
./target/release/gemini-mcp-proxy

# With a custom config path
./target/release/gemini-mcp-proxy --config /path/to/your/config.toml

# Enable detailed logging for debugging the routing engine
RUST_LOG=trace ./target/release/gemini-mcp-proxy
```

### Integrate with Gemini CLI

Add to your Gemini CLI's MCP configuration:

```json
{
  "mcpServers": {
    "gemini-proxy": {
      "command": "/path/to/your/target/release/gemini-mcp-proxy",
      "args": [],
      "transport": "stdio"
    }
  }
}
```

### Available Tools

The proxy exposes two primary tools to the Gemini CLI:

#### 1. `delegate_task`

Delegates a task to the most appropriate sub-agent based on the routing rules.

**Arguments:**
```json
{
  "task_type": "code-analysis",
  "prompt": "Review this project for technical debt and suggest improvements.",
  "agent_id": "pmat-architect-internal",  // Optional: bypass routing and force a specific agent
  "background": false,                   // Optional
  "context": {}                          // Optional
}
```

**Returns:**
```json
{
  "task_id": "uuid-here",
  "agent_id": "pmat-architect-internal",
  "status": "completed",
  "result": "..."
}
```

#### 2. `agent_status`

Queries the status of agents and tasks.

**Arguments:**
```json
{
  "task_id": "uuid-here"  // Optional
}
```

**Returns:**
```json
{
  "active_tasks": 3,
  "available_agents": ["pmat-architect-internal", "qwen-coder", "jules-agent"],
  "task_info": { ... }
}
```

## Example Workflows

### 1. Intelligent Code Analysis

```
Gemini CLI → delegate_task(
  task_type: "code-analysis",
  prompt: "Grade the technical debt in this codebase."
)
↓
Proxy's Routing Engine evaluates rules.
↓
Finds a high-priority (900) rule for "code-analysis".
↓
Selects "pmat-architect-internal" from the preferred list.
↓
Executes PMAT as a direct, in-process library call.
↓
Returns a detailed analysis report to Gemini.
```

### 2. Context-Specific Code Generation

```
Gemini CLI → delegate_task(
  task_type: "code-generation",
  prompt: "Write a new Rust macro for me."
)
↓
Proxy's Routing Engine evaluates rules.
↓
Matches the "rust" keyword in a rule with priority 250.
↓
Selects "qwen-coder" as the first choice.
↓
Spawns: qwencode --mode agent
↓
Sends ACP request via stdio and returns the result.
```

## Development

### Project Structure

```
src/
├── main.rs              # Entry point, CLI parsing
├── config.rs            # TOML config loading and injection logic
├── mcp/
│   └── mod.rs           # MCP server implementation (PMCP)
├── agents/
│   ├── mod.rs
│   ├── registry.rs      # Agent management
│   ├── router.rs        # The Intelligent Routing Engine
│   └── executor.rs      # Internal/External agent execution logic
└── rate_limit.rs        # Rate limiting tracker
```

### Testing

```bash
# Run all tests
cargo test

# Run tests for a specific module
cargo test --package gemini-mcp-proxy -- --test-threads=1 agents::router

# Run tests with detailed logging
RUST_LOG=trace cargo test -- --nocapture
```

## Troubleshooting

### Agent not spawning

Check:
1.  **Command in PATH**: `which qwencode`
2.  **Agent accepts stdio**: `echo '{"jsonrpc":"2.0","id":1}' | qwencode`
3.  **Logs**: `RUST_LOG=debug ./target/release/gemini-mcp-proxy`

### Routing issues

-   Run with `RUST_LOG=trace` to see detailed logs from the routing engine, including which rules were matched and why an agent was selected.
-   Verify your `task_type` and `keywords` in `config.toml` match the `delegate_task` arguments.

### Rate limit errors

Check current usage with the `agent_status` tool. To reset limits during development:
```bash
rm ~/.config/gemini-mcp-proxy/usage.db
```

## Contributing

1.  Fork the repository
2.  Create a feature branch (`git checkout -b feature/my-new-feature`)
3.  Make your changes
4.  Run tests: `cargo test`
5.  Submit a Pull Request

## License

MIT

## Credits

-   **PMAT** - The bundled code analysis engine.
-   **PMCP** - High-performance Rust MCP SDK.
-   **ACP** - The agent-to-agent communication protocol concept.
-   **MCP** - The Model Context Protocol standard.

---

**Built with 🦀 Rust for maximum performance, safety, and flexibility.**
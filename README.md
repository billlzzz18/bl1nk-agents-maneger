# Gemini MCP Proxy

**Dual-mode MCP/ACP orchestrator for multi-agent systems with Gemini CLI**

## Overview

This project implements an **Agent Community Protocol (ACP)** orchestrator that:

- **Acts as MCP Server** - Exposes tools to Gemini CLI via Model Context Protocol
- **Acts as ACP Client** - Communicates with sub-agents using Agent Client Protocol
- **Routes intelligently** - Selects optimal agents based on task type and capabilities
- **Manages rate limits** - Tracks usage per agent (2000/day, 60/min)
- **Runs background tasks** - Delegates long-running tasks asynchronously

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
│  │ ACP Client Interface                   │  │
│  │  - Spawn sub-agents via CLI            │  │
│  │  - JSON-RPC 2.0 over stdio             │  │
│  └────────────────────────────────────────┘  │
└────────────┬─────────────────────────────────┘
             │ ACP Protocol (JSON-RPC 2.0)
             ↓
┌──────────────────────────────────────────────┐
│  Sub-Agents (Each is MCP/ACP Server)        │
│  - qwencode, codex, jules, etc.             │
│  - Can call back to orchestrator            │
└──────────────────────────────────────────────┘
```

## Key Concepts

### ACP (Agent Community Protocol)

> **"Every Agent is an MCP Server that can call and be called by peers"**

- All agents communicate via JSON-RPC 2.0
- Bidirectional communication (agents can call each other)
- No central API keys - uses session tokens
- Rate limits are transparent (sent in responses)

### Technologies Used

1. **PMCP (Pragmatic MCP)** - High-performance Rust MCP implementation
   - TypedTool for type-safe tool definitions
   - Multiple transports (stdio, HTTP, WebSocket)
   - 16x faster than TypeScript SDK

2. **ACP (Agent Client Protocol)** - Agent-to-agent communication
   - JSON-RPC 2.0 over stdio
   - Supports AsyncRead/AsyncWrite
   - Session-based authentication

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

### Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release
```

### Configuration

Create `~/.config/gemini-mcp-proxy/config.toml`:

```toml
[server]
host = "127.0.0.1"
port = 3000
max_concurrent_tasks = 5

[main_agent]
name = "gemini"
type = "gemini-cli"

# Define your sub-agents
[[agents]]
id = "qwen-coder"
name = "Qwen Code Assistant"
type = "cli"
command = "qwencode"
args = ["--mode", "agent"]
rate_limit = { requests_per_minute = 60, requests_per_day = 2000 }
capabilities = ["code-generation", "refactoring"]
priority = 1

[[agents]]
id = "codex"
name = "Codex"
type = "cli"
command = "codex"
rate_limit = { requests_per_minute = 60, requests_per_day = 2000 }
capabilities = ["code-completion"]
priority = 2

# Routing rules
[[routing.rules]]
task_type = "code-generation"
keywords = ["write code", "implement"]
preferred_agents = ["qwen-coder"]

[[routing.rules]]
task_type = "background-task"
keywords = ["npm install", "build"]
preferred_agents = ["codex"]

[rate_limiting]
strategy = "round-robin"
track_usage = true
usage_db_path = "~/.config/gemini-mcp-proxy/usage.db"

[logging]
level = "info"
output = "stdout"
```

## Usage

### Run as MCP Server

```bash
# Run with default config
cargo run --release

# With custom config
cargo run --release -- --config /path/to/config.toml

# Enable debug logging
RUST_LOG=debug cargo run --release
```

### Integrate with Gemini CLI

Add to Gemini CLI's MCP config:

```json
{
  "mcpServers": {
    "gemini-proxy": {
      "command": "/path/to/gemini-mcp-proxy",
      "args": [],
      "transport": "stdio"
    }
  }
}
```

### Available Tools

#### 1. `delegate_task`

Delegate a task to an appropriate sub-agent.

**Arguments:**
```json
{
  "task_type": "code-generation",
  "prompt": "Write a function to calculate fibonacci",
  "agent_id": "qwen-coder",  // Optional
  "background": false,        // Optional
  "context": {}               // Optional
}
```

**Returns:**
```json
{
  "task_id": "uuid-here",
  "agent_id": "qwen-coder",
  "status": "completed",
  "result": "..."
}
```

#### 2. `agent_status`

Query agent and task status.

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
  "available_agents": ["qwen-coder", "codex"],
  "task_info": { ... }
}
```

## Example Workflows

### 1. Delegate Code Generation

```
Gemini CLI → delegate_task(
  task_type: "code-generation",
  prompt: "Create a REST API handler"
)
↓
Proxy selects qwen-coder (via routing rules)
↓
Spawns: qwencode --mode agent
↓
Sends ACP request via stdio
↓
Returns result to Gemini
```

### 2. Background Task

```
Gemini CLI → delegate_task(
  task_type: "background-task",
  prompt: "npm install",
  background: true
)
↓
Proxy spawns task in background
↓
Returns immediately with task_id
↓
Task runs asynchronously
```

## Development

### Project Structure

```
src/
├── main.rs              # Entry point
├── config.rs            # TOML config loading
├── mcp/
│   └── mod.rs          # MCP server (PMCP)
├── agents/
│   ├── mod.rs
│   ├── registry.rs     # Agent management
│   ├── router.rs       # Task routing
│   └── executor.rs     # ACP execution
└── rate_limit.rs       # Rate limiting
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_agent_registry

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### Adding New Agents

1. Add to `config.toml`:
```toml
[[agents]]
id = "my-new-agent"
name = "My Agent"
type = "cli"
command = "my-agent-cli"
capabilities = ["my-capability"]
priority = 1
rate_limit = { requests_per_minute = 60, requests_per_day = 2000 }
```

2. Add routing rule (optional):
```toml
[[routing.rules]]
task_type = "my-task"
keywords = ["my keyword"]
preferred_agents = ["my-new-agent"]
```

3. Restart proxy

## Rate Limiting

Each agent has configurable rate limits:

- **Per-minute**: 60 requests (default)
- **Per-day**: 2000 requests (default)

Limits are tracked automatically and enforced before task execution.

## Troubleshooting

### Agent not spawning

Check:
1. Command exists in PATH: `which qwencode`
2. Agent accepts stdio: `echo '{"jsonrpc":"2.0","id":1}' | qwencode`
3. Logs: `RUST_LOG=debug cargo run`

### Rate limit errors

Check usage:
```rust
// Use agent_status tool
{
  "task_id": null
}
```

Reset limits (development only):
```bash
rm ~/.config/gemini-mcp-proxy/usage.db
```

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Run tests: `cargo test`
5. Submit PR

## License

MIT

## Credits

- **PMCP** - <https://github.com/paiml/rust-mcp-sdk>
- **ACP** - Agent Client Protocol concept
- **MCP** - <https://modelcontextprotocol.io>

---

**Built with 🦀 Rust for maximum performance and safety**
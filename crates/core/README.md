# bl1nk-core

The core library for Bl1nk Agents Manager, providing the foundational logic for agent orchestration, hooks system, configuration, and protocol handling.

## 🏗️ Architecture

`bl1nk-core` is modularly designed to handle different aspects of agentic operations:

### ⚙️ Core Logic

- **`config`**: Centralized configuration management using TOML and environment overrides.
- **`agents`**: Manages agent personas (`AgentRegistry`), prompt generation, routing, and execution. Contains 16 agent modules.
- **`mcp`**: Implementation of the **Model Context Protocol**, featuring the `Orchestrator` for agent delegation.
- **`session`**: Handles persistent backend sessions, model provider setup, and process lifecycles.

### 🛠️ Utilities & Services

- **`filesystem`**: Cross-platform file operations with `.gitignore` support and path normalization.
- **`projects`**: Metadata management and organization of user workspace projects.
- **`search`**: Advanced conversation history management, full-text search, and summary generation.
- **`adapters`**: Protocol bridges for external integrations.
- **`events`**: Event emitter system for reactive programming patterns.
- **`rpc`**: Remote procedure call handling.

### 🔗 Integration

- **`hooks`**: Flexible event-driven system with 35+ hooks for intercepting tool calls and agent activities.
- **`rate_limit`**: Token-based rate limiting to prevent API overuse.

## 📦 Key Modules

### Agent System (16 modules)

| Module | Purpose |
|--------|---------|
| `register.rs` | Agent registry and management |
| `router.rs` | Intelligent agent routing |
| `executor.rs` | Task execution |
| `creator.rs` | Agent creation |
| `orchestrator.rs` | Multi-agent coordination |
| `expert.rs` | Expert agent patterns |
| `researcher.rs` | Research agent |
| `explorer.rs` | Code exploration |
| `observer.rs` | Observation patterns |
| `consultant.rs` | Consultation |
| `auditor.rs` | Auditing |
| `manager.rs` | Management |
| `planner.rs` | Planning |
| `orchestrator_junior.rs` | Junior orchestrator |
| `types.rs` | Type definitions |
| `utils.rs` | Utilities |

### Hook System (35+ hooks)

| Category | Hooks |
|----------|-------|
| Context Injection | `directory_agents_injector`, `directory_readme_injector`, `compaction_context_injector`, `rules_injector`, `ralph_loop` |
| Monitoring | `context_window_monitor`, `session_recovery`, `empty_task_response_detector`, `anthropic_context_window_limit_recovery` |
| Task Management | `todo_continuation_enforcer`, `category_skill_reminder`, `task_resume_info`, `start_work` |
| Development | `comment_checker`, `tool_output_truncator`, `thinking_block_validator`, `question_label_truncator` |
| Recovery | `edit_error_recovery`, `session_recovery`, `ralph_loop` |
| Automation | `auto_update_checker`, `auto_slash_command`, `background_notification`, `agent_usage_reminder` |

## 🚀 Features

- **Async-First**: Built on top of `tokio` for high-performance concurrent operations.
- **Type Safety**: Leverages Rust's strong type system for protocol integrity.
- **Modular Design**: Easy to extend with new agents and hooks.
- **MCP Integration**: Full Model Context Protocol support.

## 📦 Dependencies

```toml
[dependencies]
pmcp = { version = "1.8", features = ["schema-generation"] }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "1.0"
anyhow = "1.0"
thiserror = "1.0"
toml = "0.8"
chrono = "0.4"
regex = "1.0"
ignore = "0.4"
tracing = "0.1"
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
serde_yaml = "0.9"
lazy_static = "1.4"
```

## 🛠️ Usage

### Basic Setup

```rust
use bl1nk_core::{Config, Orchestrator};

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load("config.toml").await?;
    let orchestrator = Orchestrator::new(config).await;
    orchestrator.run_stdio().await?;
    Ok(())
}
```

### Using the Agent Registry

```rust
use bl1nk_core::agents::AgentRegistry;

let registry = AgentRegistry::new(agents_config);
let agents = registry.list_agents().await;
let selected = registry.select_agent("code-generation").await;
```

### Using Hooks

```rust
use bl1nk_core::hooks::{create_todo_continuation_enforcer, TodoContinuationEnforcer};

let hook = create_todo_continuation_enforcer();
hook.process(&context).await?;
```

## 🧪 Testing

```bash
# Run all tests
cargo test --all-features

# Run with coverage
cargo test --all-features -- --include-ignored

# Run specific test
cargo test --package bl1nk-core
```

## 📚 Documentation

- [Full Documentation](../docs/)
- [Architecture](../docs/ARCHITECTURE.md)
- [Agent Guide](../docs/AGENT_GUIDE.md)
- [API Reference](../API.md)

## 🤝 Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for guidelines on contributing to bl1nk-core.

## 📄 License

MIT License - see [LICENSE](../../LICENSE) for details.

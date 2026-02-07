# Architecture Documentation

## System Overview

Bl1nk Agents Manager is a **multi-agent orchestration system** built with Rust that provides:

- **MCP (Model Context Protocol)** server for CLI integration
- **48+ specialized agents** for various domains
- **35+ hooks** for automation and context injection
- **Session management** for backend integration
- **Filesystem operations** with Git support
- **Conversation search** and project management

## Core Principles

### 1. Agent Specialization

Each agent is designed for a specific domain, providing expert-level responses:

```text
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   User      │────▶│  Bl1nk      │────▶│  Specialist │
│   Query     │     │  Orchestrator│    │  Agent      │
└─────────────┘     └─────────────┘     └─────────────┘
                           │
                           ▼
                    ┌─────────────┐
                    │   Router    │
                    │  (Selects   │
                    │   correct   │
                    │   agent)    │
                    └─────────────┘
```

### 2. Event-Driven Hooks

The hooks system allows intercepting and modifying behavior:

```
┌─────────────────────────────────────────────────────────┐
│                      Hook Chain                          │
├─────────────────────────────────────────────────────────┤
│  Request → Hook1 → Hook2 → HookN → Agent Processing    │
│              ↓         ↓         ↓                      │
│          Modify    Inject    Monitor                   │
│          Request   Context   State                    │
└─────────────────────────────────────────────────────────┘
```

### 3. Type-Safe Protocol

All MCP communication uses JSON-RPC 2.0 with type-safe schemas:

```rust
// All parameters validated at runtime
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct DelegateTaskArgs {
    #[schemars(description = "Type of task")]
    pub task_type: String,
    
    #[schemars(description = "Prompt for agent")]
    pub prompt: String,
    
    #[schemars(description = "Optional agent ID")]
    pub agent_id: Option<String>,
}
```

## Component Architecture

### Layer 1: MCP Server (Public Interface)

```
┌─────────────────────────────────────┐
│       pmcp::ServerBuilder           │
├─────────────────────────────────────┤
│  Tools:                             │
│  • delegate_task (TypedTool)        │
│  • agent_status (TypedTool)         │
├─────────────────────────────────────┤
│  Transport: stdio                   │
│  Protocol: JSON-RPC 2.0 (MCP)       │
└─────────────────────────────────────┘
```

### Layer 2: Orchestration Logic

```
┌─────────────────────────────────────┐
│        Orchestrator                 │
├─────────────────────────────────────┤
│  Components:                        │
│  • AgentRegistry (RwLock)           │
│  • RateLimitTracker (RwLock)        │
│  • AgentExecutor (Arc)              │
│  • AgentRouter                      │
└─────────────────────────────────────┘
```

### Layer 3: Agent Management

```
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│  AgentRegistry   │  │   AgentRouter    │  │  AgentExecutor   │
├──────────────────┤  ├──────────────────┤  ├──────────────────┤
│ • Agent configs  │  │ • Routing rules  │  │ • Spawn process  │
│ • Task tracking  │  │ • Capability     │  │ • ACP protocol   │
│ • Status updates │  │   matching       │  │ • Background     │
└──────────────────┘  └──────────────────┘  └──────────────────┘
```

### Layer 4: Hook System

```
┌─────────────────────────────────────────────────────────┐
│                    HookRegistry                         │
├─────────────────────────────────────────────────────────┤
│  Context Injection:                                     │
│  • directory_agents_injector                            │
│  • directory_readme_injector                            │
│  • compaction_context_injector                          │
│  • rules_injector                                       │
├─────────────────────────────────────────────────────────┤
│  Monitoring & Recovery:                                  │
│  • context_window_monitor                               │
│  • session_recovery                                     │
│  • empty_task_response_detector                         │
│  • edit_error_recovery                                  │
├─────────────────────────────────────────────────────────┤
│  Task Management:                                       │
│  • todo_continuation_enforcer                            │
│  • category_skill_reminder                              │
│  • task_resume_info                                     │
└─────────────────────────────────────────────────────────┘
```

## Data Flow

### Task Delegation

```
1. User sends MCP request
   ├─> MCP Server receives via stdio
   ├─> Parses TypedTool arguments
   └─> Validates against JSON schema

2. Orchestrator processes request
   ├─> AgentRouter selects agent based on task_type
   ├─> RateLimitTracker checks limits
   └─> AgentExecutor spawns process

3. Agent execution
   ├─> Build agent prompt
   ├─> Send to agent process
   └─> Parse response

4. Return result
   ├─> Format MCP response
   ├─> Send via stdio
   └─> Update task status
```

### Hook Processing

```
Request received
      ↓
  Pre-process hooks (context injection)
      ↓
  Request validation
      ↓
  Agent execution
      ↓
  Post-process hooks (monitoring, recovery)
      ↓
  Response
```

## Configuration Architecture

### TOML Structure

```toml
[server]           # MCP server settings
host = "127.0.0.1"
port = 3000

[main_agent]       # Main agent config
name = "gemini"

[[agents]]         # Agent definitions
id = "architect"
name = "Architect"
category = "engineering"

[[routing.rules]]  # Task routing rules
task_type = "code-generation"
preferred_agents = ["code-generator"]

[rate_limiting]    # Rate limit settings
requests_per_minute = 60
requests_per_day = 2000
```

### Agent Definition

```rust
pub struct AgentConfig {
    pub id: String,              // Unique identifier
    pub name: String,            // Display name
    pub category: AgentCategory, // Category enum
    pub description: String,     // Human-readable description
    pub capabilities: Vec<String>, // Task types handled
    pub priority: u8,            // Higher = preferred
    pub rate_limit: RateLimit,   // Rate limiting config
}
```

### Hook Configuration

```rust
pub struct HookConfig {
    pub name: String,
    pub enabled: bool,
    pub options: HashMap<String, Value>,
}
```

## Concurrency Model

### Thread Safety

All shared state uses `Arc<RwLock<T>>`:

```rust
Arc<RwLock<AgentRegistry>>    // Read-heavy workload
Arc<RwLock<RateLimitTracker>> // Write-heavy workload
Arc<AgentExecutor>            // Immutable (safe to share)
```

### Background Tasks

```rust
tokio::spawn(async move {
    executor.execute_agent_task(...).await
});
```

- Non-blocking delegation
- Isolated task contexts
- Automatic cleanup on completion

## Error Handling

### Error Types

1. **MCP Errors** (`pmcp::Error`)
   - Validation errors (invalid input)
   - Internal errors (executor failures)
   - Custom errors (rate limits)

2. **Application Errors** (`anyhow::Result`)
   - Configuration errors
   - File system errors
   - Agent communication errors

3. **Hook Errors**
   - Hook execution failures
   - Context injection errors

### Error Propagation

```
MCP Request
    ↓
TypedTool validation (pmcp::Error)
    ↓
Hook processing (anyhow::Result)
    ↓
Agent execution (anyhow::Result)
    ↓
Convert to pmcp::Error
    ↓
MCP Response (error field)
```

## Performance Considerations

### Async Runtime

- Built on **Tokio** for high-performance async I/O
- Efficient task scheduling
- Non-blocking operations throughout

### Design Choices

1. **Arc over Mutex**: Allows concurrent reads
2. **RwLock pattern**: Optimizes read-heavy workloads
3. **Async I/O**: Efficient network and file operations
4. **Process isolation**: Better security and stability

### Bottlenecks

1. **Process spawn time** (~50-100ms)
   - Mitigation: Keep agents warm (future)
2. **Rate limit checks** (RwLock contention)
   - Mitigation: Rarely written, mostly reads
3. **File I/O** (disk-bound)
   - Mitigation: Async file operations

## Module Structure

### Agent System (16 modules)

| File | Purpose |
| ------ | --------- |
| `mod.rs` | Module exports |
| `register.rs` | Agent registry |
| `router.rs` | Agent routing |
| `executor.rs` | Task execution |
| `creator.rs` | Agent creation |
| `types.rs` | Type definitions |
| `orchestrator.rs` | Multi-agent coordination |
| `expert.rs` | Expert patterns |
| `researcher.rs` | Research agent |
| `explorer.rs` | Code exploration |
| `observer.rs` | Observation |
| `consultant.rs` | Consultation |
| `auditor.rs` | Auditing |
| `manager.rs` | Management |
| `planner.rs` | Planning |
| `orchestrator_junior.rs` | Junior orchestrator |

### Hook System (35+ hooks)

| Category | Count | Examples |
| ---------- | ------- | ---------- |
| Context Injection | 5 | directory_agents_injector, rules_injector |
| Monitoring | 4 | context_window_monitor, session_recovery |
| Recovery | 5 | edit_error_recovery, ralph_loop |
| Task Management | 4 | todo_continuation_enforcer |
| Development | 4 | comment_checker, tool_output_truncator |
| Automation | 13 | auto_update_checker, background_notification |

## Security Considerations

### Input Validation

1. **JSON Schema**: TypedTool enforces schemas
2. **Path validation**: Prevent directory traversal
3. **Command injection**: Whitelist commands only

### Process Isolation

- Each agent runs in separate process
- No shared memory between agents
- Clean shutdown on errors

### Rate Limiting

- Prevents DoS from single agent
- Per-agent quotas enforced
- Transparent to users

## Future Enhancements

### 1. Persistent Task Storage

```rust
// Current: In-memory HashMap
active_tasks: HashMap<String, TaskInfo>

// Future: SQLite/RocksDB
task_store: Arc<dyn TaskStore>
```

### 2. Agent Warm Pools

```rust
// Keep agents running for faster response
struct AgentPool {
    ready_agents: VecDeque<ChildProcess>,
    max_size: usize,
}
```

### 3. HTTP/WebSocket Transport

```rust
// Add to Orchestrator
pub async fn run_http(&self, addr: SocketAddr) -> Result<()> {
    // SSE transport via pmcp
}
```

---

**Last updated**: 2026-02-06

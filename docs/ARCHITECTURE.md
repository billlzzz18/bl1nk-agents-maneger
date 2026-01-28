# Architecture Documentation

## System Overview

Gemini MCP Proxy is a **dual-protocol orchestrator** that bridges:
- **MCP (Model Context Protocol)** - For Gemini CLI integration
- **ACP (Agent Client Protocol)** - For sub-agent communication

## Core Principles

### 1. Agent Community Protocol (ACP)

**Philosophy**: Every agent is both a server and a client.

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Gemini    │────▶│    Proxy    │────▶│    Qwen     │
│     CLI     │◀────│ Orchestrator│◀────│   Agent     │
└─────────────┘     └─────────────┘     └─────────────┘
     MCP                MCP + ACP            ACP
```

All communication uses **JSON-RPC 2.0** regardless of protocol.

### 2. Dual-Mode Operation

The proxy operates in two modes simultaneously:

**Mode 1: MCP Server**
- Listens on stdio
- Exposes tools to Gemini CLI
- Uses PMCP (Pragmatic MCP) SDK

**Mode 2: ACP Client**
- Spawns sub-agent processes
- Sends JSON-RPC requests over stdin
- Reads JSON-RPC responses from stdout

## Component Architecture

### Layer 1: MCP Server (Public Interface)

```rust
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

```rust
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

```rust
┌──────────────────┐  ┌──────────────────┐  ┌──────────────────┐
│  AgentRegistry   │  │   AgentRouter    │  │  AgentExecutor   │
├──────────────────┤  ├──────────────────┤  ├──────────────────┤
│ • Agent configs  │  │ • Routing rules  │  │ • Spawn process  │
│ • Task tracking  │  │ • Capability     │  │ • ACP protocol   │
│ • Status updates │  │   matching       │  │ • Background     │
└──────────────────┘  └──────────────────┘  └──────────────────┘
```

### Layer 4: Rate Limiting

```rust
┌─────────────────────────────────────┐
│      RateLimitTracker               │
├─────────────────────────────────────┤
│  Per-agent tracking:                │
│  • Requests/minute                  │
│  • Requests/day                     │
│  • Auto-reset timers                │
└─────────────────────────────────────┘
```

## Data Flow

### Synchronous Task Execution

```
1. Gemini CLI sends MCP request
   ├─> MCP Server receives via stdio
   ├─> Parses TypedTool arguments
   └─> Validates against JSON schema

2. Orchestrator processes request
   ├─> AgentRouter selects agent
   ├─> RateLimitTracker checks limits
   └─> AgentExecutor spawns process

3. ACP communication
   ├─> Spawn sub-agent CLI
   ├─> Write JSON-RPC to stdin
   ├─> Read JSON-RPC from stdout
   └─> Parse result

4. Return to Gemini CLI
   ├─> Format MCP response
   ├─> Send via stdio
   └─> Update task status
```

### Background Task Execution

```
1. Gemini CLI → delegate_task(background: true)
2. Proxy spawns tokio task
3. Returns immediately with task_id
4. Background task runs independently
5. Results stored in registry
6. Query via agent_status(task_id)
```

## Configuration Architecture

### TOML Structure

```toml
[server]           # MCP server settings
[main_agent]       # Gemini CLI config
[[agents]]         # Sub-agent definitions
[[routing.rules]]  # Task routing rules
[rate_limiting]    # Rate limit settings
[logging]          # Log configuration
```

### Agent Definition

```rust
pub struct AgentConfig {
    id: String,              // Unique identifier
    name: String,            // Display name
    agent_type: String,      // "cli" | "gemini-extension"
    command: Option<String>, // CLI command
    args: Option<Vec<String>>,
    rate_limit: RateLimit,
    capabilities: Vec<String>, // e.g., ["code-generation"]
    priority: u8,             // Higher = preferred
}
```

### Routing Rule

```rust
pub struct RoutingRule {
    task_type: String,        // e.g., "code-generation"
    keywords: Vec<String>,    // Prompt matching
    preferred_agents: Vec<String>, // Agent IDs
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

2. **ACP Errors** (agent communication)
   - Process spawn failures
   - JSON-RPC parsing errors
   - Agent-specific errors

3. **Application Errors** (`anyhow::Result`)
   - Configuration errors
   - File system errors
   - Network errors (future)

### Error Propagation

```rust
MCP Request
    ↓
TypedTool validation (pmcp::Error)
    ↓
Agent execution (anyhow::Result)
    ↓
Convert to pmcp::Error
    ↓
MCP Response (error field)
```

## Performance Considerations

### PMCP Advantages

- **16x faster** than TypeScript SDK
- **50x lower memory** usage
- SIMD-optimized parsing
- Zero-copy where possible

### Design Choices

1. **Arc over Mutex**: Allows concurrent reads
2. **RwLock pattern**: Optimizes read-heavy workloads
3. **Tokio runtime**: Efficient async I/O
4. **Process spawning**: Isolation + parallel execution

### Bottlenecks

1. **Process spawn time** (~50-100ms)
   - Mitigation: Keep agents warm (future)
2. **Rate limit checks** (mutex contention)
   - Mitigation: RwLock, rarely written
3. **JSON parsing** (CPU-bound)
   - Mitigation: SIMD acceleration

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

### 4. Bidirectional ACP

```rust
// Allow agents to call back to orchestrator
struct BidirectionalAgent {
    stdin: ChildStdin,
    stdout: ChildStdout,
    callback: Arc<dyn Fn(Request) -> Response>,
}
```

### 5. Metrics & Observability

```rust
struct Metrics {
    tasks_completed: AtomicU64,
    tasks_failed: AtomicU64,
    avg_latency: AtomicF64,
}
```

## Testing Strategy

### Unit Tests

- Config parsing and validation
- Agent selection logic
- Rate limit enforcement

### Integration Tests

- MCP server ↔ PMCP SDK
- Process spawning
- ACP communication

### Property Tests (Future)

```rust
#[cfg(test)]
proptest! {
    fn routing_always_selects_valid_agent(
        task_type: String,
        agents: Vec<AgentConfig>
    ) {
        // Property: selected agent must have matching capability
    }
}
```

## Security Considerations

### Input Validation

1. **JSON Schema**: TypedTool enforces schemas
2. **Path validation**: Prevent directory traversal
3. **Command injection**: Whitelist commands only

### Rate Limiting

- Prevents DoS from single agent
- Per-agent quotas enforced
- Transparent to users

### Process Isolation

- Each agent runs in separate process
- No shared memory
- Clean shutdown on errors

---

## Appendix: Protocol Comparison

| Feature | MCP | ACP |
|---------|-----|-----|
| Purpose | AI ↔ Editor | Agent ↔ Agent |
| Protocol | JSON-RPC 2.0 | JSON-RPC 2.0 |
| Transport | stdio, HTTP, WS | stdio, AsyncRead/Write |
| Auth | OAuth, Bearer | Session tokens |
| Bidirectional | No | Yes |
| Tool calling | Yes | Yes |
| Resources | Yes | Limited |

Both protocols use the same underlying JSON-RPC 2.0, making interoperability natural.

---

**Last updated**: 2025-01-28
# Quick Start Guide

## 🚀 Get Started in 5 Minutes

### Step 1: Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Step 2: Project Setup

```bash
# Navigate to project directory
cd bl1nk-agents-manager

# Install development tools
just setup

# Build the project
just build
```

### Step 3: Test Installation

```bash
# Verify the binary was built
./target/release/bl1nk-server --help

# Or run the server
just run
```

### Step 4: Validate Agents

```bash
# Validate all agent definitions
just validate-agents

# If issues found, fix them
just fix-agents
```

---

## 📖 First Steps

### List Available Agents

```bash
/system-agent
```

This shows all 48+ available agents.

### Get Agent Information

```bash
/system-agent:info architect
```

### Switch to an Agent

```bash
/system-agent:switch architect
```

This generates the command to set the environment variable.

---

## 🧩 Using Agents

### Delegate a Task

From within Gemini CLI:

```markdown
User: "Use the architect agent to design a web API"
Claude: [Routes to architect agent and returns design]
```

### Direct Agent Usage

```markdown
User: "@code-generator Write a Rust function for fibonacci"
Claude: [Code generator responds with code]
```

---

## 🔧 Configuration

### Create Config Directory

```bash
mkdir -p ~/.config/bl1nk
```

### Customize Configuration

Edit `config/Config.toml`:

```toml
[server]
host = "127.0.0.1"
port = 3000

[main_agent]
name = "gemini"

[[agents]]
id = "architect"
name = "Architect"
category = "engineering"

[[routing.rules]]
task_type = "code-generation"
preferred_agents = ["code-generator"]
```

---

## 🧪 Testing

### Run All Tests

```bash
just test
```

### Test Specific Component

```bash
# Test agent registry
cargo test --package bl1nk-core agents

# Test hooks
cargo test --package bl1nk-core hooks

# Test MCP
cargo test --package bl1nk-core mcp
```

---

## 📚 Learning Resources

### Documentation

| Topic | Link |
|-------|------|
| Architecture | [ARCHITECTURE.md](./ARCHITECTURE.md) |
| Agent Guide | [AGENT_GUIDE.md](./AGENT_GUIDE.md) |
| API Reference | [API.md](../API.md) |
| Project Summary | [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) |

### Example Agents

Check these agents to understand the format:

- `agents/architect.md` - Engineering agent
- `agents/code-generator.md` - Code generation
- `agents/pirate.md` - Entertainment agent
- `agents/agent-creator.md` - Utility agent

---

## 🐛 Common Issues

### Issue: "cargo: not found"

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: Agent not appearing

```bash
# Validate agents
just validate-agents

# Check agents.json
cat agents/agents.json | jq '.agents | length'
```

### Issue: Build fails

```bash
# Clean and rebuild
just clean
just build

# Check Rust version
rustc --version
```

### Issue: MCP connection failed

```bash
# Verify server is running
./target/release/bl1nk-server --help

# Check port
netstat -tulpn | grep 3000
```

---

## 🚀 Next Steps

### 1. Explore Agents

Run `/system-agent` to see all available agents.

### 2. Create Custom Agent

```bash
/system-agent:new
```

Follow the interactive wizard.

### 3. Add Hooks

Create a new hook in `crates/core/src/hooks/`.

### 4. Integrate with Claude CLI

Add to your Claude CLI config:

```json
{
  "mcpServers": {
    "bl1nk": {
      "command": "/path/to/bl1nk-server",
      "transport": "stdio"
    }
  }
}
```

---

## 📝 Development Workflow

```bash
# 1. Make changes
# 2. Format code
just fmt

# 3. Check for errors
just check

# 4. Run tests
just test

# 5. Run linter
just clippy

# 6. Build release
just build
```

---

## 🎯 Quick Reference

| Command | Description |
|---------|-------------|
| `just build` | Build release binary |
| `just run` | Run server |
| `just dev` | Hot-reload development |
| `just test` | Run all tests |
| `just fmt` | Format code |
| `just clippy` | Run linter |
| `just validate-agents` | Validate agent files |
| `just fix-agents` | Fix agent issues |

---

**Ready to go!** 🎉

For questions, check the documentation or open an issue.

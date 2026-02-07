# Agent System Guidelines

This document provides comprehensive guidelines for the Bl1nk Agent System.

## 1. Overview

Bl1nk uses a sophisticated agent system with:

- **48+ specialized agents** for different domains
- **35+ hooks** for automation and context injection
- **Type-safe routing** based on task type
- **Category-based organization** for easy discovery

### Design Philosophy

1. **Specialization**: Each agent has a specific, focused purpose
2. **Composition**: Agents can be combined for complex tasks
3. **Extensibility**: Easy to add new agents and hooks
4. **Type Safety**: All communication uses typed schemas

---

## 2. Agent Categories

### Engineering & Development (8 agents)

| Agent | Purpose | Use Case |
|-------|---------|----------|
| Architect | System design | "Design a microservice architecture" |
| Code Generator | Code creation | "Write a REST API handler" |
| Code Reviewer | Quality review | "Review this PR" |
| Code Explorer | Deep analysis | "Understand how X works" |
| Code Architect | Feature design | "Design a new feature" |
| Cloudflare | Cloudflare dev | "Create a Cloudflare Worker" |
| Fullstack Dev | Full-stack dev | "Build a web app" |
| Orchestrator | Task routing | "Delegate this complex task" |

### Research & Analysis (6 agents)

| Agent | Purpose | Use Case |
|-------|---------|----------|
| Codebase Analyzer | Implementation details | "How does X implement Y?" |
| Codebase Locator | File discovery | "Find files related to X" |
| Codebase Pattern Finder | Pattern search | "Find similar implementations" |
| Research Analyzer | Document analysis | "Analyze this research" |
| Thoughts Analyzer | Deep research | "Research topic X" |
| Web Search Researcher | Web content | "Fetch docs for X" |

### Documentation & Planning (4 agents)

| Agent | Purpose | Use Case |
|-------|---------|----------|
| Docbot Pro | Enterprise docs | "Generate API docs" |
| Docs Researcher | Library docs | "Find documentation for X" |
| Insight Documenter | Technical insights | "Document this discovery" |
| Plan Implementation Reviewer | Plan validation | "Verify this plan was executed" |

### Utilities & Tools (7 agents)

| Agent | Purpose | Use Case |
|-------|---------|----------|
| Agent Creator | Create agents | "Create a new agent for X" |
| Command Creator | Create commands | "Create a Claude command" |
| Skill Creator | Create skills | "Create a new skill" |
| Skill Reviewer | Review skills | "Review this skill" |
| Plugin Validator | Validate plugins | "Validate my plugin" |
| Task Management | Task tracking | "Track this task" |
| UI Engineer | Frontend/UI | "Build a React component" |

### Creative & Entertainment (7 agents)

| Agent | Purpose | Use Case |
|-------|---------|----------|
| Creative Writer | Creative content | "Write a poem" |
| Pirate | Fun assistance | "Help me in pirate speak" |
| Yoda | Yoda-speak | "Teach me about X, you will" |
| Shakespeare | Shakespearean | "Speak to me in iambic" |
| Cowboy | Western-speak | "Howdy, help me with X" |
| Gen Z | Gen Z slang | "No cap, explain X" |
| Comedian | Dad jokes | "Tell me a joke" |

---

## 3. Agent Design Guidelines

### Frontmatter Structure

```yaml
---
id: my-agent           # Unique ID (lowercase, hyphens)
name: My Agent          # Display name
description: |          # Trigger conditions (can span multiple lines)
  Use this agent when users need [specific functionality].
  Also trigger for: "[phrase1]", "[phrase2]"
category: utility       # Category from list above
color: blue            # UI color (optional)
tools:                 # Available tools (optional)
  - Write
  - Read
---
```

### System Prompt Structure

```markdown
You are an expert [domain]...

## Core Responsibilities

1. [Primary responsibility]
2. [Secondary responsibility]
3. [Tertiary responsibility]

## Process

1. [Step 1]
2. [Step 2]
3. [Step 3]

## Output Format

Provide your output in this format:
- [Format specification]

## Constraints

- [Constraint 1]
- [Constraint 2]

## Examples

<example>
Context: [Situation]
user: "[User message]"
assistant: "[Response before triggering]"
<commentary>
[Why this triggers]
</commentary>
</example>
```

### Best Practices

1. **Clear Scope**: Agent should have focused responsibilities
2. **Avoid Overlap**: Don't create agents that overlap with existing ones
3. **Trigger Examples**: Include 2-4 triggering examples
4. **Output Format**: Define consistent output format
5. **Constraints**: Specify what agent should NOT do
6. **Process**: Provide step-by-step process

---

## 4. Hook System

### Hook Categories

#### Context Injection (5 hooks)

| Hook | Purpose |
|------|---------|
| `directory_agents_injector` | Inject agent context from directory |
| `directory_readme_injector` | Inject README context |
| `compaction_context_injector` | Context compaction |
| `rules_injector` | Rule injection |
| `ralph_loop` | Loop detection and recovery |

#### Monitoring (4 hooks)

| Hook | Purpose |
|------|---------|
| `context_window_monitor` | Memory usage monitoring |
| `session_recovery` | Session recovery |
| `empty_task_response_detector` | Detect empty responses |
| `anthropic_context_window_limit_recovery` | Claude context recovery |

#### Recovery (5 hooks)

| Hook | Purpose |
|------|---------|
| `edit_error_recovery` | Recover from edit errors |
| `session_recovery` | Recover sessions |
| `ralph_loop` | Loop recovery |
| `question_label_truncator` | Label truncation |

#### Task Management (4 hooks)

| Hook | Purpose |
|------|---------|
| `todo_continuation_enforcer` | Enforce TODO completion |
| `category_skill_reminder` | Category/skill reminders |
| `task_resume_info` | Task resumption info |
| `start_work` | Work session startup |

#### Development (4 hooks)

| Hook | Purpose |
|------|---------|
| `comment_checker` | Check for TODO comments |
| `tool_output_truncator` | Truncate long outputs |
| `thinking_block_validator` | Validate thinking blocks |
| `thinking_mode` | Thinking mode support |

### Creating Hooks

1. Create module in `crates/core/src/hooks/`
2. Implement hook interface
3. Register in `crates/core/src/hooks/mod.rs`

---

## 5. Routing System

### Task Type Matching

Agents are selected based on:

1. **Explicit Request**: User specifies agent
2. **Task Type**: Matching task type pattern
3. **Keyword Match**: Keywords in user query
4. **Priority**: Higher priority agents preferred

### Routing Configuration

```toml
[[routing.rules]]
task_type = "code-generation"
preferred_agents = ["code-generator", "fullstack-dev"]

[routing.keywords]
"write code" = ["code-generator"]
"build app" = ["fullstack-dev"]
"design" = ["architect"]
```

---

## 6. Best Practices

### Agent Development

1. **Start Simple**: Begin with clear, focused agent
2. **Add Examples**: Include triggering examples
3. **Define Boundaries**: Specify what agent should NOT do
4. **Test Thoroughly**: Validate with real tasks
5. **Iterate**: Refine based on usage

### Hook Development

1. **Single Purpose**: Each hook should do one thing
2. **Non-Blocking**: Avoid long-running operations
3. **Error Handling**: Graceful degradation
4. **Logging**: Log hook execution

### Performance

1. **Lazy Loading**: Load agents on demand
2. **Caching**: Cache frequently used data
3. **Async**: Use async operations where possible
4. **Pooling**: Consider process pooling (future)

---

## 7. Testing

### Agent Testing

```bash
# Validate agent files
just validate-agents

# Test delegation
gemini> "Use @agent-name to do X"
```

### Hook Testing

```bash
# Run hook tests
cargo test --package bl1nk-core hooks

# Run all tests
just test
```

---

## 8. Common Patterns

### Pattern: Agent Delegation

```markdown
User: "I need to build a web API"

Claude: [Routes to orchestrator agent]
  └─> Orchestrator analyzes task
      └─> Routes to architect (for design)
          └─> Routes to code-generator (for implementation)
```

### Pattern: Context Injection

```
User: "Fix this bug"
  └─> Hook: directory_agents_injector
      └─> Injects relevant context
          └─> Agent processes with full context
```

### Pattern: Error Recovery

```
Agent: Fails to complete task
  └─> Hook: edit_error_recovery
      └─> Attempts recovery
          └─> Retries or escalates
```

---

## 9. Troubleshooting

### Wrong Agent Selected

1. Check description triggers
2. Verify routing configuration
3. Add explicit examples

### Agent Not Responding

1. Check hook chain for blocking
2. Verify rate limits
3. Check agent process health

### Hook Issues

1. Check hook order
2. Verify hook is enabled
3. Check hook configuration

---

## 10. Resources

### Documentation

- [README.md](../README.md)
- [ARCHITECTURE.md](./ARCHITECTURE.md)
- [AGENT_GUIDE.md](./AGENT_GUIDE.md)
- [API.md](../API.md)

### Examples

- `agents/architect.md`
- `agents/code-generator.md`
- `agents/agent-creator.md`

### Tools

- `just validate-agents`
- `just fix-agents`
- `just test`

---

**Last updated**: 2026-02-06

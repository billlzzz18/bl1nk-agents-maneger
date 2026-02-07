# Agent System Usage Guide

This guide explains how to use the agent system effectively, including how to discover, select, and work with agents.

## Table of Contents

1. [Quick Start](#quick-start)
2. [Discovering Agents](#discovering-agents)
3. [Selecting the Right Agent](#selecting-the-right-agent)
4. [Using Command Agents](#using-command-agents)
5. [Working with Agents](#working-with-agents)
6. [Common Patterns](#common-patterns)
7. [Troubleshooting](#troubleshooting)

---

## Quick Start

### Basic Agent Usage

```bash
# Start with default agent
gemini

# Use a specific agent
export GEMINI_SYSTEM_MD="agents/code-generator.md"
gemini

# One-off with specific agent
GEMINI_SYSTEM_MD="agents/architect.md" gemini "Design a REST API"
```

### Creating Your First Agent Interaction

```bash
# Get examples for an agent
gemini> /agent/examples code-reviewer

# Get detailed info about an agent
gemini> /agent/info code-generator

# Switch to a different agent
gemini> /agent/switch architect
```

---

## Discovering Agents

### Method 1: Browse the Catalog

The complete agent catalog is available at [`docs/AGENT_CATALOG.md`](AGENT_CATALOG.md). Use it to:

- Browse agents by functional group
- Search by tags
- Compare similar agents
- Find selection criteria

### Method 2: Use the Examples Command

```bash
# Get examples for all agents
gemini> /agent/examples

# Get examples for a specific agent
gemini> /agent/examples code-generator

# Get examples for agents in a category
gemini> /agent/examples architect
```

### Method 3: Use the Info Command

```bash
# Get detailed information about an agent
gemini> /agent/info code-reviewer

# Compare multiple agents
gemini> /agent/info code-reviewer
gemini> /agent/info pr-reviewer
```

### Method 4: Explore the Registry

Agents are registered in [`agents/agents.json`](agents/agents.json). You can:

```bash
# View all registered agents
cat agents/agents.json | jq '.agents[].id'

# Filter by category
cat agents/agents.json | jq '.agents[] | select(.category == "engineering")'

# Search by use case
cat agents/agents.json | jq '.agents[] | select(.use_cases | contains("code generation"))'
```

---

## Selecting the Right Agent

### Decision Flowchart

```
What do you need?
│
├─ CREATE something new?
│   ├─ New agent? → agent-creator
│   ├─ New command? → command-creator
│   ├─ New skill? → skill-creator
│   ├─ Full app? → fullstack-dev
│   └─ Code only? → code-generator
│
├─ ANALYZE existing code?
│   ├─ Deep understanding? → code-explorer
│   ├─ Bug finding? → code-reviewer
│   ├─ PR review? → pr-reviewer
│   └─ Find patterns? → codebase-pattern-finder
│
├─ FIND something?
│   ├─ Files? → codebase-locator
│   ├─ Docs? → docs-researcher
│   └─ Web content? → web-search-researcher
│
├─ DESIGN something?
│   ├─ System architecture? → architect
│   └─ Feature design? → code-architect
│
├─ DOCUMENT something?
│   ├─ API docs? → docbot-pro
│   ├─ Insights? → insight-documenter
│   └─ Validate plan? → plan-implementation-reviewer
│
└─ MANAGE workflow?
    ├─ Delegate? → orchestrator
    ├─ Capture learnings? → deep-reflector
    └─ Fix issue? → github-issue-fixer
```

### Quick Reference Table

| Task | Best Agent | Alternative |
|------|------------|-------------|
| Write code | code-generator | fullstack-dev |
| Review code | code-reviewer | pr-reviewer |
| Design system | architect | code-architect |
| Find files | codebase-locator | codebase-pattern-finder |
| Research | thoughts-analyzer | research-analyzer |
| Get docs | docs-researcher | web-search-researcher |
| Create agent | agent-creator | new (command) |
| Create skill | skill-creator | - |
| Switch agent | switch | - |

---

## Using Command Agents

The command agents (`/agent/*`) provide system management capabilities.

### /agent/examples

Shows example prompts and use cases for agents.

**Usage:**

```bash
# All agents (picks 3 diverse examples)
gemini> /agent/examples

# Specific agent
gemini> /agent/examples code-generator

# Category filter
gemini> /agent/examples engineering
```

**Output:**

```
Example prompts for code-generator:

1. "Write a Python function to calculate Fibonacci numbers"
   Use case: Algorithm implementation

2. "Create a REST API endpoint for user authentication"
   Use case: API development

3. "Build a React component for user registration form"
   Use case: Frontend component
```

### /agent/info

Shows detailed information about a specific agent.

**Usage:**

```bash
gemini> /agent/info code-reviewer
```

**Output:**

```
Agent: Code Reviewer
Category: engineering
Description: Reviews code for bugs, logic errors, security vulnerabilities...
Use Cases:
  - Reviewing code for bugs and security issues
  - Analyzing PRs before merge
  - Checking code quality

Tags: engineering, review, quality, medium-complexity
```

### /agent/new

Creates and registers a new system agent.

**Usage:**

```bash
gemini> /agent/new My Custom Agent Description
```

**Process:**

1. Analyzes your request
2. Generates unique ID
3. Creates agent file
4. Registers in custom agents
5. Provides switch instructions

### /agent/switch

Shows how to switch to a different agent.

**Usage:**

```bash
gemini> /agent/switch code-generator
```

**Output:**

```
To switch to the 'code-generator' agent:

a) Current Session (Requires Restart):
   export GEMINI_SYSTEM_MD="<path-to-agent>"
   gemini

b) One-off Command:
   GEMINI_SYSTEM_MD="<path-to-agent>" gemini

c) Create Alias (Recommended):
   alias gemini-code='GEMINI_SYSTEM_MD="<path-to-agent>" gemini'
```

---

## Working with Agents

### Pattern 1: Agent Delegation

```markdown
User: "I need to build a web API"

Claude: [Routes to orchestrator agent]
  └─> Orchestrator analyzes task
      └─> Routes to architect (for design)
          └─> Routes to code-generator (for implementation)
```

### Pattern 2: Sequential Agent Use

```markdown
1. Use code-explorer to understand existing code
2. Use codebase-locator to find relevant files
3. Use code-generator to implement changes
4. Use code-reviewer to review implementation
```

### Pattern 3: Parallel Agent Use

For independent tasks, you can work with multiple agents:

```markdown
# Task 1
Use code-generator to write the API endpoint

# Task 2 (independent)
Use docbot-pro to generate API documentation
```

### Pattern 4: Agent Chaining

```markdown
1. Use research-analyzer to understand requirements
2. Use architect to design solution
3. Use code-generator to implement
4. Use code-reviewer to validate
5. Use deep-reflector to capture learnings
```

---

## Common Patterns

### Pattern: Code Review Workflow

```bash
# 1. Start with code-reviewer
gemini> code-reviewer "Review this PR: https://github.com/org/repo/pull/123"

# 2. If issues found, use code-generator to fix
gemini> code-generator "Fix the security vulnerabilities found in the review"

# 3. Use pr-reviewer for final check
gemini> pr-reviewer "Final review of PR #123"
```

### Pattern: Feature Development Workflow

```bash
# 1. Design with code-architect
gemini> code-architect "Design a user authentication feature"

# 2. Generate code
gemini> code-generator "Implement user authentication based on the design"

# 3. Review implementation
gemini> code-reviewer "Review the authentication implementation"

# 4. Document
gemini> docbot-pro "Generate API documentation for authentication"
```

### Pattern: Research Workflow

```bash
# 1. Initial research
gemini> thoughts-analyzer "Research best practices for microservices"

# 2. Web search for latest info
gemini> web-search-researcher "Latest microservices patterns 2024"

# 3. Document findings
gemini> insight-documenter "Document key findings from microservices research"
```

### Pattern: Agent Creation Workflow

```bash
# 1. Create agent
gemini> agent-creator "Create an agent for database migration"

# 2. Register
gemini> new "Register my database migration agent"

# 3. Switch and test
gemini> switch "database-migration"
```

---

## Troubleshooting

### Problem: Wrong Agent Selected

**Symptoms:**

- Agent doesn't seem to understand the task
- Agent provides irrelevant responses

**Solutions:**

1. **Use explicit agent invocation**

   ```bash
   # Instead of: "Help me write code"
   # Use: @code-generator "Help me write code"
   ```

2. **Use more specific triggers**

   ```bash
   # Instead of: "Fix this"
   # Use: "Review this code for security vulnerabilities"
   ```

3. **Check agent tags**

   ```bash
   gemini> /agent/info <agent-name>
   # Verify tags match your task
   ```

### Problem: Agent Not Found

**Symptoms:**

- "Agent not found" error
- Unable to switch agents

**Solutions:**

1. **Verify agent exists**

   ```bash
   cat agents/agents.json | jq '.agents[].id'
   ```

2. **Check spelling**

   ```bash
   # Use exact agent ID
   gemini> /agent/examples <exact-agent-id>
   ```

3. **Create new agent**

   ```bash
   gemini> agent-creator "Create an agent for my specific task"
   ```

### Problem: Agent Not Responding

**Symptoms:**

- Agent hangs
- No response from agent

**Solutions:**

1. **Check session**

   ```bash
   # Restart session with different agent
   GEMINI_SYSTEM_MD="agents/orchestrator.md" gemini
   ```

2. **Check hook chain**

   ```bash
   # Review active hooks
   gemini> hookify-list
   ```

3. **Use simpler agent**

   ```bash
   # Try low-complexity agent
   GEMINI_SYSTEM_MD="agents/codebase-locator.md" gemini
   ```

### Problem: Need Help Choosing

**Solutions:**

1. **Browse catalog**

   ```bash
   cat docs/AGENT_CATALOG.md
   ```

2. **Get random examples**

   ```bash
   gemini> /agent/examples
   ```

3. **Use orchestrator**

   ```bash
   # Orchestrator can help delegate
   gemini> orchestrator "I need help with [task description]"
   ```

---

## Best Practices

### 1. Start Simple

Begin with clear, focused tasks before moving to complex workflows.

### 2. Use Explicit Triggers

Always use clear, specific language to trigger agents.

### 3. Chain Agents Sequentially

For complex tasks, use agents one after another rather than all at once.

### 4. Capture Learnings

Use deep-reflector after significant work sessions.

### 5. Keep Documentation Updated

Use docbot-pro when making significant changes.

### 6. Review Before Committing

Always use code-reviewer or pr-reviewer before merging.

### 7. Use Orchestrator for Complex Tasks

Let the orchestrator delegate to specialists.

---

## Additional Resources

- [Agent Catalog](AGENT_CATALOG.md) - Complete agent reference
- [Agent Specification](AGENT_SPECIFICATION.md) - Technical standards
- [Agent System Guidelines](guidline-agents-system.md) - System architecture
- [README](../README.md) - Project overview

---

**Guide Version:** 1.0.0  
**Last Updated:** 2026-02-07  
**Maintained By:** Agent System Team

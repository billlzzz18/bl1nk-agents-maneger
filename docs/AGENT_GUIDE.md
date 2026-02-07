# Creating Bl1nk-Compatible Agents

This guide explains how to create agents compatible with Bl1nk Agents Manager.

## Agent Requirements

### 1. Agent File Format

Agents are defined as Markdown files with YAML frontmatter:

```markdown
---
id: my-agent
name: My Agent
description: Use this agent when users need specific functionality
category: utility
color: blue
tools:
  - Write
  - Read
---

Your system prompt goes here...
```

### 2. YAML Frontmatter

| Field | Required | Description |
| --- | --- | --- |
| `id` | Yes | Unique identifier (lowercase, hyphens) |
| `name` | Yes | Display name |
| `description` | Yes | Trigger conditions and use cases |
| `category` | Yes | Category: engineering, creative, utility, entertainment |
| `color` | No | UI color (blue, green, yellow, red, magenta) |
| `tools` | No | List of available tools |

### 3. System Prompt

The system prompt defines the agent's behavior:

```markdown
You are an expert [domain]...

## Core Responsibilities

1. [Responsibility 1]
2. [Responsibility 2]
3. [Responsibility 3]

## Process

1. [Step 1]
2. [Step 2]
3. [Step 3]

## Output Format

Your output should include...
```

## Example: Creating a Code Review Agent

### Step 1: Create Agent File

Create `agents/code-reviewer.md`:

```markdown
---
id: code-reviewer
name: Code Reviewer
description: |
  Use this agent when the user asks to review code, check for bugs,
  or analyze code quality. Also trigger for: "review this PR",
  "check for security issues", "analyze this code".
category: engineering
color: red
tools:
  - Read
  - Glob
  - Grep
---

You are an elite code reviewer specializing in finding bugs, security vulnerabilities, and code quality issues.

## Core Responsibilities

1. **Analyze code** for potential bugs and logic errors
2. **Identify security vulnerabilities** (SQL injection, XSS, etc.)
3. **Check code quality** (naming, structure, comments)
4. **Verify adherence** to project conventions

## Process

1. Read and understand the code changes
2. Analyze for bugs and edge cases
3. Check for security issues
4. Verify code quality
5. Provide actionable recommendations

## Output Format

Provide your review in this format:

### Summary
[Brief overview of changes]

### Critical Issues
| Severity | File | Line | Issue | Recommendation |
|----------|------|------|-------|----------------|
| High | src/main.rs | 42 | SQL injection | Use parameterized queries |

### Suggestions
[List of improvement suggestions]

### Positive Aspects
[What was done well]
```

### Step 2: Register Agent

Add to `agents/agents.json`:

```json
{
  "id": "code-reviewer",
  "name": "Code Reviewer",
  "file": "code-reviewer.md",
  "category": "engineering",
  "description": "Reviews code for bugs, security issues, and code quality"
}
```

### Step 3: Validate Agent

```bash
just validate-agents
```

## Agent Categories

| Category | Description | Color |
| --- | --- | --- |
| `engineering` | Software development | blue |
| `creative` | Writing and creative | green |
| `utility` | Tools and utilities | yellow |
| `entertainment` | Fun and humor | magenta |

## Trigger Examples

Include trigger examples in the description:

```markdown
description: |
  Use this agent when the user asks to [action].
  Also trigger for: "[phrase1]", "[phrase2]", "[phrase3]"

  Examples:
  - User: "Review this code"
  - User: "Check for security issues"
  - User: "Analyze this PR"
```

## Best Practices

### 1. Clear Responsibilities

Define specific, focused responsibilities:

```markdown
## Core Responsibilities

1. Only review code for bugs and security
2. Do NOT suggest architectural changes (use architect agent)
3. Do NOT write code (use code-generator agent)
```

### 2. Process Steps

Provide clear step-by-step process:

```markdown
## Process

1. Read the changed files
2. Run static analysis tools
3. Check for common vulnerabilities
4. Review against project standards
5. Compile findings into report
```

### 3. Output Format

Define consistent output format:

```markdown
## Output Format

Your review should include:
- Summary (2-3 sentences)
- Critical Issues (if any)
- Suggestions (prioritized)
- Positive Aspects (at least one)
```

### 4. Examples

Add example blocks:

```markdown
<example>
Context: User wants a quick review
user: "Quick review of my latest commit"
assistant: "I'll use the code-reviewer agent to analyze your changes."
</example>
```

## Testing Your Agent

### Manual Test

1. Add agent to `agents/agents.json`
2. Run `/system-agent` to verify it appears
3. Use `/system-agent:info <agent-id>` to verify metadata
4. Test with real task

### Integration Test

```bash
# Test agent delegation
gemini-cli> "Use code-reviewer to review src/main.rs"
```

## Common Issues

### Issue: Agent not appearing

**Cause**: Missing from `agents.json`
**Fix**: Add agent to the JSON registry

### Issue: Wrong agent selected

**Cause**: Description doesn't match user query
**Fix**: Update description with better trigger phrases

### Issue: Agent behaving unexpectedly

**Cause**: System prompt too vague
**Fix**: Add clear responsibilities and constraints

---

## Agent Checklist

Before deploying:

- [ ] YAML frontmatter complete (id, name, description, category)
- [ ] System prompt has clear responsibilities
- [ ] Process steps are defined
- [ ] Output format is specified
- [ ] Trigger examples included
- [ ] Added to `agents/agents.json`
- [ ] Validated with `just validate-agents`
- [ ] Tested with real task

---

**Need help?** Check existing agents in `agents/` directory or open an issue.

# Agent System Specification Standards

This document defines the specification standards, requirements, and guidelines for all agents in the system.

## Table of Contents

1. [Agent Definition Standards](#agent-definition-standards)
2. [Metadata Requirements](#metadata-requirements)
3. [Tagging Standards](#tagging-standards)
4. [File Structure](#file-structure)
5. [Quality Criteria](#quality-criteria)
6. [Compliance Checklist](#compliance-checklist)

---

## Agent Definition Standards

### 1.1 Agent Identity

Every agent MUST have:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier (lowercase, hyphens) |
| `name` | string | Yes | Human-readable display name |
| `file` | string | Yes | Path to agent markdown file |
| `description` | string | Yes | Brief description of purpose |
| `category` | string | Yes | Primary category (see categories) |

### 1.2 Agent Description Requirements

The `description` field MUST:

- Be concise (50-200 characters)
- Start with an action verb
- Include trigger conditions
- Specify primary use case
- Avoid technical jargon (use plain language)

**Good Examples:**

```
✅ "Code review specialist for finding bugs and security issues"
✅ "System architecture planning and design documentation"
```

**Bad Examples:**

```
❌ "Does stuff with code"
❌ "Helper agent for various tasks"
```

### 1.3 Category Standards

Agents MUST belong to one of the following categories:

| Category | Description | Examples |
|----------|-------------|----------|
| `engineering` | Software development tasks | code-generator, code-reviewer |
| `utility` | General purpose tools | codebase-locator, docbot-pro |
| `creative` | Creative and entertainment | creative-writer, comedian |
| `comedy` | Persona-based entertainment | pirate, yoda, shakespeare |
| `entertainment` | Light entertainment | comedian |
| `command` | System management commands | examples, info, new, switch |

---

## Metadata Requirements

### 2.1 Recommended Metadata Fields

Agents SHOULD include these additional metadata fields:

```yaml
---
id: my-agent
name: My Agent
description: |
  Use this agent when users need [specific functionality].
  Also trigger for: "[phrase1]", "[phrase2]"
category: utility
tags: [utility, development, creation]  # Standardized tags
complexity: medium  # low, medium, high
author: Agent System Team
version: 1.0.0
created: 2026-02-07
updated: 2026-02-07
dependencies: []  # External dependencies
outputs: [code, documentation]  # Expected output types
trigger_examples:
  - "Create a new agent"
  - "Build an autonomous agent"
  - "I need an agent for X"
---
```

### 2.2 Use Cases Field

Agents SHOULD include a `use_cases` array:

```yaml
use_cases:
  - "Primary use case 1"
  - "Primary use case 2"
  - "Primary use case 3"
```

**Requirements:**

- Minimum 2 use cases
- Maximum 5 use cases
- Each use case should be actionable
- Use consistent verb tense

### 2.3 Personality Field (Optional)

For persona-based agents:

```yaml
personality: "Professional, analytical, thorough"
outputs: "Technical documentation and diagrams"
```

---

## Tagging Standards

### 3.1 Tag Categories

All agents MUST be tagged with categories from these dimensions:

#### Dimension 1: Domain Tag (Required)

| Tag | Description | Use When |
|-----|-------------|----------|
| `engineering` | Development and technical tasks | Code, architecture, engineering work |
| `utility` | General purpose tools | Navigation, search, management |
| `creative` | Creative and entertainment | Writing, persona, fun tasks |
| `command` | System commands | Agent management, exploration |

#### Dimension 2: Complexity Tag (Required)

| Tag | Level | Use When |
|-----|-------|----------|
| `low-complexity` | Low | Single task, minimal context |
| `medium-complexity` | Medium | Multi-step, moderate context |
| `high-complexity` | High | Complex analysis, extensive context |

#### Dimension 3: Function Tag (Required)

| Tag | Description |
|-----|-------------|
| `architecture` | System/feature design |
| `code-generation` | Writing code |
| `analysis` | Examining code/docs |
| `review` | Quality validation |
| `search` | Discovery/navigation |
| `research` | Deep investigation |
| `documentation` | Creating docs |
| `development` | Creating tools |
| `workflow` | Process orchestration |
| `entertainment` | Fun output |
| `conversion` | File transformation |
| `management` | System control |

#### Dimension 4: Output Tag (Recommended)

| Tag | Description |
|-----|-------------|
| `code` | Primary output is code |
| `documentation` | Primary output is docs |
| `text` | Primary output is text |
| `guidance` | Primary output is instructions |
| `persona` | Output in specific voice |

#### Dimension 5: Dependency Tag (Optional)

| Tag | Description |
|-----|-------------|
| `requires-json` | Needs registry access |
| `requires-python` | Needs Python scripts |
| `requires-file-system` | Needs file operations |
| `requires-mcp` | Needs MCP server |
| `standalone` | No dependencies |

### 3.2 Tag Syntax

Tags MUST be formatted as:

```yaml
tags:
  - domain:engineering
  - complexity:medium
  - function:code-generation
  - output:code
```

Or in simplified form:

```yaml
tags: [engineering, medium-complexity, code-generation, code]
```

### 3.3 Tag Validation Rules

1. **Minimum Tags:** Each agent MUST have at least 3 tags
2. **Maximum Tags:** Each agent SHOULD NOT exceed 6 tags
3. **Required Dimensions:** Must include Domain and Complexity
4. **Uniqueness:** No duplicate tags on single agent
5. **Consistency:** Use standardized tag names only

---

## File Structure

### 4.1 Agent File Location

All agent files MUST be located in:

```
agents/<agent-id>.md          # System agents
custom/<agent-id>.md          # Custom 4.2 agents
```

### Agent File Template

```markdown
---
id: agent-id
name: Agent Name
description: |
  Use this agent when [specific functionality is needed].
  Also trigger for: "[trigger phrase 1]", "[trigger phrase 2]"
category: [category]
tags: [tags]
complexity: [low|medium|high]
version: 1.0.0
created: YYYY-MM-DD
updated: YYYY-MM-DD
use_cases:
  - "Use case 1"
  - "Use case 2"
  - "Use case 3"
outputs: [output types]
trigger_examples:
  - "Example trigger 1"
  - "Example trigger 2"
---

# Agent Name

## Overview

[Brief overview of agent purpose and capabilities]

## Core Responsibilities

1. [Primary responsibility]
2. [Secondary responsibility]
3. [Tertiary responsibility]

## Process

### Step 1: [Step Name]
[Description of step]

### Step 2: [Step Name]
[Description of step]

### Step 3: [Step Name]
[Description of step]

## Constraints

- [Constraint 1]
- [Constraint 2]

## Examples

### Example 1
**Context:** [Situation]
**User:** "[User message]"
**Response:** "[Expected agent response]"

## Output Format

[Format specification]

## Related Agents

- [Agent 1] - [Relationship]
- [Agent 2] - [Relationship]

---

**Last Updated:** YYYY-MM-DD
**Version:** 1.0.0
```

### 4.3 Registry Format

The agent registry (`agents/agents.json`) MUST follow this format:

```json
{
  "agents": [
    {
      "id": "agent-id",
      "name": "Agent Name",
      "file": "agent-id.md",
      "category": "category",
      "description": "Agent description",
      "use_cases": ["case1", "case2"],
      "tags": ["tag1", "tag2"],
      "complexity": "medium"
    }
  ],
  "categories": {
    "category-name": {
      "name": "Display Name",
      "description": "Category description"
    }
  },
  "metadata": {
    "version": "0.2.0",
    "total_agents": 52,
    "last_updated": "2026-02-07"
  }
}
```

---

## Quality Criteria

### 5.1 Functional Requirements

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Clear Purpose | High | Agent has well-defined, focused purpose |
| Actionable Output | High | Agent produces useful, actionable results |
| Trigger Clarity | Medium | Clear conditions for agent activation |
| Scope Definition | Medium | Well-defined boundaries and limitations |

### 5.2 Technical Requirements

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Code Quality | High | Well-structured, maintainable code |
| Documentation | High | Complete, accurate documentation |
| Error Handling | Medium | Graceful error handling |
| Performance | Medium | Efficient resource usage |

### 5.3 User Experience

| Criterion | Weight | Description |
|-----------|--------|-------------|
| Ease of Use | High | Simple, intuitive interaction |
| Consistency | Medium | Predictable behavior |
| Helpfulness | Medium | Provides useful guidance |
| Completeness | Medium | Handles expected use cases |

### 5.4 Scoring Matrix

| Score | Grade | Criteria |
|-------|-------|----------|
| 90-100 | A | Exceeds all requirements |
| 80-89 | B | Meets all requirements |
| 70-79 | C | Meets most requirements |
| 60-69 | D | Meets basic requirements |
| <60 | F | Does not meet requirements |

---

## Compliance Checklist

### New Agent Checklist

- [ ] **Identity**
  - [ ] Unique `id` (lowercase, hyphens)
  - [ ] Clear `name`
  - [ ] Descriptive `description`

- [ ] **Categorization**
  - [ ] Appropriate `category`
  - [ ] Relevant `tags` (3-6)
  - [ ] Correct `complexity` level

- [ ] **Documentation**
  - [ ] Complete YAML frontmatter
  - [ ] Core responsibilities defined
  - [ ] Process steps documented
  - [ ] Constraints specified
  - [ ] Examples included

- [ ] **Quality**
  - [ ] No overlap with existing agents
  - [ ] Unique value proposition
  - [ ] Tested functionality
  - [ ] Code review passed

- [ ] **Registration**
  - [ ] Added to `agents/agents.json`
  - [ ] Metadata complete
  - [ ] Tags validated
  - [ ] Version updated

### Validation Commands

```bash
# Validate agent files
just validate-agents

# Fix common issues
just fix-agents

# Run tests
just test
```

---

## Versioning

### Semantic Versioning

Agents follow semantic versioning (MAJOR.MINOR.PATCH):

| Version Type | Increment When |
|--------------|----------------|
| MAJOR | Breaking changes to agent behavior |
| MINOR | New features, non-breaking changes |
| PATCH | Bug fixes, documentation updates |

### Changelog Entry

Every agent update MUST include a changelog entry:

```markdown
## [Version] - YYYY-MM-DD

### Added
- New feature description

### Changed
- Updated behavior description

### Fixed
- Bug fix description

### Deprecated
- Deprecated feature notice
```

---

## References

- [Agent Catalog](./AGENT_CATALOG.md)
- [Agent Usage Guide](./AGENT_USAGE_GUIDE.md)
- [Agent System Guidelines](./guidline-agents-system.md)
- [Registry Format](#43-registry-format)

---

**Document Owner:** Agent System Maintainers  
**Version:** 1.0.0  
**Last Updated:** 2026-02-07

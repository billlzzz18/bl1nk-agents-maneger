# Agent System Maintenance Guide

This document defines the mechanisms, processes, and guidelines for maintaining the agent catalog and ensuring consistency across the system.

## Table of Contents

1. [Overview](#overview)
2. [Maintenance Schedule](#maintenance-schedule)
3. [Update Procedures](#update-procedures)
4. [Validation Commands](#validation-commands)
5. [Tag Management](#tag-management)
6. [Agent Lifecycle](#agent-lifecycle)
7. [Troubleshooting](#troubleshooting)

---

## Overview

### Purpose

The maintenance mechanism ensures:
- **Consistency** across all agent definitions
- **Accuracy** of agent metadata and tags
- **Discoverability** through proper categorization
- **Quality** through validation checks
- **Evolution** as the system grows

### Key Components

| Component | Description | Location |
|-----------|-------------|----------|
| Agent Registry | Central JSON registry with all agents | `agents/agents.json` |
| Agent Files | Individual markdown files | `agents/<id>.md` |
| Command Agents | System management commands | `commands/agent/*.toml` |
| Validation Scripts | Automated validation tools | `scripts/validate_agents.py` |

---

## Maintenance Schedule

### Daily Tasks

| Task | Frequency | Owner | Notes |
|------|-----------|-------|-------|
| Health Check | Daily | Automated | Monitor agent availability |
| Error Log Review | Daily | System | Review agent failures |

### Weekly Tasks

| Task | Frequency | Owner | Notes |
|------|-----------|-------|-------|
| Usage Analysis | Weekly | System | Review agent utilization |
| Tag Review | Weekly | Maintainer | Ensure tag consistency |

### Monthly Tasks

| Task | Frequency | Owner | Notes |
|------|-----------|-------|-------|
| Catalog Audit | Monthly | Maintainer | Full catalog review |
| Performance Review | Monthly | System | Agent performance metrics |
| Deprecation Check | Monthly | Maintainer | Identify outdated agents |

### Quarterly Tasks

| Task | Frequency | Owner | Notes |
|------|-----------|-------|-------|
| Major Review | Quarterly | Team | Comprehensive system review |
| Version Update | Quarterly | Maintainer | Catalog version bump |
| Strategy Planning | Quarterly | Team | Future direction |

---

## Update Procedures

### Adding a New Agent

#### Step 1: Create Agent File

```bash
# Create the agent markdown file
touch agents/my-new-agent.md
```

**Template for agent file:**

```markdown
---
id: my-new-agent
name: My New Agent
description: |
  Use this agent when [specific functionality is needed].
  Also trigger for: "[trigger phrase 1]", "[trigger phrase 2]"
category: utility
tags: [utility, medium-complexity, development, creation]
complexity: medium
version: 1.0.0
created: 2026-02-07
updated: 2026-02-07
use_cases:
  - "Use case 1"
  - "Use case 2"
outputs: [code, documentation]
trigger_examples:
  - "Example trigger 1"
  - "Example trigger 2"
---

# My New Agent

## Overview

[Brief overview]

## Core Responsibilities

1. [Responsibility 1]
2. [Responsibility 2]

## Process

### Step 1: [Step Name]
[Description]

### Step 2: [Step Name]
[Description]

## Constraints

- [Constraint 1]
- [Constraint 2]

## Examples

### Example 1
**Context:** [Situation]
**User:** "[User message]"
**Response:** "[Expected response]"

## Related Agents

- [Agent 1] - [Relationship]
- [Agent 2] - [Relationship]

---

**Last Updated:** 2026-02-07
**Version:** 1.0.0
```

#### Step 2: Register Agent

Edit `agents/agents.json`:

```json
{
  "id": "my-new-agent",
  "name": "My New Agent",
  "file": "my-new-agent.md",
  "category": "utility",
  "description": "Agent description",
  "use_cases": ["case1", "case2"],
  "tags": ["utility", "medium-complexity", "development", "creation"],
  "complexity": "medium",
  "outputs": ["code", "documentation"]
}
```

#### Step 3: Validate

```bash
# Run validation
python3 scripts/validate_agents.py

# Fix any issues
python3 scripts/fix_agents.py
```

#### Step 4: Update Catalog Documents

- Update [`docs/AGENT_CATALOG.md`](AGENT_CATALOG.md)
- Update [`docs/AGENT_USAGE_GUIDE.md`](AGENT_USAGE_GUIDE.md)

### Updating an Existing Agent

#### Step 1: Edit Agent File

Update the markdown file in `agents/<id>.md`

#### Step 2: Update Registry

Edit `agents/agents.json` with new metadata

#### Step 3: Update Version

```yaml
version: 1.1.0  # Increment minor version
updated: 2026-02-07
```

#### Step 4: Validate and Commit

```bash
# Validate changes
python3 scripts/validate_agents.py

# Check for issues
python3 scripts/fix_agents.py
```

### Deprecating an Agent

#### Step 1: Mark as Deprecated

Update agent file:

```markdown
---
deprecated: true
deprecation_date: 2026-02-07
replacement_agent: recommended-agent
---
```

Update registry:

```json
{
  "id": "deprecated-agent",
  "status": "deprecated",
  "deprecated_date": "2026-02-07",
  "replacement": "recommended-agent"
}
```

#### Step 2: Update Documentation

- Mark in catalog
- Add deprecation notice in usage guide

#### Step 3: Migration Period

| Phase | Duration | Action |
|-------|----------|--------|
| Warning | 30 days | Display deprecation warnings |
| Removal | 60 days | Remove from active registry |

#### Step 4: Complete Removal

After migration period, remove completely from registry

---

## Validation Commands

### Validate Agents

```bash
# Run full validation
python3 scripts/validate_agents.py

# Validate specific agent
python3 scripts/validate_agents.py --agent my-agent

# Check JSON syntax
python3 scripts/validate_agents.py --json

# Check tags
python3 scripts/validate_agents.py --tags
```

### Fix Agents

```bash
# Auto-fix common issues
python3 scripts/fix_agents.py

# Fix specific issues
python3 scripts/fix_agents.py --tags
python3 scripts/fix_agents.py --format
python3 scripts/fix_agents.py --ordering
```

### Test Integration

```bash
# Run integration tests
python3 scripts/test_integration.py

# Test specific agent
python3 scripts/test_integration.py --agent my-agent
```

### Health Check

```bash
# Check system health
just test

# Validate agents
just validate-agents

# Fix agents
just fix-agents
```

---

## Tag Management

### Tag Governance

#### Adding New Tags

1. **Propose Tag**: Create RFC document
2. **Review**: Discuss in team meeting
3. **Approve**: Get consensus
4. **Implement**: Add to tag definitions
5. **Migrate**: Update affected agents

#### Tag Naming Convention

| Tag Type | Format | Example |
|----------|--------|---------|
| Domain | `[domain]:[value]` | `engineering:code-generation` |
| Complexity | `[level]:complexity` | `medium-complexity` |
| Function | `[action]:[noun]` | `code-generation` |
| Output | `[type]` | `code` |

#### Tag Validation

```bash
# Check for unused tags
python3 scripts/validate_agents.py --unused-tags

# Check for tag consistency
python3 scripts/validate_agents.py --tag-consistency

# Generate tag report
python3 scripts/validate_agents.py --tag-report
```

### Tag Categories

#### Required Tags (Must Have)

| Tag Category | Tag | Required For |
|--------------|-----|--------------|
| Domain | `engineering`, `utility`, `creative`, `command` | All agents |
| Complexity | `low-complexity`, `medium-complexity`, `high-complexity` | All agents |

#### Recommended Tags (Should Have)

| Tag Category | Examples | Recommended For |
|--------------|----------|----------------|
| Function | `architecture`, `code-generation`, `analysis`, `review` | Most agents |
| Output | `code`, `documentation`, `text`, `guidance` | Most agents |

#### Optional Tags

| Tag Category | Examples | Optional For |
|--------------|----------|--------------|
| Dependency | `requires-json`, `requires-python`, `standalone` | Agents with special needs |
| Persona | `pirate`, `yoda`, `shakespeare` | Entertainment agents |

---

## Agent Lifecycle

### Phase 1: Proposal

**Activities:**
- Document agent requirements
- Identify scope and boundaries
- Check for overlap with existing agents

**Output:** Agent proposal document

### Phase 2: Development

**Activities:**
- Create agent file
- Implement prompt
- Add to registry
- Write tests

**Output:** Functional agent

### Phase 3: Review

**Activities:**
- Code review
- Prompt review
- Tag validation
- Documentation check

**Output:** Approved agent

### Phase 4: Deployment

**Activities:**
- Merge to main
- Update catalog
- Notify users
- Monitor usage

**Output:** Live agent

### Phase 5: Maintenance

**Activities:**
- Bug fixes
- Performance optimization
- Tag updates
- Documentation updates

**Output:** Maintained agent

### Phase 6: Deprecation

**Activities:**
- Mark as deprecated
- Provide replacement
- Migration period
- Complete removal

**Output:** Removed agent

---

## Troubleshooting

### Common Issues

#### Issue: Agent Not Found

**Symptoms:**
```
Error: Agent 'my-agent' not found
```

**Solutions:**

1. Check agent exists in registry
   ```bash
   cat agents/agents.json | jq '.agents[].id' | grep my-agent
   ```

2. Verify file exists
   ```bash
   ls agents/my-agent.md
   ```

3. Run validation
   ```bash
   python3 scripts/validate_agents.py
   ```

#### Issue: Invalid Tags

**Symptoms:**
```
Warning: Unknown tag 'my-tag'
```

**Solutions:**

1. Check tag spelling
2. Add tag to definitions
3. Remove invalid tag
4. Run fix script
   ```bash
   python3 scripts/fix_agents.py --tags
   ```

#### Issue: JSON Validation Failed

**Symptoms:**
```
Error: Invalid JSON in agents.json
```

**Solutions:**

1. Check syntax
   ```bash
   cat agents/agents.json | python3 -m json.tool > /dev/null
   ```

2. Fix formatting
   ```bash
   python3 scripts/fix_agents.py --format
   ```

#### Issue: Duplicate Agent ID

**Symptoms:**
```
Error: Duplicate agent ID 'my-agent'
```

**Solutions:**

1. Find duplicates
   ```bash
   cat agents/agents.json | jq '.agents[].id' | sort | uniq -d
   ```

2. Remove duplicate
3. Validate again

### Escalation

| Issue Type | Escalation To |
|------------|---------------|
| Bug in validation | System maintainer |
| Agent malfunction | Agent author |
| Catalog update | Agent system team |
| Security issue | Security team |

---

## References

- [Agent Catalog](AGENT_CATALOG.md)
- [Agent Specification](AGENT_SPECIFICATION.md)
- [Agent Usage Guide](AGENT_USAGE_GUIDE.md)
- [Agent System Guidelines](guidline-agents-system.md)
- [Validation Scripts](../scripts/)

---

**Document Owner:** Agent System Maintainers  
**Version:** 1.0.0  
**Last Updated:** 2026-02-07  
**Review Cycle:** Quarterly

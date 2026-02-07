# Agent System Catalog

This document provides a comprehensive catalog of all agents in the system, organized by functional groups with standardized tagging.

## Table of Contents

1. [Catalog Overview](#catalog-overview)
2. [Agent Groups](#agent-groups)
3. [Tagging System](#tagging-system)
4. [Agent Registry](#agent-registry)
5. [Selection Guidelines](#selection-guidelines)

---

## Catalog Overview

| Metric | Value |
|--------|-------|
| Total Agents | 52 |
| System Agents | 48 |
| Command Agents | 4 |
| Agent Groups | 8 |
| Last Updated | 2026-02-07 |

---

## Agent Groups

### Group 1: Engineering & Architecture

**Description:** Specialized agents for software architecture, design, and technical planning. Focus on high-level system design, feature architecture, and technical decision-making.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| architect | Architect | System design and architecture planning specialist |
| code-architect | Code Architect | Feature architecture design with implementation blueprints |
| cloudflare | Cloudflare | Cloudflare Workers and Agents development |
| fullstack-dev | Fullstack Dev | Full-stack application development |

**Tags:** `engineering`, `architecture`, `design`, `planning`, `high-complexity`

**Use Cases:**

- System design and microservice architecture planning
- Technology decision documentation
- Creating architectural diagrams (Mermaid)
- Designing feature implementations with blueprints
- Cloudflare Workers deployment

**Selection Criteria:**

- Use `architect` for broad system-level design
- Use `code-architect` for feature-specific design
- Use `cloudflare` for Cloudflare platform development
- Use `fullstack-dev` for complete application development

---

### Group 2: Code Development

**Description:** Agents focused on code creation, generation, and implementation. Provide direct code output for various programming tasks.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| code-generator | Code Generator | Streamlined code generation assistant |
| ui-engineer | UI Engineer | Frontend/UI development specialist |

**Tags:** `engineering`, `code-generation`, `frontend`, `implementation`, `medium-complexity`

**Use Cases:**

- Rapid code generation for functions and classes
- Creating boilerplate and prototyping
- React component development
- Responsive UI design

**Selection Criteria:**

- Use `code-generator` for backend logic and algorithms
- Use `ui-engineer` for frontend and UI components

---

### Group 3: Code Analysis & Review

**Description:** Agents for code analysis, review, and quality assurance. Focus on examining existing code for bugs, patterns, and improvements.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| code-reviewer | Code Reviewer | Code quality and bug detection |
| code-explorer | Code Explorer | Deep codebase analysis and tracing |
| codebase-analyzer | Codebase Analyzer | Implementation detail analysis |
| pr-reviewer | PR Reviewer | Pull request quality review |
| codacy | Codacy | Codacy MCP integration |

**Tags:** `engineering`, `analysis`, `review`, `quality`, `medium-complexity`

**Use Cases:**

- Reviewing code for bugs and security vulnerabilities
- Analyzing PRs before merge
- Understanding code execution paths
- Tracing data flow through codebase

**Selection Criteria:**

- Use `code-reviewer` for general code quality
- Use `pr-reviewer` for GitHub PR reviews
- Use `code-explorer` for deep architectural understanding
- Use `codebase-analyzer` for implementation details

---

### Group 4: Codebase Navigation

**Description:** Agents for discovering, locating, and finding code patterns within the codebase. Act as enhanced search tools.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| codebase-locator | Codebase Locator | File and component discovery |
| codebase-pattern-finder | Codebase Pattern Finder | Similar implementation search |

**Tags:** `utility`, `search`, `discovery`, `navigation`, `low-complexity`

**Use Cases:**

- Finding files related to a feature
- Locating code patterns and implementations
- Discovering relevant components

**Selection Criteria:**

- Use `codebase-locator` for file and directory discovery
- Use `codebase-pattern-finder` for code example search

---

### Group 5: Research & Analysis

**Description:** Agents for deep research, document analysis, and technical investigation. Focus on extracting insights from various sources.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| thoughts-analyzer | Thoughts Analyzer | Deep dive research on topics |
| research-analyzer | Research Analyzer | Research document analysis |
| web-search-researcher | Web Search Researcher | Web content fetching and analysis |
| docs-researcher | Docs Researcher | Library documentation retrieval |
| thoughts-locator | Thoughts Locator | Document discovery in thoughts/ |

**Tags:** `utility`, `research`, `analysis`, `documentation`, `medium-complexity`

**Use Cases:**

- Deep research on technical topics
- Analyzing research documents
- Fetching web documentation
- Discovering relevant thought documents

**Selection Criteria:**

- Use `thoughts-analyzer` for general research
- Use `research-analyzer` for document analysis
- Use `web-search-researcher` for web content
- Use `docs-researcher` for library docs
- Use `thoughts-locator` for thoughts directory

---

### Group 6: Documentation & Planning

**Description:** Agents for documentation creation, maintenance, and implementation planning. Focus on documentation quality and project planning.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| docbot-pro | Docbot Pro | Enterprise documentation specialist |
| insight-documenter | Insight Documenter | Technical breakthrough documentation |
| plan-implementation-reviewer | Plan Implementation Reviewer | Plan validation and execution review |

**Tags:** `utility`, `documentation`, `planning`, `validation`, `medium-complexity`

**Use Cases:**

- Generating API documentation
- Documenting technical discoveries
- Validating implementation against plans

**Selection Criteria:**

- Use `docbot-pro` for comprehensive documentation
- Use `insight-documenter` for technical insights
- Use `plan-implementation-reviewer` for plan validation

---

### Group 7: Development Tools

**Description:** Agents for creating and managing development artifacts like agents, commands, skills, and plugins. Focus on extensibility and tooling.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| agent-creator | Agent Creator | Creating new autonomous agents |
| command-creator | Command Creator | Creating Claude Code commands |
| skill-creator | Skill Creator | Creating new skills |
| skill-reviewer | Skill Reviewer | Skill quality review |
| plugin-validator | Plugin Validator | Plugin structure validation |
| task-management | Task Management | Task tracking and context management |
| testing-guidelines | Testing Guidelines | Testing best practices |

**Tags:** `utility`, `development`, `creation`, `tooling`, `medium-complexity`

**Use Cases:**

- Creating new agents for plugins
- Building Claude Code commands
- Developing skills for extending capabilities
- Validating plugin structure

**Selection Criteria:**

- Use `agent-creator` for agent creation
- Use `command-creator` for command creation
- Use `skill-creator` for skill development
- Use `skill-reviewer` for skill review
- Use `plugin-validator` for plugin validation

---

### Group 8: Session & Workflow

**Description:** Agents for workflow management, session analysis, and project orchestration. Focus on process improvement and learning capture.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| orchestrator | Orchestrator | Task delegation and routing |
| deep-reflector | Deep Reflector | Session analysis and learning capture |
| instruction-reflector | Instruction Reflector | CLAUDE.md optimization |
| prompt-engineering-patterns | Prompt Engineering Patterns | Prompt design and optimization |
| prompt-template-system | Prompt Template System | Template system management |
| github-issue-fixer | Github Issue Fixer | GitHub issue resolution |
| kiro-workflow | Kiro Workflow | Workflow management |
| workflow-diagrams | Workflow Diagrams | Diagram creation |
| hookify-conversation-analyzer | Hookify Conversation Analyzer | Conversation analysis |
| reporter | Reporter | Reporting utility |
| human-3-coach | Human 3 Coach | Human-3 coaching |

**Tags:** `utility`, `workflow`, `orchestration`, `session`, `medium-high-complexity`

**Use Cases:**

- Delegating complex tasks to specialists
- Capturing session learnings
- Optimizing AI instructions
- Fixing GitHub issues

**Selection Criteria:**

- Use `orchestrator` for complex task delegation
- Use `deep-reflector` for session learning
- Use `instruction-reflector` for instruction optimization
- Use `github-issue-fixer` for issue resolution

---

### Group 9: Agent SDK Verification

**Description:** Specialized agents for verifying Agent SDK applications follow best practices and are deployment-ready.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| agent-sdk-verifier-py | Agent SDK Verifier Py | Python SDK verification |
| agent-sdk-verifier-ts | Agent SDK Verifier TS | TypeScript SDK verification |

**Tags:** `engineering`, `verification`, `sdk`, `deployment`, `medium-complexity`

**Use Cases:**

- Verifying Python Agent SDK applications
- Verifying TypeScript Agent SDK applications

**Selection Criteria:**

- Use `agent-sdk-verifier-py` for Python apps
- Use `agent-sdk-verifier-ts` for TypeScript apps

---

### Group 10: Creative & Entertainment

**Description:** Agents with distinctive personalities for creative writing, storytelling, and entertainment.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| creative-writer | Creative Writer | Creative writing specialist |
| comedian | Comedian | Dad joke specialist |
| pirate | Pirate | Pirate dialect assistant |
| yoda | Yoda | Yoda-speak assistant |
| shakespeare | Shakespeare | Shakespearean English assistant |
| cowboy | Cowboy | Old West dialect assistant |
| gen-z | Gen Z | Gen Z slang assistant |

**Tags:** `creative`, `entertainment`, `persona`, `writing`, `low-complexity`

**Use Cases:**

- Creative writing and poetry
- Entertainment and mood lifting
- Educational entertainment

**Selection Criteria:**

- Use based on desired personality/voice

---

### Group 11: File Conversion

**Description:** Specialized agents for file format conversion tasks.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| pdf-to-ppt | Pdf To Ppt | PDF to PowerPoint conversion |

**Tags:** `utility`, `conversion`, `file`, `document`, `low-complexity`

**Use Cases:**

- Converting PDF documents to PowerPoint presentations

---

### Group 12: Command Agents

**Description:** Command-line agents for managing and interacting with the agent system itself.

**Agents in Group:**

| ID | Name | Description |
|----|------|-------------|
| examples | Examples | Show example prompts and use cases |
| info | Info | Show detailed agent information |
| new | New | Create and register new agents |
| switch | Switch | Show how to switch agents |

**Tags:** `command`, `management`, `system`, `cli`, `low-complexity`

**Use Cases:**

- Exploring agent examples
- Getting agent information
- Creating new agents
- Switching between agents

**Selection Criteria:**

- Use `examples` to see agent use cases
- Use `info` for agent details
- Use `new` to create agents
- Use `switch` for switching guidance

---

## Tagging System

### Tag Categories

#### 1. Domain Category

| Tag | Description | Agents |
|-----|-------------|--------|
| `engineering` | Software development and engineering tasks | architect, code-generator, code-reviewer, etc. |
| `utility` | General utility and tool agents | codebase-locator, docbot-pro, etc. |
| `creative` | Creative and entertainment agents | creative-writer, comedian, etc. |
| `command` | System command and management agents | examples, info, new, switch |

#### 2. Complexity Level

| Tag | Description | Criteria |
|-----|-------------|----------|
| `low-complexity` | Simple, single-purpose tasks | Quick operations, minimal context |
| `medium-complexity` | Moderate tasks with multiple steps | Moderate context, several outputs |
| `high-complexity` | Complex tasks with extensive analysis | Deep context, multiple outputs |

#### 3. Primary Function

| Tag | Description | Examples |
|-----|-------------|----------|
| `architecture` | System and feature design | architect, code-architect |
| `code-generation` | Writing and generating code | code-generator, ui-engineer |
| `analysis` | Code and document analysis | code-explorer, research-analyzer |
| `review` | Quality review and validation | code-reviewer, pr-reviewer |
| `search` | Discovery and navigation | codebase-locator, thoughts-locator |
| `research` | Deep investigation | thoughts-analyzer, web-search-researcher |
| `documentation` | Documentation creation | docbot-pro, insight-documenter |
| `development` | Tool and artifact creation | agent-creator, skill-creator |
| `workflow` | Process orchestration | orchestrator, deep-reflector |
| `entertainment` | Fun and creative output | comedian, pirate |
| `conversion` | File format conversion | pdf-to-ppt |
| `management` | System and agent management | examples, info, new, switch |

#### 4. Output Type

| Tag | Description | Examples |
|-----|-------------|----------|
| `code` | Primary output is code | code-generator, code-reviewer |
| `documentation` | Primary output is documentation | docbot-pro, insight-documenter |
| `text` | Primary output is text/analysis | research-analyzer, thoughts-analyzer |
| `guidance` | Primary output is instructions | switch, examples |
| `persona` | Primary output in specific voice | pirate, yoda, shakespeare |

#### 5. Dependencies

| Tag | Description | Examples |
|-----|-------------|----------|
| `requires-json` | Requires registry JSON access | examples, info |
| `requires-python` | Requires Python script execution | info, switch |
| `requires-file-system` | Requires file read/write | new, agent-creator |
| `requires-mcp` | Requires MCP server access | codacy, web-search-researcher |
| `standalone` | No external dependencies | code-reviewer, creative-writer |

---

## Agent Registry

### Complete Agent List (Alphabetical)

| ID | Name | Group | Tags | Category |
|----|------|-------|------|----------|
| agent-creator | Agent Creator | Development Tools | `utility`, `development`, `creation`, `medium-complexity` | utility |
| agent-sdk-verifier-py | Agent SDK Verifier Py | Agent SDK Verification | `engineering`, `verification`, `sdk`, `medium-complexity` | engineering |
| agent-sdk-verifier-ts | Agent SDK Verifier TS | Agent SDK Verification | `engineering`, `verification`, `sdk`, `medium-complexity` | engineering |
| architect | Architect | Engineering & Architecture | `engineering`, `architecture`, `design`, `high-complexity` | engineering |
| cloudflare | Cloudflare | Engineering & Architecture | `engineering`, `cloud`, `workers`, `medium-complexity` | engineering |
| code-architect | Code Architect | Engineering & Architecture | `engineering`, `architecture`, `design`, `medium-complexity` | engineering |
| code-explorer | Code Explorer | Code Analysis & Review | `engineering`, `analysis`, `exploration`, `medium-complexity` | engineering |
| code-generator | Code Generator | Code Development | `engineering`, `code-generation`, `implementation`, `medium-complexity` | engineering |
| code-reviewer | Code Reviewer | Code Analysis & Review | `engineering`, `review`, `quality`, `medium-complexity` | engineering |
| codebase-analyzer | Codebase Analyzer | Code Analysis & Review | `engineering`, `analysis`, `implementation`, `medium-complexity` | utility |
| codebase-locator | Codebase Locator | Codebase Navigation | `utility`, `search`, `discovery`, `low-complexity` | utility |
| codebase-pattern-finder | Codebase Pattern Finder | Codebase Navigation | `utility`, `search`, `patterns`, `low-complexity` | utility |
| codacy | Codacy | Code Analysis & Review | `engineering`, `analysis`, `mcp`, `low-complexity` | utility |
| comedian | Comedian | Creative & Entertainment | `creative`, `entertainment`, `persona`, `low-complexity` | entertainment |
| command-creator | Command Creator | Development Tools | `utility`, `development`, `creation`, `medium-complexity` | utility |
| cowboy | Cowboy | Creative & Entertainment | `creative`, `entertainment`, `persona`, `low-complexity` | comedy |
| creative-writer | Creative Writer | Creative & Entertainment | `creative`, `writing`, `persona`, `low-complexity` | creative |
| deep-reflector | Deep Reflector | Session & Workflow | `utility`, `workflow`, `session`, `medium-complexity` | utility |
| docbot-pro | Docbot Pro | Documentation & Planning | `utility`, `documentation`, `enterprise`, `medium-complexity` | utility |
| docs-researcher | Docs Researcher | Research & Analysis | `utility`, `research`, `documentation`, `low-complexity` | utility |
| examples | Examples | Command Agents | `command`, `management`, `exploration`, `low-complexity` | command |
| fullstack-dev | Fullstack Dev | Engineering & Architecture | `engineering`, `fullstack`, `development`, `high-complexity` | utility |
| gen-z | Gen Z | Creative & Entertainment | `creative`, `entertainment`, `persona`, `low-complexity` | comedy |
| github-issue-fixer | Github Issue Fixer | Session & Workflow | `utility`, `github`, `workflow`, `medium-complexity` | utility |
| hookify-conversation-analyzer | Hookify Conversation Analyzer | Session & Workflow | `utility`, `analysis`, `workflow`, `medium-complexity` | utility |
| human-3-coach | Human 3 Coach | Session & Workflow | `utility`, `coaching`, `workflow`, `medium-complexity` | utility |
| info | Info | Command Agents | `command`, `management`, `information`, `low-complexity` | command |
| insight-documenter | Insight Documenter | Documentation & Planning | `utility`, `documentation`, `technical`, `medium-complexity` | utility |
| instruction-reflector | Instruction Reflector | Session & Workflow | `utility`, `optimization`, `instructions`, `medium-complexity` | utility |
| kiro-workflow | Kiro Workflow | Session & Workflow | `utility`, `workflow`, `management`, `medium-complexity` | utility |
| new | New | Command Agents | `command`, `management`, `creation`, `low-complexity` | command |
| orchestrator | Orchestrator | Session & Workflow | `utility`, `orchestration`, `delegation`, `high-complexity` | engineering |
| pdf-to-ppt | Pdf To Ppt | File Conversion | `utility`, `conversion`, `file`, `low-complexity` | utility |
| pirate | Pirate | Creative & Entertainment | `creative`, `entertainment`, `persona`, `low-complexity` | comedy |
| plan-implementation-reviewer | Plan Implementation Reviewer | Documentation & Planning | `utility`, `planning`, `validation`, `medium-complexity` | utility |
| plugin-validator | Plugin Validator | Development Tools | `utility`, `validation`, `plugin`, `medium-complexity` | utility |
| pr-reviewer | PR Reviewer | Code Analysis & Review | `engineering`, `review`, `github`, `medium-complexity` | utility |
| prompt-engineering-patterns | Prompt Engineering Patterns | Session & Workflow | `utility`, `prompts`, `optimization`, `medium-complexity` | utility |
| prompt-template-system | Prompt Template System | Session & Workflow | `utility`, `templates`, `prompts`, `medium-complexity` | utility |
| research-analyzer | Research Analyzer | Research & Analysis | `utility`, `research`, `analysis`, `medium-complexity` | utility |
| reporter | Reporter | Session & Workflow | `utility`, `reporting`, `workflow`, `low-complexity` | utility |
| shakespeare | Shakespeare | Creative & Entertainment | `creative`, `entertainment`, `persona`, `low-complexity` | comedy |
| skill-creator | Skill Creator | Development Tools | `utility`, `development`, `creation`, `medium-complexity` | utility |
| skill-reviewer | Skill Reviewer | Development Tools | `utility`, `development`, `review`, `medium-complexity` | utility |
| switch | Switch | Command Agents | `command`, `management`, `switching`, `low-complexity` | command |
| task-management | Task Management | Development Tools | `utility`, `task`, `tracking`, `low-complexity` | utility |
| testing-guidelines | Testing Guidelines | Development Tools | `utility`, `testing`, `guidelines`, `low-complexity` | utility |
| thoughts-analyzer | Thoughts Analyzer | Research & Analysis | `utility`, `research`, `analysis`, `medium-complexity` | utility |
| thoughts-locator | Thoughts Locator | Research & Analysis | `utility`, `search`, `discovery`, `low-complexity` | utility |
| ui-engineer | UI Engineer | Code Development | `engineering`, `frontend`, `ui`, `medium-complexity` | utility |
| web-search-researcher | Web Search Researcher | Research & Analysis | `utility`, `research`, `web`, `medium-complexity` | utility |
| workflow-diagrams | Workflow Diagrams | Session & Workflow | `utility`, `diagrams`, `workflow`, `low-complexity` | utility |
| yoda | Yoda | Creative & Entertainment | `creative`, `entertainment`, `persona`, `low-complexity` | comedy |

---

## Selection Guidelines

### Quick Reference Flowchart

```
Start
  |
  ├─ Need to CREATE something?
  |   ├─ New agent? → agent-creator
  |   ├─ New command? → command-creator
  |   ├─ New skill? → skill-creator
  |   └─ Full app? → fullstack-dev
  |
  ├─ Need to ANALYZE existing code?
  |   ├─ Deep understanding? → code-explorer
  |   ├─ Implementation details? → codebase-analyzer
  |   ├─ Code quality review? → code-reviewer
  |   ├─ PR review? → pr-reviewer
  |   └─ Find patterns? → codebase-pattern-finder
  |
  ├─ Need to FIND something?
  |   ├─ Files? → codebase-locator
  |   ├─ Code patterns? → codebase-pattern-finder
  |   ├─ Docs? → docs-researcher
  |   ├─ Web content? → web-search-researcher
  |   └─ Thoughts? → thoughts-locator
  |
  ├─ Need to DESIGN something?
  |   ├─ System architecture? → architect
  |   └─ Feature architecture? → code-architect
  |
  ├─ Need to DOCUMENT something?
  |   ├─ Enterprise docs? → docbot-pro
  |   ├─ Technical insights? → insight-documenter
  |   └─ Validate plan? → plan-implementation-reviewer
  |
  ├─ Need to MANAGE agents?
  |   ├─ See examples? → examples
  |   ├─ Get info? → info
  |   ├─ Create new? → new
  |   └─ Switch agent? → switch
  |
  └─ Need HELP with workflow?
      ├─ Delegate task? → orchestrator
      ├─ Capture learnings? → deep-reflector
      ├─ Optimize instructions? → instruction-reflector
      └─ Fix GitHub issue? → github-issue-fixer
```

### Decision Matrix

| Task Type | Best Agent | Alternative Agents |
|-----------|------------|---------------------|
| Write code | code-generator | fullstack-dev, ui-engineer |
| Review code | code-reviewer | pr-reviewer |
| Design system | architect | code-architect |
| Find files | codebase-locator | codebase-pattern-finder |
| Research topic | thoughts-analyzer | research-analyzer |
| Get docs | docs-researcher | web-search-researcher |
| Create agent | agent-creator | new (command) |
| Create skill | skill-creator | - |
| Switch agent | switch | - |
| Full development | orchestrator | fullstack-dev |

---

## Maintenance

### Update Frequency

- **Security Updates:** Immediate
- **Bug Fixes:** As needed
- **New Agents:** Monthly review
- **Catalog Refresh:** Quarterly

### Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-02-06 | Initial catalog |
| 0.2.0 | 2026-02-07 | Added command agents, refined groups |

---

**Document Owner:** Agent System Maintainers  
**Last Updated:** 2026-02-07

# 🤖 System Agents Manager (v0.2.0)

> **Turn your Gemini CLI into a Multi-Persona AI Team.**

This extension transforms the generic Gemini CLI assistant into a specialized workforce. It allows you to manage, switch, and maintain a library of "System Agents"—specialized system prompts designed for specific tasks like Architecture, Coding, Writing, or Entertainment.

## 🌟 Key Features

*   **📚 Curated Library:** 40+ pre-built agents (Architect, Code Generator, Pirate, Yoda, etc.).
*   **🐍 Python-Powered:** Robust management logic handled by Python scripts, ensuring stability and speed.
*   **✅ Auto-Validation:** Built-in integrity checks and auto-fixers to ensure all agents are valid and registered.
*   **🏗️ Structured Personas:** Advanced XML-based prompts for core agents (Architect, Code Generator) to minimize hallucinations.

## 📦 Installation

This extension is designed to be dropped into your Gemini CLI extensions directory.

## 🚀 Usage

### List Available Agents
View all built-in and custom agents in a clean table.
```bash
/system-agent
```

### Get Agent Details
See the full profile, personality, and use cases for a specific agent.
```bash
/system-agent:info architect
```

### Switch Agent
Generate the commands needed to switch your persona.
```bash
/system-agent:switch pirate
```

## 🧠 Core Agents

We have optimized our top-tier agents with structured XML prompting:

| ID | Name | Role |
| :--- | :--- | :--- |
| **`architect`** | **Software Architect** | Research, Planning, Diagrams. *No Coding.* |
| **`code-generator`** | **Code Generator** | Production-ready implementation. *No Chatter.* |
| **`creative-writer`** | **Creative Writer** | Poetry, Prose, Storytelling. |
| **`pirate`** | **Pirate Assistant** | Technical help in a swashbuckling dialect. |

## 🛠️ Developer Guide

### Architecture
The extension uses a Python-bridge architecture:
*   `commands/*.toml`: Simple entry points that call Python scripts.
*   `scripts/agent_manager.py`: Core logic for listing, finding, and inspecting agents.
*   `scripts/validate_agents.py`: CI/CD-like script to verify file integrity.
*   `scripts/fix_agents.py`: Auto-fixer for metadata and registration issues.

### Adding a Custom Agent
1.  Create a `.md` file in the `custom/` directory (create if not exists).
2.  Add YAML frontmatter:
    ```markdown
    ---
    name: my-agent
    description: Does cool things
    category: utility
    ---
    ```
3.  Run `/system-agent` to confirm it's loaded (the script auto-detects `custom/*.md`).

### Maintenance
Run the validation suite to ensure everything is healthy:
```bash
python3 scripts/validate_agents.py
```

## 📜 License
MIT
# 🤖 Gemini Agents Manager Extension

![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Status](https://img.shields.io/badge/status-active-success.svg)
![Python](https://img.shields.io/badge/python-3.8+-yellow.svg)

> **Turn your Gemini CLI into a Multi-Persona AI Team.**

This extension transforms the generic Gemini CLI assistant into a specialized workforce. It allows you to manage, switch, and maintain a library of **"System Agents"**—specialized system prompts designed for specific domains like Architecture, Coding, Writing, or Entertainment.

By swapping the underlying persona, you ensure that the AI follows strict behavioral rules, output formats, and domain-specific best practices, significantly reducing hallucinations and improving task performance.

---

## 📖 Table of Contents

- [Features](#-features)
- [Installation](#-installation)
- [Quick Start](#-quick-start)
- [Available Agents](#-available-agents)
- [Core Agent Profiles](#-core-agent-profiles)
- [Configuration](#-configuration)
- [Development](#-development)
- [Troubleshooting](#-troubleshooting)
- [Contributing](#-contributing)
- [License](#-license)

---

## ✨ Features

*   **📚 Curated Agent Library:** Access 40+ pre-built, high-quality agents including Software Architect, Code Generator, Pirate, Yoda, and more.
*   **🐍 Python-Powered Core:** Robust management logic handled by efficient Python scripts for listing, finding, and inspecting agents.
*   **✅ Automated Integrity:** Built-in validation suite and auto-fixers ensure your agent library remains clean, structured, and error-free.
*   **🏗️ Structured XML Personas:** Top-tier agents use advanced XML-based prompting strategies to enforce strict operational boundaries and output formats.
*   **🧠 Orchestrator Mode:** Includes a "Team Lead" agent capable of analyzing tasks and recommending the best expert for the job.

---

## 📦 Installation

This extension is designed to be installed directly into your Gemini CLI extensions directory.

### Prerequisites
*   Gemini CLI installed and configured.
*   Python 3.8 or higher installed on your system.

### Setup
1.  Navigate to your extensions directory:
    ```bash
    cd ~/.gemini/extensions/
    ```
2.  Clone this repository:
    ```bash
    git clone https://github.com/billlzzz18/bl1nk-agents-manager.git agents-manager
    ```
3.  The extension is now active. Verify installation by listing available agents:
    ```bash
    gemini /system-agent
    ```

---

## 🚀 Quick Start

### 1. List Available Agents
View all built-in and custom agents in a clean, categorized table.
```bash
/system-agent
```

### 2. Inspect an Agent
See the full profile, personality, use cases, and description for a specific agent.
```bash
/system-agent:info architect
```

### 3. Switch Persona
Generate the commands needed to switch your active agent.
```bash
/system-agent:switch pirate
```
*Follow the on-screen instructions to export the environment variable and restart your session.*

---

## 🧠 Core Agent Profiles

We have optimized our top-tier agents with structured XML prompting for maximum reliability.

| ID | Name | Role | Strengths |
| :--- | :--- | :--- | :--- |
| **`orchestrator`** | **Team Lead** | Router | Analyzes complex tasks and delegates them to the right expert. |
| **`architect`** | **Software Architect** | Planner | Research, system design, Mermaid diagrams. *Strictly no coding.* |
| **`code-generator`** | **Code Generator** | Implementer | Production-ready, clean code. *Minimal chatter.* |
| **`creative-writer`** | **Creative Writer** | Artist | Poetry, prose, storytelling, and literary adaptation. |
| **`pirate`** | **Pirate Assistant** | Fun | Technical help delivered in an authentic swashbuckling dialect. |

---

## ⚙️ Configuration

### Adding Custom Agents
You can extend the library with your own private agents.

1.  Create the `custom/` directory in the extension root if it doesn't exist.
2.  Create a new `.md` file (e.g., `custom/my-specialist.md`).
3.  Add the required YAML frontmatter:
    ```markdown
    ---
    name: my-specialist
    description: A brief description of what this agent does.
    category: engineering
    ---

    Your system prompt goes here...
    ```
4.  Run `/system-agent` to verify it has been detected.

---

## 🛠️ Development

### Project Structure
The extension uses a hybrid architecture for performance and maintainability:

```text
/
├── gemini-extension.json   # Manifest file
├── agents/                 # Built-in agent definitions (*.md + agents.json)
├── custom/                 # Directory for user-defined agents
├── commands/               # TOML entry points for CLI commands
└── scripts/                # Python logic core
    ├── agent_manager.py    # Main logic for CLI commands
    ├── validate_agents.py  # CI/CD integrity checker
    └── fix_agents.py       # Auto-repair tool for metadata
```

### Running Tests
To ensure the integrity of the agent library:

```bash
# Validate all agents (Frontmatter & Registry check)
python3 scripts/validate_agents.py
```

### Auto-Fixing Issues
If you add new agents manually and don't want to update `agents.json` by hand:

```bash
# Automatically detects new files, fixes frontmatter, and updates registry
python3 scripts/fix_agents.py
```

---

## ❓ Troubleshooting

**Q: I switched agents but nothing changed?**
A: Switching agents requires restarting the Gemini CLI session. Ensure you ran the `export` command provided by `/system-agent:switch` and then restarted the tool.

**Q: My custom agent isn't showing up.**
A: Ensure your file is in the `custom/` folder and ends with `.md`. Try running `python3 scripts/fix_agents.py` to automatically register it.

**Q: `python3` command not found.**
A: This extension relies on Python 3. Please install it and ensure it's in your system PATH.

---

## 🤝 Contributing

We welcome contributions! Whether it's a new persona, a bug fix, or a feature request.

1.  **Fork** the repository.
2.  **Create** a feature branch (`git checkout -b feature/new-agent`).
3.  **Add** your changes (if adding an agent, please use the XML structure if possible).
4.  **Validate** your changes (`python3 scripts/validate_agents.py`).
5.  **Commit** and **Push**.
6.  Open a **Pull Request**.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ for the Gemini CLI Community.**

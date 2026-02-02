# 🤖 System Agents Manager Extension

**Version:** 0.2.0
**Author:** billlzzz18 <team@bl1nk.site>
**Extension ID:** `bl1nk-agents`

This extension provides a powerful framework for managing and using specialized **System Agents** within the Gemini CLI. It allows users to switch between different personas (e.g., Software Architect, Creative Writer, Pirate) by swapping the underlying system prompt.

---

## 🌟 What are System Agents?

System Agents are specialized `system.md` files that define how the Gemini CLI behaves. Instead of a generic assistant, you can load a specific persona with expert knowledge, unique speech patterns, or strict behavioral constraints.

This extension provides:
1.  **A Curated Library**: High-quality, pre-tested agents for engineering, writing, and entertainment.
2.  **Management Commands**: CLI tools to list, inspect, and switch between agents.
3.  **Extensibility**: A structure to add your own custom agents.

---

## 🎭 Available Agents

The extension comes with a built-in library of agents located in the `agents/` directory:

### 🛠️ Engineering & Development
*   **Software Architect** (`architect`): Focuses on design, patterns, and documentation. Does *not* write implementation code.
*   **Code Generator** (`code-generator`): Streamlined, efficient coder. Minimal chatter, maximum code.

### ✍️ Creative
*   **Creative Writer** (`creative-writer`): Expert in poetry, prose, storytelling, and literary adaptation.

### 🎪 Entertainment & Comedy
*   **Dad Joke Comedian** (`comedian`): Responds to everything with a dad joke.
*   **Pirate** (`pirate`): Technical help delivered in authentic pirate dialect.
*   **Shakespeare** (`shakespeare`): Codes and speaks in iambic pentameter/Elizabethan English.
*   **Yoda** (`yoda`): Helpful he is. Code he will fix.
*   **Cowboy** (`cowboy`): Folksy wisdom and straight-shooting technical advice.
*   **Gen Z** (`gen-z`): Technical support, no cap.

---

## 🚀 Commands

This extension registers the `/system-agent` command namespace.

| Command | Description | Usage |
| :--- | :--- | :--- |
| **`/system-agent`** | List all available agents (built-in and custom). | `/system-agent` |
| **`/system-agent:info`** | Get detailed metadata and description for a specific agent. | `/system-agent:info <agent_id>` |
| **`/system-agent:switch`** | Get instructions and commands to switch your active agent. | `/system-agent:switch <agent_id>` |
| **`/system-agent:examples`** | Show example prompts and use cases for an agent. | `/system-agent:examples <agent_id>` |
| **`/system-agent:new`** | Interactive wizard to create a new custom agent. | `/system-agent:new` |

---

## 💡 How to Switch Agents

**Important:** You cannot switch agents in the *middle* of a running session because the system prompt is loaded at startup. To switch agents, you must set the `GEMINI_SYSTEM_MD` environment variable and start a new session.

The `/system-agent:switch` command will generate the exact commands you need.

**Common Methods:**

1.  **Temporary (One-off session):**
    ```bash
    GEMINI_SYSTEM_MD=~/.gemini/extensions/agents-manager/agents/pirate.md gemini
    ```

2.  **Persistent (Until shell exit):**
    ```bash
    export GEMINI_SYSTEM_MD=~/.gemini/extensions/agents-manager/agents/architect.md
    gemini
    ```

3.  **Aliases (Recommended):**
    Add these to your `.bashrc` or `.zshrc`:
    ```bash
    alias gemini-pirate="GEMINI_SYSTEM_MD=~/.gemini/extensions/agents-manager/agents/pirate.md gemini"
    alias gemini-code="GEMINI_SYSTEM_MD=~/.gemini/extensions/agents-manager/agents/code-generator.md gemini"
    ```

---

## 📂 Project Structure

For developers extending this project:

```text
/
├── gemini-extension.json   # Extension manifest
├── agents/                 # Built-in agent definitions
│   ├── agents.json         # Registry of built-in agents
│   ├── *.md                # The actual system prompt files
│   └── README.md           # Documentation for agents
└── commands/               # Command definitions (.toml)
    ├── system-agent.toml   # Main /system-agent command
    └── agent/              # Subcommands (:switch, :info, etc.)
```

### Adding a New Agent

1.  Create a new `.md` file in `agents/`.
2.  Add the agent's metadata to `agents/agents.json`.
3.  (Optional) If it's a "custom" user agent (not built-in), the logic in `system-agent.toml` looks for a `custom/agents.json` file relative to the extension path.
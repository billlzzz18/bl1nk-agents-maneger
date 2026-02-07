# bl1nk-server

The HTTP and WebSocket interface for the System Agents Manager, built on the Rocket framework.

## Overview

`bl1nk-server` acts as the bridge between the `bl1nk-core` logic and external users or UIs. It provides a RESTful API for managing conversations, agents, and filesystem resources, as well as a WebSocket endpoint for real-time event streaming.

## Features

- **Rocket Web Framework**: Robust and type-safe HTTP handling.
- **WebSocket Streaming**: Real-time updates from the backend using WebSockets.
- **Shared State Architecture**: Efficiently shares the `Orchestrator` and `GeminiBackend` across all routes.
- **Automatic Frontend Serving**: Capable of serving a frontend UI (currently placeholders).
- **Graceful Shutdown**: Integrated with Rocket's shutdown triggers.

## API Endpoints

The server exposes endpoints under the `/api` prefix:

### Session & Messages

- `POST /api/start-session`: Initialize a new agent session.
- `POST /api/send-message`: Send a message to the active agent.
- `POST /api/generate-title`: Automated conversation titling.

### Project & Chat Management

- `GET /api/projects`: List available projects.
- `GET /api/recent-chats`: Fetch history of recent interactions.
- `GET /api/conversations/<id>`: Get full details of a specific chat.

### Filesystem & Git

- `POST /api/list-directory`: Explore the filesystem.
- `POST /api/read-file-content`: Read source code or data.
- `POST /api/get-git-info`: Retrieve branch and status information.

### Real-time events

- `GET /api/ws`: WebSocket endpoint for event emission (tool confirmations, agent thoughts, etc.).

## Usage

Start the server using cargo:

```bash
cargo run --package bl1nk-server -- --log-level info
```

By default, the server listens on `0.0.0.0:1858`.

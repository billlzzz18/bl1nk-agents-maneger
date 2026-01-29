# Gemini MCP Proxy Project Context

Gemini MCP Proxy is an open-source, dual-mode orchestrator that brings an intelligent, multi-agent architecture to any terminal environment. It is designed to be a high-performance, extensible, and robust backend for AI agents like Gemini CLI.

## Project Overview

-   **Purpose:** Provide a high-performance Rust backend that acts as both an MCP Server for a main agent (like Gemini CLI) and an ACP Client to manage a community of sub-agents. It features an intelligent, policy-based routing engine and optional built-in capabilities via PMAT.
-   **Main Technologies:**
    -   **Language:** Rust (Stable, 1.82+)
    -   **Async Runtime:** Tokio
    -   **Core Libraries:** `pmcp` (MCP SDK), `clap` (CLI), `serde` (Serialization), `tracing` (Logging)
    -   **Testing:** `tokio-test`, `pretty_assertions`, `mockall`
    -   **Linting/Formatting:** `clippy`, `rustfmt`
-   **Architecture:** Monorepo structure using Cargo Workspaces.
    -   `crates/core`: The main Rust backend, containing all logic for the orchestrator, routing engine, and agent execution.
    -   `schemas/`: JSON Schema definitions for all configuration files (`settings.schema.json`, `policy.schema.json`, etc.).
    -   `agents/`: Directory for user-defined external agent configurations (`.agent.toml`).
    -   `skills/`: Directory for user-defined skills (`.skill.md`).

## Building and Running

-   **Install Dependencies:** `rustup update` and run `make setup` to install helper tools.
-   **Build All (Bundled PMAT):** `make build-bundle` (Recommended for full functionality)
-   **Build Standard:** `make build` (Lightweight, requires external agents)
-   **Run in Development:** `make dev` (Uses `cargo-watch` for hot-reloading)
-   **Run Production:** `make run` (Runs the release binary)
-   **Clean Artifacts:** `make clean`

## Testing and Quality

-   **Test Commands:**
    -   **Run All Tests:** `make test` (Runs with `--all-features`)
    -   **Run Linter:** `make clippy` (Enforces strict warnings)
    -   **Check Formatting:** `make fmt`
-   **Full Validation:** Run `make check`, `make fmt`, `make clippy`, and `make test` before submitting a PR.

## Development Conventions

-   **Contributions:** Follow the process outlined in `CONTRIBUTING.md`.
-   **Pull Requests:** Keep PRs small, focused, and linked to an existing issue.
-   **Commit Messages:** Follow the [Conventional Commits](https://www.conventionalcommits.org/) standard.
-   **Coding Style:** Adhere to Rust best practices and the patterns established in the `crates/core` package. Use `rustfmt` and `clippy` to enforce style.
-   **Error Handling:** Use the `anyhow` crate for application-level errors and `thiserror` for library-level, specific error types.

## Testing Conventions

-   **Environment Variables:** When testing code that depends on environment variables, use `std::env::set_var` within tests marked with `#[serial]` (from the `serial_test` crate) to prevent race conditions. Always clean up variables in the test's drop scope or using a custom guard.
-   **Filesystem Operations:** Use the `tempfile` crate to create temporary directories and files for tests that require filesystem interaction. This ensures tests are isolated and do not leave artifacts.

## Documentation

-   **CRITICAL:** Always use the `docs-writer` skill when you are asked to write, edit, or review any documentation for this project.
-   The main user-facing documentation is the `README.md` in the root of the workspace.
-   Architectural decisions and deep-dives should be documented in the `docs/` directory.
-   Suggest documentation updates when your code changes render existing documentation obsolete or incomplete.


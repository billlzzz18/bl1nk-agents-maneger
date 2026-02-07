# Atlas Agent

This agent manages the Atlas orchestrator functionality in the Bl1nk Agents Manager. It provides specialized functionality for the Atlas agent which coordinates complex multi-agent workflows.

## Purpose

The Atlas Agent handles orchestration operations, including:
- Multi-agent workflow coordination
- Task delegation management
- Session state orchestration
- Complex operation sequencing

## Components

- `index.rs`: Main entry point and hook registration
- `types.rs`: Type definitions and interfaces

## Usage

This agent is invoked when Atlas orchestrator operations are performed. It coordinates complex workflows involving multiple agents and manages the overall session state during orchestration.

## Configuration

The agent uses built-in orchestration patterns and does not require specific configuration.
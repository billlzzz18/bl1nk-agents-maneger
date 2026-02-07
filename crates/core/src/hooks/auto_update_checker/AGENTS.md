# Auto Update Checker Agent

This agent manages automatic update checking in the Bl1nk Agents Manager. It provides specialized functionality for checking and notifying about available updates.

## Purpose

The Auto Update Checker agent handles automatic update checking, including:
- Version comparison with remote releases
- Update notification generation
- Update availability verification
- Silent update checking

## Components

- `constants.rs`: Constant definitions for update operations
- `index.rs`: Main entry point and hook registration
- `types.rs`: Type definitions and interfaces

## Usage

This agent is automatically invoked periodically to check for available updates. It compares the current version with the latest remote release and notifies users when updates are available.

## Configuration

The agent uses built-in update checking intervals and does not require specific configuration.
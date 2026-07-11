# MCP Visualizer

A local-first desktop app to manage and monitor MCP (Model Context Protocol) servers across AI coding tools — OpenCode, Claude Code, and Codex CLI.

## Features

- **MCP Management**: View, add, edit, and remove MCP servers from a single UI that writes to each tool's config format
- **Usage Monitoring**: See remaining credits/usage for OpenAI and Anthropic via official APIs
- **MCP Telemetry**: Monitor MCP server health — response time, CPU/memory, errors, and uptime history
- **Local-First**: Runs entirely on your machine. No external services, no cloud. Your API keys stay in your OS keychain.

## Stack

- **Frontend**: React + TypeScript
- **Backend**: Rust (Tauri 2.0)
- **Desktop**: Tauri 2.0

## License

MIT
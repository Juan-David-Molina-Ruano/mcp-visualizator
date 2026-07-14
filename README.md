# MCP Visualizer

[![CI](https://github.com/Juan-David-Molina-Ruano/mcp-visualizator/actions/workflows/ci.yml/badge.svg)](https://github.com/Juan-David-Molina-Ruano/mcp-visualizator/actions/workflows/ci.yml)
[![Security Audit](https://github.com/Juan-David-Molina-Ruano/mcp-visualizator/actions/workflows/security.yml/badge.svg)](https://github.com/Juan-David-Molina-Ruano/mcp-visualizator/actions/workflows/security.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

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
- **Runtime**: Node.js 24.x

## Security

This project takes supply chain security seriously. See [SECURITY.md](SECURITY.md) for full details.

- `cargo-audit` + `cargo-deny` + `cargo-supply-chain` for Rust dependency auditing
- `npm audit` for frontend dependency scanning
- Dependabot for weekly dependency updates across all ecosystems
- Signed commits required on `main`
- CI checks must pass before merge

## License

MIT

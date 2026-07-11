# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| latest main | yes |
| older | no |

## Reporting a Vulnerability

If you discover a security vulnerability, please **DO NOT** open a public issue.

Instead, email: **security@mcp-visualizer.dev** (or use GitHub's private vulnerability reporting).

You will receive a response within 48 hours. Please include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (optional)

## Security Principles

This app is **local-first** and designed with these principles:

1. **No external services** — all data stays on your machine
2. **API keys in OS keychain** — never in plaintext config files
3. **Secret redaction** — secrets are masked before logging or IPC
4. **Minimal permissions** — CI/CD uses least-privilege GITHUB_TOKEN
5. **Signed commits** — all commits to main must be signed/verified

## CI/CD Security

- All GitHub Actions use `permissions: contents: read` by default
- No secrets are passed to PRs from forks
- Dependabot monitors dependencies for known vulnerabilities
- Commits to `main` require verified signatures
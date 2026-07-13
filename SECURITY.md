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

## Supply Chain Security

Rust and npm supply chain attacks are real. This project defends against them with multiple layers:

### Rust Supply Chain Defense

| Layer | Tool | What it catches |
|-------|------|-----------------|
| Known vulnerabilities | `cargo-audit` | RustSec advisories for unsafe or malicious crates |
| Banned/malicious crates | `cargo-deny` | Crates pulled from crates.io for malicious code, license violations |
| Publisher auditing | `cargo-supply-chain` | Author, contributor, and publisher data for human review |
| Version pinning | `Cargo.lock` (committed) | Surprise upstream updates |
| License compliance | `cargo-deny` + `deny.toml` | Unapproved or copyleft licenses |

### Banned Crates

The following crates are banned via `deny.toml` and will fail CI:

| Crate | Advisory | Reason |
|-------|----------|--------|
| `onering` | RUSTSEC-2026-0175 | Malicious code — removed from crates.io |
| `logflux` | RUSTSEC-2026-0171 | Malicious code — removed from crates.io |
| `exploration` | RUSTSEC-2026-0155 | Malicious code — removed from crates.io |

### npm Supply Chain Defense

| Layer | Tool | What it catches |
|-------|------|-----------------|
| Known vulnerabilities | `npm audit` | Packages with reported vulnerabilities |
| Version pinning | `package-lock.json` (committed) | Surprise upstream updates |
| Dependency updates | Dependabot | Weekly PRs for outdated deps |

## CI/CD Security

- All GitHub Actions use `permissions: contents: read` by default
- No secrets are passed to PRs from forks
- Dependabot monitors dependencies for known vulnerabilities (cargo, npm, github-actions)
- Commit signatures verified on all PRs to `main`
- `cargo-audit`, `cargo-deny`, and `cargo-supply-chain` run on every push to `main`, every PR targeting `main`, and on the weekly schedule
- Node version in CI matches local (24.x)
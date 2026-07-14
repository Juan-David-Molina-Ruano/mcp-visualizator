# AGENTS.md — MCP Visualizer Project Guidelines

> **MAINTENANCE RULE**: This file MUST be updated whenever a directive is added, removed, or modified. If a directive is removed, delete it. If a directive is added, document it. Never let this file drift from reality.

## Project Overview

**MCP Visualizer** — A local-first desktop app to manage and monitor MCP (Model Context Protocol) servers across AI coding tools (OpenCode, Claude Code, Codex CLI).

## Tech Stack

| Layer           | Technology         | Version             |
| --------------- | ------------------ | ------------------- |
| Frontend        | React + TypeScript | Latest              |
| Backend         | Rust               | Stable (via Rustup) |
| Desktop         | Tauri              | 2.0                 |
| Runtime         | Node.js            | 24.x                |
| Package Manager | pnpm               | 11.x                |

## Branching & Governance

- **All work goes on feature branches** — never push directly to `main`
- **Only the repo owner** (`Juan-David-Molina-Ruano`) can:
  - Approve and merge PRs
  - Bypass CI checks (admin override only)
  - Push directly to `main` (emergency only)
- **Collaborators** can:
  - Create branches
  - Submit PRs
  - Cannot bypass CI, force-push to `main`, or self-approve
- **Forks**: Allowed — discoverability matters more than false security; anyone can `git clone` regardless
- **Signed commits**: Required for all commits in PRs targeting `main` and for all direct pushes to `main` — enforced by branch protection rules and verified by the `verify-signatures` CI job.
- **PR reviews**: Required before merge — only owner approval counts
- **CI checks**: Must pass before merge (Rust + frontend + security audits)
- **Conventional commits**: Required (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `security:`)
- **No AI attribution**: Never add "Co-Authored-By" or AI tags to commit messages

## Supply Chain Security

Rust and npm supply chain attacks are real. Crates like `onering`, `logflux`, and `exploration` were pulled from crates.io for malicious code in 2025-2026. This project uses pnpm with strict supply chain defaults to defend against npm attacks from day one.

### Rust Supply Chain Defense

| Tool                 | Purpose                                              | Status            |
| -------------------- | ---------------------------------------------------- | ----------------- |
| `cargo-audit`        | Known vulnerability scanning (RustSec advisory DB)   | CI (security.yml) |
| `cargo-deny`         | License compliance + banned crates + advisory checks | CI (security.yml) |
| `cargo supply-chain` | Publisher auditing — who published each crate        | CI (security.yml) |
| `Cargo.lock`         | Version pinning — committed to repo                  | Required          |
| `deny.toml`          | Banned crate list + license policy                   | Repo root         |

### npm/pnpm Supply Chain Defense

| Tool / Setting                               | Purpose                              | Status              |
| -------------------------------------------- | ------------------------------------ | ------------------- |
| `pnpm audit`                                 | Known vulnerability scanning         | CI (security.yml)   |
| `pnpm-lock.yaml`                             | Version pinning — committed to repo  | Required            |
| `allowBuilds`                                | Block postinstall scripts by default | pnpm-workspace.yaml |
| `blockExoticSubdeps`                         | Block git/tarball transitive deps    | pnpm-workspace.yaml |
| `minimumReleaseAge: 2880` (numeric, minutes) | Delay new versions by 48h            | pnpm-workspace.yaml |
| `trustPolicy: no-downgrade`                  | Block trust-level downgrades         | pnpm-workspace.yaml |

### Banned Crates

The following crates are **banned** via `deny.toml` and will fail CI if used:

| Crate         | Reason                                  | Advisory          |
| ------------- | --------------------------------------- | ----------------- |
| `onering`     | Malicious code — removed from crates.io | RUSTSEC-2026-0175 |
| `logflux`     | Malicious code — removed from crates.io | RUSTSEC-2026-0171 |
| `exploration` | Malicious code — removed from crates.io | RUSTSEC-2026-0155 |

Any crate marked as `malicious` in RustSec will also be caught by `cargo-audit` and `cargo-deny`.

### Supply Chain Rules for Contributors

1. Never use `cargo add` without checking the crate on [crates.io](https://crates.io) and [RustSec](https://rustsec.org/advisories/)
2. Review the publisher — prefer crates from `rust-lang`, `rust-lang-nursery`, or well-known maintainers
3. Pin major versions in `Cargo.toml` (e.g., `serde = "1.0"` not `serde = "*"`)
4. `Cargo.lock` MUST be committed (binaries, not libraries)
5. If `cargo-audit` or `cargo-deny` fails in CI, the PR is blocked until resolved
6. Never disable security checks without owner approval

## CI/CD Security

- All GitHub Actions use `permissions: contents: read` by default (least privilege)
- No secrets passed to PRs from forks
- Dependabot monitors all ecosystems weekly (cargo, pnpm/npm, github-actions)
- Commit signatures verified on all PRs to `main`
- `cargo-deny` checks run on every push to `main`, every PR targeting `main`, and on the weekly schedule
- Node version in CI matches local (24.x)

## SDD Workflow

This project uses **Spec-Driven Development (SDD)** via Gentle AI:

- **Execution mode**: Interactive (pause between phases for review)
- **Artifact store**: Engram
- **PR strategy**: Ask-always if >400 changed lines
- **Review budget**: 400 lines
- **All technical artifacts**: English (code, comments, specs, docs, UI strings)
- **Conversation language**: Matches user (Spanish/English)

### SDD Phases

```text
proposal ✓ → spec ✓ → design ✓ → tasks ✓ → apply → verify → archive
```

### Current Status

| Phase    | Status  | Artifact                               |
| -------- | ------- | -------------------------------------- |
| Init     | ✓ Done  | `sdd-init/mcp-visualizador` (engram)   |
| Proposal | ✓ Done  | `sdd/mcp-visualizer/proposal` (engram) |
| Design   | ✓ Done  | `sdd/mcp-visualizer/design` (engram)   |
| Spec     | ✓ Done  | `sdd/mcp-visualizer/spec` (engram)     |
| Tasks    | ✓ Done  | `sdd/mcp-visualizer/tasks` (engram)    |
| Apply    | Pending | —                                      |

### Engram Topic Keys

| Artifact       | Topic Key                           |
| -------------- | ----------------------------------- |
| Init           | `sdd-init/mcp-visualizador`         |
| Exploration    | `sdd/mcp-visualizer/explore`        |
| Proposal       | `sdd/mcp-visualizer/proposal`       |
| Spec           | `sdd/mcp-visualizer/spec`           |
| Design         | `sdd/mcp-visualizer/design`         |
| Tasks          | `sdd/mcp-visualizer/tasks`          |
| Apply progress | `sdd/mcp-visualizer/apply-progress` |
| Verify report  | `sdd/mcp-visualizer/verify-report`  |
| Architecture   | `architecture/mcp-visualizador`     |

## Tooling Available

### MCP Servers

| Server         | Use                                                 |
| -------------- | --------------------------------------------------- |
| GitHub MCP     | PRs, issues, branches, file operations, code search |
| Supabase MCP   | Database, auth, edge functions (if needed)          |
| SQL Server MCP | Read-only queries (personal dev DB)                 |
| Stitch MCP     | UI design generation (if needed)                    |

### CodeGraph

- Index at `.codegraph/` (gitignored, local only)
- Use `codegraph_explore` MCP tool for structural codebase questions before filesystem searches
- Initialize with `gentle-ai codegraph init` if `.codegraph/` is missing

### Engram

- Persistent memory across sessions and compactions
- Project key: `mcp-visualizador` (from dirname, survives path changes)
- Git remote name: `mcp-visualizator`
- Search with `mem_search`, save with `mem_save`, get full content with `mem_get_observation`

## Skills

### Installed Skills

| Skill                              | Purpose                                     |
| ---------------------------------- | ------------------------------------------- |
| `branch-pr`                        | Create PRs with issue-first checks          |
| `chained-pr`                       | Split >400 line changes into reviewable PRs |
| `work-unit-commits`                | Atomic, reviewable commit planning          |
| `comment-writer`                   | PR comments, issue replies, review feedback |
| `cognitive-doc-design`             | Docs that reduce cognitive load             |
| `issue-creation`                   | GitHub issues with checks                   |
| `skill-registry`                   | Index and resolve skills                    |
| `sdd-init`                         | Initialize SDD context                      |
| `sdd-explore`                      | Explore ideas before committing             |
| `sdd-propose`                      | Create change proposals                     |
| `sdd-spec`                         | Write delta specs                           |
| `sdd-design`                       | Technical design                            |
| `sdd-tasks`                        | Break changes into tasks                    |
| `sdd-apply`                        | Implement tasks                             |
| `sdd-verify`                       | Validate implementation                     |
| `sdd-archive`                      | Archive completed changes                   |
| `sdd-onboard`                      | End-to-end SDD walkthrough                  |
| `judgment-day`                     | Adversarial dual review                     |
| `supabase`                         | Supabase integration                        |
| `supabase-postgres-best-practices` | Postgres optimization                       |
| `go-testing`                       | Go testing patterns (available, not used)   |
| `skill-creator`                    | Create new skills                           |
| `skill-improver`                   | Audit and improve skills                    |

### Skill Resolution Protocol

1. Orchestrator reads `.atl/skill-registry.md` at session start
2. Matches skills by file context (extensions, paths) AND task context
3. Passes matching `SKILL.md` paths into sub-agent prompts
4. Sub-agents read those exact files BEFORE task-specific work

## Code Conventions

- **Language**: All technical artifacts in English (code, comments, UI strings, docs)
- **Architecture**: Clean/Hexagonal Architecture
- **Commits**: Conventional commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `security:`)
- **No AI attribution** in commit messages
- **TDD**: Strict TDD when project supports it (detected by `sdd-init`)
- **Testing**: Table-driven tests for Rust, React Testing Library for frontend
- **Error handling**: Explicit, no silent failures
- **Logging**: Structured logging (tracing crate for Rust)

## Maintenance Rules

> This is a **living document**. It MUST reflect the current state of the project.

1. **When a directive is added**: Document it here immediately
2. **When a directive is removed**: Delete it from this file
3. **When a tool/dependency changes**: Update the relevant table
4. **When branch protection changes**: Update the Governance section
5. **When new MCP servers are added**: Update the Tooling section
6. **When skills are installed/removed**: Update the Skills section
7. **Never let this file drift from reality** — an outdated AGENTS.md is worse than no AGENTS.md

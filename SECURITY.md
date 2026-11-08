# Security Policy — Rusted OS

## Overview

Rusted OS is a Rust-first, AI-native Linux desktop operating system built on the
upstream Linux kernel and the Fedora ecosystem.

Security is a core design goal, not an afterthought:

- **Rust-first:** new system components prioritize memory safety and explicit interfaces.
- **Linux-first:** reuse proven primitives — systemd, SELinux, D-Bus, Flatpak, cgroups v2.
- **Secure by design for AI:** AI agents never receive unrestricted root access.
- **Offline-capable core:** boot, login, desktop, networking, and recovery work without AI or internet.
- **BYOK cloud AI:** users bring their own provider keys, stored in the OS credential store.

This policy explains what is in scope, how to report vulnerabilities,
and what contributors must follow to keep Rusted OS secure.

Related docs: `README.md`, `ARCHITECTURE.md`, `AGENT.md`, `ROADMAP.md`.

## Supported Versions

Rusted OS is in early development (`0.1.0`, see `Cargo.toml`).

| Version | Status | Security fixes |
|---|---|---|
| `main` branch | supported | yes — fixed on `main`, advisories published |
| `0.1.x` pre-release tags | best-effort | only for severe issues, at maintainer discretion |
| older tags / forks | not supported | please upgrade to `main` |

There are no LTS releases yet. If that changes, this table will be updated.

## Reporting a Vulnerability

**Please do not open a public GitHub issue for a suspected vulnerability.**

Use one of these private channels:

1. **GitHub Private Vulnerability Reporting** (preferred):
   repo page → `Security` tab → `Report a vulnerability`
   for `joniilmanfahmi00-collab/rusted-os`.
2. If private reporting is unavailable, open a minimal GitHub issue titled
   `Security report — please contact me` without technical details,
   and maintainers will establish a private channel.

Include in your report:

- affected component / crate (`rusted-core`, `rusted-config`, `rusted-ipc`, `rusted-security`, etc.)
- version / commit hash, Fedora base, desktop context if relevant
- steps to reproduce, impact assessment, and any logs (redact secrets)
- whether AI credentials, user data, or privilege escalation is involved
- your contact and disclosure preferences

What you can expect:

- acknowledgement within 72 hours
- triage and severity assessment within 7 days
- fix coordination and credit (if desired) before public disclosure
- a GitHub Security Advisory for significant issues

We follow coordinated disclosure. Please give maintainers reasonable time
to fix before publishing details. We will not take legal action for
good-faith research that follows this policy.

## Scope

### In scope

- `crates/rusted-core`, `crates/rusted-config`, `crates/rusted-ipc`, `crates/rusted-security`
  and future `rusted-*` crates (runtime, services, resource-manager, diagnostics)
- permission / capability abstractions and privileged-operation gating
- IPC contracts (D-Bus, Tokio-based async services) and service authentication
- configuration parsing, validation, migration, and safe defaults
- AI integration layer when present:
  Permission Gateway, provider router, context handling, automation / agent runtime,
  credential storage and BYOK handling
- resource policies (cgroups v2 / systemd) where they affect isolation
- build, CI, and release tooling in this repo where it affects supply chain

### Out of scope (coordinate upstream, but still tell us)

- upstream Linux kernel, Fedora packages, systemd, SELinux policies,
  Smithay / Iced / Slint / wgpu / Tokio / Serde themselves
- third-party AI providers (OpenAI, Anthropic, Gemini, Qwen, DeepSeek, Kimi, etc.)
- user misconfiguration or already-compromised hosts

If you are unsure, report it — we will route it correctly.

## Security Model and Guarantees

Contributors and reviewers enforce these invariants:

1. **No unrestricted root for AI.** AI assistants, agents, and automations
   operate with least privilege through explicit permission checks.
2. **Explicit authorization.** Sensitive operations require user approval
   via defined mechanisms. No silent privilege escalation.
3. **Credential isolation.** BYOK keys live in the OS secure credential store,
   never in logs, config files, or the repo. Cloud transmission is transparent
   and user-controlled. Provider switching is never silent.
4. **Core works offline.** Boot, login, compositor/shell basics, networking,
   and recovery must not depend on AI availability.
5. **Modular boundaries.** Crates expose clear contracts. Unprivileged and
   privileged services are separated. IPC inputs are validated.
6. **Resource fairness without weakening isolation.** Performance work must not
   disable SELinux, sandboxing, or Flatpak confinement to gain benchmarks.

## Secure Development Requirements

All contributions must follow `AGENT.md`:

- No secrets in code, tests, logs, or history. No API keys, tokens, or private keys.
- `unsafe` Rust requires justification, minimal scope, and tests. Prefer safe abstractions.
- Handle errors explicitly. No `unwrap` / `expect` on privileged or user-input paths.
- Validate IPC and config inputs. Define explicit service contracts.
  No unnecessary IPC or background services.
- Changes to architecture, public APIs, IPC protocols, security boundaries,
  credential storage, provider architecture, or resource strategy require
  an Architecture Decision Record (ADR) — see `AGENT.md`.
- Run before submitting:
  `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test`, and
  `cargo audit` (or `cargo deny check`) where available.
- Tests for privileged operations, permission checks, and failure modes.
  Do not remove tests to make CI pass.
- Performance claims need reproducible benchmarks. See `README.md` principles.

Prohibited without explicit maintainer approval:

- forking / patching the Linux kernel for this repo
- destructive disk, bootloader, or host package operations in code or scripts
- changing host security policies, disabling SELinux / sandboxing
- introducing ambient / unrestricted privilege for agents or services

## Dependency and Supply-Chain Security

- Keep `Cargo.lock` for applications; review dependency additions.
- Prefer maintained crates with clear licenses compatible with `GPL-3.0-or-later`.
- Report typosquatting, compromised dependencies, or suspicious build behavior immediately.
- CI is expected to enforce fmt, Clippy, and tests (see `ROADMAP.md` Phase 0).

## After a Fix

- Maintainers publish an advisory, credit reporters (opt-in), and document
  affected versions and mitigations.
- Security-sensitive refactors include regression tests and docs updates.

Thank you for helping keep Rusted OS safe, lean, and trustworthy.

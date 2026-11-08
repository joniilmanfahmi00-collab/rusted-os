# Contributing to Rusted OS

Thank you for your interest in Rusted OS — a Rust-first, AI-native Linux desktop
OS built on the upstream Linux kernel and Fedora ecosystem.

Rusted OS is early development (`0.1.0`). Contributions that are small,
modular, well-tested, and documented have the highest chance of merging quickly.

Please also read `README.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `AGENT.md`,
`SECURITY.md`, and `CODE_OF_CONDUCT.md`.

## Vision and Principles

We build a cohesive desktop, not just another themed distro:

1. **Linux first:** reuse upstream kernel, systemd, SELinux, D-Bus, Flatpak, PipeWire.
2. **Rust first:** new system components in Rust (edition 2024), memory-safe by default.
3. **Resource efficiency:** on-demand services, event-driven work, cgroups v2,
   workload-aware policies. No unnecessary background load.
4. **AI-native, not AI-dependent:** boot, login, desktop, networking, recovery
   work offline. AI is cloud-first BYOK and permission-gated.
5. **Security by design:** AI agents never get unrestricted root.
6. **Modular:** clear crate boundaries and IPC contracts, independently testable.

If your idea conflicts with these, open a discussion first — we would rather
redirect early than reject a large PR late.

## Ways to Contribute

- Report bugs with reproduction, logs, and environment (Fedora version, commit, hardware)
- Improve docs, examples, and error messages
- Add tests, benchmarks, diagnostics, and resource measurements
- Implement `ROADMAP.md` items for Phase 0 / Phase 1 (`rusted-core`, `rusted-config`,
  `rusted-ipc`, `rusted-security`, services, runtime, resource-manager)
- Propose Architecture Decision Records (ADRs) for significant choices
- Review PRs for correctness, safety, and scope discipline

We do not currently accept: kernel forks, custom bootloaders in this repo,
bulk reformatting, or AI-provider keys / secrets of any kind.

## Development Setup

Recommended: Fedora Workstation or Server (matching `ARCHITECTURE.md` foundation).

```bash
# toolchain is pinned in rust-toolchain.toml
rustc --version
cargo --version

git clone https://github.com/joniilmanfahmi00-collab/rusted-os.git
cd rusted-os

cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace
```

Useful commands:

```bash
cargo test -p rusted-core
cargo test -p rusted-config
cargo test -p rusted-ipc
cargo test -p rusted-security

cargo audit # or: cargo deny check
```

Do not run commands with elevated privileges, install/remove host packages,
modify bootloaders/partitions, or change host SELinux policies as part of
development unless maintainers explicitly approve it. See `AGENT.md` Prohibited Actions.

## Repository Layout

```text
Cargo.toml          # workspace: rusted-core, rusted-config, rusted-ipc, rusted-security
rust-toolchain.toml
ARCHITECTURE.md     # 5-layer model: Intelligence / Desktop / System Core / Linux / Hardware
BLUEPRINT.md
ROADMAP.md          # Phase 0 Foundation, Phase 1 System Core, Phase 2 Desktop, etc.
AGENT.md            # agent and contributor operating rules — must follow
crates/
  rusted-core/     # lifecycle, shared types, error handling
  rusted-config/   # typed config, validation, versioning, safe defaults
  rusted-ipc/      # D-Bus + Tokio async contracts, explicit service APIs
  rusted-security/ # permissions, capabilities, privileged-op gating
docs/
```

Technology direction (subject to ADR): Wayland, Smithay (evaluating),
Iced / Slint (evaluating), wgpu, Tokio, Serde, systemd, D-Bus, SELinux, Flatpak.

## Workflow

1. Search issues and `ROADMAP.md` to avoid duplication.
2. For anything affecting architecture, public APIs, IPC, security boundaries,
   credential storage, provider architecture, resource strategy, compositor /
   UI framework, or updates — write a short ADR proposal first.
3. Fork, create a focused branch: `codex/short-topic` or `feat/short-topic`.
4. Keep PRs small and reviewable. One concern per PR.
5. Fill the PR template: what, why, scope, tests, benchmarks, security notes.
6. Ensure CI is green: fmt, Clippy, tests, docs.

Suggested ADR format from `AGENT.md`:

```text
# ADR-NNN: Decision Title
Status: Proposed | Accepted | Superseded
## Context
## Decision
## Alternatives
## Consequences
```

Do not mark ADRs as Accepted without maintainer approval.

## Coding Conventions

- Follow `AGENT.md`: explicit interfaces, structured errors, graceful startup/shutdown.
- Rust edition 2024. Run `cargo fmt` and `cargo clippy -- -D warnings`.
- No `unwrap` / `expect` on privileged, IPC, config, or user-input paths.
- `unsafe` only with justification, minimal scope, and tests.
- Async with Tokio where appropriate; avoid unnecessary IPC and polling —
  prefer event-driven designs.
- Typed configuration with validation and versioning; safe defaults.
- Public APIs need rustdoc. Architectural changes need docs updates.
- Tests for behavior changes: unit + integration where IPC / security is involved.
- Performance changes need reproducible benchmarks — no claims without numbers.
- Never disable SELinux, sandboxing, or security services to improve numbers.
- Never commit `target/`, generated artifacts, keys, or personal data.

Commit messages — use these prefixes:

```text
feat: add resource monitoring
fix: handle service shutdown correctly
refactor: simplify configuration loading
test: add IPC integration tests
docs: document provider architecture
build: update workspace dependencies
security: restrict privileged tool execution
```

Keep messages descriptive and in English. One logical change per commit.

## Pull Request Checklist

- [ ] Scope is focused, no unrelated fixes mixed in
- [ ] `cargo fmt --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes, new tests added for behavior changes
- [ ] Docs updated (`README`, `ARCHITECTURE`, rustdoc, or `docs/` as needed)
- [ ] ADR added/updated if architectural
- [ ] Security implications considered (see `SECURITY.md`), no secrets added
- [ ] Benchmarks included if performance-related
- [ ] GPL-3.0-or-later compatibility preserved for new dependencies

A green build alone is not done — see Definition of Done in `AGENT.md`.

## AI-Assisted Contributions

AI assistance is welcome (this is an AI-native project), but:

- You are responsible for code you submit, including AI-generated parts.
- AI must not be given unrestricted system access during development.
- Disclose significant AI assistance in the PR if it affects design decisions.
- Verify provider examples without committing keys. BYOK only — use local dummy
  keys and the secure credential store abstraction.

## License

By contributing, you agree your contributions are licensed under
`GPL-3.0-or-later` (see `LICENSE`). New files should include:

```text
// SPDX-License-Identifier: GPL-3.0-or-later
```

Ensure new dependencies are GPL-3.0-or-later compatible.

## Getting Help

- Questions: GitHub Discussions / Issues
- Security issues: see `SECURITY.md` — do not open public issues
- Conduct concerns: see `CODE_OF_CONDUCT.md`

Build deliberately. Prefer proven Linux infrastructure. Keep it modular,
secure, and resource-efficient.

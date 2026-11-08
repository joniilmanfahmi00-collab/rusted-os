## Summary

<!-- What does this PR change and why? Link issues / ROADMAP phase / ADR if applicable. -->

Closes: #
Related: #
Phase: <!-- e.g. Phase 1 — Rusted System Core -->
ADR: <!-- e.g. docs/decisions/ADR-0012... or N/A -->

## Type of change

<!-- Keep one -->

- feat / fix / refactor / test / docs / build / security / chore

## Scope

<!-- Which crates / areas are affected? Keep PRs focused — one concern per PR. -->

- [ ] `rusted-core`
- [ ] `rusted-config`
- [ ] `rusted-ipc`
- [ ] `rusted-security`
- [ ] docs (`README`, `ARCHITECTURE`, `ROADMAP`, `AGENT`, `CONTRIBUTING`)
- [ ] CI / build / tooling
- [ ] other: ___

## Architecture

- [ ] No architectural change, or ADR proposal added/updated in `docs/decisions/`
- [ ] Public APIs / IPC contracts documented with rustdoc
- [ ] Follows Linux-first, Rust-first, modular principles in `ARCHITECTURE.md`
- [ ] No silent arch change — reviewed against `AGENT.md`

## Testing

<!-- Paste commands and results -->

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

- [ ] `cargo fmt` passes
- [ ] `cargo clippy` passes with `-D warnings`
- [ ] `cargo test --workspace` passes, new tests added for behavior changes
- [ ] `cargo audit` / `cargo deny check` reviewed (if dependencies changed)

## Security checklist (see SECURITY.md)

- [ ] No secrets, API keys, tokens, or personal data committed
- [ ] No `unwrap` / `expect` on privileged, IPC, config, or user-input paths
- [ ] `unsafe` justified, minimal, and tested (or N/A)
- [ ] Permission / capability checks added for privileged operations (or N/A)
- [ ] IPC / config inputs validated, no unnecessary IPC or background service added
- [ ] AI agents do not gain unrestricted root; BYOK credentials stay in OS credential store
- [ ] SELinux / sandboxing / Flatpak confinement not weakened

## Resource efficiency (if applicable)

- [ ] N/A — no perf/resource impact
- [ ] Event-driven, no unnecessary polling/background work
- [ ] Reproducible benchmark included (before/after numbers, hardware, Fedora version)

## Docs

- [ ] `README` / `ARCHITECTURE` / `ROADMAP` updated if needed
- [ ] rustdoc updated for public APIs
- [ ] Limitations and follow-ups disclosed

## Definition of Done (per AGENT.md)

- [ ] Scope met, no unrelated changes mixed in
- [ ] Conventions followed, CI green
- [ ] Security implications considered
- [ ] Remaining limitations disclosed

## License

- [ ] I agree my contribution is licensed under `GPL-3.0-or-later` and new dependencies are compatible.

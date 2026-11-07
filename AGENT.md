# Rusted OS — AI Agent & Contributor Guidelines

**Version:** 0.1  
**Status:** Active Development Guidelines

## 1. Purpose

This document defines the rules and expectations for AI coding agents and human contributors working on Rusted OS.

All contributors must follow the project's architecture, security principles, development standards, and scope.

The objective is to build a reliable, efficient, secure, and maintainable operating system — not merely to produce code quickly.

## 2. Project Context

Rusted OS is a Rust-first desktop operating system built on the upstream Linux kernel and Fedora ecosystem.

The project aims to provide:

- A custom desktop experience.
- Efficient resource management.
- Strong Linux application compatibility.
- Secure multi-provider AI integration.
- Developer-focused workflows.
- Modular and maintainable system components.

Rusted OS does not intend to replace the Linux kernel or reinvent mature Linux infrastructure without a compelling technical reason.

## 3. Mandatory Documentation

Before implementing a feature, read the relevant project documentation:

- `README.md`
- `ARCHITECTURE.md`
- `ROADMAP.md`
- `AGENT.md`
- Relevant ADRs under `docs/decisions/`

Documentation has the following precedence:

1. Explicitly approved architecture decisions (ADRs).
2. `ARCHITECTURE.md`.
3. `ROADMAP.md`.
4. This document's implementation guidelines.
5. Existing implementation and conventions.

If two documents conflict, do not silently choose one. Identify the conflict and request clarification or propose a documented resolution.

## 4. Core Development Rules

### Rust First

- Use Rust for project-owned system components by default.
- Prefer stable Rust unless a documented requirement justifies another toolchain.
- Avoid unnecessary `unsafe`.
- Document every necessary `unsafe` block.
- Prefer well-maintained crates over custom implementations of established functionality.
- Avoid unnecessary dependencies.
- Keep dependencies updated deliberately, not blindly.

### Linux First

- Use existing Linux interfaces and infrastructure.
- Do not modify or fork the Linux kernel unless explicitly approved.
- Prefer D-Bus for appropriate desktop and service communication.
- Use systemd for service lifecycle management.
- Use cgroups v2 for resource controls.
- Respect Linux filesystem, process, permission, and security conventions.

### Modular Architecture

- Keep crates focused on clearly defined responsibilities.
- Avoid circular dependencies.
- Do not create abstractions without a real use case.
- Keep public APIs small and intentional.
- Prefer composition over deeply nested abstractions.
- Avoid splitting code into separate processes unless isolation, security, lifecycle, or operational requirements justify it.

### Resource Efficiency

Resource efficiency is a core project requirement.

Agents must:

- Avoid unnecessary background services.
- Prefer event-driven operations over continuous polling.
- Avoid unnecessary threads and task spawning.
- Avoid excessive memory allocation.
- Avoid unbounded queues and caches.
- Use bounded concurrency where appropriate.
- Avoid expensive work during desktop idle periods.
- Start optional services on demand.
- Measure performance before making optimization claims.

Do not sacrifice security or system reliability for lower resource consumption.

### AI Independence

AI features must remain optional.

The following operations must not require an AI provider:

- Boot.
- Login.
- Basic desktop usage.
- Network configuration.
- Audio configuration.
- System recovery.
- Essential system administration.

Cloud AI must not become a critical dependency of the operating system.

## 5. Security Rules

Security must be considered during design, implementation, and testing.

### Privilege Boundaries

- Never grant AI agents unrestricted root access.
- Never execute AI-generated shell commands automatically with elevated privileges.
- Use explicit authorization for privileged operations.
- Keep privileged operations behind controlled system interfaces.
- Apply least-privilege principles.
- Validate all inputs crossing process and privilege boundaries.

### Permission Gateway

Sensitive operations must use an appropriate permission mechanism.

Examples include:

- Installing software.
- Removing files or applications.
- Modifying system configuration.
- Changing network or security settings.
- Executing privileged commands.
- Changing service configurations.
- Accessing protected user data.

User confirmation must be required where the operation is sensitive or destructive.

### Credentials

- Never hardcode API keys, tokens, or passwords.
- Never commit credentials.
- Never write secrets into logs.
- Never store credentials in plaintext configuration files.
- Use an appropriate OS credential store.
- Do not expose credentials through diagnostics or error messages.

### Context Access

- Do not scan the entire filesystem by default.
- Do not capture the screen continuously by default.
- Do not collect unrelated user data.
- Request access only to the context required for the task.
- Make data transmission to cloud AI providers explicit and controllable.

### System Isolation

Use established Linux security mechanisms where appropriate:

- SELinux.
- Unix permissions.
- Linux capabilities.
- systemd service isolation.
- D-Bus authorization.
- cgroups.
- Seccomp or other sandboxing mechanisms where justified.

Rust type safety is valuable but does not replace operating-system security boundaries.

## 6. AI Provider Integration Rules

AI integrations must follow the multi-provider architecture.

Supported provider targets include:

- OpenAI.
- Anthropic.
- Google Gemini.
- Alibaba Qwen.
- DeepSeek.
- Moonshot AI / Kimi.
- Hosted Llama-compatible inference providers.

Rules:

1. Do not assume all providers expose identical APIs.
2. Implement provider-specific behavior through adapters.
3. Use shared compatibility layers only where verified.
4. Do not hardcode a single provider into the desktop or core runtime.
5. Keep model selection configurable.
6. Preserve provider-specific error information safely.
7. Support streaming where the provider allows it.
8. Treat tool calling and structured output as provider capabilities, not universal guarantees.
9. Do not silently switch providers.
10. Never transmit user credentials to another provider without explicit authorization.

AI requests must be initiated only when a user or authorized application requests them.

## 7. Resource Manager Rules

The Resource Manager must cooperate with Linux rather than replace its scheduling mechanisms.

Use:

- cgroups v2.
- systemd resource controls.
- CPU scheduling priorities.
- I/O controls.
- Process and memory monitoring.

Do not implement a custom kernel scheduler.

### Developer Mode

Developer Mode should prioritize development workloads, such as:

- IDEs.
- Terminals.
- Compilers.
- Build systems.
- Development servers.
- Databases.
- Containers and virtual machines.

Non-essential indexing and maintenance workloads may be deferred when appropriate.

Never blindly terminate or suspend essential system services.

Do not disable security, networking, audio, or other critical services simply to improve benchmark numbers.

## 8. Coding Standards

### Formatting

Use:

```bash
cargo fmt --all -- --check
```

### Linting

Use:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The exact lint configuration may evolve. Follow the repository configuration.

### Testing

Use:

```bash
cargo test --workspace
```

For targeted development, run the relevant crate tests first and the workspace tests before submitting changes.

### Error Handling

- Avoid unnecessary panics in production code.
- Prefer explicit error types.
- Preserve useful error context.
- Do not silently ignore failures.
- Handle expected operational failures gracefully.

### Logging

- Use structured logging where appropriate.
- Avoid logging secrets or sensitive user data.
- Keep log volume reasonable.
- Use appropriate severity levels.
- Avoid logging continuously from idle background loops.

### Documentation

- Document public APIs.
- Explain non-obvious design decisions.
- Include examples when they improve usability.
- Update relevant documentation when behavior or architecture changes.
- Do not claim a feature exists before it is implemented.

## 9. Dependency Management

Before adding a dependency:

1. Check whether the functionality already exists in the standard library or current dependencies.
2. Evaluate maintenance activity and compatibility.
3. Review licensing.
4. Consider dependency size and transitive dependencies.
5. Consider security implications.
6. Consider compilation and runtime costs.

Do not introduce a large framework for a small isolated requirement.

Do not replace an existing dependency without a clear technical justification.

## 10. Implementation Workflow

Every feature should follow this workflow:

### Step 1 — Understand

- Read the relevant documentation.
- Inspect existing implementation.
- Identify the responsible crate or component.
- Understand the current interfaces.
- Identify dependencies and security boundaries.

### Step 2 — Plan

Before substantial implementation:

- State the problem.
- Identify the proposed solution.
- List affected files and crates.
- Identify risks and trade-offs.
- Define tests and acceptance criteria.

For small, isolated changes, a brief plan is sufficient.

### Step 3 — Implement

- Keep changes focused.
- Follow existing conventions.
- Avoid unrelated refactoring.
- Preserve public API compatibility where possible.
- Add tests alongside implementation.

### Step 4 — Validate

Run relevant checks:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

If a command cannot run, explain why.

Never claim a test, build, or benchmark passed unless it actually ran successfully.

### Step 5 — Document

Update documentation when necessary:

- Architecture changes.
- API changes.
- New dependencies.
- Security changes.
- Configuration changes.
- New features.
- Known limitations.

### Step 6 — Report

Every completed task should include:

- What changed.
- Why it changed.
- Files and crates affected.
- Tests and checks performed.
- Results.
- Known limitations.
- Remaining work.

## 11. Architecture Decision Records

Agents must not silently make major architectural decisions.

An ADR should be proposed when a change affects:

- System architecture.
- Core technology choices.
- Public interfaces.
- IPC protocols.
- Security boundaries.
- AI provider architecture.
- Credential storage.
- Resource management strategy.
- Desktop framework or compositor.
- Update and deployment strategy.

Suggested ADR format:

```text
# ADR-NNN: Decision Title

Status: Proposed | Accepted | Superseded

## Context

What problem needs to be solved?

## Decision

What approach is selected?

## Alternatives

What other approaches were considered?

## Consequences

What are the benefits, limitations, and trade-offs?
```

Do not mark a proposal as accepted without appropriate project approval.

## 12. Prohibited Actions

Unless explicitly authorized, agents must not:

- Modify or fork the Linux kernel.
- Execute destructive disk operations.
- Format partitions or modify bootloaders.
- Install or remove system packages on the host machine.
- Change host system security policies.
- Run commands with elevated privileges.
- Modify unrelated project files.
- Commit API keys or other secrets.
- Introduce unrestricted root access for AI agents.
- Silently change architectural decisions.
- Claim compatibility without testing.
- Claim performance improvements without measurements.
- Remove tests merely to make CI pass.
- Disable security services to improve performance results.

For potentially destructive operations, explain the impact and obtain explicit approval before proceeding.

## 13. Git & Contribution Guidelines

- Keep changes focused and reviewable.
- Use descriptive commit messages.
- Avoid committing generated artifacts unless required.
- Do not commit build output such as `target/`.
- Include tests for behavior changes.
- Include documentation for architectural changes.
- Avoid mixing unrelated fixes in one commit.

Suggested commit prefixes:

```text
feat: add resource monitoring
fix: handle service shutdown correctly
refactor: simplify configuration loading
test: add IPC integration tests
docs: document provider architecture
build: update workspace dependencies
security: restrict privileged tool execution
```

## 14. Handling Uncertainty

When requirements are unclear:

1. Do not invent requirements.
2. Inspect existing code and documentation.
3. Identify assumptions explicitly.
4. Prefer the smallest reversible implementation.
5. Ask for clarification when a decision could affect architecture, security, or data integrity.

If a requested implementation conflicts with the architecture, explain the conflict and propose an alternative.

## 15. Definition of Done

A task is complete only when:

- The implementation meets the requested scope.
- The code follows project conventions.
- Relevant tests pass.
- Formatting and linting pass where applicable.
- Security implications have been considered.
- Documentation is updated where necessary.
- No unrelated changes are introduced.
- Remaining limitations are disclosed.

A successful compilation alone does not mean a feature is production-ready.

---

**Final Principle**

Build deliberately. Prefer proven Linux infrastructure. Keep Rusted OS modular, secure, and resource-efficient.

An AI agent is a contributor to Rusted OS — not the owner of its architecture, security policies, or user machine.
# Rusted OS — Development Roadmap

**Version:** 0.1  
**Status:** Planning  
**Project:** Rusted OS

## 1. Vision

Rusted OS is an open-source, Rust-first desktop operating system built on the upstream Linux kernel and Fedora ecosystem.

Its goal is to combine Linux reliability, an efficient desktop experience, secure AI integration, and a developer-friendly environment.

Rusted OS is not intended to be another Linux distribution with a custom theme. It aims to build a cohesive desktop and system experience with carefully designed Rust components.

## 2. Development Principles

All development phases follow these principles:

1. **Linux First:** Use the upstream Linux kernel and established Linux infrastructure.
2. **Rust First:** Build project-owned system components primarily in Rust.
3. **Resource Efficiency:** Minimize unnecessary background workloads and resource consumption.
4. **Security by Design:** Use Linux security mechanisms and explicit permission boundaries.
5. **AI as an Optional Capability:** AI must never be required for boot, login, or normal desktop operation.
6. **Cloud-First, BYOK:** Users provide their own AI provider credentials.
7. **Modular Architecture:** Keep components independently testable and maintainable.
8. **Incremental Delivery:** Prioritize working, testable milestones over premature feature expansion.

## 3. Development Phases

### Phase 0 — Foundation & Architecture

**Goal:** Establish the project's technical foundation and development standards.

Planned work:

- [ ] Initialize the Git repository.
- [ ] Establish the Rust workspace.
- [ ] Configure the Rust toolchain.
- [ ] Define repository structure and crate naming conventions.
- [ ] Write `ARCHITECTURE.md`.
- [ ] Write `ROADMAP.md`.
- [ ] Write `AGENT.md`.
- [ ] Establish Architecture Decision Records (ADRs).
- [ ] Set up formatting and linting with `rustfmt` and Clippy.
- [ ] Establish unit and integration testing conventions.
- [ ] Configure CI for supported Rust targets.
- [ ] Establish a Fedora development environment.
- [ ] Record baseline hardware and resource benchmarks.
- [ ] Define the initial security model.

**Exit criteria:**

- The repository builds successfully.
- Workspace conventions are documented.
- CI can run formatting, linting, and tests.
- Initial architecture decisions are documented.

### Phase 1 — Rusted System Core

**Goal:** Build the foundational services that future desktop and AI components can rely on.

Core components:

- [ ] `rusted-core`
- [ ] `rusted-config`
- [ ] `rusted-ipc`
- [ ] `rusted-security`
- [ ] `rusted-services`
- [ ] `rusted-runtime`
- [ ] `rusted-diagnostics`
- [ ] `rusted-resource-manager`

Planned capabilities:

**Core Runtime**
- Application and service lifecycle abstractions.
- Shared foundational types.
- Structured error handling.
- Graceful startup and shutdown.

**Configuration**
- Typed configuration.
- Schema validation.
- Configuration versioning.
- Safe defaults and migration strategy.

**IPC**
- D-Bus integration.
- Asynchronous communication using Tokio where appropriate.
- Explicit service contracts.
- No unnecessary inter-process communication.

**Security**
- Permission and capability abstractions.
- Integration with Linux security mechanisms.
- Secure handling of privileged operations.
- Clear separation between unprivileged and privileged services.

**Resource Manager**
- Monitor CPU, memory, disk I/O, and process activity.
- Integrate with cgroups v2 and systemd resource controls.
- Support workload priorities.
- Introduce Developer Mode policies.
- Avoid terminating essential system services.
- Prefer event-driven resource management over continuous polling.

**Exit criteria:**

- Core crates compile and have meaningful tests.
- Services can communicate through defined interfaces.
- Configuration and diagnostics work.
- Resource monitoring produces measurable data.
- Security boundaries are documented and tested.

### Phase 2 — Rusted Desktop

**Goal:** Deliver a functional desktop environment built around the Rusted OS experience.

Components:

- [ ] `rusted-compositor`
- [ ] `rusted-shell`
- [ ] `rusted-launcher`
- [ ] `rusted-settings`
- [ ] `rusted-notifications`
- [ ] `rusted-system-monitor`

Technical direction:

- Wayland desktop architecture.
- Evaluate Smithay for compositor development.
- Evaluate Iced and Slint for desktop UI components.
- Use wgpu where appropriate.
- Integrate with D-Bus and existing Linux desktop services.

Planned capabilities:

- Window management.
- Desktop shell.
- Application launcher.
- Workspace management.
- System settings.
- Notification management.
- System monitoring.
- Developer Mode interface.
- Basic accessibility support.

**Exit criteria:**

- A user can log into a graphical desktop.
- Applications can launch and manage windows.
- Keyboard, mouse, display, audio, and networking integrations work.
- Essential desktop functions work without AI services.
- Desktop resource usage can be measured.

### Phase 3 — Rusted Intelligence

**Goal:** Integrate secure, multi-provider AI capabilities into the desktop.

Components:

- [ ] `rusted-ai`
- [ ] `rusted-ai-providers`
- [ ] `rusted-ai-router`
- [ ] `rusted-context`
- [ ] `rusted-agents`
- [ ] `rusted-permissions`

#### Provider Manager

Initial provider targets:

- OpenAI
- Anthropic
- Google Gemini
- Alibaba Qwen
- DeepSeek
- Moonshot AI / Kimi
- Llama-compatible hosted inference providers

Provider integrations must account for differences in authentication, endpoints, model capabilities, tool calling, and streaming.

OpenAI-compatible APIs may share implementation paths where appropriate, but compatibility must be verified rather than assumed.

#### AI Router

- Provider and model selection.
- Request routing.
- Capability-aware model selection.
- Streaming responses.
- Provider-specific error handling.
- Explicit user-controlled fallback behavior.

#### BYOK Credential Management

- Users supply their own API keys.
- Store credentials through the desktop credential store.
- Never store API keys in plaintext configuration files or logs.
- Do not transmit credentials to unrelated services.

#### Context Engine

- Explicitly scoped context access.
- User-controlled data sharing.
- No unrestricted filesystem scanning by default.
- No continuous screen capture by default.
- Transparent context selection.

#### Agent Runtime

- Tool registration and discovery.
- Controlled tool execution.
- Task lifecycle management.
- Execution limits.
- Audit-friendly activity records.

#### Permission Gateway

- Explicit permission checks.
- User confirmation for sensitive actions.
- Privileged operations handled through controlled system interfaces.
- No unrestricted root access for AI agents.

**Exit criteria:**

- At least one cloud provider works end to end.
- Users can configure their own credentials securely.
- Provider selection works.
- AI functionality is optional.
- Agent tool execution is permission-controlled.
- Sensitive operations require appropriate authorization.

### Phase 4 — Application Ecosystem & System Integration

**Goal:** Make Rusted OS practical for daily desktop usage.

Planned work:

- [ ] Flatpak integration.
- [ ] Application discovery and installation.
- [ ] Initial App Center.
- [ ] File manager.
- [ ] Terminal integration.
- [ ] System update mechanism.
- [ ] Rollback and recovery strategy.
- [ ] Fedora image-based deployment evaluation.
- [ ] Developer tooling integration.
- [ ] Documentation for application developers.

Technical direction:

- Preserve compatibility with established Linux applications.
- Evaluate Fedora Atomic and bootc for image-based system management.
- Use existing Linux application packaging and security mechanisms.
- Avoid reinventing mature infrastructure without a clear benefit.

**Exit criteria:**

- Users can install and run common Linux applications.
- System updates have a documented recovery strategy.
- Application installation does not require custom packaging for every application.
- Developer workflows are usable.

### Phase 5 — Rusted OS Alpha

**Goal:** Deliver the first bootable and testable Rusted OS release.

Alpha scope:

- [ ] Bootable x86-64 image.
- [ ] Installation or documented test deployment.
- [ ] Login and graphical desktop.
- [ ] Window management.
- [ ] Terminal and basic Linux application support.
- [ ] Network and audio integration.
- [ ] System settings.
- [ ] System monitoring.
- [ ] Resource Manager.
- [ ] Developer Mode.
- [ ] Cloud AI integration.
- [ ] BYOK credential management.
- [ ] Multi-provider architecture.
- [ ] Permission-controlled AI agents.
- [ ] System update and recovery strategy.
- [ ] Installation and troubleshooting documentation.

**Alpha acceptance criteria:**

1. The system boots and reaches a usable desktop.
2. Essential desktop operations do not depend on AI availability.
3. Network, audio, keyboard, mouse, and display work on supported hardware.
4. Users can run common Linux applications.
5. AI works through user-configured cloud credentials.
6. AI agents cannot execute privileged operations without authorization.
7. Background workload behavior is measurable and documented.
8. Developer Mode prioritizes development workloads without disabling essential security services.
9. System updates and recovery have a documented procedure.
10. Known limitations are clearly documented.

## 4. Future Development

These are intentionally outside the initial Alpha commitment.

- [ ] Optional local AI inference.
- [ ] llama.cpp integration evaluation.
- [ ] ARM64 support.
- [ ] Additional hardware compatibility.
- [ ] Advanced automation workflows.
- [ ] More desktop applications.
- [ ] Expanded accessibility features.
- [ ] Developer SDK.
- [ ] Community extension ecosystem.
- [ ] Advanced power and thermal management.

Local AI must remain optional and resource-aware. It must not become a mandatory dependency for the desktop.

## 5. Performance & Reliability Benchmarks

Performance is a design requirement, not a marketing claim.

Benchmark categories:

| Category | Metrics |
|---|---|
| Boot | Time to login and usable desktop |
| Memory | Idle RAM and per-service memory |
| CPU | Idle utilization and background CPU time |
| Wakeups | Background process wakeup frequency |
| Storage | Background disk I/O |
| Desktop | Application launch latency and responsiveness |
| Development | Build time and development workload responsiveness |
| AI | Request latency and resource consumption |

All performance claims must be supported by reproducible measurements.

Benchmarks should compare equivalent hardware, workloads, and software configurations.

## 6. Supported Hardware Strategy

Initial development target:

- x86-64.
- Integrated graphics support where available.
- 4 GB RAM as a minimum design target.
- 8 GB RAM as a more practical baseline.
- SSD-based installation preferred.

Hardware profiles are design targets, not guarantees of compatibility.

A broader hardware compatibility matrix will be maintained as testing expands.

## 7. Release Strategy

| Release | Objective |
|---|---|
| Internal Prototype | Validate core architecture |
| Developer Preview | Test core services and desktop experiments |
| Alpha | Bootable system with essential desktop and AI capabilities |
| Beta | Broader hardware and application compatibility |
| Stable | Reliable daily-use desktop distribution |

Release dates will be determined by milestone completion, not arbitrary calendar commitments.

## 8. Scope Management

Every new feature must answer:

1. Does it support the Rusted OS vision?
2. Does it introduce unnecessary resource consumption?
3. Does it create a new security boundary?
4. Can existing Linux infrastructure solve the problem?
5. Can it be tested independently?
6. Is it required for the current milestone?

Features that do not support the current milestone should remain documented as future work.

## 9. Roadmap Maintenance

This roadmap is a living document.

Changes must reflect:

- Completed milestones.
- Technical discoveries.
- Architecture decisions.
- Hardware testing results.
- Security reviews.
- Community feedback.

A roadmap item is complete only when its implementation, tests, documentation, and acceptance criteria are sufficiently satisfied.

---

**Rusted OS — Build deliberately. Keep it efficient. Make it secure.**
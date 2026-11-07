# Rusted OS — System Architecture

**Architecture Specification v0.1**  
**Project:** Rusted OS  
**Status:** Proposed  
**Foundation:** Fedora Linux + Rust  
**Architecture:** Modular, Service-Oriented, Cloud-First AI

---

# 1. Overview

Rusted OS is a Rust-first, AI-native desktop operating system built on top of the Linux kernel and Fedora ecosystem.

Its architecture focuses on five fundamental objectives:

1. **Reliability:** Preserve Linux kernel stability and reuse mature Linux infrastructure.
2. **Performance:** Minimize unnecessary resource consumption and background activities.
3. **Security:** Apply explicit permissions, capability-based access, and controlled system operations.
4. **Intelligence:** Integrate multi-provider cloud AI into the desktop environment.
5. **Modularity:** Build independently testable components with well-defined interfaces.

Rusted OS does not aim to replace the Linux kernel or recreate existing Linux infrastructure.

Instead, it introduces a custom desktop experience, system orchestration layer, resource management policies, and AI integration.

## 1.1 Core principles

| Principle | Description |
|---|---|
| Rust-first | New system components prioritize Rust |
| Linux-powered | Use upstream Linux kernel and Fedora infrastructure |
| AI-native | AI integrates with system workflows |
| Cloud-first AI | Cloud API is the initial AI execution model |
| BYOK | Users provide their own AI provider credentials |
| Resource-efficient | Background activities must be justified |
| Secure by design | AI agents never receive unrestricted system access |
| Offline-capable | Core OS functions work without internet or AI |
| Modular | Components have clear boundaries and contracts |

---

# 2. High-Level Architecture

Rusted OS consists of five logical layers.

```text
┌──────────────────────────────────────────────────────┐
│                  RUSTED INTELLIGENCE                  │
│                                                      │
│  Assistant │ AI Router │ Context │ Agent Runtime     │
│  Provider Manager │ Automation │ Permission Gateway │
├──────────────────────────────────────────────────────┤
│                    RUSTED DESKTOP                    │
│                                                      │
│  Shell │ Compositor │ Launcher │ Workspace Manager   │
│  Settings │ Notifications │ System Monitor           │
├──────────────────────────────────────────────────────┤
│                  RUSTED SYSTEM CORE                  │
│                                                      │
│  Core │ Config │ IPC │ Security │ Resource Manager   │
│  Service Integration │ Runtime │ Diagnostics         │
├──────────────────────────────────────────────────────┤
│                   LINUX FOUNDATION                   │
│                                                      │
│  Linux Kernel │ systemd │ SELinux │ D-Bus            │
│  NetworkManager │ PipeWire │ Mesa │ Flatpak           │
├──────────────────────────────────────────────────────┤
│                       HARDWARE                       │
│                                                      │
│  CPU │ RAM │ GPU │ Storage │ Network │ Peripherals   │
└──────────────────────────────────────────────────────┘
```

This is a logical architecture. It does not require every component to run as a separate process.

Components should initially be implemented as Rust libraries or services according to their operational requirements.

---

# 3. Linux Foundation

## 3.1 Kernel strategy

Rusted OS uses the upstream Linux kernel.

We will not maintain a custom kernel fork during the initial development stages.

Responsibilities inherited from Linux:

- Process scheduling.
- Memory management.
- Hardware drivers.
- Filesystem operations.
- Networking.
- Device management.
- Security primitives.
- System calls.

Rusted OS will interact with these facilities through established Linux interfaces.

## 3.2 Fedora integration

Fedora provides the initial system foundation.

Core integrations:

| Component | Responsibility |
|---|---|
| systemd | Service lifecycle and process management |
| D-Bus | System and desktop communication |
| SELinux | Mandatory access control |
| NetworkManager | Network connectivity |
| PipeWire | Audio and multimedia |
| Mesa | Graphics drivers |
| Flatpak | Application distribution |
| cgroups v2 | Resource control |

Rusted OS should reuse existing Fedora services rather than introducing competing implementations.

## 3.3 System integration policy

Every integration must have:

- A clearly defined responsibility.
- A documented interface.
- Error handling.
- Logging.
- Resource considerations.
- Security boundaries.

Direct access to privileged system operations must be restricted to trusted system components.

---

# 4. Rusted System Core

The System Core provides foundational functionality used by the rest of the operating system.

Initial components:

```text
rusted-core
rusted-config
rusted-ipc
rusted-security
rusted-services
rusted-runtime
rusted-resource-manager
```

## 4.1 rusted-core

Responsible for:

- System lifecycle abstractions.
- Shared foundational types.
- Component metadata.
- Error types.
- System state definitions.
- Common interfaces.

It should remain lightweight and avoid unnecessary dependencies.

## 4.2 rusted-config

Responsible for system and application configuration.

Requirements:

- Structured configuration.
- Schema validation.
- Configuration versioning.
- Default values.
- Safe configuration updates.
- Configuration migration.

Configuration should not contain plaintext API credentials.

## 4.3 rusted-ipc

Provides communication between Rusted OS components.

Initial integration:

- D-Bus for Linux desktop and system interoperability.
- Tokio for asynchronous application logic.
- Structured messages using Serde.

IPC contracts must be versioned where compatibility is required.

Not every internal function call needs IPC. Libraries should remain libraries when separate processes provide no meaningful benefit.

## 4.4 rusted-security

Provides common security abstractions.

Responsibilities:

- Permission definitions.
- Capability representation.
- Access policy evaluation.
- Authorization decisions.
- Permission auditing.
- Security-related interfaces.

Actual enforcement must rely on operating-system security mechanisms such as SELinux, Unix permissions, Linux capabilities, and service isolation.

A Rust permission check alone must never be treated as a complete security boundary.

## 4.5 rusted-services

Provides integration with system services.

Responsibilities:

- Service discovery.
- Service status.
- Lifecycle integration.
- Health reporting.
- Controlled service activation.

systemd remains the underlying service manager.

## 4.6 rusted-runtime

Provides runtime support for Rusted OS applications and internal components.

Responsibilities:

- Component initialization.
- Lifecycle coordination.
- Runtime configuration.
- Graceful shutdown.
- Error propagation.

---

# 5. Rusted Resource Manager

The Resource Manager is a core differentiating subsystem.

Its purpose is to protect system responsiveness by controlling unnecessary background workloads.

## 5.1 Responsibilities

- Monitor CPU utilization.
- Monitor memory usage.
- Monitor disk I/O.
- Track background service activity.
- Apply resource policies.
- Defer non-essential workloads.
- Support workload prioritization.
- Report resource consumption.

## 5.2 Architecture

```text
             SYSTEM MONITOR
                   │
                   ▼
          RESOURCE COLLECTOR
                   │
                   ▼
           RESOURCE MANAGER
                   │
          ┌────────┴────────┐
          ▼                 ▼
     POLICY ENGINE     WORKLOAD STATE
          │                 │
          └────────┬────────┘
                   ▼
            ACTION CONTROLLER
                   │
                   ▼
       Linux cgroups v2 / systemd
```

## 5.3 Workload classification

Every managed workload should belong to a resource category.

| Category | Description |
|---|---|
| Critical | Essential OS and security operations |
| Interactive | Applications currently used by the user |
| Background | Non-urgent background operations |
| On-demand | Started only when requested |
| Maintenance | Updates, indexing, diagnostics |

## 5.4 Resource policies

Examples:

- Defer file indexing during intensive compilation.
- Avoid unnecessary background polling.
- Schedule maintenance during idle periods.
- Keep AI runtime inactive when unused.
- Prevent non-essential services from consuming excessive resources.
- Allow users to configure workload priorities.

The Resource Manager must not blindly terminate services based on CPU usage.

Critical services must remain operational.

## 5.5 Developer Mode

Developer Mode applies a workload policy optimized for development.

Priority workloads:

- IDEs.
- Terminals.
- Compilers.
- Build systems.
- Development servers.
- Databases.
- Containers.

Non-essential background workloads may be deferred.

Security, networking, and essential system services remain active.

## 5.6 Performance targets

Performance targets must be established through reproducible benchmarks.

Metrics:

- Idle RAM.
- Idle CPU.
- Background wakeups.
- Disk I/O.
- Application launch time.
- Interactive responsiveness.
- Build performance.
- Resource overhead per service.

No arbitrary performance guarantees should be published before measurement.

---

# 6. Rusted Intelligence

Rusted Intelligence provides AI capabilities to the desktop environment.

Its initial architecture is cloud-first.

Local AI inference is optional and not required for normal operation or Alpha release.

## 6.1 Component architecture

```text
                  USER INTERFACE
                        │
                        ▼
                 RUSTED ASSISTANT
                        │
                        ▼
                   AI ROUTER
                        │
                        ▼
               PROVIDER MANAGER
                        │
            ┌───────────┼───────────┐
            ▼           ▼           ▼
         OPENAI      ANTHROPIC    GOOGLE
            │           │           │
            └───────────┼───────────┘
                        │
             OTHER SUPPORTED PROVIDERS
                        │
                        ▼
                 CLOUD AI APIs
```

Additional internal components:

- Context Engine.
- Agent Runtime.
- Tool Registry.
- Permission Gateway.
- Automation Engine.

## 6.2 Cloud-first architecture

Rusted OS communicates with AI providers through their supported APIs.

The user supplies their own API credentials.

Supported provider targets:

| Provider | Ecosystem |
|---|---|
| OpenAI | GPT |
| Anthropic | Claude |
| Google | Gemini |
| Meta ecosystem | Llama through supported hosting providers |
| Alibaba | Qwen |
| DeepSeek | DeepSeek |
| Moonshot AI | Kimi |

Provider availability and model capabilities depend on their respective API services.

## 6.3 Bring Your Own Key

The BYOK system must support:

- Adding provider credentials.
- Removing credentials.
- Selecting default providers.
- Selecting models.
- Configuring supported endpoints.
- Testing connectivity.
- Handling authentication errors.
- Handling rate limits.
- Displaying usage information where available.

API credentials must be stored using a secure credential store.

Examples of supported integration approaches:

- Secret Service / desktop keyring.
- OS-supported credential storage.
- Secure credential APIs.

Plaintext API keys must never be written into normal configuration files or application logs.

## 6.4 Provider adapter architecture

Each provider must implement a common internal contract.

Conceptual interface:

```rust
pub trait AIProvider {
    async fn generate(
        &self,
        request: AIRequest,
    ) -> Result<AIResponse, AIError>;

    async fn health_check(
        &self,
    ) -> Result<ProviderStatus, AIError>;
}
```

This is an architectural illustration, not a final implementation.

Provider-specific adapters handle:

- Authentication.
- Request serialization.
- Response parsing.
- Streaming responses.
- Error normalization.
- Rate limits.
- Model-specific capabilities.

OpenAI-compatible APIs may share an adapter where appropriate, but provider-specific implementations must remain possible.

## 6.5 AI Router

The AI Router selects the configured provider and model.

Routing considerations:

- User preference.
- Task requirements.
- Model capabilities.
- Provider availability.
- Rate limits.
- User-defined cost preferences.

Automatic provider fallback must be transparent and configurable.

Rusted OS must not silently send user data to another provider.

## 6.6 Context Engine

Context Engine provides relevant system context to the assistant.

Possible context sources:

- Active application.
- Selected file.
- Current workspace.
- Explicitly shared document.
- System status.
- User-approved application metadata.

Context access must be controlled.

Requirements:

- No unrestricted filesystem scanning.
- No continuous screen capture by default.
- No automatic upload of sensitive documents.
- Explicit context-sharing policies.
- User-accessible context controls.
- Clear indication of information being shared with cloud providers.

## 6.7 Agent Runtime

Agent Runtime manages AI tool execution.

Possible tools:

- File search.
- File operations.
- Application launching.
- Workspace management.
- System information.
- Approved shell commands.
- Configuration management.

The runtime must separate planning from execution.

An AI-generated instruction is not automatically an authorized system operation.

## 6.8 Permission Gateway

Every sensitive tool operation must pass through the Permission Gateway.

Example policy:

| Operation | Authorization |
|---|---|
| Read selected file | Scoped permission |
| Search approved directory | Scoped permission |
| Open application | Policy-based |
| Create file | Scoped permission |
| Modify file | Confirmation or scoped permission |
| Delete file | Explicit confirmation |
| Install application | Explicit confirmation |
| Execute shell command | Risk-based confirmation |
| Privileged command | Explicit authorization |
| Modify system settings | Explicit authorization |

AI agents must never receive unrestricted root access.

Permission grants must be revocable and auditable.

## 6.9 Local AI — Future extension

Local inference is not part of the initial mandatory architecture.

Future integrations may support:

- llama.cpp.
- Other compatible local inference runtimes.

Possible use cases:

- Offline assistance.
- Local summarization.
- Private document processing.
- Lightweight classification.

Local inference must have configurable CPU and memory limits.

It must not automatically consume system resources when unused.

---

# 7. Rusted Desktop

Rusted Desktop provides the user-facing environment.

Initial components:

```text
rusted-shell
rusted-compositor
rusted-launcher
rusted-settings
rusted-notifications
rusted-system-monitor
```

## 7.1 Desktop Shell

Responsible for:

- Desktop layout.
- Panels.
- Application interaction.
- Workspace integration.
- Assistant entry points.

## 7.2 Compositor

Wayland-based compositor.

Smithay is an initial candidate for evaluation.

Responsibilities:

- Window management.
- Display composition.
- Input handling.
- Workspace integration.
- Graphics presentation.

## 7.3 Smart Launcher

Provides:

- Application search.
- File search.
- System settings search.
- Command discovery.
- AI assistant integration.

AI suggestions must not interfere with ordinary launcher functionality.

## 7.4 System Monitor

Provides visibility into:

- CPU.
- RAM.
- Disk.
- Network.
- Running processes.
- Background services.
- Resource policies.

Users should be able to identify which components are consuming resources.

---

# 8. Service Lifecycle and Background Execution

Rusted OS follows an on-demand and event-driven service model.

## 8.1 Service categories

| Service | Execution model |
|---|---|
| Security | Always active where required |
| Networking | Managed by system configuration |
| Audio | Managed by system configuration |
| Desktop shell | Active during desktop session |
| AI Assistant | On-demand |
| AI Agent Runtime | On-demand |
| Context Engine | Event-driven |
| File Indexer | Scheduled / on-demand |
| Update Checker | Scheduled / configurable |
| Crash Reporter | Event-triggered |
| Maintenance | Scheduled / deferred |

## 8.2 Service requirements

Every Rusted OS service must document:

- Purpose.
- Startup condition.
- Shutdown condition.
- Resource requirements.
- Dependencies.
- Permissions.
- Failure behavior.
- Logging policy.

Services must support graceful shutdown wherever practical.

## 8.3 Failure isolation

A failure in an optional service must not crash the desktop or essential system components.

Examples:

- AI provider unavailable: assistant reports connection failure.
- AI Agent crashes: desktop remains functional.
- Context Engine fails: ordinary desktop operations continue.
- Indexer fails: files remain accessible.
- Resource Manager fails: Linux system scheduling continues normally.

---

# 9. Security Architecture

Security is enforced through multiple layers.

```text
               USER
                 │
                 ▼
          APPLICATION LAYER
                 │
                 ▼
          PERMISSION GATEWAY
                 │
                 ▼
           TRUSTED SERVICE
                 │
                 ▼
       LINUX SECURITY MECHANISMS
                 │
                 ▼
             SYSTEM
```

Security mechanisms:

- SELinux.
- Unix user and group permissions.
- Linux capabilities.
- systemd service isolation.
- D-Bus authorization.
- Secure credential storage.
- Scoped agent permissions.
- Auditable privileged operations.

Security policies must be enforced independently of AI provider behavior.

No AI provider should be trusted to enforce local system permissions.

---

# 10. Data and Configuration Architecture

Rusted OS separates different classes of data.

| Data | Storage strategy |
|---|---|
| System configuration | System configuration directory |
| User configuration | User configuration directory |
| API credentials | Secure credential store |
| Application data | Application-specific directories |
| Logs | systemd journal / structured logs |
| AI conversation history | User-controlled storage |
| Context metadata | Explicitly configured storage |
| Temporary files | Temporary runtime directories |

Data retention must be configurable.

Conversation history and context data must not be transmitted to cloud providers unless required by an explicitly initiated request.

---

# 11. Application Compatibility

Initial compatibility priorities:

1. Native Linux applications.
2. Flatpak applications.
3. Linux CLI tools.
4. Development environments.
5. Existing Linux desktop applications.

Windows compatibility through Wine may be evaluated later.

Rusted OS must not claim universal application compatibility without testing.

---

# 12. Repository Structure

Long-term target:

```text
rusted-os/
│
├── Cargo.toml
├── rust-toolchain.toml
├── README.md
├── LICENSE
│
├── crates/
│   ├── rusted-core/
│   ├── rusted-config/
│   ├── rusted-ipc/
│   ├── rusted-security/
│   ├── rusted-services/
│   ├── rusted-runtime/
│   ├── rusted-resource-manager/
│   │
│   ├── rusted-shell/
│   ├── rusted-compositor/
│   ├── rusted-launcher/
│   ├── rusted-settings/
│   ├── rusted-notifications/
│   ├── rusted-system-monitor/
│   │
│   ├── rusted-ai/
│   ├── rusted-ai-providers/
│   ├── rusted-ai-router/
│   ├── rusted-context/
│   ├── rusted-agents/
│   └── rusted-permissions/
│
├── apps/
│   ├── file-manager/
│   ├── terminal/
│   └── app-center/
│
├── integrations/
│   ├── wayland/
│   ├── dbus/
│   ├── flatpak/
│   ├── systemd/
│   └── credential-store/
│
├── boot/
├── installer/
├── docs/
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── decisions/
│
├── tests/
└── scripts/
```

This is the long-term structure.

Only necessary crates should be created during the initial bootstrap.

---

# 13. Initial Implementation Architecture

The first implementation must remain small.

Initial workspace:

```text
rusted-os/
├── Cargo.toml
├── rust-toolchain.toml
├── README.md
├── LICENSE
│
├── crates/
│   ├── rusted-core/
│   ├── rusted-config/
│   ├── rusted-ipc/
│   └── rusted-security/
│
├── docs/
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── decisions/
│
└── tests/
```

Initial development priorities:

1. Establish Rust workspace.
2. Define shared error and type conventions.
3. Implement `rusted-core`.
4. Implement `rusted-config`.
5. Define IPC contracts.
6. Implement initial security abstractions.
7. Establish unit and integration tests.
8. Prototype Resource Manager integration.

AI provider implementation should begin only after the foundational contracts are sufficiently stable.

---

# 14. Architecture Decision Records

Major architectural decisions must be documented in ADR files.

Initial ADR candidates:

| ADR | Decision |
|---|---|
| ADR-001 | Fedora as the initial base distribution |
| ADR-002 | Upstream Linux kernel strategy |
| ADR-003 | Rust-first implementation strategy |
| ADR-004 | Modular Cargo workspace |
| ADR-005 | D-Bus and Tokio integration |
| ADR-006 | Cloud-first BYOK AI architecture |
| ADR-007 | Multi-provider AI adapter architecture |
| ADR-008 | Permission-based AI agent execution |
| ADR-009 | Resource Manager architecture |
| ADR-010 | On-demand background service policy |
| ADR-011 | Wayland desktop architecture |
| ADR-012 | Secure credential storage |

Every ADR should contain:

- Context.
- Problem.
- Considered options.
- Decision.
- Consequences.
- Implementation status.

Decisions may be revised when new evidence justifies a change.

---

# 15. Non-Functional Requirements

Rusted OS must be evaluated beyond feature completeness.

## Reliability

- AI failure must not affect core OS operation.
- Optional service failure must be isolated.
- System updates must support recovery.
- Configuration changes must be validated.

## Performance

- Minimize idle CPU usage.
- Minimize unnecessary memory consumption.
- Avoid unnecessary background polling.
- Protect interactive workloads.
- Benchmark resource usage continuously.

## Security

- No unrestricted AI root access.
- Secure credential handling.
- Explicit authorization for sensitive operations.
- Isolation between optional services.
- Auditable privileged operations.

## Privacy

- User-controlled context.
- Transparent cloud requests.
- Configurable data retention.
- No unnecessary background telemetry.
- No automatic sensitive-data uploads.

## Maintainability

- Modular crates.
- Clear interface contracts.
- Automated testing.
- Documented architectural decisions.
- Minimal unnecessary dependencies.

---

# 16. Definition of Done — Architecture Foundation

The architecture foundation is considered ready when:

- Cargo workspace structure is established.
- Core interfaces are documented.
- Security boundaries are defined.
- IPC strategy is documented.
- Resource Manager responsibilities are defined.
- AI provider abstraction is documented.
- Credential storage strategy is documented.
- Background service policies are documented.
- Initial ADRs are created.
- CI and automated tests are operational.

This milestone does not require a bootable operating system.

It establishes the engineering foundation for subsequent implementation.

---

# 17. Final Architecture Principles

Rusted OS must follow these principles throughout development:

1. Do not reinvent stable Linux infrastructure without a strong technical reason.
2. Do not make AI a requirement for normal OS operation.
3. Do not require local AI inference.
4. Do not lock the operating system to a single AI provider.
5. Do not store API credentials in plaintext.
6. Do not give AI agents unrestricted system access.
7. Do not run unnecessary background services.
8. Do not sacrifice system reliability for AI features.
9. Do not introduce architectural complexity without measurable benefits.
10. Do not optimize performance based on assumptions; benchmark it.

---

## Closing Statement

Rusted OS is designed around the idea that an operating system should be intelligent without becoming intrusive, powerful without becoming resource-hungry, and automated without taking control away from its users.

> Rusted OS should not make users adapt to AI. AI should adapt to the user's operating environment.

> Your hardware should serve your workload, not unnecessary background processes.

**Rusted OS — An intelligent desktop, built on a solid foundation.**
# Rusted OS

<p align="center">
  <strong>A Rust-first, AI-native Linux desktop operating system.</strong>
  <br />
  Built for performance, security, and developer experience.
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Status-Early%20Development-orange" alt="Project Status" />
  <img src="https://img.shields.io/badge/Language-Rust-orange" alt="Rust" />
  <img src="https://img.shields.io/badge/Base-Fedora-blue" alt="Fedora" />
  <img src="https://img.shields.io/badge/License-GPL--3.0--or--later-green" alt="GPL-3.0-or-later" />
</p>

---

## What is Rusted OS?

Rusted OS is an open-source, Rust-first desktop operating system built on the upstream Linux kernel and Fedora ecosystem.

The project aims to combine Linux reliability, resource-efficient desktop computing, secure AI integration, and a developer-focused environment into one cohesive operating system.

Rusted OS is not intended to be another Linux distribution with a customized theme.

It is an effort to build a thoughtfully engineered desktop experience, supported by custom Rust components and proven Linux infrastructure.

## Our Vision

We believe a modern operating system should be:

- **Efficient:** Hardware resources should prioritize the applications people actually use.
- **Secure:** Security must be built into the architecture, not added as an afterthought.
- **Intelligent:** AI should assist users and automate workflows through controlled permissions.
- **Developer-friendly:** Development workloads should receive appropriate system resources.
- **Open:** Users and developers should be able to inspect, modify, and improve the software.

## Core Principles

### 1. Linux First

Rusted OS builds on the upstream Linux kernel and established Linux infrastructure.

We prefer proven technologies over reinventing existing system capabilities without a compelling reason.

### 2. Rust First

Rust is the primary language for Rusted OS-owned system components.

Memory safety, explicit interfaces, and maintainable architecture are important foundations of the project.

### 3. Resource Efficiency

An operating system should not compete unnecessarily with the applications running on it.

Rusted OS aims to minimize unnecessary background workloads through:

- On-demand services.
- Event-driven operations.
- Efficient resource monitoring.
- Linux cgroups v2 integration.
- Workload-aware resource policies.
- Developer Mode.

Performance improvements must be demonstrated through reproducible benchmarks.

### 4. AI-Native, Not AI-Dependent

AI is an integrated capability, not a requirement for operating the system.

Boot, login, basic desktop functionality, networking, and recovery must remain independent of AI availability.

Rusted OS follows a cloud-first, Bring Your Own Key (BYOK) approach.

Users configure their preferred AI provider and manage their own API credentials.

### 5. Security by Design

AI agents must never receive unrestricted root access.

Sensitive operations require appropriate authorization through controlled permission mechanisms.

User context and data access must remain explicit and user-controlled.

## AI Integration

Rusted OS is designed around a modular, multi-provider AI architecture.

Potential provider integrations include:

- OpenAI
- Anthropic Claude
- Google Gemini
- Alibaba Qwen
- DeepSeek
- Moonshot AI / Kimi
- Hosted Llama-compatible inference providers

The architecture is designed to accommodate provider-specific authentication, model capabilities, tool calling, and streaming behavior.

### BYOK Architecture

Users provide their own API credentials.

Rusted OS aims to:

- Store credentials through the operating system's secure credential store.
- Allow users to select their preferred provider and model.
- Avoid silently switching providers.
- Make cloud data transmission transparent.
- Keep AI functionality optional.

Local AI inference may be explored in the future, but it is not a requirement for the initial release.

## Technology Stack

The current technical direction includes:

| Layer | Technology |
|---|---|
| Base distribution | Fedora |
| Kernel | Upstream Linux |
| Primary language | Rust |
| Desktop protocol | Wayland |
| Compositor | Smithay (under evaluation) |
| UI framework | Iced / Slint (under evaluation) |
| Graphics | wgpu |
| Async runtime | Tokio |
| Serialization | Serde |
| Service management | systemd |
| IPC | D-Bus |
| Security | SELinux and Linux security mechanisms |
| Application ecosystem | Flatpak |

Technology selections are subject to implementation and architectural evaluation.

## Architecture

Rusted OS is organized into five logical layers.

```text
┌───────────────────────────────────────────┐
│             RUSTED INTELLIGENCE           │
│  Assistant · AI Router · Agents           │
│  Context Engine · Permission Gateway      │
├───────────────────────────────────────────┤
│               RUSTED DESKTOP              │
│  Shell · Compositor · Launcher            │
│  Settings · Notifications · Monitor       │
├───────────────────────────────────────────┤
│              RUSTED SYSTEM CORE           │
│  Core · Config · IPC · Security           │
│  Runtime · Services · Resource Manager    │
├───────────────────────────────────────────┤
│              LINUX FOUNDATION             │
│  Fedora · Linux Kernel · systemd          │
│  D-Bus · SELinux · PipeWire · Mesa        │
├───────────────────────────────────────────┤
│                  HARDWARE                 │
│             x86-64 (initial target)        │
└───────────────────────────────────────────┘
```

For more details, see [ARCHITECTURE.md](ARCHITECTURE.md).

## Development Roadmap

Rusted OS is being developed incrementally.

| Phase | Focus | Status |
|---|---|---|
| 0 | Foundation & Architecture | In planning |
| 1 | Rusted System Core | Planned |
| 2 | Rusted Desktop | Planned |
| 3 | Rusted Intelligence | Planned |
| 4 | Application Ecosystem | Planned |
| 5 | Bootable Alpha Release | Planned |

See [ROADMAP.md](ROADMAP.md) for detailed milestones and acceptance criteria.

## Project Structure

The intended repository architecture is a modular Rust workspace.

```text
rusted-os/
├── Cargo.toml
├── rust-toolchain.toml
├── README.md
├── ARCHITECTURE.md
├── ROADMAP.md
├── AGENT.md
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── LICENSE
├── crates/
│   ├── rusted-core/
│   ├── rusted-config/
│   ├── rusted-ipc/
│   └── rusted-security/
├── docs/
│   └── decisions/
└── tests/
```

This is the initial development structure. Additional crates and applications will be introduced as their responsibilities become clear.

## Getting Started

Rusted OS is currently in early development. A bootable ISO and end-user installation process are not yet available.

### Prerequisites

For initial Rust component development:

- Rust toolchain.
- Cargo.
- Git.
- A Linux development environment is recommended.

### Clone the Repository

```bash
git clone https://github.com/joniilmanfahmi00-collab/rusted-os.git
cd rusted-os
```

### Build the Workspace

Once the initial Cargo workspace and crates are implemented:

```bash
cargo build --workspace
```

Run tests:

```bash
cargo test --workspace
```

These commands describe the intended development workflow; they do not imply that all workspace components are already implemented.

## Contributing

Contributions, discussions, ideas, and technical feedback are welcome. Please start with [CONTRIBUTING.md](CONTRIBUTING.md), [SECURITY.md](SECURITY.md), and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

Before contributing:

1. Read [ARCHITECTURE.md](ARCHITECTURE.md).
2. Review [ROADMAP.md](ROADMAP.md).
3. Follow the development rules in [AGENT.md](AGENT.md) and [CONTRIBUTING.md](CONTRIBUTING.md).
4. Check existing issues before starting substantial work.
5. Discuss major architectural changes (with an ADR proposal) before implementation.

We value:

- Clear technical reasoning.
- Small, reviewable changes.
- Reproducible tests.
- Security-conscious implementations.
- Honest performance measurements.
- Thoughtful architectural decisions.

## Licensing

Rusted OS project-owned core components are intended to be distributed under the **GNU General Public License v3.0 or later (GPL-3.0-or-later)**.

This means recipients of covered distributed versions must receive the rights and corresponding source code required by the GPL.

Third-party components retain their respective original licenses.

The Linux kernel, Fedora packages, and external dependencies are not automatically relicensed under Rusted OS's project license.

See [LICENSE](LICENSE) for the full license text.

## Project Status

**Rusted OS is an experimental project in early development.**

The architecture, technology choices, and implementation are evolving.

It is not currently intended for production use or as a replacement for an existing operating system.

Our priority is to establish a reliable foundation before pursuing a complete desktop experience.

---

<p align="center">
  <strong>Build deliberately. Keep it efficient. Make it secure.</strong>
  <br />
  <sub>Rusted OS — An open-source operating system project.</sub>
</p>


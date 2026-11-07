# ADR-003: GPL-3.0-or-later for Project-Owned Components

- Status: Accepted
- Date: 2026-11-07

## Context

Rusted OS aims to preserve users' freedom to inspect, modify,
and redistribute its project-owned software.

## Decision

Project-owned Rusted OS components are licensed under
GNU GPL version 3 or, at the recipient's option, any later version.

Third-party dependencies and system components retain their
respective licenses.

## Consequences

- Workspace crates inherit GPL-3.0-or-later.
- Third-party license obligations must be reviewed.
- The Linux kernel and Fedora packages are not relicensed by this decision.
- Distribution and combination questions must be reviewed individually.
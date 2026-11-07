# ADR-004: Modular Cargo Workspace

- Status: Accepted
- Date: 2026-11-07

## Context

Rusted OS contains multiple logical components that need clear
boundaries, independent tests, and shared project conventions.

## Decision

Use a Cargo workspace with separate crates for foundational
system components.

Initial members:

- rusted-core
- rusted-config
- rusted-ipc
- rusted-security

## Consequences

- Shared package metadata is maintained at the workspace root.
- Crates can be tested independently or together.
- New crates are added when a real architectural boundary exists.
- Avoid splitting code into crates without a clear reason.
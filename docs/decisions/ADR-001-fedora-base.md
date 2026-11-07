# ADR-001: Fedora as the Base Distribution

- Status: Accepted
- Date: 2026-11-07

## Context

Rusted OS needs a reliable Linux foundation with modern packages,
established security infrastructure, and a maintainable update ecosystem.

## Decision

Use Fedora as the initial base distribution.

## Consequences

- Reuse Fedora packages and infrastructure where practical.
- Avoid maintaining a custom package ecosystem prematurely.
- Evaluate image-based deployment and rollback in a later phase.
- Reassess the base only when concrete technical requirements justify it.
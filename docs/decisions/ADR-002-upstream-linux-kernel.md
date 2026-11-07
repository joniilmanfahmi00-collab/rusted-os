# ADR-002: Use the Upstream Linux Kernel

- Status: Accepted
- Date: 2026-11-07

## Context

Maintaining a custom kernel fork creates significant long-term
maintenance, security, and compatibility costs.

## Decision

Rusted OS will use the upstream Linux kernel and Fedora kernel packages.

Kernel patches or custom modules must have a specific technical
justification and should not create an unnecessary permanent fork.

## Consequences

- Benefit from upstream fixes and hardware support.
- Keep kernel maintenance outside the initial project scope.
- Integrate through established Linux interfaces.
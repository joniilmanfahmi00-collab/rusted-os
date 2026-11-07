#![doc = "Security and permission abstractions for Rusted OS."]

//! Pure authorization decision layer.
//!
//! This crate provides a small, explicit policy: a [`Permission`] enum, a
//! [`Decision`] enum, and a deterministic [`authorize`] function.
//!
//! What this crate is *not*:
//! - Not OS-level enforcement. It does not check Linux users, groups,
//!   SELinux contexts, capabilities, or polkit rules.
//! - No filesystem access, no process execution, no privilege escalation.
//! - No async runtime, no global state, no configuration, no audit logging.
//! - No roles, sessions, tokens, or AI integration. Add those only with a
//!   documented architectural need.
//!
//! Real enforcement must happen in privileged system components that combine
//! this decision with OS identity, user confirmation UI, and auditing.
//! Callers must treat `Allow` as "policy permits" rather than "safe to skip
//! all further checks".

/// Minimal set of operations covered by the early security policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    /// Read non-secret configuration.
    ReadConfig,
    /// Modify non-secret configuration.
    WriteConfig,
    /// Run a command or tool on behalf of the caller.
    ExecuteCommand,
    /// Install software packages.
    InstallSoftware,
    /// Change system-wide settings or services.
    ModifySystem,
}

/// Authorization outcome for a single permission check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// The operation is permitted without extra steps.
    Allow,
    /// The operation is refused under the given privilege state.
    Deny,
    /// The operation may proceed only after explicit user confirmation.
    RequireConfirmation,
}

/// Returns the authorization decision for a permission.
///
/// `is_privileged` distinguishes a privileged caller from a non-privileged
/// one. It is only a boolean input to a pure function here; this crate does
/// not authenticate anyone and must not be mistaken for an identity check.
///
/// The policy is conservative and deterministic:
///
/// | Permission       | Non-privileged       | Privileged           |
/// |------------------|----------------------|----------------------|
/// | `ReadConfig`     | `Allow`              | `Allow`              |
/// | `WriteConfig`    | `Allow`              | `Allow`              |
/// | `ExecuteCommand` | `RequireConfirmation`| `RequireConfirmation`|
/// | `InstallSoftware`| `Deny`               | `RequireConfirmation`|
/// | `ModifySystem`   | `Deny`               | `RequireConfirmation`|
///
/// The function is side-effect-free: same inputs always yield the same output.
pub fn authorize(permission: Permission, is_privileged: bool) -> Decision {
    match permission {
        Permission::ReadConfig | Permission::WriteConfig => Decision::Allow,
        Permission::ExecuteCommand => Decision::RequireConfirmation,
        Permission::InstallSoftware | Permission::ModifySystem => {
            if is_privileged {
                Decision::RequireConfirmation
            } else {
                Decision::Deny
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_config_is_allowed_for_both_privilege_states() {
        assert_eq!(authorize(Permission::ReadConfig, false), Decision::Allow);
        assert_eq!(authorize(Permission::ReadConfig, true), Decision::Allow);
    }

    #[test]
    fn write_config_is_allowed_for_both_privilege_states() {
        assert_eq!(authorize(Permission::WriteConfig, false), Decision::Allow);
        assert_eq!(authorize(Permission::WriteConfig, true), Decision::Allow);
    }

    #[test]
    fn execute_command_requires_confirmation_for_both_privilege_states() {
        assert_eq!(
            authorize(Permission::ExecuteCommand, false),
            Decision::RequireConfirmation
        );
        assert_eq!(
            authorize(Permission::ExecuteCommand, true),
            Decision::RequireConfirmation
        );
    }

    #[test]
    fn install_software_denied_for_non_privileged_and_confirmed_for_privileged() {
        assert_eq!(
            authorize(Permission::InstallSoftware, false),
            Decision::Deny
        );
        assert_eq!(
            authorize(Permission::InstallSoftware, true),
            Decision::RequireConfirmation
        );
    }

    #[test]
    fn modify_system_denied_for_non_privileged_and_confirmed_for_privileged() {
        assert_eq!(authorize(Permission::ModifySystem, false), Decision::Deny);
        assert_eq!(
            authorize(Permission::ModifySystem, true),
            Decision::RequireConfirmation
        );
    }

    #[test]
    fn every_permission_returns_expected_decision() {
        let cases = [
            (Permission::ReadConfig, false, Decision::Allow),
            (Permission::ReadConfig, true, Decision::Allow),
            (Permission::WriteConfig, false, Decision::Allow),
            (Permission::WriteConfig, true, Decision::Allow),
            (
                Permission::ExecuteCommand,
                false,
                Decision::RequireConfirmation,
            ),
            (
                Permission::ExecuteCommand,
                true,
                Decision::RequireConfirmation,
            ),
            (Permission::InstallSoftware, false, Decision::Deny),
            (
                Permission::InstallSoftware,
                true,
                Decision::RequireConfirmation,
            ),
            (Permission::ModifySystem, false, Decision::Deny),
            (
                Permission::ModifySystem,
                true,
                Decision::RequireConfirmation,
            ),
        ];

        for (permission, is_privileged, expected) in cases {
            assert_eq!(
                authorize(permission, is_privileged),
                expected,
                "permission={permission:?} privileged={is_privileged}"
            );
        }
    }
}

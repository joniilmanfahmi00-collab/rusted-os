#![doc = "Configuration management for Rusted OS."]

//! Typed configuration foundation.
//!
//! This crate currently provides a minimal versioned configuration envelope:
//! a [`CURRENT_SCHEMA_VERSION`] constant and an [`AppConfig`] struct with
//! [`AppConfig::validate`].
//!
//! What this crate intentionally does *not* do yet:
//! - No domain settings (desktop, resource policy, AI provider). Those have
//!   no stable schema in the approved architecture, so they are not invented here.
//! - No API credentials. Per `ARCHITECTURE.md`, credentials belong in the OS
//!   credential store, never in plain configuration.
//! - No file I/O and no format parsing (no Serde, TOML, JSON, YAML).
//! - No filesystem paths, no async runtime, no global or singleton state.
//! - No migration logic yet; unsupported versions are rejected explicitly.

use rusted_core::{Result, RustedError};

/// Schema version accepted by this build of `rusted-config`.
///
/// Bump this only together with a documented schema change and a migration
/// path. For now the only valid value is `1`.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// Minimal versioned configuration envelope.
///
/// Future domain sections (once their schemas stabilize) should be added as
/// separate structs or fields with their own validation, not by overloading
/// this envelope. Struct construction plus [`Default`] is the intended way
/// to build values in this early stage; no builder or string loader is provided
/// because no configuration format has been fixed yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppConfig {
    /// Configuration schema version.
    ///
    /// Value `0` is reserved and never valid. Any value other than
    /// [`CURRENT_SCHEMA_VERSION`] is rejected by [`AppConfig::validate`].
    pub schema_version: u32,
}

impl Default for AppConfig {
    /// Returns a valid default: current schema version and nothing else.
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
        }
    }
}

impl AppConfig {
    /// Checks that this configuration is usable by this build.
    ///
    /// - `schema_version == 0` is rejected with [`RustedError::InvalidInput`]
    ///   because zero is reserved and never a valid version.
    /// - Any other value different from [`CURRENT_SCHEMA_VERSION`] is rejected
    ///   with [`RustedError::Unsupported`]; callers must not guess-migrate.
    /// - The current version returns `Ok(())`.
    ///
    /// Error messages contain only the offending version number, never secrets.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version == 0 {
            return Err(RustedError::InvalidInput(
                "schema_version must be >= 1, got 0".to_string(),
            ));
        }

        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(RustedError::Unsupported(format!(
                "unsupported schema version {}, supported: {CURRENT_SCHEMA_VERSION}",
                self.schema_version
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_uses_current_schema_version() {
        let config = AppConfig::default();
        assert_eq!(config.schema_version, CURRENT_SCHEMA_VERSION);
    }

    #[test]
    fn default_configuration_is_valid() {
        AppConfig::default()
            .validate()
            .expect("default config should be valid");
    }

    #[test]
    fn supported_schema_version_is_accepted() {
        let config = AppConfig {
            schema_version: CURRENT_SCHEMA_VERSION,
        };
        config.validate().expect("current version should validate");
    }

    #[test]
    fn zero_schema_version_is_invalid_input() {
        let config = AppConfig { schema_version: 0 };
        let err = config.validate().expect_err("version 0 should be rejected");

        assert_eq!(
            err,
            RustedError::InvalidInput("schema_version must be >= 1, got 0".to_string())
        );
    }

    #[test]
    fn unknown_newer_schema_version_is_unsupported() {
        let config = AppConfig {
            schema_version: CURRENT_SCHEMA_VERSION + 1,
        };
        let err = config
            .validate()
            .expect_err("newer version should be rejected");

        match err {
            RustedError::Unsupported(msg) => {
                assert!(msg.contains(&config.schema_version.to_string()));
                assert!(msg.contains(&CURRENT_SCHEMA_VERSION.to_string()));
            }
            other => panic!("expected Unsupported, got {other:?}"),
        }
    }

    #[test]
    fn far_future_schema_version_is_unsupported() {
        let config = AppConfig {
            schema_version: 999,
        };
        let err = config
            .validate()
            .expect_err("unknown version should be rejected");

        assert!(
            matches!(err, RustedError::Unsupported(_)),
            "expected Unsupported, got {err:?}"
        );
    }
}

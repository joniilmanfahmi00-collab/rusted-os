#![doc = "Foundational types and lifecycle primitives for Rusted OS."]

//! Common vocabulary for early Rusted OS components.
//!
//! This crate intentionally stays small: a shared [`RustedError`] type, a
//! [`Result`] alias, a [`VERSION`] constant, and a minimal [`Service`] trait
//! for in-process startup/shutdown coordination.
//!
//! What this crate does *not* do:
//! - No I/O, threads, async runtime, or OS service management.
//! - No replacement for systemd units, D-Bus activation, or cgroups policy.
//! - No error-source chaining or backtraces yet; add them only with a real use case.

use std::fmt;

/// Version of the `rusted-core` crate, taken from `Cargo.toml`.
///
/// Useful for diagnostics and bug reports. This identifies the crate build,
/// not the version of the whole operating system.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Common error type for Rusted OS foundation crates.
///
/// Each variant carries a short human-readable message. Messages must not
/// contain secrets such as API keys, tokens, or passwords.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustedError {
    /// Caller provided a value that failed validation.
    InvalidInput(String),
    /// A required item (config key, file, registry entry) was not found.
    NotFound(String),
    /// The operation is recognized but not available in this setup.
    Unsupported(String),
    /// An unexpected internal failure.
    Internal(String),
}

impl fmt::Display for RustedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "invalid input: {msg}"),
            Self::NotFound(msg) => write!(f, "not found: {msg}"),
            Self::Unsupported(msg) => write!(f, "unsupported: {msg}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for RustedError {}

/// Convenience alias using [`RustedError`] as the error type.
pub type Result<T> = std::result::Result<T, RustedError>;

/// Minimal coordination interface for components with an explicit
/// start/shutdown sequence.
///
/// # Desain anu dihaja minimal
///
/// Tidak semua komponen Rusted OS wajib mengimplementasikan trait ini.
/// Trait ini hanya untuk komponen in-process yang memang butuh urutan
/// startup/shutdown eksplisit (misalnya dipakai berurutan dalam test atau
/// bootstrap awal). Manajemen service level OS tetap milik systemd,
/// bukan trait ini.
///
/// # Wates pamakéan
///
/// - Sinkron wungkul, tanpa async, tanpa spawn thread.
/// - Tanpa health check, tanpa dependency injection, tanpa retry policy.
/// - `start` dipanggil sekali sebelum dipakai; `shutdown` sebaiknya aman
///   dipanggil lebih dari sekali (idempotent) tapi itu rekomendasi,
///   bukan syarat yang ditegakkan trait ini.
/// - Thread-safety dan urutan antar-service adalah tanggung jawab pemanggil,
///   bukan trait ini.
///
/// # Contoh
///
/// ```rust
/// use rusted_core::Service;
///
/// struct Demo {
///     running: bool,
/// }
///
/// impl Service for Demo {
///     fn name(&self) -> &'static str {
///         "demo"
///     }
///
///     fn start(&mut self) -> rusted_core::Result<()> {
///         self.running = true;
///         Ok(())
///     }
///
///     fn shutdown(&mut self) -> rusted_core::Result<()> {
///         self.running = false;
///         Ok(())
///     }
/// }
/// ```
pub trait Service {
    /// Stable human-readable name, used in logs and diagnostics.
    fn name(&self) -> &'static str;

    /// Start the service before first use.
    fn start(&mut self) -> Result<()>;

    /// Shut the service down. Implementations should make this safe to
    /// call more than once so cleanup paths stay simple.
    fn shutdown(&mut self) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_error_impls_std_error<T: std::error::Error>() {}

    #[test]
    fn version_is_available() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn result_alias_uses_rusted_error() {
        fn always_fails() -> Result<i32> {
            Err(RustedError::Internal("boom".to_string()))
        }

        let err = always_fails().expect_err("expected an error");
        assert_eq!(err, RustedError::Internal("boom".to_string()));
    }

    #[test]
    fn error_display_includes_kind_and_message() {
        let cases = [
            (
                RustedError::InvalidInput("bad port".to_string()),
                "invalid input: bad port",
            ),
            (
                RustedError::NotFound("config key".to_string()),
                "not found: config key",
            ),
            (
                RustedError::Unsupported("wayland debug".to_string()),
                "unsupported: wayland debug",
            ),
            (
                RustedError::Internal("boom".to_string()),
                "internal error: boom",
            ),
        ];

        for (err, expected) in cases {
            assert_eq!(err.to_string(), expected);
        }
    }

    #[test]
    fn error_implements_std_error() {
        assert_error_impls_std_error::<RustedError>();
    }

    struct MockService {
        starts: u32,
        shutdowns: u32,
        running: bool,
    }

    impl MockService {
        fn new() -> Self {
            Self {
                starts: 0,
                shutdowns: 0,
                running: false,
            }
        }
    }

    impl Service for MockService {
        fn name(&self) -> &'static str {
            "mock"
        }

        fn start(&mut self) -> Result<()> {
            self.starts += 1;
            self.running = true;
            Ok(())
        }

        fn shutdown(&mut self) -> Result<()> {
            if self.running {
                self.shutdowns += 1;
                self.running = false;
            }

            Ok(())
        }
    }

    #[test]
    fn service_lifecycle_start_then_shutdown() {
        let mut service = MockService::new();
        assert_eq!(service.name(), "mock");
        assert!(!service.running);

        service.start().expect("start should succeed");
        assert!(service.running);
        assert_eq!(service.starts, 1);

        service.shutdown().expect("shutdown should succeed");
        assert!(!service.running);
        assert_eq!(service.shutdowns, 1);
    }

    #[test]
    fn service_shutdown_is_idempotent_in_mock() {
        let mut service = MockService::new();
        service.start().expect("start should succeed");

        service.shutdown().expect("first shutdown should succeed");
        service.shutdown().expect("second shutdown should succeed");
        assert!(!service.running);
        assert_eq!(service.shutdowns, 2);
    }
}

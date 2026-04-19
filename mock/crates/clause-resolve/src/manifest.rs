//! Manifest parser stub.
//!
//! `Manifest` models a `Clause.toml`: crate name, version, and a
//! flat list of dependency names. Skeleton round: no real TOML
//! parsing. `parse_manifest(b"")` returns a default `Manifest`;
//! anything non-empty returns `ManifestError::NotImplemented`.
//!
//! Real TOML parsing via `toml_edit`, dependency graph
//! construction, feature flags, and workspace manifests are all
//! BACKLOG.

use crate::error::ManifestError;

/// Parsed `Clause.toml` contents.
///
/// Carries the crate name, version string, and a flat list of
/// dependency crate names. Real parsing (reading
/// `[dependencies.<name>]` tables with version / path / git
/// specifiers, features, workspace inheritance) is BACKLOG; the
/// skeleton only produces the default shape.
#[derive(Clone, Debug, Default)]
pub struct Manifest {
    crate_name: String,
    version: String,
    dependencies: Vec<String>,
}

impl Manifest {
    /// Crate name from `[package]`.
    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    /// Crate version from `[package]`.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Flat list of dependency crate names.
    pub fn dependencies(&self) -> &[String] {
        &self.dependencies
    }
}

/// Parse manifest bytes into a `Manifest`.
///
/// Skeleton rules:
///
/// - Empty input → `Ok(Manifest::default())`.
/// - Non-empty input → `Err(ManifestError::NotImplemented)`.
///
/// Real TOML parsing lands in a follow-up round.
pub fn parse_manifest(bytes: &[u8]) -> Result<Manifest, ManifestError> {
    if bytes.is_empty() {
        Ok(Manifest::default())
    } else {
        Err(ManifestError::NotImplemented)
    }
}

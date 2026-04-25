//! Manifest parser stub.
//!
//! `Manifest` models a `Vehje.toml`: crate name, version, and a
//! flat list of dependency names. Skeleton round: no real TOML
//! parsing. `parse_manifest(b"")` returns a default `Manifest`;
//! anything non-empty returns `ManifestError::NotImplemented`.
//!
//! Real TOML parsing via `toml_edit`, dependency graph
//! construction, feature flags, and workspace manifests are all
//! BACKLOG.

use hilavitkutin_str::Str;
use notko::Outcome;

use crate::error::ManifestError;

/// Parsed `Vehje.toml` contents.
///
/// Carries the crate name, version handle, and a flat list of
/// dependency crate names. Real parsing (reading
/// `[dependencies.<name>]` tables with version / path / git
/// specifiers, features, workspace inheritance) is BACKLOG; the
/// skeleton only produces the default shape.
#[derive(Clone, Debug, Default)]
pub struct Manifest {
    crate_name: Str,
    version: Str,
    dependencies: Vec<Str>, // lint:allow(bare_collection) reason: skeleton manifest deps list; re-expressed via #166 (manifest parser) + #131 (scheduler) + #134 (persistence) (see SHAME.md `## Manifest`); tracked: #131
}

impl Manifest {
    /// Crate name from `[package]`.
    pub fn crate_name(&self) -> Str {
        self.crate_name
    }

    /// Crate version from `[package]`.
    pub fn version(&self) -> Str {
        self.version
    }

    /// Flat list of dependency crate names.
    pub fn dependencies(&self) -> &[Str] {
        &self.dependencies
    }
}

/// Parse manifest bytes into a `Manifest`.
///
/// Skeleton rules:
///
/// - Empty input → `Outcome::Ok(Manifest::default())`.
/// - Non-empty input → `Outcome::Err(ManifestError::NotImplemented)`.
///
/// Real TOML parsing lands in a follow-up round.
pub fn parse_manifest(bytes: &[u8]) -> Outcome<Manifest, ManifestError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: file-parser byte-slice input, `&[u8]` is the canonical shape crossing the filesystem boundary; TOML bytes are not numeric data; tracked: #166

    if bytes.is_empty() {
        Outcome::Ok(Manifest::default())
    } else {
        Outcome::Err(ManifestError::NotImplemented)
    }
}

//! Manifest stub: empty input → default, non-empty → NotImplemented.

use clause_resolve::{ManifestError, parse_manifest};

#[test]
fn parse_empty_returns_default() {
    let manifest = parse_manifest(b"").expect("empty manifest parses");
    assert_eq!(manifest.crate_name(), "");
    assert_eq!(manifest.version(), "");
    assert_eq!(manifest.dependencies().len(), 0);
}

#[test]
fn parse_nonempty_returns_not_implemented() {
    let err = parse_manifest(b"not-toml").expect_err("non-empty stub errors");
    assert_eq!(err, ManifestError::NotImplemented);
}

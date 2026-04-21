//! Manifest stub: empty input → default, non-empty → NotImplemented.

use clause_resolve::{ManifestError, parse_manifest};
use hilavitkutin_str::Str;
use notko::Outcome;

#[test]
fn parse_empty_returns_default() {
    let manifest = match parse_manifest(b"") {
        Outcome::Ok(m) => m,
        Outcome::Err(_) => panic!("empty manifest parses"),
    };
    assert_eq!(manifest.crate_name(), Str::default());
    assert_eq!(manifest.version(), Str::default());
    assert_eq!(manifest.dependencies().len(), 0);
}

#[test]
fn parse_nonempty_returns_not_implemented() {
    let err = match parse_manifest(b"not-toml") {
        Outcome::Ok(_) => panic!("non-empty stub must error"),
        Outcome::Err(e) => e,
    };
    assert_eq!(err, ManifestError::NotImplemented);
}

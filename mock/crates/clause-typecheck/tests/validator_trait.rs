//! Per-STRICT validator tests.
//!
//! Asserts each of `Strict1..=Strict7` exposes the documented
//! kebab-case name and returns an empty diagnostic vec from its
//! skeleton `validate`.

use clause_resolve::Resolved;
use clause_typecheck::{
    Strict1, Strict2, Strict3, Strict4, Strict5, Strict6, Strict7, Validator, ValidatorCtx,
};

#[test]
fn strict1_name_matches() {
    assert_eq!(Strict1.name(), "no-shadow");
}

#[test]
fn strict1_validate_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    assert!(Strict1.validate(&ctx).is_empty());
}

#[test]
fn strict_names_cover_all_seven() {
    let names: [&'static str; 7] = [
        Strict1.name(),
        Strict2.name(),
        Strict3.name(),
        Strict4.name(),
        Strict5.name(),
        Strict6.name(),
        Strict7.name(),
    ];
    assert!(names.contains(&"no-shadow"));
    assert!(names.contains(&"unused"));
    assert!(names.contains(&"assign-to-immut"));
    assert!(names.contains(&"int-overflow"));
    assert!(names.contains(&"use-after-move"));
    assert!(names.contains(&"ambiguous-imports"));
    assert!(names.contains(&"clippy-batch"));
}

#[test]
fn strict_validate_all_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    assert!(Strict1.validate(&ctx).is_empty());
    assert!(Strict2.validate(&ctx).is_empty());
    assert!(Strict3.validate(&ctx).is_empty());
    assert!(Strict4.validate(&ctx).is_empty());
    assert!(Strict5.validate(&ctx).is_empty());
    assert!(Strict6.validate(&ctx).is_empty());
    assert!(Strict7.validate(&ctx).is_empty());
}

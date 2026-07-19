//! Per-STRICT validator tests.
//!
//! Asserts each of `Strict1..=Strict7` exposes the documented
//! kebab-case name and pushes nothing into a `CountingSink` from its
//! skeleton `validate`.

use arvo::USize;
use hilavitkutin_api::Len;
use hilavitkutin_api::sink::CountingSink;
use vehje_ir::Diagnostic;
use vehje_resolve::Resolved;
use vehje_typecheck::{
    Strict1,
    Strict2,
    Strict3,
    Strict4,
    Strict5,
    Strict6,
    Strict7,
    Validator,
    ValidatorCtx,
};

#[test]
fn strict1_name_matches() {
    assert_eq!(Strict1.name(), "no-shadow");
}

#[test]
fn strict1_validate_empty() {
    let resolved = Resolved::empty();
    let ctx = ValidatorCtx::new(&resolved);
    let mut sink = CountingSink::<Diagnostic>::new();
    Strict1.validate(&ctx, &mut sink);
    assert_eq!(sink.len(), USize(0));
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
    let mut sink = CountingSink::<Diagnostic>::new();
    Strict1.validate(&ctx, &mut sink);
    Strict2.validate(&ctx, &mut sink);
    Strict3.validate(&ctx, &mut sink);
    Strict4.validate(&ctx, &mut sink);
    Strict5.validate(&ctx, &mut sink);
    Strict6.validate(&ctx, &mut sink);
    Strict7.validate(&ctx, &mut sink);
    assert_eq!(sink.len(), USize(0));
}

//! Artifact / error payload tests.
//!
//! Cover the `ArtifactKind` variant discrimination, the
//! `CodegenArtifact::empty` constructor, and smoke-test each
//! `CodegenError` variant's construction + pattern match.

use vehje_codegen::{ArtifactKind, CodegenArtifact, CodegenError};

#[test]
fn artifact_kind_variants_distinct() {
    let kinds = [
        ArtifactKind::Binary,
        ArtifactKind::Object,
        ArtifactKind::Assembly,
        ArtifactKind::SourceFile,
        ArtifactKind::Config,
    ];
    for (i, a) in kinds.iter().enumerate() {
        for (j, b) in kinds.iter().enumerate() {
            if i == j {
                assert_eq!(a, b);
            } else {
                assert_ne!(a, b);
            }
        }
    }
}

#[test]
fn artifact_construction() {
    let artifact = CodegenArtifact::empty(ArtifactKind::Binary);
    assert_eq!(artifact.kind, ArtifactKind::Binary);
    assert!(artifact.bytes.is_empty());
    assert!(artifact.diagnostics.is_empty());
}

#[test]
fn codegen_error_variants_construct() {
    let errs = [
        CodegenError::TargetNotFound { name: "x" },
        CodegenError::UnsupportedFeature { feature: "y" },
        CodegenError::LoweringFailed { message: "z" },
        CodegenError::NotImplemented,
    ];
    for err in errs {
        match err {
            CodegenError::TargetNotFound { name } => assert_eq!(name, "x"),
            CodegenError::UnsupportedFeature { feature } => assert_eq!(feature, "y"),
            CodegenError::LoweringFailed { message } => assert_eq!(message, "z"),
            CodegenError::NotImplemented => {}
        }
    }
}

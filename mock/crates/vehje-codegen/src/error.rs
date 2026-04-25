//! Codegen error variants.
//!
//! `CodegenError` is a flat `Copy` enum carrying the four error
//! shapes the skeleton surface needs: an unknown target name, a
//! target that received an unsupported feature request, a
//! lowering failure, and a generic not-yet-implemented stub
//! marker.
//!
//! Each variant carries a `&'static str` where a string is
//! needed. The skeleton does not allocate per-error; a future
//! round (once real target backends surface the need) retrofits
//! a richer carrier with owned strings, spans, and per-target
//! error codes.

/// Codegen error surface.
///
/// `Copy` so callers can pattern-match + propagate without
/// worrying about move semantics. All carried strings are
/// `'static`; the skeleton does not need owned strings, and the
/// retrofit is BACKLOG.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CodegenError {
    /// The registry has no target registered under the given
    /// name. `name` carries a `'static` sentinel (`""` on miss
    /// paths; the static target name when a registered target
    /// rejects itself). Callers own the input-side string.
    TargetNotFound {
        /// Target name that was looked up. `""` when returned
        /// from a registry-miss path; caller retains the input
        /// context and re-reports if needed.
        name: &'static str,
    },
    /// The target does not support the requested feature.
    UnsupportedFeature {
        /// Feature name the target rejected. `'static`; the
        /// feature registry owns the canonical list.
        feature: &'static str,
    },
    /// The lowering pipeline failed on a unit the target
    /// otherwise accepts.
    LoweringFailed {
        /// Short human-facing message. Real backends will
        /// retrofit this to a richer carrier once diagnostic
        /// volume justifies it.
        message: &'static str,
    },
    /// The target's backend has not been implemented yet.
    /// Reserved for future partial-implementation returns; the
    /// current skeleton does not emit this variant.
    NotImplemented,
}

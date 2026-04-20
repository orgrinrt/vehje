//! AST node discriminators.
//!
//! `AstNodeKind` is the shared vocabulary between the parser
//! (producer) and every downstream phase (resolve, typecheck,
//! codegen). The initial shape ships with the coarse categories
//! the R1 design locked; per-phase refinement (distinct variants
//! per expression form, per pattern form, etc.) arrives in later
//! design rounds without breaking the enum's discriminant layout.

/// Discriminator for every AST node kind.
///
/// This round carries only the top-level categories. Expression /
/// pattern / type refinement arrives as later rounds extend the enum
/// by adding variants at the tail; existing discriminants are stable.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum AstNodeKind {
    File,
    Item,
    Fn,
    Let,
    Expr,
    Pat,
    Type,
    Block,
    Unknown,
}

impl Default for AstNodeKind {
    fn default() -> Self {
        Self::Unknown
    }
}

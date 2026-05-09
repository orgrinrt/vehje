//! Resolved program bundle.
//!
//! `Resolved` carries the original AST plus the resolver's output:
//! a `ScopeTree` and a per-`NodeId` resolution map. The map is a
//! flat `Vec<Maybe<Symbol>>` indexed by the raw `NodeId.0` value.
//! `Maybe::Isnt` entries mean "not a name reference" or "not yet
//! resolved"; follow-up rounds populate them as each resolution
//! rule lands.
//!
//! Skeleton round: every entry is `Maybe::Isnt`; the resolver does
//! not walk the AST.

use arvo::Bool;
use vehje_syntax::Ast;
use notko::Maybe;

use crate::scope::ScopeTree;
use crate::symbol::Symbol;

/// Resolver output bundle.
///
/// Holds:
///
/// - `ast`, a copy of the input AST (so downstream phases can key
///   off a single value; the AST is `Copy` via fixed-size arenas).
/// - `scopes`, the `ScopeTree` the resolver constructed.
/// - `resolution`, a flat map from `NodeId.0` to `Maybe<Symbol>`.
#[derive(Clone, Debug, Default)]
pub struct Resolved {
    ast: Ast,
    scopes: ScopeTree,
    resolution: Vec<Maybe<Symbol>>, // lint:allow(bare_collection) reason: skeleton resolution-map storage; re-expressed as scheduler-managed Column<Maybe<Symbol>> + persistence sidecar once #131 / #134 land (see SHAME.md `## Resolved`); tracked: #131
}

impl Resolved {
    /// Construct an empty `Resolved`: empty AST, default
    /// `ScopeTree`, empty resolution vec.
    pub fn empty() -> Self {
        Self {
            ast: Ast::default(),
            scopes: ScopeTree::default(),
            resolution: Vec::new(), // lint:allow(bare_collection) reason: skeleton empty-init; tracked: #131
        }
    }

    /// Construct a `Resolved` from its parts.
    pub fn new(ast: Ast, scopes: ScopeTree, resolution: Vec<Maybe<Symbol>>) -> Self { // lint:allow(bare_collection) reason: skeleton resolver hand-off surface; re-expressed once #131 lands; tracked: #131
        Self { ast, scopes, resolution }
    }

    /// Borrow the AST.
    pub fn ast(&self) -> &Ast {
        &self.ast
    }

    /// Borrow the scope tree.
    pub fn scopes(&self) -> &ScopeTree {
        &self.scopes
    }

    /// Borrow the resolution map.
    pub fn resolution(&self) -> &[Maybe<Symbol>] {
        &self.resolution
    }

    /// `true` if the bundle carries no resolved references, has
    /// an empty AST, and holds only the default (root-only)
    /// `ScopeTree`.
    pub fn is_empty(&self) -> Bool {
        Bool(
            self.ast.is_empty()
                && self.scopes.is_trivial().0
                && self.resolution.iter().all(|r| r.isnt()),
        )
    }
}

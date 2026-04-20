//! Resolver driver.
//!
//! `Resolver` owns the `ScopeTree` under construction plus a
//! cursor into it. Each follow-up round extends `resolve` with
//! the next production's walk.
//!
//! Skeleton round: `resolve` does NOT walk the AST. It returns a
//! `Resolved` whose `resolution` vec is `None`-filled to
//! `ast.len()`, preserving the downstream shape ("one resolution
//! slot per AST node") that typecheck and codegen will key off.

use clause_ir::ScopeId;
use clause_syntax::Ast;

use crate::error::ResolveError;
use crate::resolved::Resolved;
use crate::scope::ScopeTree;

/// Resolver driver state.
///
/// Carries the `ScopeTree` under construction and a cursor at the
/// current scope. `new()` starts at the root scope.
#[derive(Clone, Debug)]
pub struct Resolver {
    scopes: ScopeTree,
    current: ScopeId,
}

impl Default for Resolver {
    fn default() -> Self {
        Self::new()
    }
}

impl Resolver {
    /// Build a resolver with a fresh `ScopeTree` (root-only) and
    /// the cursor at the root.
    pub fn new() -> Self {
        let scopes = ScopeTree::new();
        let current = scopes.root();
        Self { scopes, current }
    }

    /// Borrow the scope tree under construction.
    pub fn scopes(&self) -> &ScopeTree {
        &self.scopes
    }

    /// Current (cursor) scope.
    pub fn current_scope(&self) -> ScopeId {
        self.current
    }

    /// Drive resolution over `ast`.
    ///
    /// Skeleton behaviour: no AST walk. Returns a `Resolved`
    /// bundle whose `resolution` vec is `None`-filled to
    /// `ast.len()`. Each deferred resolution rule flips a subset
    /// of those slots in its own follow-up round.
    pub fn resolve(self, ast: &Ast) -> Result<Resolved, ResolveError> {
        let resolution = vec![None; ast.len()];
        Ok(Resolved::new(*ast, self.scopes, resolution))
    }
}

//! Lexical scope tree.
//!
//! `Scope` is a single lexical scope: an optional parent and a
//! name → `Symbol` map. `ScopeTree` is the flat arena of all
//! scopes, keyed by `ScopeId` (from `clause-ir`). The root scope
//! lives at index `0` and has no parent.
//!
//! Skeleton round: the map is a `HashMap<String, Symbol>` — owned
//! strings, standard hashing. Interning + lookup-by-interned-id is
//! BACKLOG.

use std::collections::HashMap;

use clause_ir::ScopeId;
use notko::Maybe;

use crate::symbol::Symbol;

/// A single lexical scope.
///
/// `parent` is `Maybe::Isnt` for the root scope, `Maybe::Is(id)`
/// otherwise. `symbols` is a name → `Symbol` table for bindings
/// introduced directly in this scope (shadowing replaces the
/// previous entry; see `insert`).
#[derive(Clone, Debug, Default)]
pub struct Scope {
    parent: Maybe<ScopeId>,
    symbols: HashMap<String, Symbol>,
}

impl Scope {
    /// Construct a scope with the given parent.
    pub fn new(parent: Maybe<ScopeId>) -> Self {
        Self { parent, symbols: HashMap::new() }
    }

    /// Parent scope, or `Maybe::Isnt` for the root.
    pub fn parent(&self) -> Maybe<ScopeId> {
        self.parent
    }

    /// Insert a symbol under `name`. If a symbol already existed
    /// under that name in this scope, the previous occupant is
    /// returned.
    pub fn insert(&mut self, name: String, symbol: Symbol) -> Maybe<Symbol> {
        match self.symbols.insert(name, symbol) {
            Some(prev) => Maybe::Is(prev),
            None => Maybe::Isnt,
        }
    }

    /// Look up a symbol by name in this scope only (no parent
    /// walk). Parent walks are a follow-up round concern.
    pub fn lookup(&self, name: &str) -> Maybe<&Symbol> {
        match self.symbols.get(name) {
            Some(sym) => Maybe::Is(sym),
            None => Maybe::Isnt,
        }
    }
}

/// Flat arena of all scopes.
///
/// Indexed by `ScopeId`; the root scope lives at `ScopeId(0)` and
/// is created by `new()` / `default()`. `push` allocates a new
/// scope with the given parent and returns its id.
#[derive(Clone, Debug)]
pub struct ScopeTree {
    scopes: Vec<Scope>,
}

impl Default for ScopeTree {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeTree {
    /// Construct a tree containing only the root scope.
    pub fn new() -> Self {
        Self { scopes: vec![Scope::new(Maybe::Isnt)] }
    }

    /// Root scope id. Always `ScopeId(0)`.
    pub fn root(&self) -> ScopeId {
        ScopeId(0)
    }

    /// Number of scopes currently in the tree (at least 1).
    pub fn len(&self) -> usize {
        self.scopes.len()
    }

    /// `true` if the tree has only the root scope.
    pub fn is_trivial(&self) -> bool {
        self.scopes.len() <= 1
    }

    /// Borrow a scope by id, or `Maybe::Isnt` if the id is out of
    /// range.
    pub fn get(&self, id: ScopeId) -> Maybe<&Scope> {
        match self.scopes.get(id.0 as usize) {
            Some(s) => Maybe::Is(s),
            None => Maybe::Isnt,
        }
    }

    /// Mutably borrow a scope by id, or `Maybe::Isnt` if out of
    /// range.
    pub fn get_mut(&mut self, id: ScopeId) -> Maybe<&mut Scope> {
        match self.scopes.get_mut(id.0 as usize) {
            Some(s) => Maybe::Is(s),
            None => Maybe::Isnt,
        }
    }

    /// Allocate a new scope with the given parent, return its id.
    pub fn push(&mut self, parent: Maybe<ScopeId>) -> ScopeId {
        let id = ScopeId(self.scopes.len() as u32);
        self.scopes.push(Scope::new(parent));
        id
    }
}

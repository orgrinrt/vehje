//! Symbol surface.
//!
//! A `Symbol` is the resolved target of a name reference: the
//! producing declaration's node id, the lexical scope that owns
//! it, a classification tag (`SymbolKind`), and the name as text.
//!
//! Skeleton round: names are owned `String`. Interning is BACKLOG
//! (string interning will also let `Symbol` become `Copy`, which
//! the current owned-`String` design deliberately forgoes to keep
//! the surface obvious).

use clause_ir::{NodeId, ScopeId};

/// Classification of a `Symbol`.
///
/// The variants mirror the surface-language declaration forms:
/// type aliases (`Type`), value bindings / locals (`Value`), module
/// declarations (`Module`), `use` imports (`Import`), function
/// declarations (`Function`), and `const` declarations (`Const`).
///
/// `Default` is `Value`: that is the variant produced by an
/// un-annotated binding, which is the most common case.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u8)]
pub enum SymbolKind {
    /// Type alias or type declaration.
    Type,
    /// Value binding (default).
    Value,
    /// Module declaration.
    Module,
    /// Imported name via `use`.
    Import,
    /// Function declaration.
    Function,
    /// Constant declaration.
    Const,
}

impl Default for SymbolKind {
    fn default() -> Self {
        Self::Value
    }
}

/// A resolved declaration bound to a name.
///
/// Carries the name text, the owning scope, the producing AST
/// node, and the `SymbolKind` classification. The skeleton keeps
/// `name` as an owned `String`; interning is BACKLOG.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Symbol {
    name: String,
    scope: ScopeId,
    node: NodeId,
    kind: SymbolKind,
}

impl Symbol {
    /// Construct a `Symbol` from its parts.
    pub fn new(name: impl Into<String>, scope: ScopeId, node: NodeId, kind: SymbolKind) -> Self {
        Self { name: name.into(), scope, node, kind }
    }

    /// Name text of this symbol.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Scope that owns this symbol.
    pub fn scope(&self) -> ScopeId {
        self.scope
    }

    /// AST node that declared this symbol.
    pub fn node(&self) -> NodeId {
        self.node
    }

    /// Classification tag.
    pub fn kind(&self) -> SymbolKind {
        self.kind
    }
}

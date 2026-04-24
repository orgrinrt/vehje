//! Symbol surface.
//!
//! A `Symbol` is the resolved target of a name reference: the
//! producing declaration's node id, the lexical scope that owns
//! it, a classification tag (`SymbolKind`), and the name as an
//! interned `Str` handle.
//!
//! Names are `hilavitkutin_str::Str` — 4-byte Copy handles keyed
//! by interned identity. This makes `Symbol` itself `Copy`, which
//! in turn makes `ResolveError` `Copy` (the error variants carry
//! a copied name).

use vehje_ir::{NodeId, ScopeId};
use hilavitkutin_str::Str;

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
#[repr(u8)] // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C-ABI enum discriminator size; six variants pack to 3 bits in arvo terms but `#[repr]` attribute accepts only bare integer widths; tracked: #81
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
/// Carries the name handle, the owning scope, the producing AST
/// node, and the `SymbolKind` classification. `Symbol` is `Copy`
/// because `Str` is `Copy`.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Symbol {
    name: Str,
    scope: ScopeId,
    node: NodeId,
    kind: SymbolKind,
}

impl Symbol {
    /// Construct a `Symbol` from its parts.
    pub fn new(name: Str, scope: ScopeId, node: NodeId, kind: SymbolKind) -> Self {
        Self { name, scope, node, kind }
    }

    /// Name handle of this symbol.
    pub fn name(&self) -> Str {
        self.name
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

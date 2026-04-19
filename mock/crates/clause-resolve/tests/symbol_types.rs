//! Symbol + SymbolKind: variant distinctness, field round-trip,
//! default.

use clause_ir::{NodeId, ScopeId};
use clause_resolve::{Symbol, SymbolKind};

#[test]
fn symbol_kind_variants_distinct() {
    let kinds = [
        SymbolKind::Type,
        SymbolKind::Value,
        SymbolKind::Module,
        SymbolKind::Import,
        SymbolKind::Function,
        SymbolKind::Const,
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
fn symbol_holds_fields() {
    let sym = Symbol::new("x", ScopeId(3), NodeId(7), SymbolKind::Value);
    assert_eq!(sym.name(), "x");
    assert_eq!(sym.scope(), ScopeId(3));
    assert_eq!(sym.node(), NodeId(7));
    assert_eq!(sym.kind(), SymbolKind::Value);
}

#[test]
fn symbol_kind_default_is_value() {
    assert_eq!(SymbolKind::default(), SymbolKind::Value);
}

//! Symbol + SymbolKind: variant distinctness, field round-trip,
//! default.

use vehje_ir::{NodeId, ScopeId};
use vehje_resolve::{Symbol, SymbolKind};
use hilavitkutin_str::str_const;

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
    let sym = Symbol::new(str_const!("x"), ScopeId(3), NodeId(7), SymbolKind::Value);
    assert_eq!(sym.name(), str_const!("x"));
    assert_eq!(sym.scope(), ScopeId(3));
    assert_eq!(sym.node(), NodeId(7));
    assert_eq!(sym.kind(), SymbolKind::Value);
}

#[test]
fn symbol_kind_default_is_value() {
    assert_eq!(SymbolKind::default(), SymbolKind::Value);
}

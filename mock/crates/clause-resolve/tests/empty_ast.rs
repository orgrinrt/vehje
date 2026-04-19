//! resolve() over an empty AST returns an empty Resolved.

use clause_resolve::{Ast, resolve};

#[test]
fn resolve_default_ast_ok() {
    let ast = Ast::default();
    let resolved = resolve(&ast).expect("default ast resolves");
    assert_eq!(resolved.resolution().len(), 0);
}

#[test]
fn resolved_is_empty_on_empty_ast() {
    let ast = Ast::default();
    let resolved = resolve(&ast).expect("default ast resolves");
    assert!(resolved.is_empty());
}

//! ScopeTree: root construction, push, parent chain, bounds.

use arvo::USize;
use vehje_ir::ScopeId;
use vehje_resolve::ScopeTree;
use notko::Maybe;

#[test]
fn root_scope_has_no_parent() {
    let tree = ScopeTree::new();
    let root = tree.root();
    let scope = tree.get(root).unwrap();
    assert_eq!(scope.parent(), Maybe::Isnt);
}

#[test]
fn push_returns_monotonic_ids() {
    let mut tree = ScopeTree::new();
    let a = tree.push(Maybe::Is(tree.root()));
    let b = tree.push(Maybe::Is(tree.root()));
    let c = tree.push(Maybe::Is(tree.root()));
    assert_eq!(a, ScopeId(1));
    assert_eq!(b, ScopeId(2));
    assert_eq!(c, ScopeId(3));
    assert_eq!(tree.len(), USize(4));
}

#[test]
fn nested_scope_parent_chain() {
    let mut tree = ScopeTree::new();
    let a = tree.push(Maybe::Is(tree.root()));
    let b = tree.push(Maybe::Is(a));
    let scope_b = tree.get(b).unwrap();
    assert_eq!(scope_b.parent(), Maybe::Is(a));
}

#[test]
fn get_out_of_range_returns_isnt() {
    let tree = ScopeTree::new();
    assert!(tree.get(ScopeId(99)).isnt());
}

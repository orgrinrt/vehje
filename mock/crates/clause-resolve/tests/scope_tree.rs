//! ScopeTree: root construction, push, parent chain, bounds.

use clause_ir::ScopeId;
use clause_resolve::ScopeTree;

#[test]
fn root_scope_has_no_parent() {
    let tree = ScopeTree::new();
    let root = tree.root();
    let scope = tree.get(root).expect("root scope present");
    assert_eq!(scope.parent(), None);
}

#[test]
fn push_returns_monotonic_ids() {
    let mut tree = ScopeTree::new();
    let a = tree.push(Some(tree.root()));
    let b = tree.push(Some(tree.root()));
    let c = tree.push(Some(tree.root()));
    assert_eq!(a, ScopeId(1));
    assert_eq!(b, ScopeId(2));
    assert_eq!(c, ScopeId(3));
    assert_eq!(tree.len(), 4);
}

#[test]
fn nested_scope_parent_chain() {
    let mut tree = ScopeTree::new();
    let a = tree.push(Some(tree.root()));
    let b = tree.push(Some(a));
    let scope_b = tree.get(b).expect("scope B present");
    assert_eq!(scope_b.parent(), Some(a));
}

#[test]
fn get_out_of_range_returns_none() {
    let tree = ScopeTree::new();
    assert!(tree.get(ScopeId(99)).is_none());
}

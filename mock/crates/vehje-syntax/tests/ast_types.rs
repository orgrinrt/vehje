//! AST type surface: arena push/get, NodeId monotonicity, leaf
//! construction.

use notko::Maybe;
use vehje_ir::{AstNodeKind, ByteOffset, FileId, NodeId, Span};
use vehje_syntax::{Ast, AstNode};

fn span(start: u32, end: u32) -> Span {
    Span::new(FileId(0), ByteOffset(start), ByteOffset(end))
}

#[test]
fn ast_empty_is_empty() {
    let ast = Ast::empty();
    assert_eq!(ast.len(), 0);
    assert!(ast.is_empty());
    assert_eq!(ast.root(), Maybe::Isnt);
}

#[test]
fn ast_push_returns_monotonic_ids() {
    let mut ast = Ast::empty();
    let a = ast.push(AstNode::leaf(AstNodeKind::Expr, span(0, 1))).unwrap();
    let b = ast.push(AstNode::leaf(AstNodeKind::Expr, span(2, 3))).unwrap();
    let c = ast.push(AstNode::leaf(AstNodeKind::Expr, span(4, 5))).unwrap();
    assert_eq!(a, NodeId(0));
    assert_eq!(b, NodeId(1));
    assert_eq!(c, NodeId(2));
    assert_eq!(ast.len(), 3);
}

#[test]
fn ast_get_round_trip() {
    let mut ast = Ast::empty();
    let s = span(7, 11);
    let id = ast.push(AstNode::leaf(AstNodeKind::Expr, s)).unwrap();
    let fetched = ast.get(id).expect("node present");
    assert_eq!(fetched.kind, AstNodeKind::Expr);
    assert_eq!(fetched.span, s);
}

#[test]
fn astnode_leaf_has_no_children() {
    let node = AstNode::leaf(AstNodeKind::Expr, span(0, 1));
    assert_eq!(node.child_count(), 0);
    assert!(node.children().is_empty());
}

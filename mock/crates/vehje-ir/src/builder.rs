//! The node builder: one smart-constructor per Core form.
//!
//! `Builder` wraps an `Arena` and appends nodes, one method per Core
//! form, each returning the new node's handle (`Isnt` if the arena is
//! full). Content composition is monoidal: a sequence of values
//! concatenates through the child-index pool rather than a `Seq` node, so
//! there is no `Seq` Core form.

use arvo::{Bool, Maybe};

use hilavitkutin_str::Str;

use crate::arena::Arena;
use crate::node::{FamilyId, Literal, Node, NodeList, NodeRef};
use crate::span::Span;

/// Builds Core nodes into a caller-provided arena.
pub struct Builder<'a> {
    arena: Arena<'a>,
}

impl<'a> Builder<'a> {
    /// Wrap an arena.
    pub fn new(arena: Arena<'a>) -> Self {
        Self { arena }
    }

    /// The underlying arena (to read back built nodes).
    pub fn arena(&self) -> &Arena<'a> {
        &self.arena
    }

    /// Consume the builder, returning the arena.
    pub fn into_arena(self) -> Arena<'a> {
        self.arena
    }

    /// A literal value.
    pub fn lit(&mut self, value: Literal, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Lit(value), span)
    }

    /// A reference to a binding by name.
    pub fn var(&mut self, name: Str, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Var(name), span)
    }

    /// Bind `name` to `value` in scope for `body`.
    pub fn let_(&mut self, rec: Bool, name: Str, value: NodeRef, body: NodeRef, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Let { rec, name, value, body }, span)
    }

    /// Abstraction over `param`.
    pub fn lambda(&mut self, param: Str, body: NodeRef, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Lambda { param, body }, span)
    }

    /// Apply `callee` to `args`.
    pub fn apply(&mut self, callee: NodeRef, args: &[NodeRef], span: Span) -> Maybe<NodeRef> {
        let args = self.arena.alloc_list(args)?;
        self.arena.push(Node::Apply { callee, args }, span)
    }

    /// Field, member, or index access.
    pub fn project(&mut self, base: NodeRef, key: Str, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Project { base, key }, span)
    }

    /// Conditional.
    pub fn if_(&mut self, cond: NodeRef, then_branch: NodeRef, else_branch: NodeRef, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::If { cond, then_branch, else_branch }, span)
    }

    /// Pattern match over `scrutinee` with the given arm bodies.
    pub fn match_(&mut self, scrutinee: NodeRef, arms: &[NodeRef], span: Span) -> Maybe<NodeRef> {
        let arms = self.arena.alloc_list(arms)?;
        self.arena.push(Node::Match { scrutinee, arms }, span)
    }

    /// Iterate `seq`, accumulating `body` monoidally.
    pub fn iter(&mut self, seq: NodeRef, body: NodeRef, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Iter { seq, body }, span)
    }

    /// Interpolate a value into text or content.
    pub fn interp(&mut self, value: NodeRef, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Interp { value }, span)
    }

    /// A family's own construct, attached through the escape hatch.
    pub fn raw(&mut self, family: FamilyId, payload: NodeList, span: Span) -> Maybe<NodeRef> {
        self.arena.push(Node::Raw { family, payload }, span)
    }
}

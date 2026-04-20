//! Dense identifiers for AST nodes and lexical scopes.
//!
//! Both are `#[repr(transparent)]` newtypes around `u32`. They are
//! minted by the producing phase (parse for `NodeId`, resolve for
//! `ScopeId`) and consumed as opaque handles by downstream phases.

/// Dense interned identifier for AST nodes.
///
/// Assigned at parse time; stable across later phases so resolution
/// info and type info can key off the same id without re-threading
/// pointers.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct NodeId(pub u32);

/// Dense interned identifier for lexical scopes.
///
/// Assigned by the resolver when it walks the AST; used by typecheck
/// and later phases to look up binding resolution.
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ScopeId(pub u32);

//! The node-creating ANF catamorphism.
//!
//! A-normal form names every non-atomic intermediate in a fresh `Let`, so
//! effect order is explicit in the residual. `Anf::apply` reads the folded and
//! shared input through the `Rewrite` view and appends fresh nodes, flattening
//! a term to flat A-normal form with no closures and no alloc. Fresh binders are
//! minted by `Generator<BinderDomain>` (kind `0b001`), disjoint from source
//! names (`0b000`), so a hoist can never capture.

use arvo::{Bool, Identity, Maybe, USize};
use hilavitkutin_sym::{Generator, Sym};
use vehje_ir::{BinderDomain, Builder, Node, NodeRef, Span};

use crate::Rewrite;

/// A-normal form: name every intermediate so effect order is explicit in the
/// residual and no separate sequencing form is needed.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Anf;

impl Anf {
    /// Normalise the subtree rooted at `root` to flat A-normal form, appending
    /// the normalised nodes to `builder`'s arena and returning the new root.
    ///
    /// Reads the folded and shared input through `rw`; mints fresh capture-safe
    /// binders through `binders` (a `BinderDomain` `Sym`, kind `0b001`, never equal
    /// to a source name). Returns `Isnt` on arena-full, generator-exhaustion, or
    /// operand-scratch overflow, the no-alloc exhaustion contract.
    pub fn apply(
        &self,
        builder: &mut Builder<'_>,
        root: NodeRef,
        rw: &Rewrite<'_>,
        binders: &mut Generator<BinderDomain>,
    ) -> Maybe<NodeRef> {
        normalize_term(builder, root, rw, binders)
    }
}

/// The per-evaluation-context hoist-binding cap and the per-compound arity cap.
/// Overflow of either returns `Maybe::Isnt`, the no-alloc exhaustion signal.
const BIND_CAP: usize = 128; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: fixed-cap ANF hoist-scratch depth per evaluation context; overflow returns Isnt; tracked: #38
pub(crate) const ARG_CAP: usize = 64; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: fixed-cap ANF operand-arity scratch per compound; overflow returns Isnt; tracked: #38

/// A dummy `NodeRef` for stack-buffer initialisation (overwritten before read).
fn dummy_ref() -> NodeRef {
    NodeRef::new(USize::ZERO)
}

/// Append a hoist-binding, returning `Isnt` on scratch overflow.
fn record(binds: &mut [(Sym, NodeRef)], n: &mut usize, name: Sym, value: NodeRef) -> Maybe<()> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
    if *n >= binds.len() {
        return Maybe::Isnt;
    }
    binds[*n] = (name, value);
    *n += 1;
    Maybe::Is(())
}

/// ANF-normalise a whole term (its own evaluation context): flatten it into a
/// bounded hoist-binding list plus a tail, then wrap the bindings around the
/// tail in reverse so the first-evaluated binding is the outermost `Let`.
fn normalize_term(
    b: &mut Builder<'_>,
    e: NodeRef,
    rw: &Rewrite<'_>,
    binders: &mut Generator<BinderDomain>,
) -> Maybe<NodeRef> {
    let mut binds: [(Sym, NodeRef); BIND_CAP] = [(Sym::default(), dummy_ref()); BIND_CAP];
    let mut n = 0;
    let tail = flatten(b, e, rw, binders, &mut binds, &mut n)?;
    let mut body = tail;
    while n > 0 {
        n -= 1;
        let (t, v) = binds[n];
        body = b.let_(Bool::FALSE, t, v, body, Span::default())?;
    }
    Maybe::Is(body)
}

/// Flatten `e`: append `e`'s hoist-bindings to `binds[0..*n]` and return `e`'s
/// tail, an atom or a compound whose operands are all atoms. Let-chains flatten
/// into one binding list; each evaluation-context boundary (a branch, a lambda
/// or iter body, a match arm, a handle clause) gets a fresh `normalize_term`.
fn flatten(
    b: &mut Builder<'_>,
    e: NodeRef,
    rw: &Rewrite<'_>,
    binders: &mut Generator<BinderDomain>,
    binds: &mut [(Sym, NodeRef)],
    n: &mut usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
) -> Maybe<NodeRef> {
    let e = rw.resolve(e);
    match b.arena().get(e) {
        // atoms, and the opaque `Raw` leaf, reuse the input node unchanged.
        Node::Lit(_) | Node::Var(_) | Node::Raw { .. } => Maybe::Is(e),
        Node::Let { name, value, body, .. } => {
            let v = flatten(b, value, rw, binders, binds, n)?;
            record(binds, n, name, v)?;
            flatten(b, body, rw, binders, binds, n)
        }
        Node::Apply { callee, args } => {
            let ca = atomize(b, callee, rw, binders, binds, n)?;
            let mut atoms: [NodeRef; ARG_CAP] = [dummy_ref(); ARG_CAP];
            let m = atomize_list(b, args, rw, binders, binds, n, &mut atoms)?;
            b.apply(ca, &atoms[..m], Span::default())
        }
        Node::Project { base, key } => {
            let ba = atomize(b, base, rw, binders, binds, n)?;
            b.project(ba, key, Span::default())
        }
        Node::If { cond, then_branch, else_branch } => {
            let ca = atomize(b, cond, rw, binders, binds, n)?;
            let t = normalize_term(b, then_branch, rw, binders)?;
            let el = normalize_term(b, else_branch, rw, binders)?;
            b.if_(ca, t, el, Span::default())
        }
        Node::Iter { seq, body } => {
            let sa = atomize(b, seq, rw, binders, binds, n)?;
            let bo = normalize_term(b, body, rw, binders)?;
            b.iter(sa, bo, Span::default())
        }
        Node::Interp { value } => {
            let va = atomize(b, value, rw, binders, binds, n)?;
            b.interp(va, Span::default())
        }
        Node::Lambda { param, body } => {
            let bo = normalize_term(b, body, rw, binders)?;
            b.lambda(param, bo, Span::default())
        }
        Node::Match { scrutinee, arms } => {
            let sa = atomize(b, scrutinee, rw, binders, binds, n)?;
            let mut arm_roots: [NodeRef; ARG_CAP] = [dummy_ref(); ARG_CAP];
            let m = normalize_list(b, arms, rw, binders, &mut arm_roots)?;
            b.match_(sa, &arm_roots[..m], Span::default())
        }
        Node::Handle { body, clauses } => {
            let bo = normalize_term(b, body, rw, binders)?;
            let mut clause_roots: [NodeRef; ARG_CAP] = [dummy_ref(); ARG_CAP];
            let m = normalize_list(b, clauses, rw, binders, &mut clause_roots)?;
            let list = b.alloc_list(&clause_roots[..m])?;
            b.handle(bo, list, Span::default())
        }
    }
}

/// Return an atom for `e` (a `Var` or `Lit`, reused from the input). A compound
/// operand is flattened and its tail bound to a fresh minted binder, appending
/// that binding, so effect order stays explicit.
fn atomize(
    b: &mut Builder<'_>,
    e: NodeRef,
    rw: &Rewrite<'_>,
    binders: &mut Generator<BinderDomain>,
    binds: &mut [(Sym, NodeRef)],
    n: &mut usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
) -> Maybe<NodeRef> {
    let e = rw.resolve(e);
    match b.arena().get(e) {
        Node::Lit(_) | Node::Var(_) => Maybe::Is(e),
        _ => {
            let tail = flatten(b, e, rw, binders, binds, n)?;
            let t = binders.mint()?;
            record(binds, n, t, tail)?;
            b.var(t, Span::default())
        }
    }
}

/// Atomise each element of `list` into `out`, appending their hoist-bindings to
/// `binds`. Returns the element count, `Isnt` on arity or scratch overflow.
fn atomize_list(
    b: &mut Builder<'_>,
    list: vehje_ir::NodeList,
    rw: &Rewrite<'_>,
    binders: &mut Generator<BinderDomain>,
    binds: &mut [(Sym, NodeRef)],
    n: &mut usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
    out: &mut [NodeRef],
) -> Maybe<usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
    let len = list.len.0;
    if len > out.len() {
        return Maybe::Isnt;
    }
    // copy the element refs out before mutating the arena (the read borrow and
    // the append cannot overlap).
    let mut in_refs: [NodeRef; ARG_CAP] = [dummy_ref(); ARG_CAP];
    {
        let src = b.arena().list(list);
        let mut i = 0;
        while i < len {
            in_refs[i] = src[i];
            i += 1;
        }
    }
    let mut i = 0;
    while i < len {
        out[i] = atomize(b, in_refs[i], rw, binders, binds, n)?;
        i += 1;
    }
    Maybe::Is(len)
}

/// Normalise each element of `list` as its own evaluation context into `out`.
/// Returns the element count, `Isnt` on arity or scratch overflow.
fn normalize_list(
    b: &mut Builder<'_>,
    list: vehje_ir::NodeList,
    rw: &Rewrite<'_>,
    binders: &mut Generator<BinderDomain>,
    out: &mut [NodeRef],
) -> Maybe<usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal fixed-cap scratch counter/length (array index in no_std no_alloc); tracked: #38
    let len = list.len.0;
    if len > out.len() {
        return Maybe::Isnt;
    }
    let mut in_refs: [NodeRef; ARG_CAP] = [dummy_ref(); ARG_CAP];
    {
        let src = b.arena().list(list);
        let mut i = 0;
        while i < len {
            in_refs[i] = src[i];
            i += 1;
        }
    }
    let mut i = 0;
    while i < len {
        out[i] = normalize_term(b, in_refs[i], rw, binders)?;
        i += 1;
    }
    Maybe::Is(len)
}

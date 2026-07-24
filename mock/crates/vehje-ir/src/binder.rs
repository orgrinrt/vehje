//! `BinderDomain`: vehje's generative identity domain for minted binders.

use hilavitkutin_sym::{Domain, GenerativeDomain, SymKind};

/// The generative domain for binder identities minted by the compile-stage
/// passes (ANF hoisting, macro expansion).
///
/// Kind `0b001`, the value the sym-core kind table reserves for vehje's
/// minted binder. A minted `Sym` of this domain is never equal to a
/// string-kind `Sym` (kind `0b000`) whatever their ids, so a minted binder
/// cannot capture a source name and a source name cannot resolve to a minted
/// binder: the disjointness is structural, on the domain tag, not a resolver
/// convention.
pub struct BinderDomain;

impl Domain for BinderDomain {
    const KIND: SymKind = SymKind::from_raw(0b001);
}

impl GenerativeDomain for BinderDomain {}

#[cfg(test)]
mod tests {
    use super::BinderDomain;

    use arvo::strategy::Hot;
    use arvo::{Identity, USize};
    use arvo_bits::Bits;
    use hilavitkutin_str::{str_const, Str};
    use hilavitkutin_sym::{Domain, Sym};

    use crate::arena::Arena;
    use crate::builder::Builder;
    use crate::hash::hash_of;
    use crate::node::{Literal, Node, NodeRef};
    use crate::span::Span;

    fn at(m: arvo::Maybe<NodeRef>) -> NodeRef {
        match m {
            arvo::Maybe::Is(r) => r,
            arvo::Maybe::Isnt => panic!("arena full"),
        }
    }

    fn arena4() -> ([Node; 4], [Span; 4], [NodeRef; 4]) {
        (
            [Node::Lit(Literal::Unit); 4],
            [Span::default(); 4],
            [NodeRef::new(USize::ZERO); 4],
        )
    }

    // A minted binder (BinderDomain, kind 0b001) is never equal to a source
    // name (the string domain, kind 0b000), even at the same id: the domain
    // tag is part of the compared bits. A minted binder cannot capture a
    // source name and a source name cannot resolve to a minted binder. The
    // disjointness is structural, not a resolver convention.
    #[test]
    fn minted_binder_disjoint_from_source_name() {
        let id = Bits::<28, Hot>::from_raw(3);
        let minted = Sym::new(<BinderDomain as Domain>::KIND, id);
        let source = Sym::new(Str::STR_DOMAIN, id);
        assert_ne!(minted.kind(), source.kind());
        assert_ne!(minted, source);
    }

    // The builder accepts both a source name widened via `as_sym` and a minted
    // binder Sym at every binder position.
    #[test]
    fn builder_accepts_source_and_minted_binders() {
        let (mut n, mut s, mut p) = arena4();
        let mut b = Builder::new(Arena::new(&mut n, &mut s, &mut p));
        let body = at(b.lit(Literal::Unit, Span::default()));
        let source = str_const!("x").as_sym();
        let minted = Sym::new(<BinderDomain as Domain>::KIND, Bits::<28, Hot>::from_raw(0));
        let _v = at(b.var(source, Span::default()));
        let _lam = at(b.lambda(minted, body, Span::default()));
    }

    // Widening a source name to a `Sym` does not change its 32 bits, so a `Var`
    // built from a source name folds the same word it did before the flip: the
    // structural hash of a source-name `Var` is unchanged.
    #[test]
    fn str_to_sym_widening_is_byte_transparent() {
        let s = str_const!("x");
        assert_eq!(s.as_sym().to_bits(), s.to_bits());
    }

    // The domain tag flows into the structural hash: a `Var` over a minted
    // binder and a `Var` over a source name with the same id hash differently,
    // so the capture-safety disjointness holds at the hash level too.
    #[test]
    fn var_hash_reflects_binder_domain() {
        let id = Bits::<28, Hot>::from_raw(5);
        let (mut n1, mut s1, mut p1) = arena4();
        let mut b1 = Builder::new(Arena::new(&mut n1, &mut s1, &mut p1));
        let src = at(b1.var(Sym::new(Str::STR_DOMAIN, id), Span::default()));
        let a1 = b1.into_arena();

        let (mut n2, mut s2, mut p2) = arena4();
        let mut b2 = Builder::new(Arena::new(&mut n2, &mut s2, &mut p2));
        let minted = at(b2.var(Sym::new(<BinderDomain as Domain>::KIND, id), Span::default()));
        let a2 = b2.into_arena();

        assert_ne!(hash_of(&a1, src), hash_of(&a2, minted));
    }
}

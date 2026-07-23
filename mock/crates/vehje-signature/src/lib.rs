//! vehje-signature, the framework's language-level definition.
//!
//! The graded algebraic signature a language descends from: families as
//! operations of an algebraic-effect theory, each with its effect grade and
//! per-operand coeffect discipline, the attribute-grammar attachment, the
//! lease-rule schema, and the declared target set, plus the three projections
//! of the one signature. Data only; the compilation over it is
//! `vehje-runtime-gen`'s. `vehje-ir` is what a program is, `vehje-signature`
//! is what a language is.
//!
//! `#![no_std]`, no alloc: the catalogue lives in caller-provided slices.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use vehje_ir::{EffectMask, FamilyId};

/// One family operation in the algebraic-effect theory.
///
/// Its `FamilyId`, its declared effect grade (the operations it invokes), and
/// its per-operand coeffect discipline (which operands it links versus
/// consumes, the lease rule). The operations are the theory the handler
/// discipline services.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Operation {
    /// The family this operation belongs to.
    pub family: FamilyId,
    /// The operations this one invokes (its declared effect).
    pub effect: EffectMask,
    /// The per-operand lease rule.
    pub lease: LeaseRule,
}

/// The per-family lease-rule schema `vehje-check` reads to compute the
/// reachability coeffect.
///
/// The monotone function from an operand's declared effect to its
/// link-or-consume bit: retention through a host boundary is itself an effect,
/// so where the effect is precise the rule is a lookup and where it is coarse
/// the reachability grade refines it. `consumes` names the effect operations
/// that consume (rather than link) an operand.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct LeaseRule {
    /// The effect operations under which an operand is consumed.
    pub consumes: EffectMask,
}

/// A declared target's language-level record.
///
/// The family set it supports and the effect set it permits, from which the
/// inclusion check is folded. Borrowed from the caller-provided catalogue.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TargetDecl<'a> {
    /// The families this target handles.
    pub supports: &'a [FamilyId],
    /// The runtime effect operations this target permits.
    pub permits: EffectMask,
}

/// Which view of the one signature a consumer wants.
///
/// The node algebra has three projections of the same operations, kept
/// consistent by construction rather than as parallel sets.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Projection {
    /// The IR a pass folds (the `vehje-ir` node forms).
    Eliminator,
    /// The produced-value domain (a family's constructors, content-as-values).
    Introduction,
    /// The introduction forms compiled to copy-and-patch stencils (deferred).
    IntroductionCompiled,
}

/// The attachment point for an opt-in grammar-as-data front-end.
///
/// Where a consumer provides a declarative grammar, the signature carries the
/// attribute-grammar schema the general parser interprets; the synthesised
/// attributes an L-attributed evaluation computes are the initial grades. A
/// hand-written front-end leaves this attachment empty (opt-in per the input
/// contract).
// FIXME: the attribute-grammar schema shape lands with the first grammar-as-data
// consumer; M-level carries the presence flag only, so a hand-written front-end
// is represented by the absent variant.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum GrammarHook {
    /// No framework-side grammar; the consumer brings its own front-end.
    #[default]
    HandWritten,
    /// A grammar-as-data attribute-grammar schema is attached.
    AttributeGrammar,
}

/// The one source a language descends from.
///
/// The family catalogue as operations, the declared target set, and the
/// grammar hook. Built once per language at dev time, read by `vehje-check`
/// (its lease-rule schema and axiom table) and composed by `vehje-runtime-gen`.
/// Borrowed from caller-provided slices; holds data, not a compiler.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Signature<'a> {
    /// The operation catalogue, one per family operation.
    pub operations: &'a [Operation],
    /// The declared targets.
    pub targets: &'a [TargetDecl<'a>],
    /// The input-side grammar attachment.
    pub grammar: GrammarHook,
}

impl<'a> Signature<'a> {
    /// The lease rule for a family operation, or the default (link-all) rule
    /// if the family is not in the catalogue.
    pub fn lease_rule(&self, family: FamilyId) -> LeaseRule {
        let mut i = 0; // lint:allow(no-bare-numeric) reason: catalogue scan index over a caller-provided slice; tracked: #207
        while i < self.operations.len() {
            let op = self.operations[i];
            if op.family == family {
                return op.lease;
            }
            i += 1; // lint:allow(no-bare-numeric) reason: catalogue scan index; tracked: #207
        }
        LeaseRule::default()
    }

    /// The projection view selector, for `vehje-runtime-gen` to emit the
    /// interpreter arm, the value-kind, and the stencil per operation.
    pub fn projection(&self, which: Projection) -> Projection {
        which
    }
}

//! The node enum: four syntactic categories in one arena.

use notko::Maybe;

use super::{AstList, AstRef, LitKind, Name, BinOp, UnOp};
use crate::token::Span;

/// One AST node.
///
/// Variants are grouped by syntactic category. A node's children are `AstRef`s
/// and `AstList`s into the same arena, so a subtree is a span of indices and
/// nothing is owned.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Node {
    // ── types ────────────────────────────────────────────────────────────
    /// `&T` or `&mut T`. References erase to their pointee downstream, which is
    /// why the mutability is recorded and then ignored.
    TyRef { mutable: bool, inner: AstRef },
    /// `()`, `(T,)`, or `(A, B, ...)`. A parenthesised type with no trailing
    /// comma is NOT a tuple; the parser peels it.
    TyTuple { elems: AstList },
    /// A path type, with optional generic arguments.
    TyPath { path: AstRef },

    // ── paths ────────────────────────────────────────────────────────────
    /// `a::b::<T>`: segments plus optional generic arguments.
    Path { leading_colon: bool, segs: AstList, generics: AstList },
    /// One path segment: an identifier or `self`/`Self`/`super`/`crate`.
    PathSeg { name: Name },

    // ── expressions ──────────────────────────────────────────────────────
    Lit { kind: LitKind, span: Span },
    /// A path used as a value.
    ExprPath { path: AstRef },
    /// `{ stmts; tail }`. `tail` is `Isnt` when the block ends in a statement.
    Block { stmts: AstList, tail: Maybe<AstRef> },
    /// `(a, b)`; `()` is the unit value.
    ExprTuple { elems: AstList },
    If { cond: AstRef, then_block: AstRef, else_branch: Maybe<AstRef> },
    Match { scrutinee: AstRef, arms: AstList },
    /// One `pat if guard => body` arm.
    Arm { pat: AstRef, guard: Maybe<AstRef>, body: AstRef },
    For { pat: AstRef, iter: AstRef, body: AstRef },
    While { cond: AstRef, body: AstRef },
    Loop { body: AstRef },
    Return { value: Maybe<AstRef> },
    Break { value: Maybe<AstRef> },
    Continue,
    /// `|a, b| body`, with `move` recorded and erased (values are immutable, so
    /// capture is by value either way).
    Closure { moved: bool, params: AstList, body: AstRef },
    /// `Path { field: value, ..base }`.
    StructLit { path: AstRef, fields: AstList, base: Maybe<AstRef> },
    /// One `name: value` field, or `name` shorthand.
    FieldInit { name: Name, value: Maybe<AstRef> },
    /// `base.name`.
    Field { base: AstRef, name: Name },
    /// `base.name(args)`.
    MethodCall { base: AstRef, name: Name, args: AstList },
    /// `callee(args)`.
    Call { callee: AstRef, args: AstList },
    /// One call argument, with the keyword name the grammar parses and later
    /// passes currently ignore.
    Arg { name: Maybe<Name>, value: AstRef },
    /// `base[index]`.
    Index { base: AstRef, index: AstRef },
    /// `e?`.
    Question { value: AstRef },
    Unary { op: UnOp, operand: AstRef },
    Binary { op: BinOp, lhs: AstRef, rhs: AstRef },
    /// `a..b` or `a..=b`, with either end optional.
    Range { inclusive: bool, lo: Maybe<AstRef>, hi: Maybe<AstRef> },
    /// `place = value` and the compound forms; `op` is `Isnt` for plain `=`.
    Assign { op: Maybe<BinOp>, place: AstRef, value: AstRef },
    /// `path!(tokens)` in expression or item position. The body is kept as a
    /// span because a macro's tokens mean nothing until expansion.
    MacroCall { path: AstRef, body: Span },

    // ── statements ───────────────────────────────────────────────────────
    /// `let pat: Ty = init;`, with both the type and the initialiser optional.
    Let { pat: AstRef, ty: Maybe<AstRef>, init: Maybe<AstRef> },
    /// An expression in statement position.
    ExprStmt { value: AstRef },

    // ── patterns ─────────────────────────────────────────────────────────
    /// `_`
    PatWild,
    /// A literal pattern, with an optional range end.
    PatLit { kind: LitKind, span: Span, range_end: Maybe<AstRef> },
    /// A bare binding.
    PatIdent { name: Name },
    /// A multi-segment or leading-colon path.
    PatPath { path: AstRef },
    /// `Path(a, b)`.
    PatTupleStruct { path: AstRef, elems: AstList },
    /// `(a, b)`; a parenthesised pattern with no trailing comma is peeled.
    PatTuple { elems: AstList },
    /// `a..b` / `a..=b`.
    PatRange { inclusive: bool, lo: Maybe<AstRef>, hi: Maybe<AstRef> },
    /// `&p` / `&mut p`, peeled downstream like `TyRef`.
    PatRef { mutable: bool, inner: AstRef },
    /// `..`
    PatRest,
    /// `a | b`.
    PatOr { alts: AstList },

    // ── items ────────────────────────────────────────────────────────────
    /// A whole file: inner attributes, then items.
    File { attrs: AstList, items: AstList },
    /// `#[path(args)]` or `#![path(args)]`. The argument tokens stay a span,
    /// because attribute semantics are the consumer's, parsed lazily by whoever
    /// cares rather than by the grammar.
    Attr { inner: bool, path: AstRef, args: Maybe<Span> },
    /// `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`.
    Vis { restriction: Maybe<AstRef> },

    /// `fn name<G>(params) -> ret where ... { body }`. A `;` body is a
    /// signature-only declaration, which is what trait items and extern fns are.
    ItemFn {
        attrs: AstList,
        vis: Maybe<AstRef>,
        name: Name,
        generics: AstList,
        params: AstList,
        ret: Maybe<AstRef>,
        wheres: AstList,
        body: Maybe<AstRef>,
    },
    /// One function parameter: `self` in its four spellings, or `name: Ty`.
    Param { self_param: bool, name: Maybe<Name>, ty: Maybe<AstRef> },

    /// `struct Name<G>: BindTarget where ... { fields }`.
    ///
    /// The colon tail is the bind-target syntax, a constraint the checker reads
    /// and then erases.
    ItemStruct {
        attrs: AstList,
        vis: Maybe<AstRef>,
        sealed: bool,
        name: Name,
        generics: AstList,
        bind_target: Maybe<AstRef>,
        wheres: AstList,
        fields: AstList,
    },
    /// One struct field.
    Field2 {
        vis: Maybe<AstRef>,
        is_const: bool,
        is_mut: bool,
        name: Name,
        ty: AstRef,
        default: Maybe<AstRef>,
    },

    /// `enum Name<G> where ... { variants }`.
    ItemEnum {
        attrs: AstList,
        vis: Maybe<AstRef>,
        name: Name,
        generics: AstList,
        wheres: AstList,
        variants: AstList,
    },
    /// One enum variant, with an optional tuple payload.
    Variant { name: Name, payload: AstList },

    /// `trait Name<G>: Supertraits where ... { items }`.
    ItemTrait {
        attrs: AstList,
        vis: Maybe<AstRef>,
        sealed: bool,
        name: Name,
        generics: AstList,
        supertraits: AstList,
        wheres: AstList,
        items: AstList,
    },
    /// `impl<G> Trait for Type where ... { items }`, with `for_ty` absent on an
    /// inherent impl.
    ItemImpl {
        attrs: AstList,
        generics: AstList,
        ty: AstRef,
        for_ty: Maybe<AstRef>,
        wheres: AstList,
        items: AstList,
    },

    /// `type Name = Ty;`. Without the `= Ty`, an associated-type DECLARATION,
    /// which is legal only in a trait body.
    ItemTypeAlias { attrs: AstList, vis: Maybe<AstRef>, name: Name, ty: Maybe<AstRef> },
    /// `const NAME: Ty [= value];`, the initialiser absent on an associated
    /// const declaration.
    ItemConst { attrs: AstList, vis: Maybe<AstRef>, name: Name, ty: AstRef, value: Maybe<AstRef> },
    /// `static [mut] NAME: Ty = value;`
    ItemStatic {
        attrs: AstList,
        vis: Maybe<AstRef>,
        mutable: bool,
        name: Name,
        ty: AstRef,
        value: AstRef,
    },

    /// `mod path;` or `mod path { items }`.
    ItemMod { attrs: AstList, vis: Maybe<AstRef>, path: AstRef, items: Maybe<AstList> },
    /// `use tree;`
    ItemUse { attrs: AstList, vis: Maybe<AstRef>, tree: AstRef },
    /// One `use` tree: a path, optionally renamed, globbed, or branching.
    UseTree { path: Maybe<AstRef>, alias: Maybe<Name>, glob: bool, children: AstList },

    /// `macro name<G>(params) -> Ty { body }`. The return type is mandatory,
    /// which is what makes a macro a typed function at a compile stage rather
    /// than a token rewriter.
    ItemMacro {
        attrs: AstList,
        vis: Maybe<AstRef>,
        name: Name,
        generics: AstList,
        params: AstList,
        ret: AstRef,
        body: AstRef,
    },
    /// `event Name for Type { body }`: a named handler registration.
    ItemEvent { attrs: AstList, name: Name, for_ty: AstRef, body: AstRef },
    /// `expect item` / `actual item`: an obligation and its provider, chosen at
    /// whichever stage supplies it.
    ItemExpect { inner: AstRef },
    ItemActual { inner: AstRef },
    /// `extern` in its several shapes; the body is the declared item.
    ItemExtern { attrs: AstList, inner: AstRef },
    /// A macro invocation in item position.
    ItemMacroCall { attrs: AstList, path: AstRef, body: Span },

    /// A generic type parameter with its bounds.
    GenericParam { name: Name, bounds: AstList },
    /// `const N: Ty` in a generic parameter list.
    ConstParam { name: Name, ty: AstRef },
    /// One `Ty: Bounds` predicate of a where clause.
    WherePred { ty: AstRef, bounds: AstList },
}

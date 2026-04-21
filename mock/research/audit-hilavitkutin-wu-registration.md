**Date:** 2026-04-21
**Author:** substrate audit (task #132, SUB-H0)
**Scope:** Does `hilavitkutin-api`'s WorkUnit registration surface
fit clause's compile-pipeline needs? Do we need an extension round
in hilavitkutin before populating `clause-schedule` (#131)?

# Conclusion

No extension required. The shipped contract is sufficient for the
clause-schedule skeleton and for the clause compile pipeline as
designed. The implementation-level gap that remains
(`SchedulerBuilder` methods returning `self` unchanged) is a
hilavitkutin-internal concern and does not block `#131`.

# What clause needs

Clause's compile pipeline is a static, mostly-linear DAG. One
`WorkUnit` per phase. Each WU reads one or two columns and writes
one. Work granularity is per-file (a `Column<SourceFile>` record
is one file; morsels are file ranges). Incremental recompute is
coarse: re-run a WU only when its read columns are dirty. Macro
evaluation, event-triggered analyzers, and the interpreter
(#173) are `On<V>` firings over virtuals the upstream phases
raise.

Specific requirements the audit checked:

- WU type with `Read` / `Write` AccessSets and a body.
- Column descriptor for per-record state (tokens, AST nodes,
  resolved symbols, diagnostics per file, emitted artifacts).
- Resource descriptor for pipeline-wide state (interner,
  diagnostic sink, manifest, content-hash cache).
- Virtual descriptor for event-triggered phases (macro expansion
  when a `#[macro_name]` attribute fires, scheduler-introspection
  WUs for `clause explain`).
- Incremental recompute: skip WUs whose inputs are clean.
- Sink trait for diagnostics that multiple WUs can push into.
- Seq / Map resource layouts for the deferred `clause-resolve`
  fields (`Scope::symbols`, `Manifest::dependencies`, etc.) once
  the scheduler takes ownership of their storage.

# What hilavitkutin-api ships

At `~/Dev/clause-dev/hilavitkutin/mock/crates/hilavitkutin-api/`,
the shipped surface covers every line in the requirements above.

**Core trait** (`src/work_unit.rs`):

```rust
pub trait WorkUnit<Schedule = Always>: Send + Sync + 'static {
    type Read: AccessSet;
    type Write: AccessSet;
    type Hint: SchedulingHint;
    type Ctx: HasColumnReader<Self::Read>
        + HasColumnWriter<Self::Write>
        + HasResourceProvider<Self::Read>
        + HasVirtualFirer<Self::Write>
        + HasEach<Self::Read, Self::Write>
        + HasBatch<Self::Read, Self::Write>
        + HasReduce<Self::Read, Self::Write>;
    const COMMUTATIVE: Bool = Bool::FALSE;
    fn execute(&self, ctx: &Self::Ctx);
}
```

`Schedule` is either `Always` (run every pass) or `On<V>` (fire
when virtual `V` is raised). Clause uses `Always` for the
mandatory phases (lex → parse → resolve → typecheck → transpile →
codegen) and `On<V>` for event-driven macro / analyzer / doc /
fmt WUs.

**Store descriptors** (`src/store.rs`):

- `Resource<T>` — singleton shared state. Clause wires the
  `StringInterner`, the diagnostic sink, the manifest, and the
  content-hash cache as resources.
- `Column<T>` — morsel-chunked per-record state. Clause wires
  source bytes, tokens, AST nodes, resolved symbols, typecheck
  outputs, transpiled artifacts as columns.
- `Virtual<T>` — zero-data DAG edge for fire flags. Clause fires
  a `MacroExpansionPending` virtual when parse encounters a
  `#[cfg(...)]` or `#[patch(...)]` attribute needing expansion.
- `Field<T>`, `Seq<T, N: Cap>`, `Map<K, V, N: Cap>` — resource
  layouts. Clause's deferred `clause-resolve` field storages
  (tracked to #131 / #134) re-express against these once
  scheduler ownership lands.

**AccessSet mechanics** (`src/access.rs`):

`AccessSet` is a sealed marker trait with a `Contains<S>` witness
for type-level set membership. Clause's
`AccessSet` types are ordinary tuple aliases:

```rust
type LexRead   = (Resource<StringInterner>, Column<SourceFile>);
type LexWrite  = Column<TokenStream>;
```

No per-repo extension needed.

**Sink traits** (`src/sink.rs`):

`DiagnosticSink<E>: Push<E> + Len` covers the
diagnostic-emission path. Every WU that needs to emit
diagnostics reads a
`Resource<&mut dyn DiagnosticSink<Diagnostic>>` via
`HasResourceProvider`. `CountingSink` / `NullSink` / `TeeSink`
provide the test-side and compose-side combinators.

**Scheduling hints** (`src/hint.rs`):

`SchedulingHint` is a tuple of `(Urgency, Divisibility,
Significance)` with marker variants (`Immediate`, `Steady`,
`Relaxed`, `Deferred`, `Atomic`, `Adaptive`, `Interruptible`,
`Critical`, `Important`, `Normal`, `Opportunistic`, `Optional`).
Clause picks per-phase hints: lex is `Immediate`, resolve is
`Steady`, the long-tail analyzers are `Optional`.

**Incremental recompute** (engine side,
`hilavitkutin/src/plan/dirty.rs`):

`DirtyMask<MAX_STORES>` is a const-generic bitset tracking which
columns changed since last pass. The engine's `build_dag` +
`topo_sort` + WU-execution chain consult this mask to skip WUs
whose inputs are clean. This is the mechanism clause relies on
for coarse incremental (rebuild only the files whose source
bytes changed, propagate downstream). No clause-side extension
required.

# What the audit found that isn't extension-gap

One area needs explicit note rather than action:

**Scheduler builder is skeleton.** The `SchedulerBuilder` methods
(`add::<WU>()`, `column::<T>()`, `resource::<T>()`, etc.) in
`hilavitkutin/src/scheduler/mod.rs` all return `Self` unchanged.
Actual WU-tuple composition is not shipped. This is a known
hilavitkutin gap (the scheduler-execution work tied to the
engine's domain 23 + dispatch + thread-pool work). It does NOT
block `clause-schedule` from declaring the clause-side WU types,
AccessSets, and Column descriptors against the shipped contract.
Real execution wires in once hilavitkutin's engine completes its
composition + dispatch loop.

# Recommendation for #131

Populate `clause-schedule` with:

1. Clause-specific `AccessSet` tuple aliases (`LexRead`,
   `LexWrite`, `ParseRead`, etc.).
2. Clause-specific `Column<T>` / `Resource<T>` / `Virtual<T>`
   type aliases. The `T` parameters are the clause-ir and
   clause-syntax types already shipped (`Token`, `AstNode`,
   `Resolved`, `Symbol`, `Diagnostic`, etc.).
3. ZST WU structs (`LexWU`, `ParseWU`, `ResolveWU`, …) each with
   an `impl WorkUnit<Always>` (or `On<V>` where applicable)
   declaring its associated types.
4. A const `CLAUSE_PIPELINE` tuple listing the WU types in DAG
   order, used by the scheduler builder once composition lands.
5. No engine-level changes. Execution integration is out of
   scope for #131 and lands when hilavitkutin's
   `SchedulerBuilder` graduates past skeleton.

This matches the substrate principle: clause applies
hilavitkutin's contract; never invents its own.

# Cross-reference

- Substrate principle: `mock/research/substrate-principle.md`
- Port plan M0.2: `mock/research/parity-plan.md` (search
  "M0.2 — clause-schedule as WorkUnit home")
- Task: #131 (clause-schedule as hilavitkutin WorkUnit home),
  blocked by #132 (this audit — now closable).
- Downstream engine gap (not blocking): hilavitkutin
  `SchedulerBuilder` composition + dispatch loop.

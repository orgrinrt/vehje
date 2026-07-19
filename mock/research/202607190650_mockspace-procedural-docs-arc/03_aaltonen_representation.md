# Representation and execution review

**Verdict:** the columnar-value direction is right at the type-check layer and wrong as stated at the
storage layer; the registry's real payload is variable-length text and small arrays, not fixed scalars, so
"a table is a value with a type" needs a two-tier value domain (scalar columns plus arena handles) before
it is a storage claim rather than a schema claim, and the same handle shape is what makes aliasing and
no-alloc iteration both fall out for free instead of fighting each other.

## What is sound

Node vocabulary as a compile-time type parameter, one IR with monomorphised instantiation, storage as a
trait rather than a concrete type: all three match how `hilavitkutin-api` is actually built. `Resource<T>`,
`Column<T>`, `Virtual<T>` are zero-sized compile-time markers (`store.rs:18`, `76`, `155`); `ColumnValue`
is a spec-free blanket trait, not a runtime registry (`column_value.rs:25,34`). Confirming
`hilavitkutin-api` has no engine dependency is correct and the lint file backs it (`hilavitkutin-api`
forbids depending on `hilavitkutin`; contracts do not depend on implementations). The instinct to reject
`HostRef` handles is right for the reason given: there is no host here, so an opaque-handle domain would be
importing a boundary that does not exist.

## Findings

**1. `ColumnValue` models scalars; the registry is text and arrays.** `ColumnValue: Copy + 'static` with
`BIT_WIDTH = size_of::<Self>() * 8` (`column_value.rs:25,29`) is a fixed-size scalar contract, built for
arvo's bounded fixed-point numerics. `ikiuni_renderer/mock/registry/technique/*.toml` (27 files, 4480 lines
for one record kind alone) shows the real shape: `what` and `name` are variable-length strings, `sources`
and `crates` are arrays of 1 to 5+ references, several fields are optional. None of that is `Copy`. Storing
it as a literal `Column<TechniqueRecord>` forces every string field into a fixed-capacity `Seq<T, const N:
Cap>` (`store.rs:209`, backed by `arvo-storage::platform::Cap(pub USize)` at `platform.rs:75`) sized to the
worst case across all ~2700 records and 15 kinds. Observed field lengths run roughly 40 to 200 bytes; a
global worst-case `N` picked to cover one long outlier wastes 2 to 5x per record, times 2700, for every
string-shaped field. That is real memory, not a hypothetical: it is exactly the kind of cost this
workspace's own numeric substrate exists to avoid paying carelessly.

Alternative: split the value domain in two tiers from the start. Fixed, small, enum-shaped fields (record
kind tag, resolved cross-reference index, small counts) are genuine `ColumnValue` columns. Variable text and
arrays are a `(USize, USize)` offset/length handle into a byte arena, itself trivially `Copy` and therefore
itself a valid `ColumnValue`. A "record" is a struct of scalar fields plus handle fields; "a column of
records" is parallel scalar columns plus handle columns, which is SoA in fact, not only in name, and the
type system still knows the full schema at check time, so the projection rule (namespace to row, row to
field) still resolves statically. This also directly produces the answer to finding 3.

**2. No-alloc under iteration forces a sink, not an arena, as the default.** A fixed arena forecloses
unbounded output (a query over all ~2700 records into one list must either be pre-sized to the worst case
or truncate loudly, and "worst case" here is a moving target set by future registry growth, not something
the interpreter should hardcode). Streaming forecloses re-reading the emitted text before it is final (table
column-width alignment needs the row set buffered once before any byte is written). A caller-provided sink
(`&mut impl ByteEmitter`, i.e. `Push<u8> + BulkPush<u8>`, the same contract this workspace already uses at
FFI/text boundaries) is the right default because it is neutral: the caller supplies either a fixed arena
(bounded documents) or a real streaming writer (unbounded documents), and the interpreter itself commits to
neither. mockspace, not the interpreter, knows the actual document count and expected size; push the sizing
decision there.

**3. Aliasing is achievable, but only if the value domain's variable payload IS the handle from finding 1,
scoped to one pass.** "Alias, do not copy" only holds if a string/array `Value` is a reference into the
registry's own already-parsed bytes, not an index into interpreter-owned storage copied from the registry.
That means the interpreter borrows the registry immutably for the duration of one evaluate-and-render pass
and never holds a `Value` past that pass; this mirrors the WU-Ctx-scoped borrow pattern already in
`hilavitkutin-api` (`ColumnReaderApi::read` in `context.rs:104-115` is `unsafe fn` precisely because the
caller, not the type, proves the borrow is live). Structured this way (one lexical scope per render, no
`Value` escaping it, no interior mutability needed on the registry side) the borrow checker is not fought;
it is doing exactly its job. It becomes a fight only if the design also wants a persistent in-language value
that survives across renders (closures over query results, rebinding across evaluation steps); that shape
requires either copying at the boundary or unsafe lifetime erasure, and should be named as a deliberate
scope decision, not discovered as friction later.

**4. The runner's contract must already look like the engine's `Context`, not like an AST walker.**
`WorkUnit::Read` / `Write` are compile-time cons-list types (`Cons<H, T>` / `Empty`, sealed, `access.rs:39,
50, 53`), monomorphised per `WorkUnit` (`work_unit.rs:54`). A query plan parsed from DSL text at runtime
cannot synthesize a new Rust type per query, so "the interpreter's runtime query plan becomes a WorkUnit"
is not a future the design can reach; that door is shut by how `AccessSet` works, not by the engine being
unready. What transfers is narrower: per-record-kind, morsel-windowed column access, dispatched over a
fixed enum of the (known, static) 15 record kinds rather than an open runtime type. The minimal runner
should expose exactly that shape now, mirroring `ColumnReaderApi::read` and `EachApi::run`
(`context.rs:104-115, 208-214`): unsafe, index-bounded, no dynamic dispatch, one instantiation per kind. The
DSL's control flow (which kinds, what order, what filter) stays runtime forever; the per-kind record access
becomes, verbatim, one compile-time `WorkUnit` per kind when the scheduler lands. If the runner instead
ships as one function walking a runtime AST with ad hoc reads, the later swap is a rewrite of the
interpreter's iteration primitive, not a drop-in.

## What the documents assume without arguing

That 15 record kinds, fixed at compile time, is a stable premise: true today, but the registry is
TOML and can add a 16th kind without a Rust rebuild, at which point the fixed-enum dispatch in finding 4
needs a real answer (a build-time codegen step reading the registry schema, most likely, since a runtime
open type is off the table per finding 4). That the projection type rule generalizes past table/row/scalar
to array-of-references fields (`sources`, `crates` are arrays, not scalars) without a new node shape.

## Open questions I cannot settle alone

Whether `hilavitkutin-str::Str` is the intended home for variable text (interner identity, but a second
handle kind alongside the arena-offset handle from finding 1) or whether the arena-offset handle alone
should carry all of it, is a maintainer call. The real `Cap` bound `Seq<T, N>` needs per field, across all
207 TOML files and 15 kinds, not the one file sampled here; that is a mechanical measurement pass, not a
design question, but it has to run before any `N` is picked.

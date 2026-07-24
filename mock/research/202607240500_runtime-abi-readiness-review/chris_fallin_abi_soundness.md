# vehje runtime-ABI boundary through the verify-then-emit soundness lens (Chris Fallin)

## Verdict in one line

Not yet ready to build against: the type surface that is landed (`entry.rs`, `sink.rs`, `wire/residual.rs`,
`encode.rs`) is honest and mostly sound for what it declares, but the value-arena types in `value.rs`, the
type the whole "wire form equals in-process form" claim rests on, are missing `#[repr(C)]` on every composite
struct and on the `ValueTag` enum, which means the crate's own design goal (materialise the arena directly
over host-lent, cross-language memory with no decode step) cannot be built soundly against the types as they
stand today. That is a defect in what has landed, not a deferred mechanism, because it sits in the type
declarations themselves, not in the unwritten export body.

## What is sound and established (specific, cite file:line)

- **`BatchColumn` is a correct `#[repr(C)]` descriptor.** `entry.rs:36-48`: five fields, every one a pointer or
  a `usize`, `#[repr(C)]` applied, so field order is the declared order with no reordering. A C or Zig host
  reading this struct gets the layout the Rust side wrote. `VehjeSink` (`sink.rs:41-50`) is the same story, and
  it is the one type in the crate that gets an explicit layout assertion (`sink.rs:74-77`, `size_of::<VehjeSink>()
  == 3 * size_of::<*mut c_void>()`), which is exactly the right kind of test for a cross-language struct.
- **The reserve-then-commit sink contract is a sound, minimal iteratee.** `sink.rs:17-32`: `ReserveFn` returns
  null on backpressure (the documented signal), `CommitFn` accepts a commit smaller than the hint (the reserve
  is an upper bound, stated at `sink.rs:29-30`). Two function pointers plus opaque userdata is the entire
  contract; nothing hidden.
- **`ValueRef` and `RegionId` are honest `#[repr(transparent)]` newtypes over `arvo::USize`, which is itself
  `#[repr(transparent)]` over `usize`** (`value.rs:20-22,40-42`; confirmed at `arvo-storage/src/platform.rs:45-47`).
  These two types alone are sound to reinterpret across a language boundary.
- **The residual encoder (`encode.rs`) is genuinely format-agnostic and IR-preserving.** `encode` (`encode.rs:111-206`)
  walks the checked arena once in node-index order and drives a trait, never touching the wire format directly;
  `FlatArenaEncoder` (`wire/serialize.rs:115-257`) is the one tier-0 implementation, little-endian, fixed
  seven-word node records, and it back-patches only the single `blob_len` header word by design (`BLOB_LEN_AT`,
  computed once at `finish`), never any node or pool byte. Two round-trip tests exist and pass by construction
  (`wire/serialize.rs:303-393`) exercising both the pool path and the string-blob path, reading the header,
  every node field, and the blob back byte-for-byte.
- **The value-arena's children-before-parents ordering is the right shape for a cheap acyclic decode.** The
  design claim (`value.rs:1-15`, DEEPDIVE §1) that acyclicity reduces to one monotone-index check on an
  untrusted arena is correct in principle: if every child index is required to be strictly less than its
  parent's index, a linear scan enforces it in one pass with no visited-set. Nothing in the shipped code
  enforces this yet (see Finding 4), but the underlying claim is sound and the one test that exists
  (`value.rs:184-229`) is consistent with it (node 2 the root, referencing node 1, referencing node 0).
- **`ABI_VERSION` exists as a single version word on the public boundary** (`entry.rs:22-27`), which is the
  right instinct: a wire-shape change should be a version bump the host can refuse to load against, not a
  silent reinterpretation. See Finding 6 for what is still missing to make this actually work.
- **The crate's own no-alloc / bare-primitive discipline is followed correctly at the boundary.** Every bare
  `usize` / `*const u8` / `u32` in `entry.rs`, `sink.rs`, `wire/serialize.rs` carries a `lint:allow` with a
  reason tying it to the FFI boundary and a tracked task (#207), which is the documented exception this
  workspace's `no-bare-primitives.md` names, applied correctly. `encode.rs`'s internal walk stays on `arvo`
  types (`Bool`, `Int<64, Hot>`, `USize`, `Maybe`) throughout; only the wire-writing leaf (`wire/serialize.rs`)
  touches raw bytes, which is the correct place for that boundary to live.
- **Zero `unsafe` blocks anywhere in the reviewed crates.** Confirmed by grep across
  `vehje-runtime-abi/src/**` and `vehje-runtime-driver/src/**`: the only `unsafe` token in either crate is the
  `unsafe extern "C" fn` in the `BatchedColumnEntry` type alias (`entry.rs:56`), a function-pointer type, not a
  function body. Nothing in the shipped code performs a raw pointer-to-slice reinterpretation, a transmute, or
  a `from_raw_parts` call. This bounds the current soundness exposure precisely: whatever is wrong with the
  value-arena's layout (Finding 1) is a landmine for the code that has not been written yet, not a live unsound
  path today.

## Findings

### 1. `ValueNode` and every type it composes lack `#[repr(C)]`, which breaks the crate's own zero-copy design claim

The gap: `ValueTag` (`value.rs:49-65`), `ValueList` (`value.rs:69-75`), `BlobSpan` (`value.rs:84-90`), `ValueNode`
(`value.rs:102-112`), and `Region` (`value.rs:118-124`) all carry only `#[derive(Copy, Clone, Eq, PartialEq,
Debug)]`. None carries `#[repr(C)]`, and `ValueTag` (a seven-variant field-less enum) carries no `#[repr(u8)]` or
equivalent discriminant repr either. Rust's default struct and enum representations are explicitly unspecified:
field order, padding, and enum discriminant width are free for the compiler to choose and are not guaranteed
stable even between two builds of the same source with the same rustc.

Why it blocks: the crate's own design language says the wire form and the in-process form are meant to be the
same bytes. `sink.rs:1-9` states "the backing memory is host-lent up front, so the runtime holds no output
memory after a call, which is what makes the wire form equal to the in-process form"; the DEEPDIVE repeats it
verbatim (`DEEPDIVE_VALUE_TRANSPORT.md.tmpl:40-42`). `value.rs:126-131` says the same about `ValueArena` itself:
"over host-lent backing... the wire form equals the in-process form." Read plainly, that is a zero-copy design:
the Zig runtime writes `ValueNode` records directly into host-lent memory, and the Rust host wraps that same
memory as `&[ValueNode]` with no decode pass in between. That only works if `ValueNode`'s Rust layout is the
same layout the Zig side writes, which requires a `#[repr(C)]` (or an explicit, documented byte layout the Zig
side is told to match, the way `wire/serialize.rs`'s doc comment spells out the tier-0 residual's seven-word
records). Today there is no such contract for `ValueNode`. Two rustc builds of this same crate, or a
codegen-flag change, are permitted by the language to lay out `ValueNode`'s four fields in a different order or
width, silently, with no compile error, because nothing pins the layout.

Is it a defect or a deferred mechanism: **a defect in what has landed**, not a deferred mechanism. The value
side's `BatchColumn` sibling in `entry.rs` got the repr right; the value-arena types did not, and the fix is
cheap and belongs now, while these types are still being defined, not after the reinterpretation code that
depends on them gets written against an unstable layout.

Concrete fix: add `#[repr(C)]` to `ValueList`, `BlobSpan`, `ValueNode`, `Region`, and `#[repr(u8)]` (or
`#[repr(u32)]`, matching whatever width the eventual wire encoding of the tag picks) to `ValueTag`. Follow with
a `sink.rs:74`-style layout assertion (`size_of::<ValueNode>()`, field offsets via `core::mem::offset_of!`) so a
future layout-affecting change to any of these types fails a test instead of silently reinterpreting bytes
wrong. The same is owed to the residual side's `Block` / `Function` / `NodeRange` / `SuccRange` / `Signature`
(`wire/residual.rs:44-118`) if those are ever meant to be read directly rather than always going through the
tier-0 encoder's own word-by-word writer, which is the one place in the crate today that does not rely on
struct repr (it writes fields individually as `u32` words, so it is unaffected by this finding).

### 2. The input record layout crossing at `BatchColumn.records` is undocumented anywhere in shipped source

The gap: `entry.rs:42-43` documents `records: *const u8` as "a pointer to the column of W packed input
records" and `width: usize` as "the column width W". Nothing in `vehje-runtime-abi` or `vehje-runtime-driver`
states what one record's byte layout is: its size, its field order, its encoding of the values a residual's
entry parameters bind to. A grep across the crate finds no `InputRecord`-shaped type, no record-size constant,
no per-record accessor. The concept is discussed in several `mock/research/` memos (the runtime-value-transfer
study, the bench-arc synthesis) but none of that has landed as a type or a documented byte contract in the
crate under review.

Why it blocks: a consumer runtime cannot implement `BatchedColumnEntry` without knowing how to read `records`.
"Packed" states a property (no per-record padding to a fixed stride, presumably) without stating the property
that actually matters: what a record contains and in what order. This is not a matter of taste; it is the one
piece of the entry contract a runtime author needs before writing a single line against this ABI.

Is it a defect or a deferred mechanism: closer to a deferred mechanism than Finding 1, since the record schema
plausibly depends on the residual's own signature (a function's argument list, `wire/residual.rs:114-117`) and
so may not be nameable as a fixed type until that machinery is further along. But it is undocumented as
deferred: there is no FIXME, no BACKLOG entry, no SHAME entry naming "the input record layout" the way the
crate is careful to FIXME the export body, the stencil section, and the generational-reference flag. An
undocumented gap in a `pub` field's doc comment reads as complete when it is not.

Concrete fix: either land a `RecordLayout` or `InputSchema` type deriving the per-record byte layout from a
function's argument signature, or, at minimum, add a FIXME at `entry.rs:42-43` naming what is missing and
which future landing closes it, matching the discipline the rest of the file already uses (`entry.rs:58-64`).

### 3. `Reader::validate` is a no-op stub with no type-level enforcement that it runs before an untrusted read

The gap: `vehje-runtime-driver/src/lib.rs:69-71`, `Reader::validate` unconditionally returns `Outcome::Ok(())`.
Meanwhile `ValueArena::get` and `ValueArena::children` (`value.rs:169-176`) index directly (`self.nodes[at.index().0]`,
`&self.pool[l.start.0..l.start.0 + l.len.0]`) with no bounds check beyond Rust's own slice-indexing panic. There
is nothing in either type that prevents a caller from calling `Reader::value()` and reading nodes without ever
calling `Reader::validate()` first; the two methods are independent, and the type system does not distinguish
"a `Reader` whose arena has been validated" from "a `Reader` over a raw, possibly-adversarial arena."

Why it costs: the DESIGN doc is explicit that this split is intentional for the in-process trusted path ("the
in-process path may skip and the untrusted path must run it", `DESIGN.md.tmpl:19-20` and the `validate` doc
comment). That is a legitimate design choice; a purely in-process call from the same compiler process does not
need to re-validate its own output. But `VehjeSink`'s own doc comment states the out-of-process form is "the
executable's stdout pipe" (`sink.rs:5-6`), meaning this ABI is explicitly meant to also carry values from a
context where the producer is not trusted. Nothing in the types tells a caller which situation they are in;
the discipline "call validate() first when the value came from an untrusted process" is enforced by nobody
except the reader of the doc comment. A caller who reads a malformed `ValueRef` (out of range, or a `ValueList`
whose `start + len` overflows the pool) gets a Rust panic, not a graceful `Outcome::Isnt`, on the very API that
advertises itself as "the safe, bounds-checked reader."

Is it a defect or a deferred mechanism: the no-op body is explicitly FIXME'd (`vehje-runtime-driver/src/lib.rs:66-68`)
and matches the "M-level accepts a well-formed arena" framing, so the missing validation logic itself is a
deferred mechanism, honestly marked. What is not deferred, and is a live gap today, is that nothing in the type
system distinguishes a validated reader from an unvalidated one, so even once `validate` is implemented, an API
consumer can still skip it and call `get`/`children` directly on untrusted input. That absence of a typestate
is a defect the crate can fix now, independent of when `validate`'s body lands.

Concrete fix: split `Reader` into an unvalidated wrapper and a `ValidatedReader` (or a `Validated<Reader<'a>>`)
that only `Reader::validate` can produce, and move `get`/`children`-shaped access onto the validated type only.
This is exactly the "illegal states unrepresentable" move this workspace's own type-system discipline asks for
(`harness-the-type-system.md`): "has this arena been checked" is exactly the kind of fact that belongs in the
type, not in a doc comment a caller has to remember to honor.

### 4. `ValueArena::get`/`children` panic rather than error on an out-of-range reference, even after `validate` exists

The gap: `value.rs:169-176`. `get` indexes `self.nodes[at.index().0]`; `children` slices `self.pool[l.start.0
.. l.start.0 + l.len.0]`. Both are ordinary safe Rust indexing, so an out-of-range `ValueRef` or `ValueList`
does not produce memory unsafety (Rust's bounds check catches it), but it does produce an unrecoverable panic
rather than a typed error the caller can turn into a `DriverError`.

Why it costs: once `Reader::validate` is implemented and actually walks the arena (Finding 3's deferred half),
these two methods presumably become safe to call unconditionally after validation succeeds, so this finding is
lower-severity than it looks in isolation, contingent on validate genuinely rejecting every malformed shape
`get`/`children` could panic on (including the arithmetic overflow in `l.start.0 + l.len.0`, which panics in
debug builds and silently wraps in release, a second latent hazard independent of the OOB index itself).

Is it a defect or a deferred mechanism: a defect in the interface, not yet exercised because nothing calls it
on untrusted data today. Worth fixing at the same time as Finding 3's typestate, since a `ValidatedReader` only
needs to guarantee `get`/`children` never panic if the validation pass itself is airtight against every field
these two methods touch, index bounds and the `start + len` overflow both.

Concrete fix: either keep `get`/`children` panicking but make them accessible only through the post-validation
typestate (so a panic there is genuinely unreachable, a documented invariant, not a live hazard), or give them
`Maybe<ValueNode>` / `Maybe<&[ValueRef]>` return types for defense in depth regardless of the typestate. The
former is cheaper and matches the crate's existing "typed structural decode... is then linear and acyclic by
construction" framing (`value.rs:8-9`); the latter is the belt-and-suspenders choice.

### 5. `DiagnosticsSchema` and the block/function tables are declared but the encoder does not populate them

The gap: `wire/residual.rs:213-216` (`DiagnosticsSchema` doc comment) and `wire/residual.rs:11-13` (module doc)
both state plainly that the block table, function table, and diagnostics arrays are schema-now,
population-later; `FlatArenaEncoder` writes only the header, node records, pool, and blob
(`wire/serialize.rs:113-122`). A `Residual` value assembled from what `serialize` produces today would have to
either fabricate empty `BlockTable`/`FunctionTable`/`DiagnosticsSchema` values or not exist as a `Residual` at
all; nothing in the reviewed crates currently constructs a `Residual<'img>` from a serialized byte buffer (no
decode path exists on the Rust side either, matching the value-arena side's absence noted in Finding 1).

Is it a defect or a deferred mechanism: **deferred and explicitly marked**, matching BACKLOG.md.tmpl's "the
stencil section producer" and "the per-reference generational flag" entries in spirit even though this specific
gap is not separately itemized there. The module doc comment is honest about the state; this is the crate
behaving exactly as `strict-by-design-quality-pressure.md` asks (a documented gap, not a stopgap that fakes
population).

Concrete fix: none owed now beyond keeping the BACKLOG entry current; flagged here only so the readiness
assessment below counts it correctly as "known missing," not "silently missing."

### 6. `ABI_VERSION` is a Rust constant with no exported symbol and no load-time check, so nothing enforces it yet

The gap: `ABI_VERSION` (`entry.rs:22-27`) is a `pub const`, not a `#[no_mangle]` exported symbol. The crate
builds as an `rlib` today (`Cargo.toml:26`, with a comment explaining the cdylib export is restored at M3).
There is no code anywhere in either reviewed crate that compares a host's expected version against a runtime's
actual version and refuses to load on mismatch; `vehje-runtime-driver/BACKLOG.md.tmpl:16-17` names this
explicitly as deferred ("ABI version check at load time... refuse on mismatch").

Is it a defect or a deferred mechanism: deferred, and honestly tracked in the driver's own BACKLOG. Noted here
only because "ABI versioning" was named explicitly in the review's scope: the version word's existence is
sound groundwork, but versioning as a mechanism (something that actually prevents a mismatched host and runtime
from talking to each other) does not exist yet.

### 7. The two crates' `SHAME.md.tmpl` no-std entries do not match what is actually in source

The gap: `vehje-runtime-abi/SHAME.md.tmpl:5-6` and `vehje-runtime-driver/SHAME.md.tmpl:5-6` both claim the crate
root carries `#![cfg_attr(not(feature = "std"), no_std)]` with `default = ["std"]`. Actual source:
`vehje-runtime-abi/src/lib.rs:21` is a bare `#![no_std]` (no `cfg_attr`), and `vehje-runtime-abi/Cargo.toml:19`
has `default = []` with no `std` feature declared at all. `vehje-runtime-driver/src/lib.rs:13` is likewise a
bare `#![no_std]`, though `vehje-runtime-driver/Cargo.toml:18-24` does have `default = ["std"]` and an (empty,
currently unused by any `cfg(feature = "std")` in source) `std` feature, so the driver's SHAME entry is half
right and half stale.

Why it matters: not a soundness defect in the ABI itself, a `cl-claim-sketch-discipline.md`-shaped drift, a
Tier-2 design artefact making a claim about source state that source does not currently back. A reader
consulting SHAME to understand why `no-std` shows as a known gap gets a description of a mechanism (the
`cfg_attr` std-feature toggle) that is not what is actually gating anything today; the crates are simply
`#![no_std]` outright, unconditionally, on both core paths, which is stricter than what SHAME describes, not
looser.

Concrete fix: update both SHAME entries to match current source, or remove them if the described gap no longer
applies (the abi crate in particular looks like it no longer needs a `std`-feature dispensation at all; the
driver's `std` feature exists in `Cargo.toml` but gates nothing in source yet, which is itself worth a one-line
note in the driver's SHAME rather than the current stale description).

## The unsafe boundary specifically

**`#[repr(C)]` layout.** `BatchColumn` and `VehjeSink` get it right: `#[repr(C)]`, pointer/usize fields only,
`VehjeSink` additionally gets a layout-size test. The value-arena's node-level types (`ValueNode`, `ValueList`,
`BlobSpan`, `Region`, `ValueTag`) do not get it at all (Finding 1). This is the one place in the boundary where
the stated design intent (zero-copy, wire form equals in-process form) and the actual type declarations
disagree, and it is the most consequential finding in this review because it sits under the crate's central
design claim, not under a corner of it.

**The `unsafe extern "C"` entry contract.** The signature (`entry.rs:56`) and its surrounding doc comments state
the shape of the call (a pointer to `BatchColumn`, results stream through the sink, no return value, fallibility
lives in the value not the return) clearly and correctly as prose. What is missing is a formal `# Safety`
section on the type alias's doc comment enumerating the caller's pointer, lifetime, and aliasing obligations in
the form Rust's own convention expects for an `unsafe fn`: that `residual` must point to at least `residual_len`
readable bytes for the call's duration; that `records` must point to a valid encoding of exactly `width` records
(circularly, this cannot be stated precisely until Finding 2's record layout exists); that `sink` must be
non-null, valid for the call's duration, and that its two function pointers must themselves uphold their own
documented contracts; and whether the runtime may retain any of these pointers past the call returning (the
`VehjeSink` doc comment does state "the runtime never retains it past the call" for the sink specifically,
`sink.rs:39-40`, which is exactly the right shape of statement and should be mirrored for `residual` and
`records`). None of this is stated as UB-triggering conditions today; it is implied by careful prose but not
codified the way an `unsafe fn`'s safety contract is meant to be. Since the actual export does not exist yet
(honestly FIXME'd, `entry.rs:58-64`), this is fixable cheaply, before any caller can get it wrong, by adding the
`# Safety` section now, alongside the export, rather than after the fact.

**The value arena and its safe `Reader`.** Two separate questions, and they get different answers. First: is
there a live path to an out-of-bounds or type-confused *memory* read today? No. Zero `unsafe` code exists in
either crate that would perform such a read; the entire cross-language, cross-process materialisation of a
`ValueArena` from raw bytes is unwritten. Second: is the *design*, as declared in the current types, sound
enough to support writing that materialisation code safely when it lands? No, not yet, because of Finding 1
(no repr(C) on the node types) and Finding 3 (no typestate forcing validation before read on the untrusted
path). Both are fixable now, while the types are still small and before any unsafe reinterpretation code is
written against them; fixing them after that code lands would mean auditing and likely rewriting whatever
unsafe transmute or `from_raw_parts` call gets built on top of the current, unstable layout.

## Readiness for a consumer runtime

Not ready today, and the minimal path to ready is short and concrete:

A Zig (or any other) runtime author could, right now, hand-write a `BatchedColumnEntry`-shaped function and a
`VehjeSink`-shaped pair of callbacks correctly, because those two types are sound `#[repr(C)]` today. They
could not write anything useful inside that function, because the residual it should parse (the tier-0 flat
arena format) is documented in prose (`encode.rs:9-17`, `wire/serialize.rs:9-16`) precisely enough to
hand-implement a decoder against (magic, version, tier, three counts, root, then fixed seven-word node records,
then the pool, then the blob, all little-endian), which is good news: the wire format itself, as far as it
goes, is specified tightly enough to build a compatible reader today, independent of anything Rust-side. What
they could not do is produce a conforming *value* back to the host, because the value-arena's byte layout is
not specified anywhere (Finding 1's gap: no repr(C), so there is no stable target layout to write bytes
into), and they could not interpret the input records at all (Finding 2).

Minimal set to land before "build against this" is a fair statement to make to a runtime author, in priority
order:

1. `#[repr(C)]` (or an explicit documented byte layout matching what the tier-0 encoder already does for the
   residual) on every value-arena node type, plus a layout test. Without this, nothing on the produced-value
   side of the ABI has a stable target, which blocks a runtime from ever writing a conforming value back.
2. A documented input-record byte layout at `BatchColumn.records`. Without this, nothing can call in.
3. The `# Safety` contract on `BatchedColumnEntry` and the actual `#[no_mangle]` export with its panic guard
   (already tracked, `entry.rs:58-64`).
4. `Reader::validate`'s real body, and the typestate split from Finding 3, before any out-of-process (stdout
   pipe) consumer is treated as safe to read from without a separate manual audit each time.

Everything else reviewed here (the residual's block/function/diagnostics tables, the bytecode tier, the stencil
section, the generational-reference flag, the ABI version load-time check) is legitimately later work and
already tracked as such; none of it blocks a first conforming runtime implementation the way the four items
above do.

## Open questions

Whether the value-arena's node types should get `#[repr(C)]` directly (matching `BatchColumn`'s approach,
simplest, but constrains field order and padding to whatever C-compatible layout the derive produces) or
whether they should follow the residual's own precedent and go through an explicit word-by-word wire writer
like `FlatArenaEncoder` (more code, but keeps the in-memory Rust type free to have any layout and makes the
wire format a documented, versioned contract independent of the Rust struct's shape). The tradeoff is
zero-copy simplicity (repr(C), the design's stated preference per `value.rs:126-131` and the DEEPDIVE) against
the residual side's own more defensive pattern of never letting the wire byte layout be implicitly whatever a
struct derive happens to produce.

Whether `Reader`'s validated/unvalidated split (Finding 3) should be a typestate on `Reader` itself or should
instead be pushed onto `ValueArena` (so `ValueArena<'buf>` stays the raw, host-lent view and a new
`ValidatedArena<'buf>` wraps it), given that `ValueArena` is the type that actually owns `get`/`children`
today, not `Reader`.

Whether the input-record layout (Finding 2) is meant to be a fixed schema derivable purely from a residual's
function signature (in which case it plausibly belongs as a type in `wire/residual.rs` alongside `Signature`),
or whether it needs richer per-position value-kind tags that do not exist yet (the same gap `Signature`'s own
doc comment names at `wire/residual.rs:87-88`), which would mean the record layout cannot be finalized before
that value-kind vocabulary lands.

# Findings: one operation definition, four discharge sites

**Date:** 2026-07-26
**Outcome: WORKS.**
**Toolchain:** Zig 0.16.0, rustc release mode, aarch64-apple-darwin.

## What was tested

The admissibility test the second independent read derived
(`mock/research/202607260700_semantics-projection-second-read.md`): a
representation for family-operation semantics must be evaluable by
rustc-compiled Rust and by Zig, at comptime and at runtime, or the meaning gets
written twice. A closed five-opcode stack encoding was driven through all four
sites and the results compared.

## Result

All four agree, and the agreement is byte-identical rather than asserted.

**Bundler grade, Rust compile time.** `const FOLDED_DIFFSQ: i64 = eval(&OP_DIFFSQ, &[5, 3]);`
plus `const _: () = assert!(FOLDED_DIFFSQ == 16);` compile. A `const` that failed
to fold is a compile error, not a slow path, so compilation is the proof. The
same `const fn` body serves the runtime case, so the const-fold at
`vehje-lower/src/lib.rs:126-128` would not need its own separately-authored
evaluator.

**Runtime grade, Rust.** The corpus output in `rust_out.txt`.

**LanguageAuthor grade, Zig comptime.** The corpus output in `zig_out.txt`.
`diff rust_out.txt zig_out.txt` is empty.

**Runtime grade, Zig interpretation.** `proj.zig`'s first test drives all four
operations over the corpus and asserts `evalRuntime` equals `evalComptime` at
every point. Both tests pass.

## The specialisation is real, and was checked rather than trusted

The Deegen precedent in the design-space exploration warns against believing an
optimiser specialised an interpretation loop away without looking. It did not
need believing. Across an `export` boundary, so the optimiser cannot see the call
site:

```
0000000000000f4 <_proj.diffsq_specialised>:
  stp  x29, x30, [sp, #-0x10]!
  mov  x29, sp
  sub  x8, x0, x1        ; a - b
  add  x9, x1, x0        ; a + b
  mul  x0, x8, x9        ; (a-b)*(a+b)
  ldp  x29, x30, [sp], #0x10
  ret
```

Three arithmetic instructions. The stack array, the program bytes, the `pc`
walk, and the dispatch switch are all gone. The interpreted arm compiled from
the same bytes occupies 244 bytes, symbol `_diffsq_interpreted` at offset 0
running to `_diffsq_specialised` at 0xf4.

The operation chosen for this, `(a - b) * (a + b)`, is deliberately one an
optimiser could rewrite to `a*a - b*b`. Neither side did, and both produce the
same values, so the comparison is not measuring a reassociation that happened to
cancel.

## What this does and does not establish

**Establishes** that a closed primitive vocabulary carried as data satisfies the
admissibility test, on the pinned toolchain, for an operation of more than one
opcode. Both expert reads agreed independently, each grounded in quoted canon,
that such a vocabulary fits the canon; this is that agreement made operational
rather than argued.

**Does not establish** that five opcodes suffice for a real family. Coverage is a
separate question and the sketch deliberately avoids confounding it with
admissibility: a vocabulary too small to express a real family still answers the
admissibility question, and a large one would obscure it.

**Does not establish** anything about relative cost. There is no timer here, by
design. Per `bench-in-bench-harness-never-sketches.md` a feasibility question
with a WORKS/FAILS outcome is a sketch; the specialised-versus-interpreted timing
comparison is a real fork and belongs in a harness cell under `mock/benches/`
with per-variant isolation and committed CSV. The 7-instruction versus 244-byte
gap says the cell is worth building; it is not itself a measurement.

**Does not settle** the three items the two reads disagreed on, which are the
lead designer's: whether nominal binding to hand-authored code is foreclosed as
the home of semantics or merely costly, whether the declarative encoding is a
coequal default or must earn its place against Frontier A, and whether the
vocabulary is framework-level or consumer-opt-in. This sketch is compatible with
every answer to those, which is why it was built first.

## What it unblocks

The representation question for `Signature::Operation` can proceed on a shape
now shown to work at every discharge site, rather than on an argument that it
would. The next steps in order: the coverage question (what vocabulary a real
arithmetic family needs), then `generate` actually reading the signature, then
the harness cell for the cost comparison.

## Reproducing

```
zig test proj.zig                     # both Zig projections agree
zig build-lib proj.zig -O ReleaseFast # then objdump -d libproj.a
rustc -O proj.rs -o proj_rs && ./proj_rs > rust_out.txt
zig build-exe emit.zig -O ReleaseFast && ./emit 2> zig_out.txt
diff rust_out.txt zig_out.txt         # empty
```

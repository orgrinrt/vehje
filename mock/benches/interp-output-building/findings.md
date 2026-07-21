# Interpolation output-building throughput: the templating text-generation hot path

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 5M template instantiations of `"Hi {name}, {count} items worth {total}.\n"`
(33 bytes out each), 5-run best. Zig 0.16.0, aarch64. Artifacts: `output.zig`, `output.csv`.
**Settles:** the output-building path for the `Interp` core form, the text-generation hot path that op's emphasized
biggest consumers (doc DSLs, config, typst/scribble-nature templating) run all day. Sizes the lowering-strategy
fork and the reserve/commit sink API.

## Why this probe

Templating consumers produce output text: literal runs interleaved with formatted computed values. The `Interp`
core form lowers to a sequence of "append literal span" and "append formatted value" into the reserve/commit
output sink. Because producing output IS what these consumers do, the throughput and the lowering shape are
load-bearing, and neither was benched (cfg-interp and interp-dispatch measure program EXECUTION, not text
output-building). Three strategies:

- **A, format-to-temp-then-copy:** format each value into a scratch buffer, then memcpy it into the output. The
  naive shape a generic formatter takes.
- **B, format-in-place:** format each value directly at the output cursor, no scratch, no double copy.
- **C, span-list interpreted:** the template is a data-driven step list `[LIT, SVAL, LIT, IVAL, LIT, IVAL, LIT]`
  walked at runtime, literals memcpy'd wholesale, values formatted in place. This is the lowering the interpreter
  tier naturally uses (the template is data the generic interpreter walks).

## Results

5M instantiations, 33 output bytes each:

| strategy | ns/template | output MB/s |
|---|---|---|
| A, format-to-temp + copy | 20.58 | 1538 |
| B, format-in-place | **13.54** | **2337** |
| C, span-list interpreted | 30.45 | 1039 |

## The finding: two decisions, and output-building is never a bottleneck under any of them

**1. The sink API must expose the cursor for in-place value formatting (B beats A by 1.5x).** Formatting a value
into a scratch buffer and then copying it into the output (A) is 1.5x slower than formatting directly at the
output cursor (B), purely from the avoided scratch write and second copy. So the reserve/commit output sink should
expose the reserved span's cursor and let values format directly into it, not offer only a format-to-string-then-
append interface. This is a concrete sink API design point: give the value formatters the destination cursor.

**2. Specialized straight-line (B) is 2.2x faster than the interpreted span-list (C), but the span-list is still
~1 GB/s, so the interpreter tier can use it freely.** Walking the data-driven step list (C) costs a branch and an
indirect per step, making it 2.2x slower than fully-inlined straight-line appends (B), the same
interpreter-versus-specialized gap seen across the tiers. But C still produces output at 1039 MB/s, which is
enormous for text: even a 10 MB document is under 10 ms, and real templated documents are kilobytes, produced in
microseconds. So output-building is not a bottleneck under ANY strategy. The interpreter tier uses the flexible
data-driven span-list (C) without concern; a compile-side specialization of each `Interp` into straight-line
appends (B) is available for a consumer that genuinely produces huge output volumes, but it is an optional
optimization, not a necessity.

This mirrors and reinforces the interp-vs-native ceiling finding: the flexible interpreted form is already far
faster than any realistic need, and specialization buys a fixed ~2x that only matters at extreme volume. For the
templating consumers, the design should ship the data-driven span-list lowering as the default (it composes with
the residual being data, not code) and reserve specialization for the rare high-volume case.

## Design impact

- Reserve/commit output sink API: expose the reserved-span cursor so value formatters write in place. Do not
  route values through a scratch-buffer-then-append path (1.5x slower for nothing).
- `Interp` lowering: the default is the data-driven span-list (literal spans as constants in the residual, value
  holes as format steps), walked by the interpreter tier at ~1 GB/s, which is far beyond any realistic templating
  need. Compile-side specialization into straight-line appends is an optional ~2x for extreme-volume consumers.
- Literal runs are static byte spans in the residual (constants), memcpy'd wholesale; only the value holes are
  computed. This keeps the residual compact (literals are shared constant data) and the output path a sequence of
  memcpy + format-in-place.

## Boundary

One representative template (three value holes, four literal runs, 33 bytes out). A template that is almost all
literal text (a large document with few holes) is even faster (mostly wholesale memcpy, approaching memory
bandwidth); a template dominated by expensive value formatting (floats, nested structures) shifts cost onto the
formatters, where format-in-place's advantage over format-to-temp grows. The integer formatter here is a minimal
hand-rolled decimal writer (to compare strategies, not formatter implementations); a full formatter with padding
and locale would be slower per value but does not change the strategy ranking.

## Artifacts
- `output.zig` (the three output-building strategies over a representative template), `output.csv`.

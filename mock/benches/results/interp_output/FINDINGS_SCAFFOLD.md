# Interp output-building (carrier-harness port): the templating text hot path

Scaffold for `interp_output`. Numbers filled in after the harness run.

## What this replaces

The standalone Zig bench (`interp-output-building/output.zig`, hand-timed, own
CSV) and the first-pass harness port (`hx_output`), which PORT_NOTES flagged as
"within noise at harness sizes; the original 1.5x needed longer values / higher
volume than a per-call byte budget exercises." This port fixes the volume
defect: ITERS=8 passes over the N input bytes, each byte driving one template
instantiation, and the produced text is folded into the accumulator so the full
output is materialized and the result tracks the FFI input. All three
strategies produce byte-identical text for the same data index, so the folded
8-byte output matches and the harness cross-validates on the produced text
(verified: temp, in-place, and span-list agree at every checked size; output
varies with the input).

## What it measures

Instantiate "Hi {name}, {count} items worth {total}.\n" (about 25 to 36 bytes
out) per input byte, with the template data index derived from the FFI input.
Three lowering strategies:

- format-to-temp+copy (A): format each value into a scratch buffer, then memcpy
  it into the output. The naive generic-formatter shape.
- format-in-place (B, baseline): format each value directly at the output
  cursor, no scratch, no second copy.
- span-list (C): the interpreter-tier lowering. The template is a data-driven
  step list walked at runtime; literals memcpy'd wholesale, values formatted in
  place.

The integer writer is identical across strategies, so it is not what is
compared. Report ns/template and derive MB/s.

## Result

| strategy | ns/template | output MB/s |
|---|---|---|
| A format-to-temp+copy | TBD | TBD |
| B format-in-place (base) | TBD | TBD |
| C span-list | TBD | TBD |

MB/s = (bytes-per-template * templates-per-call) / per-call-time. Bytes per
template is recorded from the produced text; templates-per-call is N * ITERS.

Expected direction (from the prior Zig run): B beats A by roughly 1.5x, purely
from the avoided scratch write and second copy; C is the slowest of the three
(a branch and an indirect per step) but still around 1 GB/s, far beyond any
realistic templating need.

## What it says for the design

- The reserve/commit sink must expose the reserved-span cursor so value
  formatters write in place (B over A). Do not route values through a
  scratch-buffer-then-append path.
- The Interp default lowering is the data-driven span-list (literal spans as
  constants in the residual, value holes as format steps), walked by the
  interpreter tier at roughly 1 GB/s. Output-building is not a bottleneck under
  any strategy. Compile-side specialization into straight-line appends (B) is an
  optional roughly 2x for extreme-volume consumers.

## Cost-model sanity line (to complete at run)

At n=16384, ITERS=8, that is 131072 template instantiations per call, about
3.5 to 4.5 MB of output. Confirm ns/template lands in the low-tens-of-ns range
a handful of memcpys plus two integer formats should cost, and that MB/s is in
the low-GB/s range; confirm near-linear scaling with N.

## Caveats

One representative template (three value holes, four literal runs). A
literal-heavy template is faster (mostly wholesale memcpy); a
value-formatting-heavy template shifts cost onto the formatters, widening B's
lead over A. The per-byte output fold is equal across strategies, so it does not
bias the ranking, but it does add a constant that compresses the absolute
spread; the relative ranking is what the design reads.

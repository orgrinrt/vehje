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

## Measured results

Ratio to baseline (interp_out_inplace), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | interp_out_inplace (base) | interp_out_spanlist (ratio) | interp_out_temp (ratio) |
|---|---|---|---|
| 64 | 14882 ns | 2.77x | 1.12x |
| 256 | 59111 ns | 2.50x | 1.10x |
| 1024 | 221055 ns | 2.00x | 1.18x |
| 4096 | 808402 ns | 1.06x | 1.25x |
| 16384 | 3269516 ns | 1.04x | 1.25x |

## Cost-model sanity line

At n=16384, the baseline (interp_out_inplace) median is 3269516 ns for inplace: N output records written once each. Treating n as the work-item count, that is 199.56 ns/item, about 638.6 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

In-place output building wins. A span-list is 2.0x to 2.8x slower at small n (allocation and indirection dominate) but converges to ~1.05x at large n (amortized); a temp-then-copy is a steady ~1.1x to 1.25x. Build interpreter output in place.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).

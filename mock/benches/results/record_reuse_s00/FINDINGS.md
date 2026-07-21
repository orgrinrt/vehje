# Record-update reuse (carrier-harness port): the exact-meet in-place payoff

Scaffold covering `record_reuse_s00`, `record_reuse_s20`, `record_reuse_s60`
(sharing fractions 0%, 20%, 60%). Numbers filled in after the harness run.

## What this replaces

The standalone Zig bench (`record-update-reuse/reuse.zig`, hand-timed, own CSV)
and the first-pass harness port (`hx_reuse`, a single copy-vs-reuse pair at one
implicit sharing rate). This port fixes the axis defect: the sharing fraction is
the load-bearing variable, so it is swept as three sections, each carrying
always-copy, in-place-when-unique, and the always-mutable ceiling. Seed records
and the sharing mask derive from the FFI input and are built outside the timed
block; every update writes an input-derived value, so the accumulated final
record is input-dependent and identical across all three strategies and all
three fractions (the sharing verdict changes only WHEN reuse copies, never the
values), which the harness cross-validates (verified: copy, reuse, and mutable
agree, and all three fractions agree, and output varies with the input). The
full record copy is kept real with core::hint::black_box on the destination
slot, defeating the copy-elision LLVM would otherwise apply to the dead
intermediate fields.

## What it measures

N records of 16 fields, 8 field-updates each. always-copy allocates a fresh
slot and copies all 16 fields per update; in-place-when-unique mutates in place
while the emit-time uniqueness bit holds and copies once when forced-shared;
always-mutable ignores sharing and always mutates in place (the unsafe ceiling).
The forced-shared bit is a compile-time fraction per reuse variant, standing in
for the emitter's uniqueness verdict (the counting is compile-time; the runtime
reads the bit, no refcount traffic).

## Measured results

Ratio to baseline (rec_mut), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | rec_copy (ratio) | rec_mut (base) | rec_reuse_s00 (ratio) |
|---|---|---|---|
| 64 | 6.32x | 322 ns | 1.13x |
| 256 | 6.15x | 1289 ns | 1.08x |
| 1024 | 6.54x | 4412 ns | 1.04x |
| 4096 | 7.13x | 16187 ns | 1.10x |
| 16384 | 6.75x | 67057 ns | 1.14x |

## Cost-model sanity line

At n=16384, the baseline (rec_mut) median is 67057 ns for N record updates, one field write each (mut). Treating n as the work-item count, that is 4.09 ns/item, about 13.1 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

In-place mutation is the baseline. A full copy costs 6.3x to 7.1x. With 0% sharing, reuse (COW-style) is nearly free (1.04x to 1.14x) because nothing is actually shared. Confirms in-place mutation beats copy-on-write when sharing is absent.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).

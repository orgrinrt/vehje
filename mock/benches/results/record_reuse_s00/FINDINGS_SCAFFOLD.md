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

## Result (ns/update, normalised against the in-place-when-unique baseline)

| shared | always-copy | reuse (base) | mutable ceiling | reuse speedup |
|---|---|---|---|---|
| 0% | TBD | TBD | TBD | TBD |
| 20% | TBD | TBD | TBD | TBD |
| 60% | TBD | TBD | TBD | TBD |

Expected direction (from the prior Zig run): reuse is roughly 9x faster than
copy at 0% sharing (hitting the mutable ceiling exactly, zero copies), roughly
3x at 20%, roughly 1.5x at 60%, and never loses (the worst case degenerates to
always-copy). copy is flat across fractions (it ignores sharing); mutable is the
flat ceiling.

## What it says for the design

- Build the exact-meet reuse analysis: it is a load-bearing enabler, not an
  optional optimization. Functional record updates lower to in-place mutation
  wherever emit-time uniqueness holds, giving near-ceiling throughput for the
  mostly-unique common case (locally-built templating/config/script records,
  the 0-to-20% regime) and a strict win otherwise.
- The runtime reads the emitted uniqueness bit and branches; it does no
  reference counting. One flag per update site in the residual.

## Cost-model sanity line (to complete at run)

At n=16384, 8 updates each, that is 131072 updates per call. Divide per-call
time by that for ns/update. Confirm the copy path lands near a 128-byte memcpy
plus a field write (a few ns) and the in-place path near a single field write
(sub-ns to low-ns); confirm copy is roughly flat across the three fractions and
reuse rises with the sharing fraction toward the copy line.

## Caveats

Fixed-width 16-field records; wider or nested records make copy-on-write more
expensive, widening reuse's lead, so 16 fields is a conservative midpoint. The
sharing fraction is workload-dependent; locally-built records are overwhelmingly
unique (0 to 20%). copy and mutable are input-and-fraction-independent, so one
binary of each is shared across the three sections. The emit-time uniqueness
analysis itself is a compile-side cost not benched here; this sizes the runtime
payoff of its verdict.

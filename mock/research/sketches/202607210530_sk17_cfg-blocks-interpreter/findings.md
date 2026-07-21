# SK17 findings: the CFG-of-blocks interpreter (Cluster C control-flow fix)

**Date:** 2026-07-21
**Outcome:** WORKS. Toolchain: Zig 0.16.0. Artifact: `cfg.zig`.
**Settles:** the Cluster C interpreter control-flow gap (Cluster B's naive `for n in 0..len` scan handles only
straight-line dataflow; SK17 shows the CFG-of-straight-line-blocks fix runs branches, loops, and calls).

## What was demonstrated

A register-machine CFG interpreter that computes `double(sum(1..5)) = 30`, exercising every control form Cluster
C named as missing from the naive scan:

- **Forward linear scan WITHIN a block** is preserved (`runBlockScan` streams a block's instructions in order,
  the cache-friendly fast path).
- **Control flow ONLY at the block terminator:** fallthrough (next block), conditional branch (takes ONE arm, so
  the effectful-both-arms bug is gone), loop (a back-edge to the header block), call (pushes a frame and switches
  to the callee entry), ret (pops and resumes the caller).
- **Depth-cap-bounded frame stack:** calls use a fixed-capacity `[DEPTH_CAP]Frame` stack (asserted `sp <
  DEPTH_CAP`), the same finite depth-cap quantity the design already carries, so calls add no unbounded state and
  no heap.

The result is correct (loop sums 1..5 = 15, the call doubles it to 30), confirming the block-CFG model handles
the control flow the linear scan could not, while keeping the linear scan where it applies.

## Design impact

The Cluster C fix is buildable: the interpreter is a CFG of straight-line blocks, linear-scan within, control at
terminators, with a depth-cap-bounded frame stack for calls. This is standard structured dispatch over the Core
control forms (`If`/`Match`/`Iter`/`Apply`) that already exist, confirmed working. The wire format owes the block
table + function table (node ranges, terminator kinds, successors, block-arg signatures) noted in the solution
set; this sketch validates the execution model they encode. Combined with BN1 (switch dispatch wins), the block's
inner scan uses switch dispatch.

## Artifacts
- `cfg.zig` (the CFG interpreter + the sum-loop-plus-call program).

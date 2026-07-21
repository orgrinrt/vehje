# Interner-merge cost (the serial tail of the parallel compile)

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 2000 mods x 256 strings (512K local strings, ~78% drawn from a shared
4096-word vocabulary), 5-run best, single-threaded open-addressing merge. Zig 0.16.0, aarch64. Artifact:
`merge.zig`.
**Settles:** whether the per-mod-local-interner contract (which makes the compile stage parallel) has a cheap or
an expensive re-merge into the single composed string table, i.e. whether the parallel-compile win survives the
serial tail it implies. Closes the follow-on question the compile-stage-parallelism finding opened.

## Why this probe

The compile-stage-parallelism result showed near-linear cold-load scaling, but only because mods intern strings
into per-mod-local tables (no shared interner mutation at the parallel boundary). That contract has a cost it did
not measure: one composed runtime wants one string table, so the parallel compile is followed by a merge that
dedupes the N per-mod tables into a global table and rewrites every mod's local string-ref to a global id. If
that merge is expensive it serialises away the parallel win (Amdahl). This measures it.

## Results

512K local strings across 2000 mods, ~78% overlap (each mod draws 200 of its 256 strings from a shared 4096-word
vocabulary, the realistic shape: mods share keywords, field names, common literals), merged single-threaded into
a global open-addressing table with a per-string local-to-global remap:

- Merge time: **6.19 ms** (82.8 M-strings/s)
- 512K local -> **71.7K distinct** globals (7.1x dedup, confirming the heavy-overlap model)
- The merge is **~34%** of the 18 ms parallel compile for the same 2000 mods

## The finding: the merge is cheap in absolute terms but is a real serial tail; net cold load is still ~4.2x

The merge runs at 83 M-strings/s, so in absolute terms it is cheap (6ms for a 2000-mod stack). But relative to
the parallel compile it is not negligible: it is a serial tail of ~34% of the parallelised work. The honest
cold-load arithmetic is therefore:

- serial compile (1 thread, no merge needed since it interns straight into the global table): ~102 ms
- parallel compile (8 threads, per-mod-local) + serial merge: 18 ms + 6.2 ms = **24.2 ms**, a **4.2x** net win

So the per-mod-local-interner strategy still wins cold load by a large margin (4.2x here), but Amdahl caps it
below the raw 5.6x compile speedup because of the merge tail. This is the correct, non-inflated number to budget
against.

Two design consequences:

- The per-mod-local-interner contract is validated as a net win, so keep it: parallel compile plus merge (24 ms)
  beats serial compile (102 ms) decisively, and the merge is a bounded, predictable, single-pass hash-dedupe.
- The merge is the next parallelisation target IF cold load ever needs to go lower. It is itself parallelisable
  (a concurrent/sharded hash table, or a two-level merge tree over per-thread partial tables) since it is a pure
  dedupe with no ordering dependence. That is a later optimisation, not needed now: at 6ms for 2000 mods the
  merge is well inside any reasonable load budget. Noting it so the Amdahl tail is a known lever, not a surprise.

## Design impact

- Cold-load model, refined: `parallel_compile(cores) + serial_merge`, where the merge is a single-pass
  hash-dedupe at ~80 M-strings/s over the total local-string count. For a 2000-mod stack that is ~24 ms on this
  laptop, not the ~18 ms the raw compile speedup alone would suggest.
- The warm path (mod-stack-load) is unaffected: editing one mod re-interns only that mod's strings and re-merges
  incrementally (its local-to-global remap updates; unchanged mods keep their remap), so warm iteration does not
  pay the full merge.
- The dedup ratio (7.1x here) is workload-dependent; a stack of mods with little shared vocabulary dedupes less
  and produces a larger global table, but the merge throughput (strings/s) is roughly flat because it is bounded
  by hashing and probing, not by the distinct count.

## Boundary

Models independent mods with a shared vocabulary (the common case). It does not model string CONTENT length
(strings are rendered short here); very long string literals would shift the merge toward memory bandwidth on the
hashing pass, lowering strings/s but not changing the shape of the conclusion. Cross-mod symbol resolution (a mod
referencing another mod's exported name) is a separate concern handled after the merge, on the global ids.

## Artifacts
- `merge.zig` (per-mod-local string corpus with tunable overlap + the single-pass open-addressing merge + remap).

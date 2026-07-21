# BN0 findings: the comptime cost cliff (refines 1845/05)

**Date:** 2026-07-21
**Type:** compile-time scaling measurement (the runtime bench harness does not apply; this measures Zig comptime
evaluation time, isolated with `-fno-emit-bin` to exclude final codegen). Zig 0.16.0.
**Settles:** the owed 1845/05 scaling-cost probe, and the justification (and the boundary) for the three-loci
split (content validation to a native `build.zig` step).
**Data:** `cost_cliff.csv` (comptime_ms vs n, three variants). Baseline compile ~350-400ms.

## The three variants (intern's "always bench several variants")

| n | table type-gen (@Enum N fields) | content O(N) fold | content O(N^2) fold |
|---|---|---|---|
| 100 | 703 | 404 | 366 |
| 500 | 812 | 357 | 844 |
| 1000 | (interp) | (interp) | 2473 |
| 2000 | 2165 | 380 | 9730 |
| 8000 | 7762 | 502 | (skipped) |
| 20000 | 19054 | 918 | (blows up) |

## Reading

- **O(N) content folding at comptime is cheap and near-linear.** 20000 records validated/folded in ~918ms
  (~27us/record over baseline). The "folding bundled content at comptime hits a cliff" framing is too strong:
  linear comptime work over content is tractable to tens of thousands of items.
- **The cliff is SUPERLINEARITY, not content-at-comptime.** The O(N^2) fold blows up quadratically (500->844,
  1000->2473, 2000->9730 ms, and >120s by n=4000 under full codegen), regardless of it being "content." Any
  superlinear comptime algorithm blows up.
- **Type generation (`@Enum` with N fields) is ~linear but heavy per-element** (~950us/field at n=20000,
  dominated by `comptimePrint` + type construction). Fine for a realistic language table (tens of families,
  sub-second); expensive only at thousands, which no real family set reaches.

## Design implication (refines 1845/05)

The three-loci split (comptime for type generation, a native `build.zig` step for content validation, native at
the consumer) is justified, but the boundary is sharper than "all content to build.zig":

- Comptime is fine for: type generation from the small language table (tens of families), and modest O(N) content
  validation (up to ~tens of thousands of items, sub-second).
- Move to the native `build.zig` step when: the content validation is SUPERLINEAR (any O(N^2)-ish check, e.g. a
  naive all-pairs conflict check), OR the content volume is large (hundreds of thousands or more items, where
  even linear comptime at ~27us/item reaches seconds-to-minutes while native does it in milliseconds).

So the rule for the design proper: keep type-gen and small/linear content checks at comptime; route
superlinear-or-large content validation to the native build step. The 1845/05 conclusion holds; its boundary is
now measured, not assumed.

## Artifacts
- `cost_cliff.csv`, the templates `tbl_tmpl.zig` / `cnt1_tmpl.zig` (O(N)) / `cnt_tmpl.zig` (O(N^2)).

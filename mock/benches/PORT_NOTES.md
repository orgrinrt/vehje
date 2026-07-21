# Prior benches redone in the harness (port status)

The night's earlier standalone-Zig benches are redone as mockspace-harness benches (Rust cdylib variants,
`gen_port.py`, normalised + procedural highlights). Each is `hx_<name>` in `bench.toml`. This notes which
ported faithfully, which carry a modelling caveat, and which prior "benches" are not harness-shaped.

## Reproduce the original finding (harness-confirmed)

| harness bench | prior finding | harness result |
|---|---|---|
| `hx_dispatch` | BN1: switch dispatch fastest | switch dominates (+475% vs fnptr at n=4096) |
| `hx_iter_fusion` | fusion beats materialized | fused dominates (+371%) |
| `hx_field` | Project: direct-offset fastest | direct dominates (+150% vs hash) |
| `hx_reach` | whole-column OR beats delta semi-naive (shallow) | whole dominates (+31%) |
| `hx_resolve` | flat shadow-stack O(1) beats scope-chain | flat dominates (+1201% vs linear) |
| `hx_recwidth` | BN2: 24B inline beats 16B pool-spill | rec24 dominates (+20% vs rec16) |
| `hx_cfg` | CFG-of-blocks beats recursive tree-walk | cfgblock dominates (+895%) |
| `hx_multishot` | multi-shot adds bounded resumption cost | single dominates (multishot ~4.5x) |
| `hx_interner` | FNV ~= FxHash, string-hash-bound | fnv fastest (within noise of fx) |
| `hx_closure` | non-escaping: linked ~= flat (create-many) | no variant beats linked (tied) |
| `hx_reuse` | exact-meet in-place beats copy (copy defeated via black_box) | reuse dominates (+23% vs copy) |

## Modelling caveat (harness sizes / LLVM behaviour differ from the original)

- `hx_cheaplower` (const-fold + CSE): the benefit is NODE-COUNT reduction (76% fewer nodes), not per-op
  time; LLVM already CSEs the recompute, so the hash-cons lookup is pure overhead in a time bench.
- `hx_output` (format-in-place vs temp): within noise at harness sizes; the original 1.5x needed longer
  values / higher volume than a per-call byte budget exercises.
- `hx_effect`, `hx_tnum`: micro-op transfer-function races (thermometer-OR vs branch-max; tadd vs tor);
  both cheap, so which is "baseline" is arbitrary. The design finding (thermometer encoding makes the
  lattice a flat OR; tnum linear ops are ~1ns) stands independently.

## Not harness-shaped (kept as sketches/measurements, not timing benches)

- Cache-at-scale effects (`arena-locality`, large-table gather): need buffers > L1 and N far beyond the
  16384-byte harness cap; dropped from the port (would only measure noise at these sizes).
- `runtime-binary-size`, `comptime-cost-cliff`: size / compile-time measurements, not runtime timing.
- Feasibility sketches (cert-gen, dual-locus, copy-and-patch MAP_JIT, adversarial load-verifier, streaming,
  transpile, compile-stage parallelism, cross-mod dep-DAG): WORKS/FAILS + numbers, not variant races. They
  live in `mock/research/sketches/` as the audit trail; the copy-and-patch stencil IS benched in-harness as
  `branch_tier`.

## Branch strategies (the primary arc, separately)

The full branch-strategy suite is in-harness: 2-way (`branch_*`), multiway (`multiway_*`), tier
(`branch_tier`: interp/native/copy-and-patch-stencil), and the per-branch archetype showdown (`arch_*`
interp, `archn_*` native). See `BRANCH_STRATEGY_MAP.md`.

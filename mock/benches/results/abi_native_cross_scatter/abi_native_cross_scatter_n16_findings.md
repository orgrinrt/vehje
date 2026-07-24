# abi_native_cross (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_scatter_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_scatter_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_scatter_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_scatter_native_ffi_w has the worst median (12.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_scatter_null_entry at 2.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_scatter_null_entry dominates: 361% faster than the next best (abi_native_cross_scatter_inproc_native)

abi_native_cross_scatter_null_entry (2.55 us) leads abi_native_cross_scatter_inproc_native (11.72 us) by 361%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_scatter_null_entry beats baseline by 79% (significant)

abi_native_cross_scatter_null_entry is -9.75 us (79%) faster than baseline abi_native_cross_scatter_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_scatter_native_ffi_w is an outlier: 4.8x slower than the field

abi_native_cross_scatter_native_ffi_w (12.27 us) is 4.8x the fastest (2.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest abi_native_cross_scatter_null_entry (2.55 us) to slowest abi_native_cross_scatter_native_ffi_w (12.27 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_scatter_null_entry** at 2545.2 ns median (-79.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.82x (fastest 2545.2 ns, slowest 12271.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13988ns | 13970ns | 13508ns | 13869ns | 14407ns | -4.78% |
| abi_native_cross_scatter_native_ffi_w | 14690ns | 14579ns | 14511ns | 14557ns | 14980ns | base |
| abi_native_cross_scatter_null_entry | 4812ns | 4795ns | 4638ns | 4776ns | 4952ns | -67.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 11713ns | 11330ns | 12055ns | -5.36% | 0.001 |
| abi_native_cross_scatter_native_ffi_w | 12377ns | 12172ns | 12653ns | base | 0.001 |
| abi_native_cross_scatter_null_entry | 2544ns | 2466ns | 2604ns | -79.45% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 5597.1 | 11655.6 | 11713.2 | n/a |
| abi_native_cross_scatter_native_ffi_w | 26293.7 | 12564.6 | 12376.5 | n/a |
| abi_native_cross_scatter_null_entry | 25947.4 | 2615.6 | 2543.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_native_cross_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.001 | 21.0% |
| abi_native_cross_scatter_native_ffi_w | 0.001 | 20.1% |
| abi_native_cross_scatter_null_entry | 0.006 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 13988ns | 13988ns | -4.78% |
| abi_native_cross_scatter_native_ffi_w | 14690ns | 14690ns | base |
| abi_native_cross_scatter_null_entry | 4812ns | 4812ns | -67.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_scatter_native_ffi_w | 12272ns | base | --- | [12205, 12653] | --- | --- | --- | --- |
| abi_native_cross_scatter_inproc_native | 11722ns | -777.1ns (-6.3%) | [-1026, -187]ns | [11362, 12055] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_native_cross_scatter_null_entry | 2545ns | -9748.4ns (-79.4%) | [-10101, -9650]ns | [2482, 2604] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_scatter_native_ffi_w | abi_native_cross_scatter_inproc_native | abi_native_cross_scatter_null_entry |
|---|---|---|---|
| 1 | 12172ns | +0.4% | -79.7% |
| 2 | 12312ns | -3.5% | -79.6% |
| 3 | 12238ns | -5.4% | -79.0% |
| 4 | 12993ns | -8.7% | -80.1% |
| 5 | 12288ns | -7.3% | -79.7% |
| 6 | 12256ns | -7.6% | -78.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_scatter_inproc_native | 0.201 | moderate+ |
| abi_native_cross_scatter_native_ffi_w | -0.231 | moderate- |
| abi_native_cross_scatter_null_entry | -0.156 | ok |

**Consistency summary:**

- **abi_native_cross_scatter_inproc_native**: won 5/6, lost 1/6
- **abi_native_cross_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_scatter_inproc_native | 123377.0ns | 11713.2ns | 1053.3% | HIGH |
| abi_native_cross_scatter_native_ffi_w | 141653.5ns | 12376.5ns | 1144.5% | HIGH |
| abi_native_cross_scatter_null_entry | 116256.4ns | 2543.6ns | 4570.5% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_scatter_inproc_native (n=6, range 11330.0-12055.4 ns)
  11330.0 |########################################
  11366.3 |########################################
  11402.5 |
  11438.8 |
  11475.1 |
  11511.4 |
  11547.6 |########################################
  11583.9 |
  11620.2 |
  11656.4 |
  11692.7 |
  11729.0 |
  11765.2 |
  11801.5 |
  11837.8 |########################################
  11874.0 |########################################
  11910.3 |
  11946.6 |
  11982.9 |
  12019.1 |
  (0 below, 1 above range)

abi_native_cross_scatter_native_ffi_w (n=6, range 12172.5-12652.7 ns)
  12172.5 |########################################
  12196.5 |
  12220.5 |########################################
  12244.5 |########################################
  12268.5 |########################################
  12292.5 |########################################
  12316.6 |
  12340.6 |
  12364.6 |
  12388.6 |
  12412.6 |
  12436.6 |
  12460.6 |
  12484.6 |
  12508.6 |
  12532.7 |
  12556.7 |
  12580.7 |
  12604.7 |
  12628.7 |
  (0 below, 1 above range)

abi_native_cross_scatter_null_entry (n=6, range 2465.8-2604.0 ns)
   2465.8 |########################################
   2472.7 |
   2479.6 |
   2486.5 |
   2493.4 |########################################
   2500.4 |
   2507.3 |
   2514.2 |########################################
   2521.1 |
   2528.0 |
   2534.9 |
   2541.8 |
   2548.7 |
   2555.6 |
   2562.5 |
   2569.4 |########################################
   2576.4 |
   2583.3 |########################################
   2590.2 |
   2597.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_scatter_inproc_native**: bridge=1059.1% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_native_ffi_w**: bridge=1151.8% of algo (FFI overhead may distort results)
- **abi_native_cross_scatter_null_entry**: bridge=4570.1% of algo (FFI overhead may distort results)

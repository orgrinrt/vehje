# abi_soa_win (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_scatter_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_scatter_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_scatter_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_scatter_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_scatter_null_entry at 2.51 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_scatter_null_entry dominates: 35380% faster than the next best (abi_soa_win_scatter_soa_payload)

abi_soa_win_scatter_null_entry (2.51 us) leads abi_soa_win_scatter_soa_payload (890.93 us) by 35380%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_scatter_null_entry beats baseline by 100% (significant)

abi_soa_win_scatter_null_entry is -2.14 ms (100%) faster than baseline abi_soa_win_scatter_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_scatter_scalar_payload is an outlier: 854.3x slower than the field

abi_soa_win_scatter_scalar_payload (2.15 ms) is 854.3x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 854.3x the fastest

Fastest abi_soa_win_scatter_null_entry (2.51 us) to slowest abi_soa_win_scatter_scalar_payload (2.15 ms): 854.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_scatter_null_entry** at 2511.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 854.26x (fastest 2511.1 ns, slowest 2145079.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4817ns | 4835ns | 4621ns | 4782ns | 4968ns | -99.78% |
| abi_soa_win_scatter_scalar_payload | 2149061ns | 2148031ns | 2142357ns | 2147232ns | 2155156ns | base |
| abi_soa_win_scatter_soa_payload | 893331ns | 893538ns | 889958ns | 892586ns | 896135ns | -58.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 2514ns | 2401ns | 2610ns | -99.88% | 0.025 |
| abi_soa_win_scatter_scalar_payload | 2146136ns | 2139762ns | 2152070ns | base | 0.000 |
| abi_soa_win_scatter_soa_payload | 890726ns | 887488ns | 893439ns | -58.50% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 28004.5 | 2744.6 | 2513.6 | n/a |
| abi_soa_win_scatter_scalar_payload | 51487.2 | 2145340.5 | 2146135.8 | n/a |
| abi_soa_win_scatter_soa_payload | 39706.0 | 890642.9 | 890726.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_soa_win_scatter_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_scatter_null_entry | 0.025 | 95.6% |
| abi_soa_win_scatter_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_scatter_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_scatter_null_entry | 4817ns | 4817ns | -99.78% |
| abi_soa_win_scatter_scalar_payload | 2149061ns | 2149061ns | base |
| abi_soa_win_scatter_soa_payload | 893331ns | 893331ns | -58.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_scatter_scalar_payload | 2145079ns | base | --- | [2141259, 2152070] | --- | --- | --- | --- |
| abi_soa_win_scatter_null_entry | 2511ns | -2142497.5ns (-99.9%) | [-2149565, -2138805]ns | [2420, 2610] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_scatter_soa_payload | 890928ns | -1255182.3ns (-58.5%) | [-1260810, -1250236]ns | [887812, 893439] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_scatter_scalar_payload | abi_soa_win_scatter_null_entry | abi_soa_win_scatter_soa_payload |
|---|---|---|---|
| 1 | 2142755ns | -99.9% | -58.3% |
| 2 | 2154958ns | -99.9% | -58.7% |
| 3 | 2145162ns | -99.9% | -58.6% |
| 4 | 2139762ns | -99.9% | -58.5% |
| 5 | 2149182ns | -99.9% | -58.5% |
| 6 | 2144996ns | -99.9% | -58.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_scatter_null_entry | -0.073 | ok |
| abi_soa_win_scatter_scalar_payload | -0.390 | moderate- |
| abi_soa_win_scatter_soa_payload | 0.114 | ok |

**Consistency summary:**

- **abi_soa_win_scatter_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_scatter_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_scatter_null_entry | 113390.5ns | 2513.6ns | 4511.1% | HIGH |
| abi_soa_win_scatter_scalar_payload | 6491296.8ns | 2146135.8ns | 302.5% | HIGH |
| abi_soa_win_scatter_soa_payload | 2713430.0ns | 890726.1ns | 304.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_scatter_null_entry (n=6, range 2401.2-2610.2 ns)
   2401.2 |####################
   2411.6 |
   2422.1 |
   2432.5 |####################
   2443.0 |
   2453.4 |
   2463.9 |
   2474.3 |
   2484.8 |
   2495.2 |
   2505.7 |########################################
   2516.1 |
   2526.6 |
   2537.0 |
   2547.5 |
   2557.9 |
   2568.4 |####################
   2578.8 |
   2589.3 |
   2599.8 |
  (0 below, 1 above range)

abi_soa_win_scatter_scalar_payload (n=6, range 2139762.5-2152069.6 ns)
  2139762.5 |####################
  2140377.9 |
  2140993.2 |
  2141608.6 |
  2142223.9 |####################
  2142839.3 |
  2143454.6 |
  2144070.0 |
  2144685.3 |########################################
  2145300.7 |
  2145916.0 |
  2146531.4 |
  2147146.8 |
  2147762.1 |
  2148377.5 |
  2148992.8 |####################
  2149608.2 |
  2150223.5 |
  2150838.9 |
  2151454.2 |
  (0 below, 1 above range)

abi_soa_win_scatter_soa_payload (n=6, range 887487.5-893438.8 ns)
  887487.5 |########################################
  887785.1 |
  888082.6 |########################################
  888380.2 |
  888677.8 |
  888975.3 |
  889272.9 |
  889570.4 |
  889868.0 |
  890165.6 |########################################
  890463.1 |
  890760.7 |
  891058.2 |
  891355.8 |########################################
  891653.4 |
  891950.9 |
  892248.5 |########################################
  892546.1 |
  892843.6 |
  893141.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_scatter_null_entry**: bridge=4509.7% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_scalar_payload**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_soa_win_scatter_soa_payload**: bridge=304.7% of algo (FFI overhead may distort results)

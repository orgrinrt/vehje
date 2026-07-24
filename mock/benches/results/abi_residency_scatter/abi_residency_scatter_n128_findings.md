# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 79974% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (2.65 us) leads abi_residency_scatter_reused_buffer (2.12 ms) by 79974%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.12 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 802.1x slower than the field

abi_residency_scatter_fresh_alloc (2.13 ms) is 802.1x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 802.1x the fastest

Fastest abi_residency_scatter_null_entry (2.65 us) to slowest abi_residency_scatter_fresh_alloc (2.13 ms): 802.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 2651.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 802.15x (fastest 2651.1 ns, slowest 2126536.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2129494ns | 2129045ns | 2126316ns | 2128276ns | 2132910ns | +0.24% |
| abi_residency_scatter_null_entry | 4921ns | 4862ns | 4727ns | 4843ns | 5135ns | -99.77% |
| abi_residency_scatter_reused_buffer | 2124313ns | 2125375ns | 2113040ns | 2124417ns | 2129795ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2126972ns | 2123804ns | 2130410ns | +0.25% | 0.000 |
| abi_residency_scatter_null_entry | 2685ns | 2590ns | 2799ns | -99.87% | 0.048 |
| abi_residency_scatter_reused_buffer | 2121763ns | 2110620ns | 2127234ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 39311.8 | 2126268.4 | 2126971.9 | n/a |
| abi_residency_scatter_null_entry | 26114.9 | 2721.3 | 2684.8 | n/a |
| abi_residency_scatter_reused_buffer | 38459.0 | 2122081.0 | 2121763.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.1% |
| abi_residency_scatter_null_entry | 0.048 | 97.7% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2129494ns | 2129494ns | +0.24% |
| abi_residency_scatter_null_entry | 4921ns | 4921ns | -99.77% |
| abi_residency_scatter_reused_buffer | 2124313ns | 2124313ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2122804ns | base | --- | [2115252, 2127234] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2126537ns | no significant difference | [-1456, +10384]ns | [2123969, 2130410] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_scatter_null_entry | 2651ns | -2120144.3ns (-99.9%) | [-2124490, -2112601]ns | [2604, 2799] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2110620ns | +0.6% | -99.9% |
| 2 | 2120715ns | +0.3% | -99.9% |
| 3 | 2126143ns | +0.4% | -99.9% |
| 4 | 2124893ns | +0.1% | -99.9% |
| 5 | 2128324ns | -0.2% | -99.9% |
| 6 | 2119885ns | +0.3% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.039 | ok |
| abi_residency_scatter_null_entry | -0.068 | ok |
| abi_residency_scatter_reused_buffer | 0.144 | ok |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 1/6, lost 4/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6423007.4ns | 2126971.9ns | 302.0% | HIGH |
| abi_residency_scatter_null_entry | 118259.6ns | 2684.8ns | 4404.8% | HIGH |
| abi_residency_scatter_reused_buffer | 6406167.2ns | 2121763.3ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2123804.2-2130410.4 ns)
  2123804.2 |########################################
  2124134.5 |
  2124464.8 |
  2124795.1 |
  2125125.4 |
  2125455.8 |
  2125786.1 |
  2126116.4 |####################
  2126446.7 |
  2126777.0 |########################################
  2127107.3 |
  2127437.6 |
  2127767.9 |
  2128098.2 |
  2128428.5 |
  2128758.9 |
  2129089.2 |
  2129419.5 |
  2129749.8 |
  2130080.1 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 2589.6-2799.4 ns)
   2589.6 |########################################
   2600.1 |
   2610.6 |########################################
   2621.1 |
   2631.6 |########################################
   2642.0 |
   2652.5 |########################################
   2663.0 |
   2673.5 |
   2684.0 |
   2694.5 |
   2705.0 |
   2715.5 |
   2726.0 |########################################
   2736.5 |
   2746.9 |
   2757.4 |
   2767.9 |
   2778.4 |
   2788.9 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2110620.4-2127233.5 ns)
  2110620.4 |########################################
  2111451.1 |
  2112281.7 |
  2113112.4 |
  2113943.0 |
  2114773.7 |
  2115604.3 |
  2116435.0 |
  2117265.7 |
  2118096.3 |
  2118927.0 |
  2119757.6 |########################################
  2120588.3 |########################################
  2121418.9 |
  2122249.6 |
  2123080.3 |
  2123910.9 |
  2124741.6 |########################################
  2125572.2 |########################################
  2126402.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=4441.0% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=301.9% of algo (FFI overhead may distort results)

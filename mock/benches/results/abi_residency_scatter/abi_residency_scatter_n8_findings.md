# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 67851% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (3.13 us) leads abi_residency_scatter_reused_buffer (2.12 ms) by 67851%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.12 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 680.4x slower than the field

abi_residency_scatter_fresh_alloc (2.13 ms) is 680.4x the fastest (3.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_scatter_reused_buffer shows alternating (throttle bounce) (autocorr -0.54)

abi_residency_scatter_reused_buffer's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 680.4x the fastest

Fastest abi_residency_scatter_null_entry (3.13 us) to slowest abi_residency_scatter_fresh_alloc (2.13 ms): 680.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_residency_scatter_fresh_alloc's edge over baseline is significant but tiny (-5 ns, 0.00%)

abi_residency_scatter_fresh_alloc differs from baseline abi_residency_scatter_reused_buffer by -5 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 3127.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 680.38x (fastest 3127.1 ns, slowest 2127603.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2129552ns | 2130104ns | 2110544ns | 2127370ns | 2142330ns | +0.12% |
| abi_residency_scatter_null_entry | 5393ns | 5424ns | 5068ns | 5336ns | 5642ns | -99.75% |
| abi_residency_scatter_reused_buffer | 2127069ns | 2127457ns | 2118682ns | 2125590ns | 2133480ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2127036ns | 2108169ns | 2139697ns | +0.12% | 0.000 |
| abi_residency_scatter_null_entry | 3108ns | 2925ns | 3248ns | -99.85% | 0.003 |
| abi_residency_scatter_reused_buffer | 2124581ns | 2116224ns | 2131005ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 38479.0 | 2124701.9 | 2127035.6 | n/a |
| abi_residency_scatter_null_entry | 26326.1 | 3151.5 | 3107.9 | n/a |
| abi_residency_scatter_reused_buffer | 36637.6 | 2125338.7 | 2124580.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.1% |
| abi_residency_scatter_null_entry | 0.003 | 93.5% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2129552ns | 2129552ns | +0.12% |
| abi_residency_scatter_null_entry | 5393ns | 5393ns | -99.75% |
| abi_residency_scatter_reused_buffer | 2127069ns | 2127069ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2124906ns | base | --- | [2117831, 2131005] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2127604ns | no significant difference | [-14496, +21866]ns | [2113806, 2139697] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_scatter_null_entry | 3127ns | -2121734.2ns (-99.9%) | [-2127951, -2114733]ns | [2949, 3248] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2134170ns | -1.2% | -99.9% |
| 2 | 2116224ns | +0.6% | -99.8% |
| 3 | 2127377ns | +0.1% | -99.9% |
| 4 | 2119438ns | +1.4% | -99.9% |
| 5 | 2122435ns | -0.1% | -99.8% |
| 6 | 2127840ns | -0.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.173 | ok |
| abi_residency_scatter_null_entry | -0.351 | moderate- |
| abi_residency_scatter_reused_buffer | -0.539 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 2/6, lost 2/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6414813.8ns | 2127035.6ns | 301.6% | HIGH |
| abi_residency_scatter_null_entry | 118430.4ns | 3107.9ns | 3810.6% | HIGH |
| abi_residency_scatter_reused_buffer | 6415046.8ns | 2124580.6ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2108169.2-2139697.1 ns)
  2108169.2 |####################
  2109745.6 |
  2111322.0 |
  2112898.4 |
  2114474.8 |
  2116051.2 |
  2117627.6 |
  2119204.0 |####################
  2120780.4 |
  2122356.8 |
  2123933.2 |
  2125509.5 |####################
  2127085.9 |
  2128662.3 |########################################
  2130238.7 |
  2131815.1 |
  2133391.5 |
  2134967.9 |
  2136544.3 |
  2138120.7 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 2924.6-3247.5 ns)
   2924.6 |########################################
   2940.7 |
   2956.9 |
   2973.0 |########################################
   2989.2 |
   3005.3 |
   3021.5 |
   3037.6 |
   3053.8 |
   3069.9 |
   3086.1 |
   3102.2 |
   3118.3 |########################################
   3134.5 |########################################
   3150.6 |
   3166.8 |
   3182.9 |
   3199.1 |
   3215.2 |########################################
   3231.4 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2116224.2-2131005.0 ns)
  2116224.2 |####################
  2116963.2 |
  2117702.3 |
  2118441.3 |
  2119180.4 |####################
  2119919.4 |
  2120658.4 |
  2121397.5 |
  2122136.5 |####################
  2122875.6 |
  2123614.6 |
  2124353.6 |
  2125092.7 |
  2125831.7 |
  2126570.8 |
  2127309.8 |########################################
  2128048.8 |
  2128787.9 |
  2129526.9 |
  2130266.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=3790.1% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=301.9% of algo (FFI overhead may distort results)

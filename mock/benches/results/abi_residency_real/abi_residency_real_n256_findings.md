# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 66437% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (3.19 us) leads abi_residency_real_reused_buffer (2.13 ms) by 66437%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.12 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 668.2x slower than the field

abi_residency_real_fresh_alloc (2.13 ms) is 668.2x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 668.2x the fastest

Fastest abi_residency_real_null_entry (3.19 us) to slowest abi_residency_real_fresh_alloc (2.13 ms): 668.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 3193.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 668.22x (fastest 3193.8 ns, slowest 2134134.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2137201ns | 2136751ns | 2132058ns | 2136133ns | 2141374ns | +0.28% |
| abi_residency_real_null_entry | 5491ns | 5507ns | 5252ns | 5444ns | 5680ns | -99.74% |
| abi_residency_real_reused_buffer | 2131317ns | 2127459ns | 2122393ns | 2126008ns | 2143744ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2134596ns | 2129610ns | 2138787ns | +0.27% | 0.000 |
| abi_residency_real_null_entry | 3207ns | 3089ns | 3327ns | -99.85% | 0.080 |
| abi_residency_real_reused_buffer | 2128744ns | 2119745ns | 2141090ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 41063.5 | 2133694.1 | 2134595.9 | 0 |
| abi_residency_real_null_entry | 27170.0 | 3218.5 | 3206.6 | n/a |
| abi_residency_real_reused_buffer | 42418.0 | 2130196.1 | 2128744.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.1% |
| abi_residency_real_null_entry | 0.080 | 96.7% |
| abi_residency_real_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2137201ns | 2137201ns | +0.28% |
| abi_residency_real_null_entry | 5491ns | 5491ns | -99.74% |
| abi_residency_real_reused_buffer | 2131317ns | 2131317ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2125010ns | base | --- | [2120133, 2141090] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2134134ns | no significant difference | [-5965, +16284]ns | [2130866, 2138787] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_real_null_entry | 3194ns | -2121798.2ns (-99.8%) | [-2137875, -2116939]ns | [3099, 3327] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2148199ns | -0.6% | -99.8% |
| 2 | 2120520ns | +0.4% | -99.8% |
| 3 | 2123280ns | +0.5% | -99.9% |
| 4 | 2119745ns | +1.0% | -99.8% |
| 5 | 2133980ns | +0.1% | -99.9% |
| 6 | 2126740ns | +0.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.085 | ok |
| abi_residency_real_null_entry | -0.147 | ok |
| abi_residency_real_reused_buffer | -0.210 | moderate- |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 1/6, lost 4/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6444350.6ns | 2134595.9ns | 301.9% | HIGH |
| abi_residency_real_null_entry | 120700.3ns | 3206.6ns | 3764.1% | HIGH |
| abi_residency_real_reused_buffer | 6433049.7ns | 2128744.1ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2129610.4-2138787.2 ns)
  2129610.4 |########################################
  2130069.2 |
  2130528.1 |
  2130986.9 |
  2131445.8 |
  2131904.6 |########################################
  2132363.5 |
  2132822.3 |
  2133281.1 |
  2133740.0 |########################################
  2134198.8 |########################################
  2134657.7 |
  2135116.5 |
  2135575.4 |########################################
  2136034.2 |
  2136493.0 |
  2136951.9 |
  2137410.7 |
  2137869.6 |
  2138328.4 |
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 3088.7-3326.7 ns)
   3088.7 |########################################
   3100.6 |########################################
   3112.5 |
   3124.4 |
   3136.3 |
   3148.2 |
   3160.1 |
   3172.0 |########################################
   3183.9 |
   3195.8 |########################################
   3207.7 |
   3219.6 |
   3231.5 |
   3243.4 |
   3255.3 |
   3267.2 |
   3279.1 |
   3291.0 |
   3302.9 |########################################
   3314.8 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2119745.4-2141089.5 ns)
  2119745.4 |########################################
  2120812.6 |
  2121879.8 |
  2122947.0 |####################
  2124014.2 |
  2125081.4 |
  2126148.6 |####################
  2127215.9 |
  2128283.1 |
  2129350.3 |
  2130417.5 |
  2131484.7 |
  2132551.9 |
  2133619.1 |####################
  2134686.3 |
  2135753.5 |
  2136820.7 |
  2137887.9 |
  2138955.1 |
  2140022.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=3792.3% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)

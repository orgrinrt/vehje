# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_soa_win_real_null_entry dominates: 63396% faster than the next best (abi_soa_win_real_scalar_payload)

abi_soa_win_real_null_entry (3.42 us) leads abi_soa_win_real_scalar_payload (2.17 ms) by 63396%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.17 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_soa_payload is an outlier: 635.8x slower than the field

abi_soa_win_real_soa_payload (2.17 ms) is 635.8x the fastest (3.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 635.8x the fastest

Fastest abi_soa_win_real_null_entry (3.42 us) to slowest abi_soa_win_real_soa_payload (2.17 ms): 635.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3417.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 635.81x (fastest 3417.5 ns, slowest 2172875.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5752ns | 5717ns | 5590ns | 5694ns | 5918ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2172756ns | 2173083ns | 2168004ns | 2172358ns | 2175730ns | base |
| abi_soa_win_real_soa_payload | 2176423ns | 2175952ns | 2162953ns | 2175105ns | 2185137ns | +0.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3444ns | 3347ns | 3547ns | -99.84% | 0.001 |
| abi_soa_win_real_scalar_payload | 2169647ns | 2165207ns | 2172457ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 2173194ns | 2159961ns | 2181718ns | +0.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27895.6 | 3527.6 | 3443.7 | n/a |
| abi_soa_win_real_scalar_payload | 60835.1 | 2168859.5 | 2169646.5 | n/a |
| abi_soa_win_real_soa_payload | 62347.1 | 2170795.2 | 2173194.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.001 | 97.9% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_real_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5752ns | 5752ns | -99.74% |
| abi_soa_win_real_scalar_payload | 2172756ns | 2172756ns | base |
| abi_soa_win_real_soa_payload | 2176423ns | 2176423ns | +0.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2169990ns | base | --- | [2166493, 2172457] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3418ns | -2166569.8ns (-99.8%) | [-2168928, -2163110]ns | [3367, 3547] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 2172876ns | no significant difference | [-3638, +10244]ns | [2164988, 2181718] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2167778ns | -99.8% | +0.3% |
| 2 | 2169171ns | -99.8% | +0.1% |
| 3 | 2172869ns | -99.8% | +0.6% |
| 4 | 2172046ns | -99.8% | -0.1% |
| 5 | 2170808ns | -99.8% | +0.3% |
| 6 | 2165207ns | -99.8% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.204 | moderate- |
| abi_soa_win_real_scalar_payload | 0.115 | ok |
| abi_soa_win_real_soa_payload | -0.369 | moderate- |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 120013.5ns | 3443.7ns | 3485.0% | HIGH |
| abi_soa_win_real_scalar_payload | 6570360.1ns | 2169646.5ns | 302.8% | HIGH |
| abi_soa_win_real_soa_payload | 6584641.5ns | 2173194.1ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 3347.1-3546.7 ns)
   3347.1 |#############
   3357.1 |
   3367.1 |
   3377.0 |
   3387.0 |#############
   3397.0 |
   3407.0 |
   3416.9 |########################################
   3426.9 |
   3436.9 |
   3446.9 |
   3456.9 |
   3466.8 |
   3476.8 |
   3486.8 |
   3496.8 |
   3506.7 |
   3516.7 |
   3526.7 |
   3536.7 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2165207.1-2172457.3 ns)
  2165207.1 |########################################
  2165569.6 |
  2165932.1 |
  2166294.6 |
  2166657.1 |
  2167019.6 |
  2167382.2 |
  2167744.7 |########################################
  2168107.2 |
  2168469.7 |
  2168832.2 |########################################
  2169194.7 |
  2169557.2 |
  2169919.7 |
  2170282.2 |
  2170644.8 |########################################
  2171007.3 |
  2171369.8 |
  2171732.3 |########################################
  2172094.8 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 2159961.2-2181718.3 ns)
  2159961.2 |########################################
  2161049.1 |
  2162136.9 |
  2163224.8 |
  2164312.6 |
  2165400.5 |
  2166488.3 |
  2167576.2 |
  2168664.0 |
  2169751.9 |########################################
  2170839.8 |########################################
  2171927.6 |
  2173015.5 |
  2174103.3 |########################################
  2175191.2 |
  2176279.0 |########################################
  2177366.9 |
  2178454.7 |
  2179542.6 |
  2180630.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3523.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=302.9% of algo (FFI overhead may distort results)

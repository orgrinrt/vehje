# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 10478% faster than the next best (abi_marshal_real_soa_native)

abi_marshal_real_marshal_null (20.51 us) leads abi_marshal_real_soa_native (2.17 ms) by 10478%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.15 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 107.4x slower than the field

abi_marshal_real_soa_transposed (2.20 ms) is 107.4x the fastest (20.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} (10478% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} with a 10478% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 107.4x the fastest

Fastest abi_marshal_real_marshal_null (20.51 us) to slowest abi_marshal_real_soa_transposed (2.20 ms): 107.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 20510.2 ns median (-99.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 107.36x (fastest 20510.2 ns, slowest 2201945.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2174192ns | 2172646ns | 2165220ns | 2172281ns | 2181546ns | base |
| abi_marshal_real_marshal_null | 23026ns | 22841ns | 22201ns | 22746ns | 23860ns | -98.94% |
| abi_marshal_real_soa_native | 2174495ns | 2172228ns | 2165312ns | 2171646ns | 2183360ns | +0.01% |
| abi_marshal_real_soa_transposed | 2204907ns | 2204791ns | 2201900ns | 2203910ns | 2207904ns | +1.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2171301ns | 2162336ns | 2178523ns | base | 0.000 |
| abi_marshal_real_marshal_null | 20715ns | 20031ns | 21482ns | -99.05% | 0.000 |
| abi_marshal_real_soa_native | 2171722ns | 2162470ns | 2180495ns | +0.02% | 0.000 |
| abi_marshal_real_soa_transposed | 2202092ns | 2199374ns | 2204947ns | +1.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 52254.8 | 2168412.1 | 2171300.8 | n/a |
| abi_marshal_real_marshal_null | 28574.5 | 20926.8 | 20714.8 | n/a |
| abi_marshal_real_soa_native | 50775.8 | 2171525.0 | 2171721.9 | 4 |
| abi_marshal_real_soa_transposed | 49961.3 | 2201256.9 | 2202091.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.9% |
| abi_marshal_real_marshal_null | 0.000 | 97.7% |
| abi_marshal_real_soa_native | 0.000 | 0.9% |
| abi_marshal_real_soa_transposed | 0.000 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2174192ns | 2174192ns | base |
| abi_marshal_real_marshal_null | 23026ns | 23026ns | -98.94% |
| abi_marshal_real_soa_native | 2174495ns | 2174495ns | +0.01% |
| abi_marshal_real_soa_transposed | 2204907ns | 2204907ns | +1.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2169908ns | base | --- | [2165472, 2178523] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 20510ns | -2149109.2ns (-99.0%) | [-2158012, -2144636]ns | [20153, 21482] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2169470ns | no significant difference | [-8589, +10058]ns | [2165201, 2180495] | no | 1.0000 | 1.0000 | 0 |
| abi_marshal_real_soa_transposed | 2201946ns | +31326.9ns (+1.4%) | [+23825, +37221]ns | [2199383, 2204947] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2169070ns | -99.0% | -0.3% | +1.5% |
| 2 | 2184878ns | -99.1% | -0.5% | +0.9% |
| 3 | 2168609ns | -99.0% | +0.1% | +1.7% |
| 4 | 2172167ns | -99.1% | +0.7% | +1.3% |
| 5 | 2170745ns | -99.1% | -0.1% | +1.3% |
| 6 | 2162336ns | -99.1% | +0.3% | +1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.233 | moderate- |
| abi_marshal_real_marshal_null | 0.087 | ok |
| abi_marshal_real_soa_native | -0.232 | moderate- |
| abi_marshal_real_soa_transposed | 0.420 | moderate+ |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 3/6, lost 3/6
- **abi_marshal_real_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6565031.7ns | 2171300.8ns | 302.4% | HIGH |
| abi_marshal_real_marshal_null | 172266.4ns | 20714.8ns | 831.6% | HIGH |
| abi_marshal_real_soa_native | 6571671.7ns | 2171721.9ns | 302.6% | HIGH |
| abi_marshal_real_soa_transposed | 6659777.0ns | 2202091.8ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2162335.8-2178522.7 ns)
  2162335.8 |########################################
  2163145.1 |
  2163954.5 |
  2164763.8 |
  2165573.2 |
  2166382.5 |
  2167191.9 |
  2168001.2 |########################################
  2168810.6 |########################################
  2169619.9 |
  2170429.2 |########################################
  2171238.6 |
  2172047.9 |########################################
  2172857.3 |
  2173666.6 |
  2174476.0 |
  2175285.3 |
  2176094.7 |
  2176904.0 |
  2177713.4 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 20030.8-21481.7 ns)
  20030.8 |########################################
  20103.3 |
  20175.9 |
  20248.4 |########################################
  20321.0 |########################################
  20393.5 |
  20466.1 |
  20538.6 |
  20611.1 |########################################
  20683.7 |
  20756.2 |
  20828.8 |
  20901.3 |
  20973.9 |
  21046.4 |
  21118.9 |
  21191.5 |
  21264.0 |
  21336.6 |########################################
  21409.1 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2162470.4-2180494.5 ns)
  2162470.4 |####################
  2163371.6 |
  2164272.8 |
  2165174.0 |
  2166075.2 |
  2166976.4 |
  2167877.6 |########################################
  2168778.9 |
  2169680.1 |
  2170581.3 |####################
  2171482.5 |
  2172383.7 |
  2173284.9 |
  2174186.1 |####################
  2175087.3 |
  2175988.5 |
  2176889.7 |
  2177790.9 |
  2178692.1 |
  2179593.3 |
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2199373.8-2204946.6 ns)
  2199373.8 |########################################
  2199652.4 |
  2199931.1 |
  2200209.7 |
  2200488.4 |
  2200767.0 |
  2201045.7 |
  2201324.3 |####################
  2201602.9 |
  2201881.6 |
  2202160.2 |
  2202438.9 |####################
  2202717.5 |
  2202996.2 |
  2203274.8 |
  2203553.4 |
  2203832.1 |####################
  2204110.7 |
  2204389.4 |
  2204668.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=842.4% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=302.6% of algo (FFI overhead may distort results)

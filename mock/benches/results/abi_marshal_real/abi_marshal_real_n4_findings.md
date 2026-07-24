# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 10207% faster than the next best (abi_marshal_real_aos)

abi_marshal_real_marshal_null (20.85 us) leads abi_marshal_real_aos (2.15 ms) by 10207%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.13 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 103.9x slower than the field

abi_marshal_real_soa_transposed (2.17 ms) is 103.9x the fastest (20.85 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_real_soa_native shows alternating (throttle bounce) (autocorr -0.77)

abi_marshal_real_soa_native's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} (10207% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} with a 10207% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 103.9x the fastest

Fastest abi_marshal_real_marshal_null (20.85 us) to slowest abi_marshal_real_soa_transposed (2.17 ms): 103.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 20847.1 ns median (-99.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 103.89x (fastest 20847.1 ns, slowest 2165798.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2151088ns | 2151243ns | 2140860ns | 2150137ns | 2157627ns | base |
| abi_marshal_real_marshal_null | 23031ns | 23184ns | 21890ns | 23049ns | 23576ns | -98.93% |
| abi_marshal_real_soa_native | 2152989ns | 2154190ns | 2143444ns | 2151366ns | 2160195ns | +0.09% |
| abi_marshal_real_soa_transposed | 2170422ns | 2168291ns | 2166444ns | 2167813ns | 2176322ns | +0.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2148609ns | 2138398ns | 2155150ns | base | 0.000 |
| abi_marshal_real_marshal_null | 20718ns | 19728ns | 21228ns | -99.04% | 0.000 |
| abi_marshal_real_soa_native | 2150402ns | 2140827ns | 2157480ns | +0.08% | 0.000 |
| abi_marshal_real_soa_transposed | 2167884ns | 2163886ns | 2173814ns | +0.90% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 40247.6 | 2149567.3 | 2148609.0 | n/a |
| abi_marshal_real_marshal_null | 27832.7 | 20725.8 | 20718.0 | n/a |
| abi_marshal_real_soa_native | 42206.9 | 2150309.3 | 2150402.3 | n/a |
| abi_marshal_real_soa_transposed | 39831.8 | 2168310.2 | 2167883.8 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.9% |
| abi_marshal_real_marshal_null | 0.000 | 94.6% |
| abi_marshal_real_soa_native | 0.000 | 0.9% |
| abi_marshal_real_soa_transposed | 0.000 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2151088ns | 2151088ns | base |
| abi_marshal_real_marshal_null | 23031ns | 23031ns | -98.93% |
| abi_marshal_real_soa_native | 2152989ns | 2152989ns | +0.09% |
| abi_marshal_real_soa_transposed | 2170422ns | 2170422ns | +0.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2148740ns | base | --- | [2141937, 2155150] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 20847ns | -2127892.8ns (-99.0%) | [-2134607, -2121173]ns | [20079, 21228] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2151649ns | no significant difference | [-9262, +11732]ns | [2142078, 2157480] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_real_soa_transposed | 2165798ns | +20860.9ns (+1.0%) | [+11458, +25506]ns | [2164040, 2173814] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2153097ns | -99.1% | +0.2% | +1.2% |
| 2 | 2145476ns | -99.0% | -0.2% | +1.0% |
| 3 | 2138398ns | -99.0% | +0.9% | +1.2% |
| 4 | 2157203ns | -99.0% | -0.6% | +0.3% |
| 5 | 2148829ns | -99.0% | +0.1% | +0.8% |
| 6 | 2148650ns | -99.0% | +0.2% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.326 | moderate- |
| abi_marshal_real_marshal_null | -0.421 | moderate- |
| abi_marshal_real_soa_native | -0.767 | HIGH- (thermal bounce) |
| abi_marshal_real_soa_transposed | 0.086 | ok |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 2/6, lost 4/6
- **abi_marshal_real_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6491445.0ns | 2148609.0ns | 302.1% | HIGH |
| abi_marshal_real_marshal_null | 169639.0ns | 20718.0ns | 818.8% | HIGH |
| abi_marshal_real_soa_native | 6492295.8ns | 2150402.3ns | 301.9% | HIGH |
| abi_marshal_real_soa_transposed | 6548084.8ns | 2167883.8ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2138398.3-2155150.0 ns)
  2138398.3 |####################
  2139235.9 |
  2140073.5 |
  2140911.1 |
  2141748.6 |
  2142586.2 |
  2143423.8 |
  2144261.4 |
  2145099.0 |####################
  2145936.6 |
  2146774.1 |
  2147611.7 |
  2148449.3 |########################################
  2149286.9 |
  2150124.5 |
  2150962.1 |
  2151799.7 |
  2152637.2 |####################
  2153474.8 |
  2154312.4 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 19727.9-21227.9 ns)
  19727.9 |########################################
  19802.9 |
  19877.9 |
  19952.9 |
  20027.9 |
  20102.9 |
  20177.9 |
  20252.9 |
  20327.9 |
  20402.9 |########################################
  20477.9 |
  20552.9 |
  20627.9 |########################################
  20702.9 |
  20777.9 |
  20852.9 |
  20927.9 |
  21002.9 |########################################
  21077.9 |########################################
  21152.9 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2140827.1-2157479.8 ns)
  2140827.1 |########################################
  2141659.7 |
  2142492.4 |
  2143325.0 |########################################
  2144157.6 |
  2144990.3 |
  2145822.9 |
  2146655.5 |
  2147488.2 |
  2148320.8 |
  2149153.4 |
  2149986.1 |
  2150818.7 |########################################
  2151651.3 |########################################
  2152484.0 |
  2153316.6 |
  2154149.2 |
  2154981.9 |
  2155814.5 |
  2156647.1 |########################################
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2163885.8-2173813.5 ns)
  2163885.8 |########################################
  2164382.2 |
  2164878.6 |####################
  2165375.0 |
  2165871.3 |
  2166367.7 |####################
  2166864.1 |
  2167360.5 |
  2167856.9 |
  2168353.3 |
  2168849.7 |####################
  2169346.1 |
  2169842.4 |
  2170338.8 |
  2170835.2 |
  2171331.6 |
  2171828.0 |
  2172324.4 |
  2172820.8 |
  2173317.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=816.0% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=302.1% of algo (FFI overhead may distort results)

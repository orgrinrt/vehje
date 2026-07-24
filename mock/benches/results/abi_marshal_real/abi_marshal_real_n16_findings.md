# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 7687% faster than the next best (abi_marshal_real_aos)

abi_marshal_real_marshal_null (27.64 us) leads abi_marshal_real_aos (2.15 ms) by 7687%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.13 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 80.4x slower than the field

abi_marshal_real_soa_transposed (2.22 ms) is 80.4x the fastest (27.64 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_real_soa_native shows alternating (throttle bounce) (autocorr -0.72)

abi_marshal_real_soa_native's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} (7687% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_aos, abi_marshal_real_soa_native, abi_marshal_real_soa_transposed} with a 7687% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 80.4x the fastest

Fastest abi_marshal_real_marshal_null (27.64 us) to slowest abi_marshal_real_soa_transposed (2.22 ms): 80.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 27644.8 ns median (-98.7% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 80.41x (fastest 27644.8 ns, slowest 2223041.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2156422ns | 2155113ns | 2150936ns | 2154194ns | 2162509ns | base |
| abi_marshal_real_marshal_null | 30024ns | 29911ns | 29295ns | 29854ns | 30644ns | -98.61% |
| abi_marshal_real_soa_native | 2182590ns | 2182553ns | 2175489ns | 2180992ns | 2188536ns | +1.21% |
| abi_marshal_real_soa_transposed | 2254919ns | 2225501ns | 2209919ns | 2220785ns | 2328620ns | +4.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2153866ns | 2148398ns | 2159808ns | base | 0.000 |
| abi_marshal_real_marshal_null | 27740ns | 27060ns | 28315ns | -98.71% | 0.001 |
| abi_marshal_real_soa_native | 2180096ns | 2173049ns | 2186017ns | +1.22% | 0.000 |
| abi_marshal_real_soa_transposed | 2252258ns | 2207422ns | 2325654ns | +4.57% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 40344.1 | 2155195.6 | 2153865.6 | n/a |
| abi_marshal_real_marshal_null | 28710.2 | 28232.4 | 27740.3 | n/a |
| abi_marshal_real_soa_native | 41171.5 | 2179959.2 | 2180096.2 | n/a |
| abi_marshal_real_soa_transposed | 44068.9 | 2231706.9 | 2252257.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 1.3% |
| abi_marshal_real_marshal_null | 0.001 | 97.9% |
| abi_marshal_real_soa_native | 0.000 | 1.2% |
| abi_marshal_real_soa_transposed | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2156422ns | 2156422ns | base |
| abi_marshal_real_marshal_null | 30024ns | 30024ns | -98.61% |
| abi_marshal_real_soa_native | 2182590ns | 2182590ns | +1.21% |
| abi_marshal_real_soa_transposed | 2254919ns | 2254919ns | +4.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2152628ns | base | --- | [2149161, 2159808] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 27645ns | -2125366.8ns (-98.7%) | [-2132114, -2120895]ns | [27261, 28315] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2179994ns | +25920.8ns (+1.2%) | [+18724, +34048]ns | [2174277, 2186017] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_real_soa_transposed | 2223041ns | +69087.9ns (+3.2%) | [+53063, +173026]ns | [2208079, 2325654] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2153955ns | -98.7% | +1.2% | +3.9% |
| 2 | 2160108ns | -98.7% | +1.1% | +2.3% |
| 3 | 2149924ns | -98.7% | +1.2% | +2.7% |
| 4 | 2151300ns | -98.7% | +1.7% | +12.2% |
| 5 | 2159509ns | -98.7% | +0.6% | +2.8% |
| 6 | 2148398ns | -98.7% | +1.5% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.483 | moderate- |
| abi_marshal_real_marshal_null | -0.124 | ok |
| abi_marshal_real_soa_native | -0.721 | HIGH- (thermal bounce) |
| abi_marshal_real_soa_transposed | -0.285 | moderate- |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 0/6, lost 6/6
- **abi_marshal_real_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6507618.6ns | 2153865.6ns | 302.1% | HIGH |
| abi_marshal_real_marshal_null | 194333.8ns | 27740.3ns | 700.5% | HIGH |
| abi_marshal_real_soa_native | 6585266.3ns | 2180096.2ns | 302.1% | HIGH |
| abi_marshal_real_soa_transposed | 6739104.1ns | 2252257.8ns | 299.2% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2148397.5-2159808.1 ns)
  2148397.5 |########################################
  2148968.0 |
  2149538.6 |########################################
  2150109.1 |
  2150679.6 |
  2151250.2 |########################################
  2151820.7 |
  2152391.2 |
  2152961.8 |
  2153532.3 |########################################
  2154102.8 |
  2154673.4 |
  2155243.9 |
  2155814.4 |
  2156385.0 |
  2156955.5 |
  2157526.0 |
  2158096.6 |
  2158667.1 |
  2159237.6 |########################################
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 27059.6-28315.4 ns)
  27059.6 |########################################
  27122.4 |
  27185.2 |
  27248.0 |
  27310.8 |
  27373.5 |
  27436.3 |########################################
  27499.1 |
  27561.9 |########################################
  27624.7 |
  27687.5 |########################################
  27750.3 |
  27813.1 |########################################
  27875.9 |
  27938.7 |
  28001.5 |
  28064.2 |
  28127.0 |
  28189.8 |
  28252.6 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2173049.2-2186017.1 ns)
  2173049.2 |########################################
  2173697.6 |
  2174346.0 |
  2174994.4 |########################################
  2175642.8 |
  2176291.2 |
  2176939.6 |
  2177588.0 |
  2178236.4 |
  2178884.8 |
  2179533.2 |########################################
  2180181.5 |########################################
  2180829.9 |
  2181478.3 |
  2182126.7 |
  2182775.1 |
  2183423.5 |########################################
  2184071.9 |
  2184720.3 |
  2185368.7 |
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2207422.5-2325653.7 ns)
  2207422.5 |########################################
  2213334.1 |
  2219245.6 |####################
  2225157.2 |####################
  2231068.7 |
  2236980.3 |####################
  2242891.9 |
  2248803.4 |
  2254715.0 |
  2260626.5 |
  2266538.1 |
  2272449.7 |
  2278361.2 |
  2284272.8 |
  2290184.3 |
  2296095.9 |
  2302007.5 |
  2307919.0 |
  2313830.6 |
  2319742.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=705.4% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=301.7% of algo (FFI overhead may distort results)

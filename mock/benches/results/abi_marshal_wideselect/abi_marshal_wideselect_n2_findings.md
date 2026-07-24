# abi_marshal (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_wideselect_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_wideselect_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_wideselect_marshal_null dominates: 17125% faster than the next best (abi_marshal_wideselect_soa_native)

abi_marshal_wideselect_marshal_null (11.91 us) leads abi_marshal_wideselect_soa_native (2.05 ms) by 17125%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_wideselect_marshal_null beats baseline by 99% (significant)

abi_marshal_wideselect_marshal_null is -2.04 ms (99%) faster than baseline abi_marshal_wideselect_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_wideselect_soa_transposed is an outlier: 173.5x slower than the field

abi_marshal_wideselect_soa_transposed (2.07 ms) is 173.5x the fastest (11.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_wideselect_marshal_null} vs {abi_marshal_wideselect_soa_native, abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_transposed} (17125% apart)

The field splits into a fast tier {abi_marshal_wideselect_marshal_null} and a slow tier {abi_marshal_wideselect_soa_native, abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_transposed} with a 17125% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 173.5x the fastest

Fastest abi_marshal_wideselect_marshal_null (11.91 us) to slowest abi_marshal_wideselect_soa_transposed (2.07 ms): 173.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_wideselect_marshal_null** at 11905.6 ns median (-99.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 173.47x (fastest 11905.6 ns, slowest 2065290.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2058275ns | 2056558ns | 2055270ns | 2056261ns | 2062797ns | base |
| abi_marshal_wideselect_marshal_null | 14410ns | 14119ns | 13835ns | 14100ns | 15162ns | -99.30% |
| abi_marshal_wideselect_soa_native | 2052951ns | 2053290ns | 2045102ns | 2051630ns | 2058857ns | -0.26% |
| abi_marshal_wideselect_soa_transposed | 2081659ns | 2067850ns | 2060587ns | 2066292ns | 2115246ns | +1.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2055774ns | 2052828ns | 2060186ns | base | 0.000 |
| abi_marshal_wideselect_marshal_null | 12163ns | 11677ns | 12798ns | -99.41% | 0.000 |
| abi_marshal_wideselect_soa_native | 2050418ns | 2042690ns | 2056293ns | -0.26% | 0.000 |
| abi_marshal_wideselect_soa_transposed | 2078995ns | 2058037ns | 2112388ns | +1.13% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 40374.0 | 2057964.8 | 2055773.9 | n/a |
| abi_marshal_wideselect_marshal_null | 28040.2 | 12368.5 | 12163.2 | n/a |
| abi_marshal_wideselect_soa_native | 41039.5 | 2051463.9 | 2050417.6 | 0 |
| abi_marshal_wideselect_soa_transposed | 43345.6 | 2074511.5 | 2078994.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_wideselect_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_wideselect_aos | 0.000 | 0.6% |
| abi_marshal_wideselect_marshal_null | 0.000 | 98.1% |
| abi_marshal_wideselect_soa_native | 0.000 | 0.6% |
| abi_marshal_wideselect_soa_transposed | 0.000 | 0.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_wideselect_aos | 2058275ns | 2058275ns | base |
| abi_marshal_wideselect_marshal_null | 14410ns | 14410ns | -99.30% |
| abi_marshal_wideselect_soa_native | 2052951ns | 2052951ns | -0.26% |
| abi_marshal_wideselect_soa_transposed | 2081659ns | 2081659ns | +1.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2054115ns | base | --- | [2053021, 2060186] | --- | --- | --- | --- |
| abi_marshal_wideselect_marshal_null | 11906ns | -2042138.9ns (-99.4%) | [-2047577, -2041116]ns | [11786, 12798] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_wideselect_soa_native | 2050766ns | no significant difference | [-15260, +2179]ns | [2044193, 2056293] | no | 0.2188 | 0.2188 | 0 |
| abi_marshal_wideselect_soa_transposed | 2065291ns | no significant difference | [-647, +58634]ns | [2059305, 2112388] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_wideselect_aos | abi_marshal_wideselect_marshal_null | abi_marshal_wideselect_soa_native | abi_marshal_wideselect_soa_transposed |
|---|---|---|---|---|
| 1 | 2054214ns | -99.4% | -0.1% | +0.2% |
| 2 | 2053215ns | -99.4% | -0.4% | +0.8% |
| 3 | 2065691ns | -99.4% | -1.1% | -0.2% |
| 4 | 2054681ns | -99.4% | -0.1% | +0.8% |
| 5 | 2054016ns | -99.4% | +0.3% | +0.3% |
| 6 | 2052828ns | -99.4% | -0.2% | +4.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_wideselect_aos | -0.209 | moderate- |
| abi_marshal_wideselect_marshal_null | 0.101 | ok |
| abi_marshal_wideselect_soa_native | 0.103 | ok |
| abi_marshal_wideselect_soa_transposed | -0.103 | ok |

**Consistency summary:**

- **abi_marshal_wideselect_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_wideselect_soa_native**: won 4/6, lost 1/6
- **abi_marshal_wideselect_soa_transposed**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 6215013.3ns | 2055773.9ns | 302.3% | HIGH |
| abi_marshal_wideselect_marshal_null | 146301.9ns | 12163.2ns | 1202.8% | HIGH |
| abi_marshal_wideselect_soa_native | 6197640.1ns | 2050417.6ns | 302.3% | HIGH |
| abi_marshal_wideselect_soa_transposed | 6324747.5ns | 2078994.6ns | 304.2% | HIGH |

## Distribution (algo ns)

```
abi_marshal_wideselect_aos (n=6, range 2052827.5-2060185.8 ns)
  2052827.5 |####################
  2053195.4 |####################
  2053563.3 |
  2053931.2 |########################################
  2054299.2 |
  2054667.1 |####################
  2055035.0 |
  2055402.9 |
  2055770.8 |
  2056138.7 |
  2056506.6 |
  2056874.6 |
  2057242.5 |
  2057610.4 |
  2057978.3 |
  2058346.2 |
  2058714.1 |
  2059082.1 |
  2059450.0 |
  2059817.9 |
  (0 below, 1 above range)

abi_marshal_wideselect_marshal_null (n=6, range 11676.7-12798.1 ns)
  11676.7 |####################
  11732.8 |
  11788.8 |
  11844.9 |########################################
  11901.0 |####################
  11957.0 |
  12013.1 |
  12069.2 |
  12125.3 |
  12181.3 |
  12237.4 |
  12293.5 |
  12349.5 |
  12405.6 |
  12461.7 |
  12517.8 |
  12573.8 |
  12629.9 |
  12686.0 |####################
  12742.0 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_native (n=6, range 2042690.0-2056293.3 ns)
  2042690.0 |####################
  2043370.2 |
  2044050.3 |
  2044730.5 |
  2045410.7 |####################
  2046090.8 |
  2046771.0 |
  2047451.2 |
  2048131.3 |
  2048811.5 |####################
  2049491.6 |
  2050171.8 |
  2050852.0 |
  2051532.1 |
  2052212.3 |########################################
  2052892.5 |
  2053572.6 |
  2054252.8 |
  2054933.0 |
  2055613.1 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_transposed (n=6, range 2058036.7-2112388.0 ns)
  2058036.7 |########################################
  2060754.3 |####################
  2063471.8 |
  2066189.4 |
  2068906.9 |####################
  2071624.5 |####################
  2074342.1 |
  2077059.6 |
  2079777.2 |
  2082494.8 |
  2085212.3 |
  2087929.9 |
  2090647.5 |
  2093365.0 |
  2096082.6 |
  2098800.1 |
  2101517.7 |
  2104235.3 |
  2106952.8 |
  2109670.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_wideselect_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_marshal_null**: bridge=1230.3% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_native**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_transposed**: bridge=302.1% of algo (FFI overhead may distort results)

# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 17617% faster than the next best (abi_marshal_real_soa_native)

abi_marshal_real_marshal_null (12.18 us) leads abi_marshal_real_soa_native (2.16 ms) by 17617%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 99% (significant)

abi_marshal_real_marshal_null is -2.16 ms (99%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 178.6x slower than the field

abi_marshal_real_soa_transposed (2.18 ms) is 178.6x the fastest (12.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_real_soa_transposed shows alternating (throttle bounce) (autocorr -0.54)

abi_marshal_real_soa_transposed's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} (17617% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} with a 17617% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 178.6x the fastest

Fastest abi_marshal_real_marshal_null (12.18 us) to slowest abi_marshal_real_soa_transposed (2.18 ms): 178.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 12182.5 ns median (-99.4% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 178.55x (fastest 12182.5 ns, slowest 2175199.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2169397ns | 2170291ns | 2163940ns | 2169969ns | 2171267ns | base |
| abi_marshal_real_marshal_null | 14492ns | 14398ns | 14110ns | 14307ns | 14960ns | -99.33% |
| abi_marshal_real_soa_native | 2160896ns | 2161113ns | 2157143ns | 2159839ns | 2164358ns | -0.39% |
| abi_marshal_real_soa_transposed | 2177835ns | 2178119ns | 2169751ns | 2177465ns | 2182433ns | +0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2166537ns | 2161109ns | 2168387ns | base | 0.000 |
| abi_marshal_real_marshal_null | 12255ns | 11938ns | 12637ns | -99.43% | 0.000 |
| abi_marshal_real_soa_native | 2158174ns | 2154512ns | 2161584ns | -0.39% | 0.000 |
| abi_marshal_real_soa_transposed | 2174976ns | 2166952ns | 2179593ns | +0.39% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 49891.1 | 2166200.1 | 2166537.1 | n/a |
| abi_marshal_real_marshal_null | 27450.8 | 12387.9 | 12255.3 | n/a |
| abi_marshal_real_soa_native | 49600.4 | 2156427.9 | 2158174.2 | n/a |
| abi_marshal_real_soa_transposed | 52478.8 | 2174170.0 | 2174975.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.6% |
| abi_marshal_real_marshal_null | 0.000 | 98.0% |
| abi_marshal_real_soa_native | 0.000 | 0.6% |
| abi_marshal_real_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2169397ns | 2169397ns | base |
| abi_marshal_real_marshal_null | 14492ns | 14492ns | -99.33% |
| abi_marshal_real_soa_native | 2160896ns | 2160896ns | -0.39% |
| abi_marshal_real_soa_transposed | 2177835ns | 2177835ns | +0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2167535ns | base | --- | [2163690, 2168387] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 12182ns | -2155134.4ns (-99.4%) | [-2155999, -2151712]ns | [11946, 12637] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2158391ns | -9222.0ns (-0.4%) | [-12606, -3261]ns | [2154548, 2161584] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_transposed | 2175199ns | +7466.3ns (+0.3%) | [+2198, +15651]ns | [2170135, 2179593] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2161109ns | -99.4% | -0.0% | +0.7% |
| 2 | 2168738ns | -99.4% | -0.3% | +0.2% |
| 3 | 2168036ns | -99.4% | -0.6% | +0.4% |
| 4 | 2167933ns | -99.4% | -0.3% | +0.3% |
| 5 | 2166270ns | -99.4% | -0.5% | +0.8% |
| 6 | 2167136ns | -99.4% | -0.5% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | -0.182 | ok |
| abi_marshal_real_marshal_null | -0.205 | moderate- |
| abi_marshal_real_soa_native | -0.395 | moderate- |
| abi_marshal_real_soa_transposed | -0.535 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 5/6, lost 0/6
- **abi_marshal_real_soa_transposed**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6553094.5ns | 2166537.1ns | 302.5% | HIGH |
| abi_marshal_real_marshal_null | 145207.2ns | 12255.3ns | 1184.9% | HIGH |
| abi_marshal_real_soa_native | 6523476.5ns | 2158174.2ns | 302.3% | HIGH |
| abi_marshal_real_soa_transposed | 6578038.7ns | 2174975.8ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2161109.2-2168387.0 ns)
  2161109.2 |########################################
  2161473.1 |
  2161837.0 |
  2162200.9 |
  2162564.8 |
  2162928.7 |
  2163292.6 |
  2163656.4 |
  2164020.3 |
  2164384.2 |
  2164748.1 |
  2165112.0 |
  2165475.9 |
  2165839.8 |
  2166203.7 |########################################
  2166567.6 |
  2166931.5 |########################################
  2167295.4 |
  2167659.3 |########################################
  2168023.2 |########################################
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 11938.3-12637.1 ns)
  11938.3 |########################################
  11973.2 |####################
  12008.2 |
  12043.1 |
  12078.1 |
  12113.0 |
  12147.9 |
  12182.9 |
  12217.8 |
  12252.8 |
  12287.7 |
  12322.6 |
  12357.6 |####################
  12392.5 |
  12427.5 |
  12462.4 |
  12497.3 |
  12532.3 |####################
  12567.2 |
  12602.2 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2154511.7-2161584.1 ns)
  2154511.7 |########################################
  2154865.3 |
  2155218.9 |
  2155572.6 |
  2155926.2 |####################
  2156279.8 |
  2156633.4 |
  2156987.1 |
  2157340.7 |
  2157694.3 |
  2158047.9 |
  2158401.5 |
  2158755.2 |
  2159108.8 |
  2159462.4 |
  2159816.0 |
  2160169.7 |
  2160523.3 |####################
  2160876.9 |
  2161230.5 |####################
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2166952.5-2179593.1 ns)
  2166952.5 |####################
  2167584.5 |
  2168216.6 |
  2168848.6 |
  2169480.6 |
  2170112.7 |
  2170744.7 |
  2171376.7 |
  2172008.8 |
  2172640.8 |
  2173272.8 |####################
  2173904.9 |
  2174536.9 |####################
  2175168.9 |
  2175801.0 |########################################
  2176433.0 |
  2177065.0 |
  2177697.1 |
  2178329.1 |
  2178961.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=1194.2% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=302.5% of algo (FFI overhead may distort results)

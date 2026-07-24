# abi_marshal (real)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_real_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_real_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_real_marshal_null dominates: 20981% faster than the next best (abi_marshal_real_soa_native)

abi_marshal_real_marshal_null (10.15 us) leads abi_marshal_real_soa_native (2.14 ms) by 20981%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_real_marshal_null beats baseline by 100% (significant)

abi_marshal_real_marshal_null is -2.14 ms (100%) faster than baseline abi_marshal_real_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_real_soa_transposed is an outlier: 212.1x slower than the field

abi_marshal_real_soa_transposed (2.15 ms) is 212.1x the fastest (10.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_real_marshal_null} vs {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} (20981% apart)

The field splits into a fast tier {abi_marshal_real_marshal_null} and a slow tier {abi_marshal_real_soa_native, abi_marshal_real_aos, abi_marshal_real_soa_transposed} with a 20981% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 212.1x the fastest

Fastest abi_marshal_real_marshal_null (10.15 us) to slowest abi_marshal_real_soa_transposed (2.15 ms): 212.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_real_marshal_null** at 10147.9 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 212.15x (fastest 10147.9 ns, slowest 2152849.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2154489ns | 2155071ns | 2149336ns | 2154032ns | 2157750ns | base |
| abi_marshal_real_marshal_null | 12466ns | 12363ns | 12016ns | 12329ns | 12895ns | -99.42% |
| abi_marshal_real_soa_native | 2142459ns | 2142094ns | 2135689ns | 2140736ns | 2148429ns | -0.56% |
| abi_marshal_real_soa_transposed | 2156501ns | 2155720ns | 2150034ns | 2154132ns | 2163287ns | +0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_real_aos | 2151633ns | 2146579ns | 2154873ns | base | 0.000 |
| abi_marshal_real_marshal_null | 10201ns | 9853ns | 10531ns | -99.53% | 0.000 |
| abi_marshal_real_soa_native | 2139673ns | 2132965ns | 2145595ns | -0.56% | 0.000 |
| abi_marshal_real_soa_transposed | 2153601ns | 2146995ns | 2160339ns | +0.09% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_real_aos | 53831.9 | 2151685.3 | 2151633.4 | n/a |
| abi_marshal_real_marshal_null | 27962.5 | 10325.6 | 10201.3 | n/a |
| abi_marshal_real_soa_native | 51743.8 | 2140748.1 | 2139672.9 | n/a |
| abi_marshal_real_soa_transposed | 52160.8 | 2152280.1 | 2153601.1 | 1 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_real_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_real_aos | 0.000 | 0.5% |
| abi_marshal_real_marshal_null | 0.000 | 97.1% |
| abi_marshal_real_soa_native | 0.000 | 0.5% |
| abi_marshal_real_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_real_aos | 2154489ns | 2154489ns | base |
| abi_marshal_real_marshal_null | 12466ns | 12466ns | -99.42% |
| abi_marshal_real_soa_native | 2142459ns | 2142459ns | -0.56% |
| abi_marshal_real_soa_transposed | 2156501ns | 2156501ns | +0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_real_aos | 2152178ns | base | --- | [2147849, 2154873] | --- | --- | --- | --- |
| abi_marshal_real_marshal_null | 10148ns | -2142026.1ns (-99.5%) | [-2144656, -2137615]ns | [9925, 10531] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_native | 2139244ns | -11987.6ns (-0.6%) | [-15976, -7918]ns | [2134180, 2145595] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_real_soa_transposed | 2152849ns | no significant difference | [-1648, +6486]ns | [2147616, 2160339] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_real_aos | abi_marshal_real_marshal_null | abi_marshal_real_soa_native | abi_marshal_real_soa_transposed |
|---|---|---|---|---|
| 1 | 2149119ns | -99.5% | -0.6% | -0.1% |
| 2 | 2146579ns | -99.5% | -0.6% | +0.1% |
| 3 | 2151014ns | -99.5% | -0.4% | +0.1% |
| 4 | 2153343ns | -99.5% | -0.5% | +0.5% |
| 5 | 2153734ns | -99.5% | -0.9% | -0.1% |
| 6 | 2156011ns | -99.5% | -0.3% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_real_aos | 0.469 | moderate+ |
| abi_marshal_real_marshal_null | -0.359 | moderate- |
| abi_marshal_real_soa_native | -0.245 | moderate- |
| abi_marshal_real_soa_transposed | 0.098 | ok |

**Consistency summary:**

- **abi_marshal_real_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_real_soa_native**: won 6/6, lost 0/6
- **abi_marshal_real_soa_transposed**: won 0/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_real_aos | 6513369.1ns | 2151633.4ns | 302.7% | HIGH |
| abi_marshal_real_marshal_null | 140343.1ns | 10201.3ns | 1375.7% | HIGH |
| abi_marshal_real_soa_native | 6479125.5ns | 2139672.9ns | 302.8% | HIGH |
| abi_marshal_real_soa_transposed | 6515603.9ns | 2153601.1ns | 302.5% | HIGH |

## Distribution (algo ns)

```
abi_marshal_real_aos (n=6, range 2146579.2-2154872.7 ns)
  2146579.2 |########################################
  2146993.9 |
  2147408.6 |
  2147823.2 |
  2148237.9 |
  2148652.6 |
  2149067.2 |########################################
  2149481.9 |
  2149896.6 |
  2150311.3 |
  2150726.0 |########################################
  2151140.6 |
  2151555.3 |
  2151970.0 |
  2152384.7 |
  2152799.3 |
  2153214.0 |########################################
  2153628.7 |########################################
  2154043.4 |
  2154458.0 |
  (0 below, 1 above range)

abi_marshal_real_marshal_null (n=6, range 9852.9-10530.9 ns)
   9852.9 |########################################
   9886.8 |
   9920.7 |
   9954.6 |
   9988.5 |########################################
  10022.4 |
  10056.3 |
  10090.2 |########################################
  10124.1 |
  10158.0 |########################################
  10191.9 |
  10225.8 |
  10259.7 |
  10293.6 |
  10327.5 |
  10361.4 |
  10395.3 |
  10429.2 |
  10463.1 |########################################
  10497.0 |
  (0 below, 1 above range)

abi_marshal_real_soa_native (n=6, range 2132965.4-2145594.8 ns)
  2132965.4 |########################################
  2133596.9 |
  2134228.3 |
  2134859.8 |########################################
  2135491.3 |
  2136122.8 |########################################
  2136754.2 |
  2137385.7 |
  2138017.2 |
  2138648.6 |
  2139280.1 |
  2139911.6 |
  2140543.0 |
  2141174.5 |
  2141806.0 |########################################
  2142437.4 |########################################
  2143068.9 |
  2143700.4 |
  2144331.9 |
  2144963.3 |
  (0 below, 1 above range)

abi_marshal_real_soa_transposed (n=6, range 2146994.6-2160338.5 ns)
  2146994.6 |########################################
  2147661.8 |########################################
  2148329.0 |
  2148996.2 |
  2149663.4 |
  2150330.6 |
  2150997.8 |
  2151665.0 |
  2152332.2 |########################################
  2152999.4 |########################################
  2153666.6 |
  2154333.8 |
  2155001.0 |
  2155668.2 |
  2156335.4 |########################################
  2157002.6 |
  2157669.8 |
  2158337.0 |
  2159004.2 |
  2159671.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_real_aos**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_marshal_real_marshal_null**: bridge=1377.1% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_native**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_marshal_real_soa_transposed**: bridge=302.3% of algo (FFI overhead may distort results)

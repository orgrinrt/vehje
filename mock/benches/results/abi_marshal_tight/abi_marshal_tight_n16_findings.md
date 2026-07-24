# abi_marshal (tight)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_tight_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_tight_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_tight_marshal_null dominates: 7271% faster than the next best (abi_marshal_tight_aos)

abi_marshal_tight_marshal_null (27.49 us) leads abi_marshal_tight_aos (2.03 ms) by 7271%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_tight_marshal_null beats baseline by 99% (significant)

abi_marshal_tight_marshal_null is -2.00 ms (99%) faster than baseline abi_marshal_tight_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_tight_soa_transposed is an outlier: 77.0x slower than the field

abi_marshal_tight_soa_transposed (2.12 ms) is 77.0x the fastest (27.49 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_tight_marshal_null} vs {abi_marshal_tight_aos, abi_marshal_tight_soa_native, abi_marshal_tight_soa_transposed} (7271% apart)

The field splits into a fast tier {abi_marshal_tight_marshal_null} and a slow tier {abi_marshal_tight_aos, abi_marshal_tight_soa_native, abi_marshal_tight_soa_transposed} with a 7271% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 77.0x the fastest

Fastest abi_marshal_tight_marshal_null (27.49 us) to slowest abi_marshal_tight_soa_transposed (2.12 ms): 77.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_tight_marshal_null** at 27488.9 ns median (-98.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 76.96x (fastest 27488.9 ns, slowest 2115502.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2032976ns | 2028619ns | 2021972ns | 2026445ns | 2048273ns | base |
| abi_marshal_tight_marshal_null | 29596ns | 29759ns | 28636ns | 29521ns | 30190ns | -98.54% |
| abi_marshal_tight_soa_native | 2111405ns | 2081144ns | 2075555ns | 2079597ns | 2177043ns | +3.86% |
| abi_marshal_tight_soa_transposed | 2123879ns | 2118201ns | 2111520ns | 2117008ns | 2140365ns | +4.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2030418ns | 2019442ns | 2045512ns | base | 0.000 |
| abi_marshal_tight_marshal_null | 27328ns | 26435ns | 27861ns | -98.65% | 0.001 |
| abi_marshal_tight_soa_native | 2108857ns | 2073019ns | 2174321ns | +3.86% | 0.000 |
| abi_marshal_tight_soa_transposed | 2121268ns | 2109108ns | 2137781ns | +4.47% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_tight_aos | 40816.7 | 2031239.8 | 2030418.5 | n/a |
| abi_marshal_tight_marshal_null | 27586.4 | 27880.4 | 27328.5 | n/a |
| abi_marshal_tight_soa_native | 41984.4 | 2101534.6 | 2108856.6 | n/a |
| abi_marshal_tight_soa_transposed | 39837.1 | 2120338.7 | 2121268.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_marshal_tight_marshal_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_tight_aos | 0.000 | 1.3% |
| abi_marshal_tight_marshal_null | 0.001 | 96.2% |
| abi_marshal_tight_soa_native | 0.000 | 1.3% |
| abi_marshal_tight_soa_transposed | 0.000 | 1.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_tight_aos | 2032976ns | 2032976ns | base |
| abi_marshal_tight_marshal_null | 29596ns | 29596ns | -98.54% |
| abi_marshal_tight_soa_native | 2111405ns | 2111405ns | +3.86% |
| abi_marshal_tight_soa_transposed | 2123879ns | 2123879ns | +4.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2026204ns | base | --- | [2019539, 2045512] | --- | --- | --- | --- |
| abi_marshal_tight_marshal_null | 27489ns | -1999066.0ns (-98.7%) | [-2017689, -1992515]ns | [26635, 27861] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_tight_soa_native | 2078736ns | +55879.6ns (+2.8%) | [+45152, +134282]ns | [2073513, 2174321] | YES | 0.0313 | 0.0313 | 0 |
| abi_marshal_tight_soa_transposed | 2115503ns | +90116.0ns (+4.4%) | [+87800, +94634]ns | [2110521, 2137781] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_tight_aos | abi_marshal_tight_marshal_null | abi_marshal_tight_soa_native | abi_marshal_tight_soa_transposed |
|---|---|---|---|---|
| 1 | 2060442ns | -98.6% | +9.8% | +4.6% |
| 2 | 2030583ns | -98.6% | +2.3% | +4.5% |
| 3 | 2019636ns | -98.7% | +3.3% | +4.7% |
| 4 | 2023293ns | -98.6% | +2.8% | +4.4% |
| 5 | 2029115ns | -98.7% | +2.2% | +4.3% |
| 6 | 2019442ns | -98.6% | +2.7% | +4.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_tight_aos | 0.087 | ok |
| abi_marshal_tight_marshal_null | -0.178 | ok |
| abi_marshal_tight_soa_native | -0.045 | ok |
| abi_marshal_tight_soa_transposed | 0.120 | ok |

**Consistency summary:**

- **abi_marshal_tight_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_tight_soa_native**: won 0/6, lost 6/6
- **abi_marshal_tight_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_tight_aos | 6138492.4ns | 2030418.5ns | 302.3% | HIGH |
| abi_marshal_tight_marshal_null | 191553.1ns | 27328.5ns | 700.9% | HIGH |
| abi_marshal_tight_soa_native | 6360693.5ns | 2108856.6ns | 301.6% | HIGH |
| abi_marshal_tight_soa_transposed | 6404131.5ns | 2121268.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_marshal_tight_aos (n=6, range 2019442.1-2045512.5 ns)
  2019442.1 |########################################
  2020745.6 |
  2022049.1 |####################
  2023352.7 |
  2024656.2 |
  2025959.7 |
  2027263.2 |
  2028566.7 |####################
  2029870.3 |####################
  2031173.8 |
  2032477.3 |
  2033780.8 |
  2035084.3 |
  2036387.9 |
  2037691.4 |
  2038994.9 |
  2040298.4 |
  2041601.9 |
  2042905.5 |
  2044209.0 |
  (0 below, 1 above range)

abi_marshal_tight_marshal_null (n=6, range 26435.0-27861.0 ns)
  26435.0 |########################################
  26506.3 |
  26577.6 |
  26648.9 |
  26720.2 |
  26791.5 |########################################
  26862.8 |
  26934.1 |
  27005.4 |
  27076.7 |
  27148.0 |
  27219.3 |
  27290.6 |
  27361.9 |
  27433.2 |########################################
  27504.5 |########################################
  27575.8 |########################################
  27647.1 |
  27718.4 |
  27789.7 |
  (0 below, 1 above range)

abi_marshal_tight_soa_native (n=6, range 2073018.7-2174321.2 ns)
  2073018.7 |########################################
  2078083.8 |#############
  2083149.0 |#############
  2088214.1 |
  2093279.2 |
  2098344.3 |
  2103409.5 |
  2108474.6 |
  2113539.7 |
  2118604.8 |
  2123670.0 |
  2128735.1 |
  2133800.2 |
  2138865.4 |
  2143930.5 |
  2148995.6 |
  2154060.7 |
  2159125.9 |
  2164191.0 |
  2169256.1 |
  (0 below, 1 above range)

abi_marshal_tight_soa_transposed (n=6, range 2109107.9-2137781.0 ns)
  2109107.9 |####################
  2110541.6 |####################
  2111975.2 |
  2113408.9 |
  2114842.5 |########################################
  2116276.2 |
  2117709.8 |
  2119143.5 |
  2120577.2 |####################
  2122010.8 |
  2123444.5 |
  2124878.1 |
  2126311.8 |
  2127745.4 |
  2129179.1 |
  2130612.8 |
  2132046.4 |
  2133480.1 |
  2134913.7 |
  2136347.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_tight_aos**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_tight_marshal_null**: bridge=700.9% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_native**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)

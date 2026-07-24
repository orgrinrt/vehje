# abi_marshal (tight)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_tight_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_tight_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_tight_marshal_null dominates: 9762% faster than the next best (abi_marshal_tight_aos)

abi_marshal_tight_marshal_null (20.52 us) leads abi_marshal_tight_aos (2.02 ms) by 9762%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_tight_marshal_null beats baseline by 99% (significant)

abi_marshal_tight_marshal_null is -2.00 ms (99%) faster than baseline abi_marshal_tight_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_tight_soa_transposed is an outlier: 100.4x slower than the field

abi_marshal_tight_soa_transposed (2.06 ms) is 100.4x the fastest (20.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_tight_marshal_null shows alternating (throttle bounce) (autocorr -0.59)

abi_marshal_tight_marshal_null's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_tight_marshal_null} vs {abi_marshal_tight_aos, abi_marshal_tight_soa_native, abi_marshal_tight_soa_transposed} (9762% apart)

The field splits into a fast tier {abi_marshal_tight_marshal_null} and a slow tier {abi_marshal_tight_aos, abi_marshal_tight_soa_native, abi_marshal_tight_soa_transposed} with a 9762% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 100.4x the fastest

Fastest abi_marshal_tight_marshal_null (20.52 us) to slowest abi_marshal_tight_soa_transposed (2.06 ms): 100.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_tight_marshal_null** at 20518.8 ns median (-99.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 100.38x (fastest 20518.8 ns, slowest 2059742.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2025343ns | 2026121ns | 2019431ns | 2023980ns | 2030345ns | base |
| abi_marshal_tight_marshal_null | 22788ns | 22796ns | 22059ns | 22623ns | 23400ns | -98.87% |
| abi_marshal_tight_soa_native | 2038137ns | 2041025ns | 2023716ns | 2037411ns | 2046435ns | +0.63% |
| abi_marshal_tight_soa_transposed | 2060570ns | 2062309ns | 2051179ns | 2060084ns | 2065995ns | +1.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2022784ns | 2016943ns | 2027778ns | base | 0.000 |
| abi_marshal_tight_marshal_null | 20534ns | 19904ns | 21101ns | -98.98% | 0.000 |
| abi_marshal_tight_soa_native | 2035470ns | 2021083ns | 2043609ns | +0.63% | 0.000 |
| abi_marshal_tight_soa_transposed | 2058021ns | 2048667ns | 2063395ns | +1.74% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_tight_aos | 41091.3 | 2025781.7 | 2022783.5 | n/a |
| abi_marshal_tight_marshal_null | 27555.3 | 20755.5 | 20534.0 | n/a |
| abi_marshal_tight_soa_native | 41856.8 | 2036068.8 | 2035469.8 | n/a |
| abi_marshal_tight_soa_transposed | 40690.3 | 2057362.0 | 2058021.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_tight_marshal_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_tight_aos | 0.000 | 1.0% |
| abi_marshal_tight_marshal_null | 0.000 | 97.0% |
| abi_marshal_tight_soa_native | 0.000 | 1.0% |
| abi_marshal_tight_soa_transposed | 0.000 | 1.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_tight_aos | 2025343ns | 2025343ns | base |
| abi_marshal_tight_marshal_null | 22788ns | 22788ns | -98.87% |
| abi_marshal_tight_soa_native | 2038137ns | 2038137ns | +0.63% |
| abi_marshal_tight_soa_transposed | 2060570ns | 2060570ns | +1.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2023562ns | base | --- | [2017011, 2027778] | --- | --- | --- | --- |
| abi_marshal_tight_marshal_null | 20519ns | -2002852.5ns (-99.0%) | [-2007485, -1996411]ns | [19982, 21101] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_tight_soa_native | 2038447ns | +11788.1ns (+0.6%) | [+2564, +23706]ns | [2024353, 2043609] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_marshal_tight_soa_transposed | 2059742ns | +34727.1ns (+1.7%) | [+31184, +39801]ns | [2050925, 2063395] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_tight_aos | abi_marshal_tight_marshal_null | abi_marshal_tight_soa_native | abi_marshal_tight_soa_transposed |
|---|---|---|---|---|
| 1 | 2026209ns | -99.0% | +0.6% | +1.5% |
| 2 | 2026500ns | -99.0% | -0.3% | +1.8% |
| 3 | 2017078ns | -99.0% | +0.5% | +1.6% |
| 4 | 2020915ns | -99.0% | +1.2% | +2.1% |
| 5 | 2029056ns | -99.0% | +0.6% | +1.6% |
| 6 | 2016943ns | -98.9% | +1.1% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_tight_aos | -0.342 | moderate- |
| abi_marshal_tight_marshal_null | -0.593 | HIGH- (thermal bounce) |
| abi_marshal_tight_soa_native | 0.193 | ok |
| abi_marshal_tight_soa_transposed | -0.517 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_marshal_tight_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_tight_soa_native**: won 1/6, lost 5/6
- **abi_marshal_tight_soa_transposed**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_tight_aos | 6117892.8ns | 2022783.5ns | 302.4% | HIGH |
| abi_marshal_tight_marshal_null | 170267.6ns | 20534.0ns | 829.2% | HIGH |
| abi_marshal_tight_soa_native | 6154025.5ns | 2035469.8ns | 302.3% | HIGH |
| abi_marshal_tight_soa_transposed | 6215354.5ns | 2058021.1ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_marshal_tight_aos (n=6, range 2016943.3-2027777.9 ns)
  2016943.3 |########################################
  2017485.0 |
  2018026.8 |
  2018568.5 |
  2019110.2 |
  2019651.9 |
  2020193.7 |
  2020735.4 |####################
  2021277.1 |
  2021818.9 |
  2022360.6 |
  2022902.3 |
  2023444.1 |
  2023985.8 |
  2024527.5 |
  2025069.2 |
  2025611.0 |
  2026152.7 |########################################
  2026694.4 |
  2027236.2 |
  (0 below, 1 above range)

abi_marshal_tight_marshal_null (n=6, range 19903.8-21100.6 ns)
  19903.8 |########################################
  19963.6 |
  20023.5 |########################################
  20083.3 |
  20143.2 |
  20203.0 |########################################
  20262.8 |
  20322.7 |
  20382.5 |
  20442.4 |
  20502.2 |
  20562.0 |
  20621.9 |
  20681.7 |
  20741.6 |
  20801.4 |########################################
  20861.2 |########################################
  20921.1 |
  20980.9 |
  21040.8 |
  (0 below, 1 above range)

abi_marshal_tight_soa_native (n=6, range 2021083.3-2043609.4 ns)
  2021083.3 |########################################
  2022209.6 |
  2023335.9 |
  2024462.2 |
  2025588.5 |
  2026714.8 |########################################
  2027841.1 |
  2028967.4 |
  2030093.7 |
  2031220.0 |
  2032346.3 |
  2033472.6 |
  2034598.9 |
  2035725.2 |
  2036851.5 |########################################
  2037977.8 |
  2039104.1 |########################################
  2040230.4 |########################################
  2041356.7 |
  2042483.0 |
  (0 below, 1 above range)

abi_marshal_tight_soa_transposed (n=6, range 2048666.7-2063395.2 ns)
  2048666.7 |####################
  2049403.1 |
  2050139.6 |
  2050876.0 |
  2051612.4 |
  2052348.8 |
  2053085.3 |####################
  2053821.7 |
  2054558.1 |
  2055294.5 |
  2056031.0 |
  2056767.4 |####################
  2057503.8 |
  2058240.3 |
  2058976.7 |
  2059713.1 |
  2060449.5 |
  2061186.0 |
  2061922.4 |########################################
  2062658.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_tight_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_tight_marshal_null**: bridge=832.4% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_native**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_transposed**: bridge=301.8% of algo (FFI overhead may distort results)

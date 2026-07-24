# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_cold_scalar is an outlier: 773.9x slower than the field

abi_cross_cold_tight_cold_scalar (2.05 ms) is 773.9x the fastest (2.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (74463% apart)

The field splits into a fast tier {abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 74463% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 773.9x the fastest

Fastest abi_cross_cold_tight_cold_null (2.65 us) to slowest abi_cross_cold_tight_cold_scalar (2.05 ms): 773.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_cold_tight_cold_null** at 2654.8 ns median (-3.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 773.95x (fastest 2654.8 ns, slowest 2054679.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 4896ns | 4897ns | 4784ns | 4891ns | 4960ns | -4.82% |
| abi_cross_cold_tight_cold_scalar | 2075150ns | 2057978ns | 2041598ns | 2053218ns | 2124822ns | +40238.81% |
| abi_cross_cold_tight_warm_null | 5144ns | 5067ns | 4914ns | 5021ns | 5445ns | base |
| abi_cross_cold_tight_warm_scalar | 2062444ns | 2056303ns | 2045995ns | 2053790ns | 2083649ns | +39991.82% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 2651ns | 2611ns | 2686ns | -5.22% | 0.048 |
| abi_cross_cold_tight_cold_scalar | 2071594ns | 2038246ns | 2120983ns | +73971.92% | 0.000 |
| abi_cross_cold_tight_warm_null | 2797ns | 2665ns | 2963ns | base | 0.046 |
| abi_cross_cold_tight_warm_scalar | 2058999ns | 2042695ns | 2080048ns | +73521.56% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 31610.1 | 2693.5 | 2650.8 | 15 |
| abi_cross_cold_tight_cold_scalar | 74006.8 | 2072344.0 | 2071594.2 | n/a |
| abi_cross_cold_tight_warm_null | 29368.3 | 2832.2 | 2796.7 | n/a |
| abi_cross_cold_tight_warm_scalar | 66829.2 | 2056667.5 | 2058998.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_cross_cold_tight_cold_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.048 | 98.3% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_tight_warm_null | 0.046 | 94.8% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 4896ns | 4896ns | -4.82% |
| abi_cross_cold_tight_cold_scalar | 2075150ns | 2075150ns | +40238.81% |
| abi_cross_cold_tight_warm_null | 5144ns | 5144ns | base |
| abi_cross_cold_tight_warm_scalar | 2062444ns | 2062444ns | +39991.82% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 2753ns | base | --- | [2674, 2963] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 2655ns | -87.3ns (-3.2%) | [-341, -10]ns | [2612, 2686] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_cold_tight_cold_scalar | 2054680ns | +2051949.1ns (+74531.0%) | [+2036379, +2118065]ns | [2039120, 2120983] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2052819ns | +2050077.9ns (+74463.0%) | [+2041168, +2077360]ns | [2044130, 2080048] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 2665ns | +0.5% | +77175.2% | +77597.5% |
| 2 | 2797ns | -3.7% | +73199.8% | +72939.5% |
| 3 | 2800ns | -6.7% | +72704.9% | +73436.4% |
| 4 | 2710ns | -1.2% | +80014.9% | +77013.1% |
| 5 | 3126ns | -15.8% | +66152.1% | +65332.9% |
| 6 | 2683ns | -2.6% | +75925.6% | +76183.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.199 | ok |
| abi_cross_cold_tight_cold_scalar | -0.187 | ok |
| abi_cross_cold_tight_warm_null | -0.453 | moderate- |
| abi_cross_cold_tight_warm_scalar | -0.266 | moderate- |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 5/6, lost 1/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 123254.1ns | 2650.8ns | 4649.8% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6292892.7ns | 2071594.2ns | 303.8% | HIGH |
| abi_cross_cold_tight_warm_null | 121754.4ns | 2796.7ns | 4353.5% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6238610.3ns | 2058998.8ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 2610.8-2685.9 ns)
   2610.8 |########################################
   2614.6 |
   2618.3 |
   2622.1 |
   2625.8 |
   2629.6 |####################
   2633.3 |
   2637.1 |
   2640.8 |
   2644.6 |
   2648.3 |
   2652.1 |
   2655.8 |
   2659.6 |
   2663.3 |
   2667.1 |
   2670.8 |
   2674.6 |####################
   2678.3 |####################
   2682.1 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2038245.8-2120982.7 ns)
  2038245.8 |########################################
  2042382.6 |
  2046519.5 |####################
  2050656.3 |
  2054793.2 |
  2058930.0 |####################
  2063066.9 |
  2067203.7 |####################
  2071340.6 |
  2075477.4 |
  2079614.2 |
  2083751.1 |
  2087887.9 |
  2092024.8 |
  2096161.6 |
  2100298.5 |
  2104435.3 |
  2108572.2 |
  2112709.0 |
  2116845.9 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 2665.0-2962.9 ns)
   2665.0 |########################################
   2679.9 |########################################
   2694.8 |########################################
   2709.7 |
   2724.6 |
   2739.5 |
   2754.4 |
   2769.3 |
   2784.2 |########################################
   2799.1 |########################################
   2813.9 |
   2828.8 |
   2843.7 |
   2858.6 |
   2873.5 |
   2888.4 |
   2903.3 |
   2918.2 |
   2933.1 |
   2948.0 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2042694.6-2080047.5 ns)
  2042694.6 |########################################
  2044562.2 |########################################
  2046429.9 |########################################
  2048297.5 |
  2050165.2 |
  2052032.8 |
  2053900.5 |
  2055768.1 |
  2057635.8 |########################################
  2059503.4 |
  2061371.1 |
  2063238.7 |
  2065106.3 |
  2066974.0 |
  2068841.6 |########################################
  2070709.3 |
  2072576.9 |
  2074444.6 |
  2076312.2 |
  2078179.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=4618.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=4377.9% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=303.0% of algo (FFI overhead may distort results)

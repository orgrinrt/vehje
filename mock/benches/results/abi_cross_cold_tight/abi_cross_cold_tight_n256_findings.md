# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_cold_scalar is an outlier: 646.7x slower than the field

abi_cross_cold_tight_cold_scalar (2.05 ms) is 646.7x the fastest (3.17 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null) are a dead heat (<1%)

abi_cross_cold_tight_warm_null (3.17 us) and abi_cross_cold_tight_cold_null (3.17 us) differ by 0.03%, inside the noise, even though the wider field spreads 64571.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (abi_cross_cold_tight_warm_null)

The baseline abi_cross_cold_tight_warm_null is the fastest (3.17 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (64481% apart)

The field splits into a fast tier {abi_cross_cold_tight_warm_null, abi_cross_cold_tight_cold_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 64481% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 646.7x the fastest

Fastest abi_cross_cold_tight_warm_null (3.17 us) to slowest abi_cross_cold_tight_cold_scalar (2.05 ms): 646.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_tight_cold_null's edge over baseline is significant but tiny (-29 ns, 0.93%)

abi_cross_cold_tight_cold_null differs from baseline abi_cross_cold_tight_warm_null by -29 ns (0.93%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (abi_cross_cold_tight_warm_null) is the fastest** at 3171.7 ns median
- 2 variants significantly slower than baseline
- Spread: 646.72x (fastest 3171.7 ns, slowest 2051196.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5493ns | 5483ns | 5389ns | 5452ns | 5605ns | -0.95% |
| abi_cross_cold_tight_cold_scalar | 2051434ns | 2054769ns | 2042053ns | 2051340ns | 2056266ns | +36893.44% |
| abi_cross_cold_tight_warm_null | 5545ns | 5531ns | 5378ns | 5487ns | 5716ns | base |
| abi_cross_cold_tight_warm_scalar | 2051625ns | 2052509ns | 2042293ns | 2049962ns | 2058786ns | +36896.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 3170ns | 3129ns | 3206ns | -0.54% | 0.081 |
| abi_cross_cold_tight_cold_scalar | 2047850ns | 2038457ns | 2052613ns | +64146.28% | 0.000 |
| abi_cross_cold_tight_warm_null | 3187ns | 3134ns | 3248ns | base | 0.080 |
| abi_cross_cold_tight_warm_scalar | 2047954ns | 2038698ns | 2055117ns | +64149.53% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 32776.8 | 3170.4 | 3170.2 | 215 |
| abi_cross_cold_tight_cold_scalar | 78760.8 | 2045885.4 | 2047850.0 | n/a |
| abi_cross_cold_tight_warm_null | 29059.8 | 3181.4 | 3187.5 | n/a |
| abi_cross_cold_tight_warm_scalar | 72313.3 | 2049159.9 | 2047953.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_cross_cold_tight_cold_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.081 | 98.6% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.2% |
| abi_cross_cold_tight_warm_null | 0.081 | 98.7% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 5493ns | 5493ns | -0.95% |
| abi_cross_cold_tight_cold_scalar | 2051434ns | 2051434ns | +36893.44% |
| abi_cross_cold_tight_warm_null | 5545ns | 5545ns | base |
| abi_cross_cold_tight_warm_scalar | 2051625ns | 2051625ns | +36896.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 3172ns | base | --- | [3143, 3248] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 3172ns | no significant difference | [-73, +50]ns | [3132, 3206] | no | 0.6875 | 0.6875 | 0 |
| abi_cross_cold_tight_cold_scalar | 2051197ns | +2048043.1ns (+64572.4%) | [+2036514, +2049431]ns | [2039740, 2052613] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2048819ns | +2045636.7ns (+64496.5%) | [+2036754, +2051908]ns | [2039925, 2055117] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 3265ns | -1.5% | +62419.9% | +62876.5% |
| 2 | 3152ns | -0.7% | +64923.9% | +65064.3% |
| 3 | 3155ns | +1.2% | +64964.2% | +64604.0% |
| 4 | 3134ns | +2.0% | +65398.1% | +65177.4% |
| 5 | 3231ns | -3.0% | +63435.7% | +63414.9% |
| 6 | 3189ns | -1.1% | +63825.5% | +63833.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.390 | moderate- |
| abi_cross_cold_tight_cold_scalar | -0.027 | ok |
| abi_cross_cold_tight_warm_null | -0.157 | ok |
| abi_cross_cold_tight_warm_scalar | -0.093 | ok |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 4/6, lost 2/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 126375.2ns | 3170.2ns | 3986.3% | HIGH |
| abi_cross_cold_tight_cold_scalar | 6219186.7ns | 2047850.0ns | 303.7% | HIGH |
| abi_cross_cold_tight_warm_null | 122839.6ns | 3187.5ns | 3853.8% | HIGH |
| abi_cross_cold_tight_warm_scalar | 6218120.4ns | 2047953.7ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 3129.2-3205.8 ns)
   3129.2 |########################################
   3133.0 |########################################
   3136.9 |
   3140.7 |
   3144.5 |
   3148.4 |
   3152.2 |########################################
   3156.0 |
   3159.9 |
   3163.7 |
   3167.5 |
   3171.4 |
   3175.2 |
   3179.0 |
   3182.9 |
   3186.7 |
   3190.5 |########################################
   3194.4 |########################################
   3198.2 |
   3202.0 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2038456.7-2052613.1 ns)
  2038456.7 |####################
  2039164.5 |
  2039872.3 |
  2040580.2 |####################
  2041288.0 |
  2041995.8 |
  2042703.6 |
  2043411.5 |
  2044119.3 |
  2044827.1 |
  2045534.9 |
  2046242.7 |
  2046950.6 |
  2047658.4 |
  2048366.2 |
  2049074.0 |
  2049781.9 |####################
  2050489.7 |
  2051197.5 |
  2051905.3 |########################################
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 3133.7-3247.7 ns)
   3133.7 |####################
   3139.4 |
   3145.1 |
   3150.8 |########################################
   3156.5 |
   3162.2 |
   3167.9 |
   3173.6 |
   3179.3 |
   3185.0 |####################
   3190.7 |
   3196.4 |
   3202.1 |
   3207.8 |
   3213.5 |
   3219.2 |
   3224.9 |
   3230.6 |####################
   3236.3 |
   3242.0 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2038697.5-2055116.9 ns)
  2038697.5 |########################################
  2039518.5 |
  2040339.4 |########################################
  2041160.4 |
  2041981.4 |
  2042802.3 |
  2043623.3 |
  2044444.3 |
  2045265.2 |########################################
  2046086.2 |
  2046907.2 |
  2047728.1 |
  2048549.1 |
  2049370.1 |
  2050191.0 |
  2051012.0 |
  2051833.0 |########################################
  2052653.9 |
  2053474.9 |
  2054295.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: bridge=3987.7% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: bridge=3886.3% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=303.5% of algo (FFI overhead may distort results)

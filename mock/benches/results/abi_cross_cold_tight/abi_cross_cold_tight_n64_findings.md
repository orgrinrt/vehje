# abi_cross_cold (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_cold_tight_warm_null**

## Highlights

Baseline for all deltas below: **abi_cross_cold_tight_warm_null**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_cold_tight_cold_scalar is an outlier: 863.8x slower than the field

abi_cross_cold_tight_cold_scalar (2.35 ms) is 863.8x the fastest (2.72 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_cold_tight_cold_null is fastest but the noisiest (CV 54.5%)

abi_cross_cold_tight_cold_null wins on median (2.72 us) yet has the highest variance (CV 54.5%), while abi_cross_cold_tight_cold_scalar is the steadiest (CV 5.0%, 2.35 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null) are a dead heat (<1%)

abi_cross_cold_tight_cold_null (2.72 us) and abi_cross_cold_tight_warm_null (2.75 us) differ by 0.83%, inside the noise, even though the wider field spreads 86283.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null} vs {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} (85533% apart)

The field splits into a fast tier {abi_cross_cold_tight_cold_null, abi_cross_cold_tight_warm_null} and a slow tier {abi_cross_cold_tight_warm_scalar, abi_cross_cold_tight_cold_scalar} with a 85533% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 863.8x the fastest

Fastest abi_cross_cold_tight_cold_null (2.72 us) to slowest abi_cross_cold_tight_cold_scalar (2.35 ms): 863.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_cross_cold_tight_cold_null is inconsistent: worst-20% is 1.8x its best-20%

abi_cross_cold_tight_cold_null's best 20% of batches run at 2.56 us but its worst 20% at 4.68 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### abi_cross_cold_tight_cold_null's edge over baseline is significant but tiny (-20 ns, 0.71%)

abi_cross_cold_tight_cold_null differs from baseline abi_cross_cold_tight_warm_null by -20 ns (0.71%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_cross_cold_tight_cold_null** at 2722.9 ns median (-0.8% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 863.83x (fastest 2722.9 ns, slowest 2352136.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 6370ns | 5256ns | 4867ns | 5139ns | 8970ns | +7.61% |
| abi_cross_cold_tight_cold_scalar | 2383059ns | 2357552ns | 2259492ns | 2333178ns | 2519665ns | +40155.85% |
| abi_cross_cold_tight_warm_null | 5920ns | 5303ns | 4899ns | 5175ns | 7547ns | base |
| abi_cross_cold_tight_warm_scalar | 2388691ns | 2357767ns | 2165720ns | 2330458ns | 2587526ns | +40250.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 3322ns | 2560ns | 4682ns | +4.70% | 0.019 |
| abi_cross_cold_tight_cold_scalar | 2377078ns | 2254361ns | 2512399ns | +74829.17% | 0.000 |
| abi_cross_cold_tight_warm_null | 3172ns | 2537ns | 4226ns | base | 0.020 |
| abi_cross_cold_tight_warm_scalar | 2382745ns | 2160872ns | 2581401ns | +75007.81% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 53407.2 | 3967.7 | 3321.7 | n/a |
| abi_cross_cold_tight_cold_scalar | 126641.2 | 2385348.2 | 2377077.9 | n/a |
| abi_cross_cold_tight_warm_null | 40726.5 | 3338.6 | 3172.4 | n/a |
| abi_cross_cold_tight_warm_scalar | 114646.6 | 2343717.6 | 2382745.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.025 Gops/s** (abi_cross_cold_tight_warm_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_cold_tight_cold_null | 0.024 | 93.2% |
| abi_cross_cold_tight_cold_scalar | 0.000 | 0.1% |
| abi_cross_cold_tight_warm_null | 0.023 | 92.4% |
| abi_cross_cold_tight_warm_scalar | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_cold_tight_cold_null | 6370ns | 6370ns | +7.61% |
| abi_cross_cold_tight_cold_scalar | 2383059ns | 2383059ns | +40155.85% |
| abi_cross_cold_tight_warm_null | 5920ns | 5920ns | base |
| abi_cross_cold_tight_warm_scalar | 2388691ns | 2388691ns | +40250.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_cold_tight_warm_null | 2746ns | base | --- | [2545, 4226] | --- | --- | --- | --- |
| abi_cross_cold_tight_cold_null | 2723ns | no significant difference | [-1484, +1951]ns | [2560, 4682] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_cold_tight_cold_scalar | 2352136ns | +2349556.2ns (+85575.3%) | [+2262484, +2509676]ns | [2266698, 2512399] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_cold_tight_warm_scalar | 2351153ns | +2347078.0ns (+85485.1%) | [+2212962, +2578678]ns | [2215681, 2581401] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_cold_tight_warm_null | abi_cross_cold_tight_cold_null | abi_cross_cold_tight_cold_scalar | abi_cross_cold_tight_warm_scalar |
|---|---|---|---|---|
| 1 | 2909ns | +128.0% | +88820.8% | +86393.0% |
| 2 | 5543ns | -50.8% | +41013.3% | +41847.0% |
| 3 | 2537ns | +7.0% | +95990.6% | +104214.0% |
| 4 | 2554ns | +0.3% | +94748.3% | +88806.3% |
| 5 | 2885ns | -5.4% | +78040.8% | +74800.3% |
| 6 | 2606ns | -1.8% | +87461.8% | +91107.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_cold_tight_cold_null | -0.018 | ok |
| abi_cross_cold_tight_cold_scalar | -0.212 | moderate- |
| abi_cross_cold_tight_warm_null | -0.203 | moderate- |
| abi_cross_cold_tight_warm_scalar | -0.172 | ok |

**Consistency summary:**

- **abi_cross_cold_tight_cold_null**: won 3/6, lost 3/6
- **abi_cross_cold_tight_cold_scalar**: won 0/6, lost 6/6
- **abi_cross_cold_tight_warm_scalar**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_cold_tight_cold_null | 148806.8ns | 3321.7ns | 4479.9% | HIGH |
| abi_cross_cold_tight_cold_scalar | 7261979.6ns | 2377077.9ns | 305.5% | HIGH |
| abi_cross_cold_tight_warm_null | 131101.5ns | 3172.4ns | 4132.5% | HIGH |
| abi_cross_cold_tight_warm_scalar | 7199248.7ns | 2382745.1ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_cold_tight_cold_null (n=6, range 2560.0-4681.6 ns)
   2560.0 |##########################
   2666.1 |########################################
   2772.2 |
   2878.2 |
   2984.3 |
   3090.4 |
   3196.5 |
   3302.6 |
   3408.7 |
   3514.7 |
   3620.8 |
   3726.9 |
   3833.0 |
   3939.1 |
   4045.2 |
   4151.2 |
   4257.3 |
   4363.4 |
   4469.5 |
   4575.6 |
  (0 below, 1 above range)

abi_cross_cold_tight_cold_scalar (n=6, range 2254361.2-2512399.1 ns)
  2254361.2 |########################################
  2267263.1 |########################################
  2280165.0 |########################################
  2293066.9 |
  2305968.8 |
  2318870.7 |
  2331772.6 |
  2344674.5 |
  2357576.4 |
  2370478.3 |
  2383380.2 |
  2396282.1 |
  2409184.0 |
  2422085.9 |########################################
  2434987.8 |########################################
  2447889.7 |
  2460791.6 |
  2473693.5 |
  2486595.4 |
  2499497.3 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_null (n=6, range 2537.1-4226.2 ns)
   2537.1 |########################################
   2621.6 |
   2706.0 |
   2790.5 |
   2874.9 |##########################
   2959.4 |
   3043.8 |
   3128.3 |
   3212.8 |
   3297.2 |
   3381.7 |
   3466.1 |
   3550.6 |
   3635.0 |
   3719.5 |
   3804.0 |
   3888.4 |
   3972.9 |
   4057.3 |
   4141.8 |
  (0 below, 1 above range)

abi_cross_cold_tight_warm_scalar (n=6, range 2160872.5-2581401.5 ns)
  2160872.5 |########################################
  2181898.9 |
  2202925.4 |
  2223951.8 |
  2244978.3 |
  2266004.7 |########################################
  2287031.2 |
  2308057.6 |########################################
  2329084.1 |
  2350110.5 |
  2371137.0 |########################################
  2392163.4 |
  2413189.9 |
  2434216.3 |
  2455242.8 |
  2476269.2 |
  2497295.7 |########################################
  2518322.1 |
  2539348.6 |
  2560375.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_cold_tight_cold_null**: CV=44.6% (high variance, measurements may be unstable)
- **abi_cross_cold_tight_cold_null**: bridge=4601.5% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_cold_scalar**: bridge=306.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_null**: CV=33.8% (high variance, measurements may be unstable)
- **abi_cross_cold_tight_warm_null**: bridge=4317.2% of algo (FFI overhead may distort results)
- **abi_cross_cold_tight_warm_scalar**: bridge=309.6% of algo (FFI overhead may distort results)

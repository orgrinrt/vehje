# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (3.57 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 749.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 dominates: 52% faster than the next best (carrier_vert_wideselect_vert4)

carrier_vert_wideselect_vert8 (749.36 us) leads carrier_vert_wideselect_vert4 (1.14 ms) by 52%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_wideselect_vert8 beats baseline by 79% (significant)

carrier_vert_wideselect_vert8 is -2.83 ms (79%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 4.8x slower than the field

carrier_vert_wideselect_scalar (3.57 ms) is 4.8x the fastest (749.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_vert_wideselect_vert8 (749.36 us) to slowest carrier_vert_wideselect_scalar (3.57 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 749356.9 ns median (-79.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.76x (fastest 749356.9 ns, slowest 3566462.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3530211ns | 3569928ns | 3416124ns | 3522560ns | 3598731ns | base |
| carrier_vert_wideselect_vert4 | 1141750ns | 1140705ns | 1129457ns | 1138147ns | 1153302ns | -67.66% |
| carrier_vert_wideselect_vert8 | 744903ns | 752647ns | 712622ns | 743874ns | 762588ns | -78.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3526455ns | 3412405ns | 3594685ns | base | 0.001 |
| carrier_vert_wideselect_vert4 | 1138103ns | 1127018ns | 1149124ns | -67.73% | 0.004 |
| carrier_vert_wideselect_vert8 | 741533ns | 708728ns | 759367ns | -78.97% | 0.006 |

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.001 | 19.9% |
| carrier_vert_wideselect_vert4 | 0.004 | 62.3% |
| carrier_vert_wideselect_vert8 | 0.005 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 3530211ns | 3530211ns | base |
| carrier_vert_wideselect_vert4 | 1141750ns | 1141750ns | -67.66% |
| carrier_vert_wideselect_vert8 | 744903ns | 744903ns | -78.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3566462ns | base | --- | [3418219, 3594685] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 1136962ns | -2425734.0ns (-68.0%) | [-2457723, -2281600]ns | [1128224, 1149124] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 749357ns | -2829074.6ns (-79.3%) | [-2866839, -2658852]ns | [715877, 759367] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 3412405ns | -66.9% | -77.7% |
| 2 | 3595731ns | -68.2% | -79.2% |
| 3 | 3556907ns | -67.5% | -79.7% |
| 4 | 3424032ns | -66.6% | -77.9% |
| 5 | 3593639ns | -68.5% | -80.3% |
| 6 | 3576018ns | -68.5% | -79.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | -0.344 | moderate- |
| carrier_vert_wideselect_vert4 | 0.284 | moderate+ |
| carrier_vert_wideselect_vert8 | -0.488 | moderate- |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3530592.6ns | 3526455.3ns | 100.1% | HIGH |
| carrier_vert_wideselect_vert4 | 1148423.4ns | 1138102.9ns | 100.9% | HIGH |
| carrier_vert_wideselect_vert8 | 754220.7ns | 741533.5ns | 101.7% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 3412404.6-3594685.0 ns)
  3412404.6 |########################################
  3421518.6 |########################################
  3430632.6 |
  3439746.7 |
  3448860.7 |
  3457974.7 |
  3467088.7 |
  3476202.7 |
  3485316.8 |
  3494430.8 |
  3503544.8 |
  3512658.8 |
  3521772.8 |
  3530886.9 |
  3540000.9 |
  3549114.9 |########################################
  3558228.9 |
  3567342.9 |########################################
  3576457.0 |
  3585571.0 |########################################
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 1127017.5-1149123.6 ns)
  1127017.5 |########################################
  1128122.8 |
  1129228.1 |########################################
  1130333.4 |
  1131438.7 |########################################
  1132544.0 |
  1133649.3 |
  1134754.6 |
  1135859.9 |
  1136965.2 |
  1138070.5 |
  1139175.8 |
  1140281.1 |
  1141386.4 |########################################
  1142491.7 |
  1143597.0 |########################################
  1144702.3 |
  1145807.6 |
  1146912.9 |
  1148018.2 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 708728.3-759366.9 ns)
  708728.3 |########################################
  711260.2 |
  713792.2 |
  716324.1 |
  718856.0 |
  721387.9 |########################################
  723919.9 |
  726451.8 |
  728983.7 |
  731515.6 |
  734047.6 |
  736579.5 |
  739111.4 |
  741643.4 |
  744175.3 |
  746707.2 |########################################
  749239.1 |########################################
  751771.1 |
  754303.0 |########################################
  756834.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=101.3% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=100.5% of algo (FFI overhead may distort results)

# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), real profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_real_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_real_null dominates: 326% faster than the next best (carrier_cold_real_switch)

carrier_cold_real_null (31.46 us) leads carrier_cold_real_switch (134.17 us) by 326%, a clear separation rather than a photo finish. CV 11.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_real_null beats baseline by 76% (significant)

carrier_cold_real_null is -101.89 us (76%) faster than baseline carrier_cold_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_real_fntable is an outlier: 4.8x slower than the field

carrier_cold_real_fntable (151.15 us) is 4.8x the fastest (31.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cold_real_null is fastest but the noisiest (CV 11.7%)

carrier_cold_real_null wins on median (31.46 us) yet has the highest variance (CV 11.7%), while carrier_cold_real_threaded is the steadiest (CV 0.8%, 138.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {carrier_cold_real_null} vs {carrier_cold_real_switch, carrier_cold_real_threaded, carrier_cold_real_fntable} (326% apart)

The field splits into a fast tier {carrier_cold_real_null} and a slow tier {carrier_cold_real_switch, carrier_cold_real_threaded, carrier_cold_real_fntable} with a 326% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_cold_real_null (31.46 us) to slowest carrier_cold_real_fntable (151.15 us): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_cold_real_null** at 31458.8 ns median (-76.6% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.80x (fastest 31458.8 ns, slowest 151149.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_real_fntable | 153271ns | 153438ns | 151545ns | 152941ns | 154629ns | +11.98% |
| carrier_cold_real_null | 34560ns | 33883ns | 30071ns | 33152ns | 38916ns | -74.75% |
| carrier_cold_real_switch | 136873ns | 136541ns | 134090ns | 136442ns | 138912ns | base |
| carrier_cold_real_threaded | 141068ns | 140939ns | 139776ns | 140712ns | 142247ns | +3.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_real_fntable | 150820ns | 148965ns | 152122ns | +12.18% | 0.007 |
| carrier_cold_real_null | 32141ns | 27845ns | 36399ns | -76.09% | 0.032 |
| carrier_cold_real_switch | 134443ns | 131926ns | 136359ns | base | 0.008 |
| carrier_cold_real_threaded | 138588ns | 136914ns | 139769ns | +3.08% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_real_fntable | 946002 | 862034 | 1.097 | 1.11× |
| carrier_cold_real_null | 375190 | 1183894 | 0.317 | 0.44× |
| carrier_cold_real_switch | 850630 | 689246 | 1.234 | 1.00× |
| carrier_cold_real_threaded | 867272 | 725096 | 1.196 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_cold_real_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_real_fntable | 0.007 | 18.4% |
| carrier_cold_real_null | 0.033 | 88.5% |
| carrier_cold_real_switch | 0.008 | 20.8% |
| carrier_cold_real_threaded | 0.007 | 20.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_real_fntable | 153271ns | 153271ns | +11.98% |
| carrier_cold_real_null | 34560ns | 34560ns | -74.75% |
| carrier_cold_real_switch | 136873ns | 136873ns | base |
| carrier_cold_real_threaded | 141068ns | 141068ns | +3.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_real_switch | 134168ns | base | --- | [132801, 136359] | --- | --- | --- | --- |
| carrier_cold_real_fntable | 151149ns | +16856.6ns (+12.6%) | [+14370, +17906]ns | [149188, 152122] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_real_null | 31459ns | -101894.8ns (-75.9%) | [-105666, -99344]ns | [28565, 36399] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_real_threaded | 138579ns | +4391.5ns (+3.3%) | [+3129, +4914]ns | [137414, 139769] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_real_switch | carrier_cold_real_fntable | carrier_cold_real_null | carrier_cold_real_threaded |
|---|---|---|---|---|
| 1 | 131926ns | +13.3% | -77.8% | +3.8% |
| 2 | 134325ns | +10.9% | -75.3% | +3.6% |
| 3 | 134011ns | +13.7% | -77.8% | +2.9% |
| 4 | 133676ns | +12.8% | -74.6% | +3.2% |
| 5 | 137808ns | +10.2% | -71.8% | +1.7% |
| 6 | 134909ns | +12.3% | -79.4% | +3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_real_fntable | 0.061 | ok |
| carrier_cold_real_null | -0.324 | moderate- |
| carrier_cold_real_switch | -0.018 | ok |
| carrier_cold_real_threaded | -0.092 | ok |

**Consistency summary:**

- **carrier_cold_real_fntable**: won 0/6, lost 6/6
- **carrier_cold_real_null**: won 6/6, lost 0/6
- **carrier_cold_real_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_real_fntable | 150969.1ns | 150820.0ns | 100.1% | HIGH |
| carrier_cold_real_null | 87293.6ns | 32141.0ns | 271.6% | HIGH |
| carrier_cold_real_switch | 139770.3ns | 134442.6ns | 104.0% | HIGH |
| carrier_cold_real_threaded | 138834.9ns | 138587.6ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_cold_real_fntable (n=6, range 148965.0-152122.5 ns)
  148965.0 |########################################
  149122.9 |
  149280.8 |########################################
  149438.6 |
  149596.5 |
  149754.4 |
  149912.2 |
  150070.1 |
  150228.0 |
  150385.9 |
  150543.8 |
  150701.6 |########################################
  150859.5 |
  151017.4 |
  151175.2 |
  151333.1 |########################################
  151491.0 |
  151648.9 |
  151806.8 |########################################
  151964.6 |
  (0 below, 1 above range)

carrier_cold_real_null (n=6, range 27845.0-36398.8 ns)
  27845.0 |########################################
  28272.7 |
  28700.4 |
  29128.1 |########################################
  29555.8 |########################################
  29983.4 |
  30411.1 |
  30838.8 |
  31266.5 |
  31694.2 |
  32121.9 |
  32549.6 |
  32977.2 |########################################
  33404.9 |
  33832.6 |########################################
  34260.3 |
  34688.0 |
  35115.7 |
  35543.4 |
  35971.1 |
  (0 below, 1 above range)

carrier_cold_real_switch (n=6, range 131926.2-136358.8 ns)
  131926.2 |########################################
  132147.8 |
  132369.5 |
  132591.1 |
  132812.7 |
  133034.3 |
  133256.0 |
  133477.6 |########################################
  133699.2 |
  133920.8 |########################################
  134142.5 |########################################
  134364.1 |
  134585.7 |
  134807.4 |########################################
  135029.0 |
  135250.6 |
  135472.2 |
  135693.9 |
  135915.5 |
  136137.1 |
  (0 below, 1 above range)

carrier_cold_real_threaded (n=6, range 136914.2-139769.0 ns)
  136914.2 |####################
  137056.9 |
  137199.7 |
  137342.4 |
  137485.2 |
  137627.9 |
  137770.6 |
  137913.4 |########################################
  138056.1 |
  138198.8 |
  138341.6 |
  138484.3 |
  138627.1 |
  138769.8 |
  138912.5 |
  139055.3 |####################
  139198.0 |
  139340.7 |####################
  139483.5 |
  139626.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_real_fntable**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_cold_real_null**: bridge=277.1% of algo (FFI overhead may distort results)
- **carrier_cold_real_switch**: bridge=104.0% of algo (FFI overhead may distort results)
- **carrier_cold_real_threaded**: bridge=100.2% of algo (FFI overhead may distort results)

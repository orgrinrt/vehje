# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), scatter profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_scatter_null dominates: 394% faster than the next best (carrier_cold_scatter_switch)

carrier_cold_scatter_null (5.80 us) leads carrier_cold_scatter_switch (28.61 us) by 394%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_scatter_null beats baseline by 80% (significant)

carrier_cold_scatter_null is -22.80 us (80%) faster than baseline carrier_cold_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_scatter_fntable is an outlier: 5.8x slower than the field

carrier_cold_scatter_fntable (33.78 us) is 5.8x the fastest (5.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_scatter_null} vs {carrier_cold_scatter_switch, carrier_cold_scatter_threaded, carrier_cold_scatter_fntable} (394% apart)

The field splits into a fast tier {carrier_cold_scatter_null} and a slow tier {carrier_cold_scatter_switch, carrier_cold_scatter_threaded, carrier_cold_scatter_fntable} with a 394% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.8x the fastest

Fastest carrier_cold_scatter_null (5.80 us) to slowest carrier_cold_scatter_fntable (33.78 us): 5.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_cold_scatter_null** at 5796.9 ns median (-79.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.83x (fastest 5796.9 ns, slowest 33777.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_scatter_fntable | 36635ns | 36064ns | 35147ns | 35841ns | 38571ns | +17.25% |
| carrier_cold_scatter_null | 8078ns | 8076ns | 7962ns | 8049ns | 8181ns | -74.14% |
| carrier_cold_scatter_switch | 31244ns | 30925ns | 30295ns | 30815ns | 32363ns | base |
| carrier_cold_scatter_threaded | 31701ns | 31425ns | 29932ns | 31130ns | 33441ns | +1.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_scatter_fntable | 34233ns | 32823ns | 36091ns | +18.18% | 0.007 |
| carrier_cold_scatter_null | 5803ns | 5738ns | 5857ns | -79.97% | 0.044 |
| carrier_cold_scatter_switch | 28966ns | 28040ns | 30078ns | base | 0.009 |
| carrier_cold_scatter_threaded | 29400ns | 27762ns | 31131ns | +1.50% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_scatter_fntable | 434383 | 430946 | 1.008 | 1.14× |
| carrier_cold_scatter_null | 299051 | 1374037 | 0.218 | 0.79× |
| carrier_cold_scatter_switch | 380460 | 353059 | 1.078 | 1.00× |
| carrier_cold_scatter_threaded | 393651 | 398919 | 0.987 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.045 Gops/s** (carrier_cold_scatter_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_scatter_fntable | 0.008 | 17.0% |
| carrier_cold_scatter_null | 0.044 | 99.0% |
| carrier_cold_scatter_switch | 0.009 | 20.1% |
| carrier_cold_scatter_threaded | 0.009 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_scatter_fntable | 36635ns | 36635ns | +17.25% |
| carrier_cold_scatter_null | 8078ns | 8078ns | -74.14% |
| carrier_cold_scatter_switch | 31244ns | 31244ns | base |
| carrier_cold_scatter_threaded | 31701ns | 31701ns | +1.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_scatter_switch | 28608ns | base | --- | [28212, 30078] | --- | --- | --- | --- |
| carrier_cold_scatter_fntable | 33778ns | +5298.8ns (+18.5%) | [+3018, +7483]ns | [32829, 36091] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_scatter_null | 5797ns | -22798.6ns (-79.7%) | [-24310, -22380]ns | [5756, 5857] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_scatter_threaded | 29176ns | no significant difference | [-1710, +2650]ns | [27894, 31131] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_scatter_switch | carrier_cold_scatter_fntable | carrier_cold_scatter_null | carrier_cold_scatter_threaded |
|---|---|---|---|---|
| 1 | 28640ns | +30.4% | -79.8% | -3.1% |
| 2 | 28577ns | +21.9% | -79.6% | +6.5% |
| 3 | 28040ns | +20.0% | -79.0% | -0.1% |
| 4 | 28918ns | +17.3% | -80.0% | +2.6% |
| 5 | 28384ns | +15.6% | -79.7% | +12.1% |
| 6 | 31238ns | +5.1% | -81.6% | -8.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_scatter_fntable | 0.288 | moderate+ |
| carrier_cold_scatter_null | 0.243 | moderate+ |
| carrier_cold_scatter_switch | -0.115 | ok |
| carrier_cold_scatter_threaded | -0.381 | moderate- |

**Consistency summary:**

- **carrier_cold_scatter_fntable**: won 0/6, lost 6/6
- **carrier_cold_scatter_null**: won 6/6, lost 0/6
- **carrier_cold_scatter_threaded**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_scatter_fntable | 104803.1ns | 34232.7ns | 306.1% | HIGH |
| carrier_cold_scatter_null | 87441.4ns | 5803.2ns | 1506.8% | HIGH |
| carrier_cold_scatter_switch | 90950.6ns | 28966.0ns | 314.0% | HIGH |
| carrier_cold_scatter_threaded | 96411.7ns | 29400.0ns | 327.9% | HIGH |

## Distribution (algo ns)

```
carrier_cold_scatter_fntable (n=6, range 32822.9-36091.4 ns)
  32822.9 |########################################
  32986.3 |
  33149.8 |
  33313.2 |
  33476.6 |####################
  33640.0 |
  33803.5 |####################
  33966.9 |
  34130.3 |
  34293.7 |
  34457.2 |
  34620.6 |
  34784.0 |####################
  34947.5 |
  35110.9 |
  35274.3 |
  35437.7 |
  35601.2 |
  35764.6 |
  35928.0 |
  (0 below, 1 above range)

carrier_cold_scatter_null (n=6, range 5737.9-5857.2 ns)
   5737.9 |####################
   5743.9 |
   5749.8 |
   5755.8 |
   5761.8 |
   5767.7 |####################
   5773.7 |
   5779.7 |
   5785.6 |
   5791.6 |########################################
   5797.6 |
   5803.5 |
   5809.5 |
   5815.5 |
   5821.4 |####################
   5827.4 |
   5833.4 |
   5839.3 |
   5845.3 |
   5851.3 |
  (0 below, 1 above range)

carrier_cold_scatter_switch (n=6, range 28040.4-30077.7 ns)
  28040.4 |####################
  28142.3 |
  28244.1 |
  28346.0 |####################
  28447.9 |
  28549.7 |########################################
  28651.6 |
  28753.5 |
  28855.3 |####################
  28957.2 |
  29059.1 |
  29160.9 |
  29262.8 |
  29364.6 |
  29466.5 |
  29568.4 |
  29670.2 |
  29772.1 |
  29874.0 |
  29975.8 |
  (0 below, 1 above range)

carrier_cold_scatter_threaded (n=6, range 27762.1-31130.6 ns)
  27762.1 |########################################
  27930.5 |########################################
  28098.9 |
  28267.4 |
  28435.8 |
  28604.2 |########################################
  28772.6 |
  28941.1 |
  29109.5 |
  29277.9 |
  29446.3 |
  29614.8 |########################################
  29783.2 |
  29951.6 |
  30120.0 |
  30288.5 |########################################
  30456.9 |
  30625.3 |
  30793.8 |
  30962.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_scatter_fntable**: bridge=306.1% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_null**: bridge=1508.0% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_switch**: bridge=314.3% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_threaded**: bridge=332.6% of algo (FFI overhead may distort results)

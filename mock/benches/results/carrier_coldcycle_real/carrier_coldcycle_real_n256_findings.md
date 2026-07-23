# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), real profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_real_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_real_null dominates: 386% faster than the next best (carrier_cold_real_switch)

carrier_cold_real_null (6.06 us) leads carrier_cold_real_switch (29.45 us) by 386%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_real_null beats baseline by 79% (significant)

carrier_cold_real_null is -23.34 us (79%) faster than baseline carrier_cold_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_real_fntable is an outlier: 5.7x slower than the field

carrier_cold_real_fntable (34.24 us) is 5.7x the fastest (6.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cold_real_null} vs {carrier_cold_real_switch, carrier_cold_real_threaded, carrier_cold_real_fntable} (386% apart)

The field splits into a fast tier {carrier_cold_real_null} and a slow tier {carrier_cold_real_switch, carrier_cold_real_threaded, carrier_cold_real_fntable} with a 386% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.7x the fastest

Fastest carrier_cold_real_null (6.06 us) to slowest carrier_cold_real_fntable (34.24 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_cold_real_null** at 6057.0 ns median (-79.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.65x (fastest 6057.0 ns, slowest 34241.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_real_fntable | 36766ns | 36513ns | 35390ns | 36263ns | 38209ns | +14.33% |
| carrier_cold_real_null | 8285ns | 8284ns | 8114ns | 8247ns | 8429ns | -74.24% |
| carrier_cold_real_switch | 32159ns | 31804ns | 29872ns | 31530ns | 34244ns | base |
| carrier_cold_real_threaded | 31917ns | 32168ns | 29480ns | 31733ns | 33413ns | -0.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_real_fntable | 34398ns | 32915ns | 35781ns | +15.26% | 0.007 |
| carrier_cold_real_null | 6025ns | 5878ns | 6103ns | -79.81% | 0.042 |
| carrier_cold_real_switch | 29843ns | 27632ns | 31873ns | base | 0.009 |
| carrier_cold_real_threaded | 29637ns | 27225ns | 31080ns | -0.69% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_real_fntable | 435593 | 429408 | 1.014 | 1.13× |
| carrier_cold_real_null | 301934 | 1339444 | 0.225 | 0.78× |
| carrier_cold_real_switch | 386315 | 350021 | 1.104 | 1.00× |
| carrier_cold_real_threaded | 391885 | 382628 | 1.024 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.044 Gops/s** (carrier_cold_real_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_real_fntable | 0.007 | 17.2% |
| carrier_cold_real_null | 0.042 | 97.0% |
| carrier_cold_real_switch | 0.009 | 20.0% |
| carrier_cold_real_threaded | 0.009 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_real_fntable | 36766ns | 36766ns | +14.33% |
| carrier_cold_real_null | 8285ns | 8285ns | -74.24% |
| carrier_cold_real_switch | 32159ns | 32159ns | base |
| carrier_cold_real_threaded | 31917ns | 31917ns | -0.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_real_switch | 29445ns | base | --- | [28212, 31873] | --- | --- | --- | --- |
| carrier_cold_real_fntable | 34241ns | +4981.0ns (+16.9%) | [+2780, +5902]ns | [33170, 35781] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_real_null | 6057ns | -23342.1ns (-79.3%) | [-25890, -22225]ns | [5914, 6103] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_real_threaded | 29884ns | no significant difference | [-2110, +1436]ns | [27946, 31080] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_real_switch | carrier_cold_real_fntable | carrier_cold_real_null | carrier_cold_real_threaded |
|---|---|---|---|---|
| 1 | 28792ns | +18.3% | -79.1% | +0.2% |
| 2 | 29162ns | +22.4% | -79.0% | +6.3% |
| 3 | 32633ns | +9.9% | -81.3% | -5.3% |
| 4 | 27632ns | +19.1% | -78.5% | +3.7% |
| 5 | 29729ns | +15.8% | -79.5% | -8.4% |
| 6 | 31112ns | +7.4% | -81.1% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_real_fntable | -0.118 | ok |
| carrier_cold_real_null | -0.315 | moderate- |
| carrier_cold_real_switch | -0.457 | moderate- |
| carrier_cold_real_threaded | -0.145 | ok |

**Consistency summary:**

- **carrier_cold_real_fntable**: won 0/6, lost 6/6
- **carrier_cold_real_null**: won 6/6, lost 0/6
- **carrier_cold_real_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_real_fntable | 105582.7ns | 34397.5ns | 306.9% | HIGH |
| carrier_cold_real_null | 89185.1ns | 6024.6ns | 1480.4% | HIGH |
| carrier_cold_real_switch | 93850.8ns | 29843.4ns | 314.5% | HIGH |
| carrier_cold_real_threaded | 93467.5ns | 29636.9ns | 315.4% | HIGH |

## Distribution (algo ns)

```
carrier_cold_real_fntable (n=6, range 32914.6-35780.7 ns)
  32914.6 |########################################
  33057.9 |
  33201.2 |
  33344.5 |########################################
  33487.8 |
  33631.1 |
  33774.4 |
  33917.7 |########################################
  34061.0 |
  34204.3 |
  34347.6 |########################################
  34490.9 |
  34634.2 |
  34777.5 |
  34920.8 |
  35064.1 |
  35207.4 |
  35350.7 |
  35494.0 |
  35637.3 |########################################
  (0 below, 1 above range)

carrier_cold_real_null (n=6, range 5877.5-6103.1 ns)
   5877.5 |####################
   5888.8 |
   5900.1 |
   5911.3 |
   5922.6 |
   5933.9 |
   5945.2 |####################
   5956.5 |
   5967.8 |
   5979.0 |
   5990.3 |
   6001.6 |
   6012.9 |
   6024.2 |####################
   6035.5 |
   6046.7 |
   6058.0 |
   6069.3 |
   6080.6 |########################################
   6091.9 |
  (0 below, 1 above range)

carrier_cold_real_switch (n=6, range 27632.1-31872.7 ns)
  27632.1 |########################################
  27844.1 |
  28056.2 |
  28268.2 |
  28480.2 |
  28692.2 |########################################
  28904.3 |
  29116.3 |########################################
  29328.3 |
  29540.4 |########################################
  29752.4 |
  29964.4 |
  30176.5 |
  30388.5 |
  30600.5 |
  30812.5 |
  31024.6 |########################################
  31236.6 |
  31448.6 |
  31660.7 |
  (0 below, 1 above range)

carrier_cold_real_threaded (n=6, range 27225.0-31080.2 ns)
  27225.0 |####################
  27417.8 |
  27610.5 |
  27803.3 |
  27996.0 |
  28188.8 |
  28381.6 |
  28574.3 |####################
  28767.1 |####################
  28959.8 |
  29152.6 |
  29345.4 |
  29538.1 |
  29730.9 |
  29923.6 |
  30116.4 |
  30309.2 |
  30501.9 |
  30694.7 |
  30887.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_real_fntable**: bridge=309.2% of algo (FFI overhead may distort results)
- **carrier_cold_real_null**: bridge=1473.0% of algo (FFI overhead may distort results)
- **carrier_cold_real_switch**: bridge=314.8% of algo (FFI overhead may distort results)
- **carrier_cold_real_threaded**: bridge=311.4% of algo (FFI overhead may distort results)

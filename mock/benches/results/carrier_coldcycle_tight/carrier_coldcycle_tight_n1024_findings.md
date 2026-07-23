# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), tight profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_tight_null dominates: 14% faster than the next best (carrier_cold_tight_threaded)

carrier_cold_tight_null (31.44 us) leads carrier_cold_tight_threaded (35.98 us) by 14%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_tight_null beats baseline by 25% (significant)

carrier_cold_tight_null is -10.65 us (25%) faster than baseline carrier_cold_tight_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_tight_threaded shows alternating (throttle bounce) (autocorr -0.56)

carrier_cold_tight_threaded's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_cold_tight_null** at 31436.0 ns median (-25.0% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.64x (fastest 31436.0 ns, slowest 51642.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_tight_fntable | 54030ns | 53848ns | 53348ns | 53817ns | 54691ns | +21.92% |
| carrier_cold_tight_null | 33760ns | 33737ns | 33119ns | 33607ns | 34311ns | -23.82% |
| carrier_cold_tight_switch | 44317ns | 44249ns | 43737ns | 44171ns | 44825ns | base |
| carrier_cold_tight_threaded | 38372ns | 38168ns | 37983ns | 38142ns | 38911ns | -13.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_tight_fntable | 51720ns | 51026ns | 52225ns | +23.11% | 0.020 |
| carrier_cold_tight_null | 31448ns | 30974ns | 31912ns | -25.14% | 0.033 |
| carrier_cold_tight_switch | 42011ns | 41398ns | 42585ns | base | 0.024 |
| carrier_cold_tight_threaded | 36063ns | 35612ns | 36576ns | -14.16% | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_tight_fntable | 496797 | 1249746 | 0.398 | 1.21× |
| carrier_cold_tight_null | 402455 | 1468644 | 0.274 | 0.98× |
| carrier_cold_tight_switch | 411724 | 1019662 | 0.404 | 1.00× |
| carrier_cold_tight_threaded | 459042 | 1411543 | 0.325 | 1.11× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.033 Gops/s** (carrier_cold_tight_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_tight_fntable | 0.020 | 60.0% |
| carrier_cold_tight_null | 0.033 | 98.5% |
| carrier_cold_tight_switch | 0.024 | 73.9% |
| carrier_cold_tight_threaded | 0.028 | 86.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_tight_fntable | 54030ns | 54030ns | +21.92% |
| carrier_cold_tight_null | 33760ns | 33760ns | -23.82% |
| carrier_cold_tight_switch | 44317ns | 44317ns | base |
| carrier_cold_tight_threaded | 38372ns | 38372ns | -13.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_tight_switch | 41910ns | base | --- | [41538, 42585] | --- | --- | --- | --- |
| carrier_cold_tight_fntable | 51643ns | +9422.3ns (+22.5%) | [+9017, +10688]ns | [51291, 52225] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_tight_null | 31436ns | -10650.0ns (-25.4%) | [-11023, -10014]ns | [30998, 31912] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_tight_threaded | 35976ns | -5912.7ns (-14.1%) | [-6539, -5392]ns | [35635, 36576] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_tight_switch | carrier_cold_tight_fntable | carrier_cold_tight_null | carrier_cold_tight_threaded |
|---|---|---|---|---|
| 1 | 41398ns | +26.7% | -23.0% | -14.0% |
| 2 | 42910ns | +20.4% | -25.5% | -15.9% |
| 3 | 42121ns | +22.6% | -26.4% | -14.8% |
| 4 | 41677ns | +24.8% | -25.2% | -12.3% |
| 5 | 41698ns | +22.4% | -25.7% | -14.5% |
| 6 | 42260ns | +22.0% | -25.0% | -13.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_tight_fntable | -0.130 | ok |
| carrier_cold_tight_null | 0.119 | ok |
| carrier_cold_tight_switch | -0.315 | moderate- |
| carrier_cold_tight_threaded | -0.556 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_cold_tight_fntable**: won 0/6, lost 6/6
- **carrier_cold_tight_null**: won 6/6, lost 0/6
- **carrier_cold_tight_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_tight_fntable | 105251.7ns | 51719.7ns | 203.5% | HIGH |
| carrier_cold_tight_null | 94474.8ns | 31448.5ns | 300.4% | HIGH |
| carrier_cold_tight_switch | 87054.9ns | 42010.8ns | 207.2% | HIGH |
| carrier_cold_tight_threaded | 108368.1ns | 36062.7ns | 300.5% | HIGH |

## Distribution (algo ns)

```
carrier_cold_tight_fntable (n=6, range 51025.8-52225.2 ns)
  51025.8 |####################
  51085.8 |
  51145.7 |
  51205.7 |
  51265.7 |
  51325.7 |
  51385.6 |
  51445.6 |
  51505.6 |####################
  51565.5 |
  51625.5 |########################################
  51685.5 |
  51745.4 |
  51805.4 |
  51865.4 |
  51925.3 |
  51985.3 |####################
  52045.3 |
  52105.3 |
  52165.2 |
  (0 below, 1 above range)

carrier_cold_tight_null (n=6, range 30973.8-31911.9 ns)
  30973.8 |########################################
  31020.7 |########################################
  31067.6 |
  31114.5 |
  31161.4 |########################################
  31208.3 |
  31255.2 |
  31302.1 |
  31349.0 |
  31395.9 |
  31442.8 |
  31489.8 |
  31536.7 |
  31583.6 |
  31630.5 |
  31677.4 |########################################
  31724.3 |
  31771.2 |
  31818.1 |########################################
  31865.0 |
  (0 below, 1 above range)

carrier_cold_tight_switch (n=6, range 41397.9-42585.4 ns)
  41397.9 |########################################
  41457.3 |
  41516.7 |
  41576.0 |
  41635.4 |########################################
  41694.8 |########################################
  41754.2 |
  41813.5 |
  41872.9 |
  41932.3 |
  41991.7 |
  42051.0 |
  42110.4 |########################################
  42169.8 |
  42229.2 |########################################
  42288.5 |
  42347.9 |
  42407.3 |
  42466.7 |
  42526.0 |
  (0 below, 1 above range)

carrier_cold_tight_threaded (n=6, range 35611.7-36576.2 ns)
  35611.7 |########################################
  35659.9 |
  35708.2 |
  35756.4 |
  35804.6 |
  35852.8 |####################
  35901.1 |
  35949.3 |
  35997.5 |
  36045.7 |####################
  36094.0 |
  36142.2 |
  36190.4 |
  36238.7 |
  36286.9 |
  36335.1 |
  36383.3 |
  36431.6 |
  36479.8 |
  36528.0 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_tight_fntable**: bridge=203.7% of algo (FFI overhead may distort results)
- **carrier_cold_tight_null**: bridge=300.1% of algo (FFI overhead may distort results)
- **carrier_cold_tight_switch**: bridge=207.4% of algo (FFI overhead may distort results)
- **carrier_cold_tight_threaded**: bridge=299.8% of algo (FFI overhead may distort results)

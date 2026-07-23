# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), scatter profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_scatter_null dominates: 314% faster than the next best (carrier_cold_scatter_switch)

carrier_cold_scatter_null (32.04 us) leads carrier_cold_scatter_switch (132.70 us) by 314%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_scatter_null beats baseline by 76% (significant)

carrier_cold_scatter_null is -101.20 us (76%) faster than baseline carrier_cold_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_scatter_fntable is an outlier: 4.7x slower than the field

carrier_cold_scatter_fntable (150.89 us) is 4.7x the fastest (32.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cold_scatter_fntable shows alternating (throttle bounce) (autocorr -0.62)

carrier_cold_scatter_fntable's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cold_scatter_null} vs {carrier_cold_scatter_switch, carrier_cold_scatter_threaded, carrier_cold_scatter_fntable} (314% apart)

The field splits into a fast tier {carrier_cold_scatter_null} and a slow tier {carrier_cold_scatter_switch, carrier_cold_scatter_threaded, carrier_cold_scatter_fntable} with a 314% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.7x the fastest

Fastest carrier_cold_scatter_null (32.04 us) to slowest carrier_cold_scatter_fntable (150.89 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_cold_scatter_null** at 32042.2 ns median (-75.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.71x (fastest 32042.2 ns, slowest 150893.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_scatter_fntable | 153006ns | 153150ns | 151230ns | 152594ns | 154512ns | +12.87% |
| carrier_cold_scatter_null | 34217ns | 34390ns | 32016ns | 34192ns | 35356ns | -74.76% |
| carrier_cold_scatter_switch | 135556ns | 135201ns | 134813ns | 135127ns | 136572ns | base |
| carrier_cold_scatter_threaded | 142442ns | 142510ns | 140824ns | 142055ns | 143831ns | +5.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_scatter_fntable | 150477ns | 148636ns | 151801ns | +13.04% | 0.007 |
| carrier_cold_scatter_null | 31858ns | 29833ns | 32932ns | -76.07% | 0.032 |
| carrier_cold_scatter_switch | 133122ns | 132458ns | 134174ns | base | 0.008 |
| carrier_cold_scatter_threaded | 140009ns | 138473ns | 141258ns | +5.17% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_scatter_fntable | 943086 | 867925 | 1.087 | 1.11× |
| carrier_cold_scatter_null | 400618 | 1389599 | 0.288 | 0.47× |
| carrier_cold_scatter_switch | 850243 | 694582 | 1.224 | 1.00× |
| carrier_cold_scatter_threaded | 875370 | 735148 | 1.191 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.034 Gops/s** (carrier_cold_scatter_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_scatter_fntable | 0.007 | 19.8% |
| carrier_cold_scatter_null | 0.032 | 93.1% |
| carrier_cold_scatter_switch | 0.008 | 22.5% |
| carrier_cold_scatter_threaded | 0.007 | 21.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_scatter_fntable | 153006ns | 153006ns | +12.87% |
| carrier_cold_scatter_null | 34217ns | 34217ns | -74.76% |
| carrier_cold_scatter_switch | 135556ns | 135556ns | base |
| carrier_cold_scatter_threaded | 142442ns | 142442ns | +5.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_scatter_switch | 132695ns | base | --- | [132496, 134174] | --- | --- | --- | --- |
| carrier_cold_scatter_fntable | 150893ns | +17825.6ns (+13.4%) | [+15021, +19220]ns | [148738, 151801] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_scatter_null | 32042ns | -101200.5ns (-76.3%) | [-102518, -100072]ns | [30600, 32932] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cold_scatter_threaded | 140101ns | +6421.1ns (+4.8%) | [+5714, +8526]ns | [138668, 141258] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_scatter_switch | carrier_cold_scatter_fntable | carrier_cold_scatter_null | carrier_cold_scatter_threaded |
|---|---|---|---|---|
| 1 | 134976ns | +10.3% | -75.6% | +4.4% |
| 2 | 132808ns | +14.2% | -77.5% | +5.2% |
| 3 | 132458ns | +12.2% | -76.3% | +6.9% |
| 4 | 132535ns | +14.4% | -75.1% | +4.5% |
| 5 | 133372ns | +12.6% | -76.0% | +4.1% |
| 6 | 132582ns | +14.6% | -75.8% | +6.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_scatter_fntable | -0.616 | HIGH- (thermal bounce) |
| carrier_cold_scatter_null | -0.212 | moderate- |
| carrier_cold_scatter_switch | -0.057 | ok |
| carrier_cold_scatter_threaded | -0.284 | moderate- |

**Consistency summary:**

- **carrier_cold_scatter_fntable**: won 0/6, lost 6/6
- **carrier_cold_scatter_null**: won 6/6, lost 0/6
- **carrier_cold_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_scatter_fntable | 150689.7ns | 150477.2ns | 100.1% | HIGH |
| carrier_cold_scatter_null | 97014.4ns | 31858.3ns | 304.5% | HIGH |
| carrier_cold_scatter_switch | 138563.7ns | 133121.7ns | 104.1% | HIGH |
| carrier_cold_scatter_threaded | 139967.9ns | 140008.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_cold_scatter_fntable (n=6, range 148636.2-151800.6 ns)
  148636.2 |####################
  148794.4 |####################
  148952.6 |
  149110.9 |
  149269.1 |
  149427.3 |
  149585.5 |
  149743.7 |
  149902.0 |
  150060.2 |####################
  150218.4 |
  150376.6 |
  150534.8 |
  150693.1 |
  150851.3 |
  151009.5 |
  151167.7 |
  151325.9 |
  151484.2 |########################################
  151642.4 |
  (0 below, 1 above range)

carrier_cold_scatter_null (n=6, range 29832.9-32932.5 ns)
  29832.9 |####################
  29987.9 |
  30142.9 |
  30297.8 |
  30452.8 |
  30607.8 |
  30762.8 |
  30917.8 |
  31072.7 |
  31227.7 |####################
  31382.7 |
  31537.7 |
  31692.7 |
  31847.6 |
  32002.6 |########################################
  32157.6 |
  32312.6 |
  32467.6 |
  32622.5 |
  32777.5 |####################
  (0 below, 1 above range)

carrier_cold_scatter_switch (n=6, range 132457.9-134173.8 ns)
  132457.9 |########################################
  132543.7 |####################
  132629.5 |
  132715.3 |
  132801.1 |####################
  132886.9 |
  132972.7 |
  133058.4 |
  133144.2 |
  133230.0 |
  133315.8 |####################
  133401.6 |
  133487.4 |
  133573.2 |
  133659.0 |
  133744.8 |
  133830.6 |
  133916.4 |
  134002.2 |
  134088.0 |
  (0 below, 1 above range)

carrier_cold_scatter_threaded (n=6, range 138472.9-141258.4 ns)
  138472.9 |########################################
  138612.2 |
  138751.4 |########################################
  138890.7 |
  139030.0 |
  139169.3 |
  139308.5 |
  139447.8 |
  139587.1 |########################################
  139726.4 |
  139865.6 |
  140004.9 |
  140144.2 |
  140283.4 |
  140422.7 |########################################
  140562.0 |
  140701.3 |
  140840.5 |########################################
  140979.8 |
  141119.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_scatter_fntable**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_null**: bridge=303.4% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_switch**: bridge=104.3% of algo (FFI overhead may distort results)
- **carrier_cold_scatter_threaded**: bridge=100.0% of algo (FFI overhead may distort results)

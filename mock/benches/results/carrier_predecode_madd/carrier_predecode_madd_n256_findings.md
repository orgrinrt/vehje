# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_threaded shows alternating (throttle bounce) (autocorr -0.64)

carrier_pre_madd_threaded's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### carrier_pre_madd_threaded's edge over baseline is significant but tiny (12 ns, 0.10%)

carrier_pre_madd_threaded differs from baseline carrier_pre_madd_switch by 12 ns (0.10%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 9821.5 ns median (-17.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.22x (fastest 9821.5 ns, slowest 11990.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 14398ns | 14420ns | 14188ns | 14381ns | 14530ns | -0.00% |
| carrier_pre_madd_fntable | 13877ns | 14090ns | 12623ns | 14061ns | 14228ns | -3.62% |
| carrier_pre_madd_null | 13297ns | 13291ns | 13052ns | 13252ns | 13488ns | -7.65% |
| carrier_pre_madd_regcache | 12539ns | 12488ns | 12303ns | 12473ns | 12754ns | -12.92% |
| carrier_pre_madd_switch | 14399ns | 14470ns | 13846ns | 14432ns | 14625ns | base |
| carrier_pre_madd_threaded | 14356ns | 14317ns | 13899ns | 14235ns | 14765ns | -0.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 11976ns | 11801ns | 12087ns | +1.10% | 0.021 |
| carrier_pre_madd_fntable | 11408ns | 10358ns | 11679ns | -3.69% | 0.022 |
| carrier_pre_madd_null | 10661ns | 10527ns | 10734ns | -10.00% | 0.024 |
| carrier_pre_madd_regcache | 9895ns | 9755ns | 10102ns | -16.46% | 0.026 |
| carrier_pre_madd_switch | 11845ns | 11465ns | 12013ns | base | 0.022 |
| carrier_pre_madd_threaded | 11798ns | 11385ns | 12087ns | -0.40% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_pre_madd_direct | 304783 | 582205 | 0.523 | 1.04× |
| carrier_pre_madd_fntable | 306668 | 929614 | 0.330 | 1.04× |
| carrier_pre_madd_null | 292218 | 878020 | 0.333 | 1.00× |
| carrier_pre_madd_regcache | 283558 | 1200689 | 0.236 | 0.97× |
| carrier_pre_madd_switch | 293556 | 738531 | 0.397 | 1.00× |
| carrier_pre_madd_threaded | 299475 | 792256 | 0.378 | 1.02× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.021 | 81.4% |
| carrier_pre_madd_fntable | 0.022 | 84.1% |
| carrier_pre_madd_null | 0.024 | 91.4% |
| carrier_pre_madd_regcache | 0.026 | 99.3% |
| carrier_pre_madd_switch | 0.022 | 82.1% |
| carrier_pre_madd_threaded | 0.022 | 82.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 14398ns | 14398ns | -0.00% |
| carrier_pre_madd_fntable | 13877ns | 13877ns | -3.62% |
| carrier_pre_madd_null | 13297ns | 13297ns | -7.65% |
| carrier_pre_madd_regcache | 12539ns | 12539ns | -12.92% |
| carrier_pre_madd_switch | 14399ns | 14399ns | base |
| carrier_pre_madd_threaded | 14356ns | 14356ns | -0.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 11888ns | base | --- | [11635, 12013] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 11991ns | no significant difference | [-47, +374]ns | [11850, 12087] | no | 0.8594 | 0.6875 | 0 |
| carrier_pre_madd_fntable | 11604ns | -331.6ns (-2.8%) | [-758, -222]ns | [10942, 11679] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_null | 10676ns | -1205.7ns (-10.1%) | [-1348, -998]ns | [10574, 10734] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 9821ns | -1923.9ns (-16.2%) | [-2192, -1735]ns | [9762, 10102] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 11781ns | no significant difference | [-466, +312]ns | [11527, 12087] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 11465ns | +5.5% | -9.7% | -8.2% | -14.8% | +3.7% |
| 2 | 11815ns | +0.7% | -2.4% | -9.4% | -17.4% | +1.7% |
| 3 | 11960ns | +1.0% | -3.1% | -11.0% | -15.0% | -4.8% |
| 4 | 12002ns | -0.7% | -2.5% | -11.5% | -18.4% | +1.3% |
| 5 | 11805ns | -0.0% | -1.3% | -9.0% | -15.0% | -1.1% |
| 6 | 12024ns | +0.4% | -3.4% | -10.8% | -18.1% | -3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.392 | moderate- |
| carrier_pre_madd_fntable | 0.056 | ok |
| carrier_pre_madd_null | -0.132 | ok |
| carrier_pre_madd_regcache | -0.474 | moderate- |
| carrier_pre_madd_switch | 0.059 | ok |
| carrier_pre_madd_threaded | -0.636 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 1/6, lost 4/6
- **carrier_pre_madd_fntable**: won 6/6, lost 0/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 95171.0ns | 11975.8ns | 794.7% | HIGH |
| carrier_pre_madd_fntable | 93110.5ns | 11407.9ns | 816.2% | HIGH |
| carrier_pre_madd_null | 91318.2ns | 10661.3ns | 856.5% | HIGH |
| carrier_pre_madd_regcache | 90088.6ns | 9895.1ns | 910.4% | HIGH |
| carrier_pre_madd_switch | 90706.4ns | 11845.3ns | 765.8% | HIGH |
| carrier_pre_madd_threaded | 93049.4ns | 11798.2ns | 788.7% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 11800.8-12087.1 ns)
  11800.8 |########################################
  11815.1 |
  11829.4 |
  11843.7 |
  11858.1 |
  11872.4 |
  11886.7 |########################################
  11901.0 |########################################
  11915.3 |
  11929.6 |
  11944.0 |
  11958.3 |
  11972.6 |
  11986.9 |
  12001.2 |
  12015.5 |
  12029.8 |
  12044.2 |
  12058.5 |########################################
  12072.8 |########################################
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 10357.5-11678.5 ns)
  10357.5 |####################
  10423.6 |
  10489.6 |
  10555.7 |
  10621.7 |
  10687.8 |
  10753.8 |
  10819.9 |
  10885.9 |
  10952.0 |
  11018.0 |
  11084.1 |
  11150.1 |
  11216.2 |
  11282.2 |
  11348.3 |
  11414.3 |
  11480.4 |####################
  11546.4 |####################
  11612.5 |########################################
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 10526.7-10734.4 ns)
  10526.7 |########################################
  10537.1 |
  10547.5 |
  10557.9 |
  10568.2 |
  10578.6 |
  10589.0 |
  10599.4 |
  10609.8 |
  10620.2 |########################################
  10630.6 |
  10640.9 |########################################
  10651.3 |
  10661.7 |
  10672.1 |
  10682.5 |
  10692.9 |
  10703.2 |########################################
  10713.6 |########################################
  10724.0 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 9755.0-10102.1 ns)
   9755.0 |########################################
   9772.4 |
   9789.7 |####################
   9807.1 |
   9824.4 |
   9841.8 |####################
   9859.1 |
   9876.5 |
   9893.8 |
   9911.2 |
   9928.5 |
   9945.9 |
   9963.3 |
   9980.6 |
   9998.0 |
  10015.3 |####################
  10032.7 |
  10050.0 |
  10067.4 |
  10084.7 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 11465.0-12013.2 ns)
  11465.0 |####################
  11492.4 |
  11519.8 |
  11547.2 |
  11574.6 |
  11602.0 |
  11629.4 |
  11656.9 |
  11684.3 |
  11711.7 |
  11739.1 |
  11766.5 |
  11793.9 |########################################
  11821.3 |
  11848.7 |
  11876.1 |
  11903.5 |
  11930.9 |
  11958.3 |####################
  11985.7 |####################
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 11385.4-12087.3 ns)
  11385.4 |####################
  11420.5 |
  11455.6 |
  11490.7 |
  11525.8 |
  11560.9 |
  11596.0 |
  11631.1 |
  11666.2 |########################################
  11701.3 |
  11736.3 |
  11771.4 |
  11806.5 |
  11841.6 |
  11876.7 |####################
  11911.8 |
  11946.9 |
  11982.0 |####################
  12017.1 |
  12052.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=794.1% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=802.7% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=850.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=909.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=766.7% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=789.2% of algo (FFI overhead may distort results)

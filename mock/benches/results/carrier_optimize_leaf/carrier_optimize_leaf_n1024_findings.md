# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, leaf profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_leaf_none**

## Highlights

Baseline for all deltas below: **carrier_opt_leaf_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_leaf_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_leaf_none has the worst median (45.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_leaf_all at 10.93 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_leaf_all dominates: 23% faster than the next best (carrier_opt_leaf_cse)

carrier_opt_leaf_all (10.93 us) leads carrier_opt_leaf_cse (13.46 us) by 23%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_leaf_all beats baseline by 76% (significant)

carrier_opt_leaf_all is -34.50 us (76%) faster than baseline carrier_opt_leaf_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_leaf_none is an outlier: 4.1x slower than the field

carrier_opt_leaf_none (45.22 us) is 4.1x the fastest (10.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_leaf_cse shows alternating (throttle bounce) (autocorr -0.81)

carrier_opt_leaf_cse's per-pass series has lag-1 autocorrelation -0.81, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_leaf_all, carrier_opt_leaf_cse, carrier_opt_leaf_canon} vs {carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} (129% apart)

The field splits into a fast tier {carrier_opt_leaf_all, carrier_opt_leaf_cse, carrier_opt_leaf_canon} and a slow tier {carrier_opt_leaf_fold, carrier_opt_leaf_dce, carrier_opt_leaf_none} with a 129% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest carrier_opt_leaf_all (10.93 us) to slowest carrier_opt_leaf_none (45.22 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_leaf_all** at 10934.8 ns median (-75.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 4.14x (fastest 10934.8 ns, slowest 45216.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_leaf_all | 13345ns | 13330ns | 12927ns | 13205ns | 13762ns | -72.41% |
| carrier_opt_leaf_canon | 16844ns | 16321ns | 14626ns | 16175ns | 18957ns | -65.18% |
| carrier_opt_leaf_cse | 15737ns | 15842ns | 14637ns | 15517ns | 16616ns | -67.47% |
| carrier_opt_leaf_dce | 46441ns | 46367ns | 41539ns | 45612ns | 50136ns | -4.00% |
| carrier_opt_leaf_fold | 34240ns | 33998ns | 32602ns | 33707ns | 35859ns | -29.22% |
| carrier_opt_leaf_none | 48374ns | 47708ns | 46233ns | 47512ns | 50739ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_leaf_all | 11004ns | 10663ns | 11366ns | -76.01% | 0.093 |
| carrier_opt_leaf_canon | 14490ns | 12444ns | 16624ns | -68.41% | 0.071 |
| carrier_opt_leaf_cse | 13377ns | 12457ns | 14093ns | -70.84% | 0.077 |
| carrier_opt_leaf_dce | 44017ns | 39360ns | 47618ns | -4.05% | 0.023 |
| carrier_opt_leaf_fold | 31898ns | 30366ns | 33413ns | -30.47% | 0.032 |
| carrier_opt_leaf_none | 45874ns | 43829ns | 48249ns | base | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_leaf_all | 315891 | 1405632 | 0.225 | 0.73× |
| carrier_opt_leaf_canon | 327236 | 1395195 | 0.235 | 0.75× |
| carrier_opt_leaf_cse | 324960 | 1492988 | 0.218 | 0.75× |
| carrier_opt_leaf_dce | 455574 | 1961494 | 0.232 | 1.05× |
| carrier_opt_leaf_fold | 393108 | 2227051 | 0.177 | 0.90× |
| carrier_opt_leaf_none | 435399 | 1801531 | 0.242 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.096 Gops/s** (carrier_opt_leaf_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_leaf_all | 0.094 | 97.5% |
| carrier_opt_leaf_canon | 0.074 | 77.1% |
| carrier_opt_leaf_cse | 0.076 | 79.3% |
| carrier_opt_leaf_dce | 0.023 | 24.2% |
| carrier_opt_leaf_fold | 0.032 | 33.7% |
| carrier_opt_leaf_none | 0.023 | 23.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_leaf_all | 13345ns | 13345ns | -72.41% |
| carrier_opt_leaf_canon | 16844ns | 16844ns | -65.18% |
| carrier_opt_leaf_cse | 15737ns | 15737ns | -67.47% |
| carrier_opt_leaf_dce | 46441ns | 46441ns | -4.00% |
| carrier_opt_leaf_fold | 34240ns | 34240ns | -29.22% |
| carrier_opt_leaf_none | 48374ns | 48374ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_leaf_none | 45216ns | base | --- | [44156, 48249] | --- | --- | --- | --- |
| carrier_opt_leaf_all | 10935ns | -34499.8ns (-76.3%) | [-37156, -32953]ns | [10711, 11366] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_canon | 13828ns | -31148.8ns (-68.9%) | [-34565, -28438]ns | [13017, 16624] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_cse | 13455ns | -31668.3ns (-70.0%) | [-35121, -30700]ns | [12583, 14093] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_leaf_dce | 43977ns | no significant difference | [-7250, +3226]ns | [40455, 47618] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_leaf_fold | 31628ns | -13587.9ns (-30.1%) | [-16355, -11985]ns | [30652, 33413] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_leaf_none | carrier_opt_leaf_all | carrier_opt_leaf_canon | carrier_opt_leaf_cse | carrier_opt_leaf_dce | carrier_opt_leaf_fold |
|---|---|---|---|---|---|---|
| 1 | 43829ns | -74.2% | -69.0% | -69.3% | +12.2% | -30.7% |
| 2 | 44954ns | -76.3% | -69.3% | -69.2% | +2.5% | -30.4% |
| 3 | 45478ns | -76.3% | -72.6% | -72.6% | -8.6% | -29.7% |
| 4 | 46568ns | -75.5% | -66.9% | -69.2% | -6.7% | -29.5% |
| 5 | 49931ns | -78.5% | -72.3% | -74.5% | -21.2% | -38.0% |
| 6 | 44482ns | -75.0% | -59.9% | -69.7% | +0.0% | -23.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_leaf_all | -0.489 | moderate- |
| carrier_opt_leaf_canon | -0.150 | ok |
| carrier_opt_leaf_cse | -0.812 | HIGH- (thermal bounce) |
| carrier_opt_leaf_dce | 0.122 | ok |
| carrier_opt_leaf_fold | -0.222 | moderate- |
| carrier_opt_leaf_none | -0.036 | ok |

**Consistency summary:**

- **carrier_opt_leaf_all**: won 6/6, lost 0/6
- **carrier_opt_leaf_canon**: won 6/6, lost 0/6
- **carrier_opt_leaf_cse**: won 6/6, lost 0/6
- **carrier_opt_leaf_dce**: won 3/6, lost 2/6
- **carrier_opt_leaf_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_leaf_all | 91407.8ns | 11003.9ns | 830.7% | HIGH |
| carrier_opt_leaf_canon | 91400.2ns | 14490.0ns | 630.8% | HIGH |
| carrier_opt_leaf_cse | 91238.8ns | 13376.9ns | 682.1% | HIGH |
| carrier_opt_leaf_dce | 107013.4ns | 44016.5ns | 243.1% | HIGH |
| carrier_opt_leaf_fold | 95743.3ns | 31897.6ns | 300.2% | HIGH |
| carrier_opt_leaf_none | 99028.4ns | 45873.7ns | 215.9% | HIGH |

## Distribution (algo ns)

```
carrier_opt_leaf_all (n=6, range 10663.3-11366.2 ns)
  10663.3 |########################################
  10698.4 |
  10733.6 |########################################
  10768.7 |########################################
  10803.9 |
  10839.0 |
  10874.2 |
  10909.3 |
  10944.5 |
  10979.6 |
  11014.8 |
  11049.9 |
  11085.1 |########################################
  11120.2 |
  11155.4 |
  11190.5 |
  11225.7 |
  11260.8 |
  11296.0 |########################################
  11331.1 |
  (0 below, 1 above range)

carrier_opt_leaf_canon (n=6, range 12443.7-16623.9 ns)
  12443.7 |####################
  12652.7 |
  12861.7 |
  13070.7 |
  13279.8 |
  13488.8 |####################
  13697.8 |########################################
  13906.8 |
  14115.8 |
  14324.8 |
  14533.8 |
  14742.8 |
  14951.8 |
  15160.9 |
  15369.9 |####################
  15578.9 |
  15787.9 |
  15996.9 |
  16205.9 |
  16414.9 |
  (0 below, 1 above range)

carrier_opt_leaf_cse (n=6, range 12457.1-14092.7 ns)
  12457.1 |####################
  12538.9 |
  12620.7 |
  12702.4 |####################
  12784.2 |
  12866.0 |
  12947.8 |
  13029.6 |
  13111.3 |
  13193.1 |
  13274.9 |
  13356.7 |
  13438.5 |########################################
  13520.2 |
  13602.0 |
  13683.8 |
  13765.6 |####################
  13847.4 |
  13929.1 |
  14010.9 |
  (0 below, 1 above range)

carrier_opt_leaf_dce (n=6, range 39360.4-47617.7 ns)
  39360.4 |########################################
  39773.3 |
  40186.1 |
  40599.0 |
  41011.9 |
  41424.7 |########################################
  41837.6 |
  42250.5 |
  42663.3 |
  43076.2 |########################################
  43489.1 |
  43901.9 |
  44314.8 |########################################
  44727.6 |
  45140.5 |
  45553.4 |
  45966.2 |########################################
  46379.1 |
  46792.0 |
  47204.8 |
  (0 below, 1 above range)

carrier_opt_leaf_fold (n=6, range 30366.2-33412.7 ns)
  30366.2 |########################################
  30518.5 |
  30670.8 |
  30823.2 |########################################
  30975.5 |
  31127.8 |
  31280.2 |########################################
  31432.5 |
  31584.8 |
  31737.1 |
  31889.4 |########################################
  32041.8 |
  32194.1 |
  32346.4 |
  32498.8 |
  32651.1 |
  32803.4 |########################################
  32955.7 |
  33108.0 |
  33260.4 |
  (0 below, 1 above range)

carrier_opt_leaf_none (n=6, range 43829.2-48249.4 ns)
  43829.2 |########################################
  44050.2 |
  44271.2 |########################################
  44492.2 |
  44713.2 |
  44934.2 |########################################
  45155.2 |
  45376.3 |########################################
  45597.3 |
  45818.3 |
  46039.3 |
  46260.3 |
  46481.3 |########################################
  46702.3 |
  46923.3 |
  47144.3 |
  47365.3 |
  47586.3 |
  47807.3 |
  48028.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_leaf_all**: bridge=833.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_canon**: bridge=663.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_cse**: bridge=675.5% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_dce**: bridge=236.3% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_fold**: bridge=300.2% of algo (FFI overhead may distort results)
- **carrier_opt_leaf_none**: bridge=223.7% of algo (FFI overhead may distort results)

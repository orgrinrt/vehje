# Predecoded dispatch shape, real profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_real_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_real_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_real_null dominates: 23% faster than the next best (carrier_pre_real_direct)

carrier_pre_real_null (26.35 us) leads carrier_pre_real_direct (32.39 us) by 23%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_pre_real_null beats baseline by 31% (significant)

carrier_pre_real_null is -11.89 us (31%) faster than baseline carrier_pre_real_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_pre_real_fntable is an outlier: 2.1x slower than the field

carrier_pre_real_fntable (54.69 us) is 2.1x the fastest (26.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} vs {carrier_pre_real_fntable} (29% apart)

The field splits into a fast tier {carrier_pre_real_null, carrier_pre_real_direct, carrier_pre_real_threaded, carrier_pre_real_switch, carrier_pre_real_regcache} and a slow tier {carrier_pre_real_fntable} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_real_null** at 26353.5 ns median (-30.6% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 2.08x (fastest 26353.5 ns, slowest 54685.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_real_direct | 35076ns | 34721ns | 33711ns | 34582ns | 36499ns | -13.22% |
| carrier_pre_real_fntable | 58027ns | 56997ns | 52329ns | 56465ns | 63220ns | +43.56% |
| carrier_pre_real_null | 28600ns | 28671ns | 28124ns | 28607ns | 28827ns | -29.24% |
| carrier_pre_real_regcache | 44685ns | 44769ns | 42144ns | 44426ns | 46346ns | +10.55% |
| carrier_pre_real_switch | 40419ns | 40266ns | 39264ns | 40136ns | 41421ns | base |
| carrier_pre_real_threaded | 38985ns | 38888ns | 38280ns | 38721ns | 39735ns | -3.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_real_direct | 32716ns | 31462ns | 34016ns | -14.20% | 0.031 |
| carrier_pre_real_fntable | 55677ns | 50167ns | 60688ns | +46.01% | 0.018 |
| carrier_pre_real_null | 26284ns | 25839ns | 26482ns | -31.07% | 0.039 |
| carrier_pre_real_regcache | 42409ns | 39995ns | 43997ns | +11.22% | 0.024 |
| carrier_pre_real_switch | 38131ns | 37032ns | 39089ns | base | 0.027 |
| carrier_pre_real_threaded | 36666ns | 35998ns | 37387ns | -3.84% | 0.028 |

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_pre_real_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_real_direct | 0.032 | 79.8% |
| carrier_pre_real_fntable | 0.019 | 47.3% |
| carrier_pre_real_null | 0.039 | 98.0% |
| carrier_pre_real_regcache | 0.024 | 60.8% |
| carrier_pre_real_switch | 0.027 | 68.0% |
| carrier_pre_real_threaded | 0.028 | 70.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_real_direct | 35076ns | 35076ns | -13.22% |
| carrier_pre_real_fntable | 58027ns | 58027ns | +43.56% |
| carrier_pre_real_null | 28600ns | 28600ns | -29.24% |
| carrier_pre_real_regcache | 44685ns | 44685ns | +10.55% |
| carrier_pre_real_switch | 40419ns | 40419ns | base |
| carrier_pre_real_threaded | 38985ns | 38985ns | -3.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_real_switch | 37991ns | base | --- | [37314, 39089] | --- | --- | --- | --- |
| carrier_pre_real_direct | 32391ns | -5238.5ns (-13.8%) | [-6347, -4662]ns | [31740, 34016] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_fntable | 54685ns | +17056.9ns (+44.9%) | [+13569, +22009]ns | [51657, 60688] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_null | 26354ns | -11894.7ns (-31.3%) | [-12607, -11042]ns | [26015, 26482] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_regcache | 42496ns | +4423.7ns (+11.6%) | [+3011, +5400]ns | [40735, 43997] | YES | 0.0391 | 0.0313 | 0 |
| carrier_pre_real_threaded | 36584ns | no significant difference | [-2884, +73]ns | [36028, 37387] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_real_switch | carrier_pre_real_direct | carrier_pre_real_fntable | carrier_pre_real_null | carrier_pre_real_regcache | carrier_pre_real_threaded |
|---|---|---|---|---|---|---|
| 1 | 37596ns | -14.5% | +33.4% | -31.3% | +16.1% | +0.0% |
| 2 | 37758ns | -13.0% | +46.7% | -30.0% | +11.3% | -4.1% |
| 3 | 38579ns | -18.4% | +37.8% | -31.4% | +11.4% | -4.2% |
| 4 | 37032ns | -13.5% | +46.0% | -29.0% | +12.0% | +0.4% |
| 5 | 38224ns | -14.6% | +44.7% | -31.5% | +4.6% | -5.8% |
| 6 | 39599ns | -11.1% | +66.6% | -33.1% | +12.0% | -8.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_real_direct | 0.063 | ok |
| carrier_pre_real_fntable | 0.021 | ok |
| carrier_pre_real_null | -0.176 | ok |
| carrier_pre_real_regcache | -0.293 | moderate- |
| carrier_pre_real_switch | -0.106 | ok |
| carrier_pre_real_threaded | -0.148 | ok |

**Consistency summary:**

- **carrier_pre_real_direct**: won 6/6, lost 0/6
- **carrier_pre_real_fntable**: won 0/6, lost 6/6
- **carrier_pre_real_null**: won 6/6, lost 0/6
- **carrier_pre_real_regcache**: won 0/6, lost 6/6
- **carrier_pre_real_threaded**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_real_direct | 101259.9ns | 32715.6ns | 309.5% | HIGH |
| carrier_pre_real_fntable | 112794.2ns | 55676.5ns | 202.6% | HIGH |
| carrier_pre_real_null | 107430.7ns | 26283.7ns | 408.7% | HIGH |
| carrier_pre_real_regcache | 118593.1ns | 42409.5ns | 279.6% | HIGH |
| carrier_pre_real_switch | 116439.6ns | 38131.4ns | 305.4% | HIGH |
| carrier_pre_real_threaded | 112556.4ns | 36666.2ns | 307.0% | HIGH |

## Distribution (algo ns)

```
carrier_pre_real_direct (n=6, range 31462.5-34016.4 ns)
  31462.5 |########################################
  31590.2 |
  31717.9 |
  31845.6 |
  31973.3 |########################################
  32101.0 |########################################
  32228.7 |
  32356.4 |
  32484.1 |
  32611.8 |########################################
  32739.5 |########################################
  32867.2 |
  32994.9 |
  33122.6 |
  33250.3 |
  33378.0 |
  33505.7 |
  33633.4 |
  33761.1 |
  33888.8 |
  (0 below, 1 above range)

carrier_pre_real_fntable (n=6, range 50166.7-60687.7 ns)
  50166.7 |####################
  50692.8 |
  51218.8 |
  51744.8 |
  52270.9 |
  52796.9 |####################
  53323.0 |
  53849.0 |####################
  54375.1 |
  54901.1 |########################################
  55427.2 |
  55953.2 |
  56479.3 |
  57005.3 |
  57531.4 |
  58057.4 |
  58583.5 |
  59109.5 |
  59635.6 |
  60161.6 |
  (0 below, 1 above range)

carrier_pre_real_null (n=6, range 25839.2-26482.1 ns)
  25839.2 |########################################
  25871.3 |
  25903.5 |
  25935.6 |
  25967.8 |
  25999.9 |
  26032.1 |
  26064.2 |
  26096.4 |
  26128.5 |
  26160.7 |########################################
  26192.8 |
  26224.9 |
  26257.1 |########################################
  26289.2 |
  26321.4 |
  26353.5 |
  26385.7 |
  26417.8 |########################################
  26450.0 |########################################
  (0 below, 1 above range)

carrier_pre_real_regcache (n=6, range 39995.4-43997.3 ns)
  39995.4 |########################################
  40195.5 |
  40395.6 |
  40595.7 |
  40795.8 |
  40995.9 |
  41196.0 |
  41396.1 |########################################
  41596.2 |
  41796.3 |
  41996.4 |########################################
  42196.4 |
  42396.5 |
  42596.6 |
  42796.7 |########################################
  42996.8 |
  43196.9 |
  43397.0 |
  43597.1 |########################################
  43797.2 |
  (0 below, 1 above range)

carrier_pre_real_switch (n=6, range 37032.1-39089.0 ns)
  37032.1 |########################################
  37134.9 |
  37237.8 |
  37340.6 |
  37443.5 |
  37546.3 |########################################
  37649.2 |
  37752.0 |########################################
  37854.9 |
  37957.7 |
  38060.6 |
  38163.4 |########################################
  38266.2 |
  38369.1 |
  38471.9 |
  38574.8 |########################################
  38677.6 |
  38780.5 |
  38883.3 |
  38986.2 |
  (0 below, 1 above range)

carrier_pre_real_threaded (n=6, range 35997.9-37386.9 ns)
  35997.9 |########################################
  36067.3 |
  36136.8 |
  36206.2 |####################
  36275.7 |
  36345.1 |
  36414.6 |
  36484.0 |
  36553.5 |
  36622.9 |
  36692.4 |
  36761.8 |
  36831.3 |
  36900.7 |####################
  36970.2 |
  37039.6 |
  37109.1 |####################
  37178.5 |
  37248.0 |
  37317.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_real_direct**: bridge=311.0% of algo (FFI overhead may distort results)
- **carrier_pre_real_fntable**: bridge=205.2% of algo (FFI overhead may distort results)
- **carrier_pre_real_null**: bridge=408.0% of algo (FFI overhead may distort results)
- **carrier_pre_real_regcache**: bridge=284.6% of algo (FFI overhead may distort results)
- **carrier_pre_real_switch**: bridge=304.7% of algo (FFI overhead may distort results)
- **carrier_pre_real_threaded**: bridge=307.2% of algo (FFI overhead may distort results)

# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: carrier_pre_madd_regcache** at 9819.2 ns median (-18.1% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.23x (fastest 9819.2 ns, slowest 12065.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 14649ns | 14647ns | 14565ns | 14637ns | 14708ns | +0.86% |
| carrier_pre_madd_fntable | 14608ns | 14436ns | 12655ns | 14389ns | 15911ns | +0.58% |
| carrier_pre_madd_null | 13119ns | 13294ns | 12327ns | 13165ns | 13448ns | -9.67% |
| carrier_pre_madd_regcache | 12416ns | 12415ns | 12336ns | 12403ns | 12475ns | -14.51% |
| carrier_pre_madd_switch | 14523ns | 14528ns | 14464ns | 14514ns | 14567ns | base |
| carrier_pre_madd_threaded | 14458ns | 14352ns | 14093ns | 14341ns | 14815ns | -0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 12078ns | 12029ns | 12141ns | +0.78% | 0.021 |
| carrier_pre_madd_fntable | 12052ns | 10429ns | 13239ns | +0.56% | 0.021 |
| carrier_pre_madd_null | 10631ns | 10017ns | 10847ns | -11.30% | 0.024 |
| carrier_pre_madd_regcache | 9850ns | 9756ns | 9949ns | -17.81% | 0.026 |
| carrier_pre_madd_switch | 11985ns | 11906ns | 12040ns | base | 0.021 |
| carrier_pre_madd_threaded | 11838ns | 11637ns | 12103ns | -1.22% | 0.022 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_pre_madd_regcache; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.021 | 80.9% |
| carrier_pre_madd_fntable | 0.022 | 82.3% |
| carrier_pre_madd_null | 0.024 | 90.4% |
| carrier_pre_madd_regcache | 0.026 | 99.4% |
| carrier_pre_madd_switch | 0.021 | 81.3% |
| carrier_pre_madd_threaded | 0.022 | 83.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 14649ns | 14649ns | +0.86% |
| carrier_pre_madd_fntable | 14608ns | 14608ns | +0.58% |
| carrier_pre_madd_null | 13119ns | 13119ns | -9.67% |
| carrier_pre_madd_regcache | 12416ns | 12416ns | -14.51% |
| carrier_pre_madd_switch | 14523ns | 14523ns | base |
| carrier_pre_madd_threaded | 14458ns | 14458ns | -0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 11993ns | base | --- | [11922, 12040] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 12065ns | +78.2ns (+0.7%) | [+23, +180]ns | [12030, 12141] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_fntable | 11847ns | no significant difference | [-853, +1227]ns | [11069, 13239] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_madd_null | 10788ns | -1227.6ns (-10.2%) | [-1695, -1138]ns | [10259, 10847] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 9819ns | -2125.4ns (-17.7%) | [-2230, -2049]ns | [9782, 9949] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 11755ns | no significant difference | [-339, +133]ns | [11657, 12103] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 11906ns | +1.0% | -12.4% | -11.8% | -17.6% | -1.9% |
| 2 | 11938ns | +2.0% | -1.9% | -9.4% | -17.0% | +1.3% |
| 3 | 12058ns | +0.4% | -1.6% | -10.7% | -17.1% | -2.0% |
| 4 | 11983ns | +0.9% | -1.2% | -9.7% | -18.0% | -2.9% |
| 5 | 12022ns | +0.1% | +0.0% | -9.6% | -18.4% | -2.8% |
| 6 | 12002ns | +0.3% | +20.4% | -16.5% | -18.7% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.002 | ok |
| carrier_pre_madd_fntable | 0.070 | ok |
| carrier_pre_madd_null | -0.141 | ok |
| carrier_pre_madd_regcache | 0.209 | moderate+ |
| carrier_pre_madd_switch | 0.044 | ok |
| carrier_pre_madd_threaded | -0.232 | moderate- |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 0/6, lost 5/6
- **carrier_pre_madd_fntable**: won 4/6, lost 1/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 6/6, lost 0/6
- **carrier_pre_madd_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 94764.2ns | 12078.5ns | 784.6% | HIGH |
| carrier_pre_madd_fntable | 93672.3ns | 12051.9ns | 777.2% | HIGH |
| carrier_pre_madd_null | 90093.0ns | 10631.1ns | 847.4% | HIGH |
| carrier_pre_madd_regcache | 89591.9ns | 9849.9ns | 909.6% | HIGH |
| carrier_pre_madd_switch | 89649.0ns | 11984.9ns | 748.0% | HIGH |
| carrier_pre_madd_threaded | 93308.3ns | 11838.3ns | 788.2% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 12028.8-12140.6 ns)
  12028.8 |########################################
  12034.4 |
  12040.0 |####################
  12045.6 |
  12051.2 |
  12056.8 |
  12062.3 |
  12067.9 |
  12073.5 |
  12079.1 |
  12084.7 |####################
  12090.3 |
  12095.9 |
  12101.5 |
  12107.1 |####################
  12112.6 |
  12118.2 |
  12123.8 |
  12129.4 |
  12135.0 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 10429.2-13239.1 ns)
  10429.2 |####################
  10569.7 |
  10710.2 |
  10850.7 |
  10991.2 |
  11131.7 |
  11272.2 |
  11412.7 |
  11553.2 |
  11693.7 |####################
  11834.2 |########################################
  11974.7 |####################
  12115.2 |
  12255.7 |
  12396.2 |
  12536.7 |
  12677.2 |
  12817.7 |
  12958.2 |
  13098.7 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 10016.7-10846.9 ns)
  10016.7 |####################
  10058.2 |
  10099.7 |
  10141.2 |
  10182.7 |
  10224.2 |
  10265.8 |
  10307.3 |
  10348.8 |
  10390.3 |
  10431.8 |
  10473.3 |####################
  10514.8 |
  10556.3 |
  10597.8 |
  10639.4 |
  10680.9 |
  10722.4 |
  10763.9 |####################
  10805.4 |########################################
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 9755.8-9948.8 ns)
   9755.8 |########################################
   9765.4 |
   9775.1 |
   9784.7 |
   9794.4 |
   9804.0 |########################################
   9813.7 |########################################
   9823.3 |########################################
   9833.0 |
   9842.6 |
   9852.3 |
   9861.9 |
   9871.6 |
   9881.2 |
   9890.9 |
   9900.5 |########################################
   9910.2 |
   9919.8 |
   9929.5 |
   9939.1 |
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 11906.2-12040.0 ns)
  11906.2 |########################################
  11912.9 |
  11919.6 |
  11926.3 |
  11933.0 |########################################
  11939.7 |
  11946.3 |
  11953.0 |
  11959.7 |
  11966.4 |
  11973.1 |
  11979.8 |########################################
  11986.5 |
  11993.2 |
  11999.9 |########################################
  12006.5 |
  12013.2 |
  12019.9 |########################################
  12026.6 |
  12033.3 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 11637.1-12103.1 ns)
  11637.1 |########################################
  11660.4 |########################################
  11683.7 |########################################
  11707.0 |
  11730.3 |
  11753.6 |
  11776.9 |
  11800.2 |########################################
  11823.5 |
  11846.8 |
  11870.1 |
  11893.4 |
  11916.7 |
  11940.0 |
  11963.3 |
  11986.6 |
  12009.9 |
  12033.2 |
  12056.5 |
  12079.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=786.1% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=788.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=837.6% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=909.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=744.8% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=797.3% of algo (FFI overhead may distort results)

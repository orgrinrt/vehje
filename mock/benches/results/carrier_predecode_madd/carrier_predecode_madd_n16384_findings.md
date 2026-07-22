# Predecoded dispatch shape, madd profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_madd_threaded shows alternating (throttle bounce) (autocorr -0.53)

carrier_pre_madd_threaded's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_pre_madd_null, carrier_pre_madd_threaded, carrier_pre_madd_fntable, carrier_pre_madd_direct, carrier_pre_madd_switch} vs {carrier_pre_madd_regcache} (29% apart)

The field splits into a fast tier {carrier_pre_madd_null, carrier_pre_madd_threaded, carrier_pre_madd_fntable, carrier_pre_madd_direct, carrier_pre_madd_switch} and a slow tier {carrier_pre_madd_regcache} with a 29% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_pre_madd_null** at 689321.7 ns median (-10.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.44x (fastest 689321.7 ns, slowest 990537.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_madd_direct | 767958ns | 768158ns | 761488ns | 766710ns | 773065ns | -0.06% |
| carrier_pre_madd_fntable | 764253ns | 763186ns | 760507ns | 763136ns | 767801ns | -0.54% |
| carrier_pre_madd_null | 691142ns | 691602ns | 685287ns | 689572ns | 696424ns | -10.05% |
| carrier_pre_madd_regcache | 990738ns | 992801ns | 930694ns | 986104ns | 1027712ns | +28.94% |
| carrier_pre_madd_switch | 768385ns | 768715ns | 760542ns | 766829ns | 774642ns | base |
| carrier_pre_madd_threaded | 762174ns | 760527ns | 758440ns | 760094ns | 767161ns | -0.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_madd_direct | 765698ns | 759245ns | 770786ns | -0.05% | 0.021 |
| carrier_pre_madd_fntable | 761927ns | 758182ns | 765430ns | -0.54% | 0.022 |
| carrier_pre_madd_null | 688833ns | 683005ns | 694039ns | -10.08% | 0.024 |
| carrier_pre_madd_regcache | 988421ns | 928349ns | 1025361ns | +29.02% | 0.017 |
| carrier_pre_madd_switch | 766081ns | 758303ns | 772277ns | base | 0.021 |
| carrier_pre_madd_threaded | 759890ns | 756112ns | 764883ns | -0.81% | 0.022 |

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_pre_madd_null; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_madd_direct | 0.021 | 89.2% |
| carrier_pre_madd_fntable | 0.022 | 89.8% |
| carrier_pre_madd_null | 0.024 | 99.1% |
| carrier_pre_madd_regcache | 0.017 | 69.0% |
| carrier_pre_madd_switch | 0.021 | 89.1% |
| carrier_pre_madd_threaded | 0.022 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_madd_direct | 767958ns | 767958ns | -0.06% |
| carrier_pre_madd_fntable | 764253ns | 764253ns | -0.54% |
| carrier_pre_madd_null | 691142ns | 691142ns | -10.05% |
| carrier_pre_madd_regcache | 990738ns | 990738ns | +28.94% |
| carrier_pre_madd_switch | 768385ns | 768385ns | base |
| carrier_pre_madd_threaded | 762174ns | 762174ns | -0.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_madd_switch | 766400ns | base | --- | [759566, 772277] | --- | --- | --- | --- |
| carrier_pre_madd_direct | 765892ns | no significant difference | [-9607, +11220]ns | [760416, 770786] | no | 0.6875 | 0.6875 | 0 |
| carrier_pre_madd_fntable | 760909ns | no significant difference | [-11158, +913]ns | [759442, 765430] | no | 0.3646 | 0.2188 | 0 |
| carrier_pre_madd_null | 689322ns | -74876.2ns (-9.8%) | [-84217, -72651]ns | [683138, 694039] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_regcache | 990537ns | +226261.9ns (+29.5%) | [+178427, +262331]ns | [949365, 1025361] | YES (adj: no) | 0.0781 | 0.0313 | 0 |
| carrier_pre_madd_threaded | 758251ns | no significant difference | [-15498, +1567]ns | [756535, 764883] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_madd_switch | carrier_pre_madd_direct | carrier_pre_madd_fntable | carrier_pre_madd_null | carrier_pre_madd_regcache | carrier_pre_madd_threaded |
|---|---|---|---|---|---|---|
| 1 | 776798ns | -1.4% | -2.1% | -10.8% | +25.0% | -2.6% |
| 2 | 760829ns | +1.8% | -0.0% | -9.8% | +32.8% | +0.3% |
| 3 | 765533ns | -0.5% | -0.6% | -9.5% | +21.3% | -1.1% |
| 4 | 758303ns | +1.1% | -0.0% | -9.9% | +34.9% | +0.1% |
| 5 | 767756ns | -0.2% | +0.2% | -11.0% | +33.9% | -0.1% |
| 6 | 767268ns | -1.0% | -0.8% | -9.4% | +26.5% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_madd_direct | -0.314 | moderate- |
| carrier_pre_madd_fntable | -0.397 | moderate- |
| carrier_pre_madd_null | -0.322 | moderate- |
| carrier_pre_madd_regcache | -0.417 | moderate- |
| carrier_pre_madd_switch | -0.290 | moderate- |
| carrier_pre_madd_threaded | -0.530 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_pre_madd_direct**: won 4/6, lost 2/6
- **carrier_pre_madd_fntable**: won 3/6, lost 1/6
- **carrier_pre_madd_null**: won 6/6, lost 0/6
- **carrier_pre_madd_regcache**: won 0/6, lost 6/6
- **carrier_pre_madd_threaded**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_madd_direct | 814387.7ns | 765698.2ns | 106.4% | HIGH |
| carrier_pre_madd_fntable | 798917.6ns | 761927.3ns | 104.9% | HIGH |
| carrier_pre_madd_null | 725324.7ns | 688833.0ns | 105.3% | HIGH |
| carrier_pre_madd_regcache | 1024854.6ns | 988421.1ns | 103.7% | HIGH |
| carrier_pre_madd_switch | 801642.8ns | 766081.0ns | 104.6% | HIGH |
| carrier_pre_madd_threaded | 796306.7ns | 759889.6ns | 104.8% | HIGH |

## Distribution (algo ns)

```
carrier_pre_madd_direct (n=6, range 759245.4-770786.2 ns)
  759245.4 |####################
  759822.4 |
  760399.5 |
  760976.5 |
  761553.6 |####################
  762130.6 |
  762707.7 |
  763284.7 |
  763861.7 |
  764438.8 |
  765015.8 |
  765592.9 |####################
  766169.9 |########################################
  766747.0 |
  767324.0 |
  767901.0 |
  768478.1 |
  769055.1 |
  769632.2 |
  770209.2 |
  (0 below, 1 above range)

carrier_pre_madd_fntable (n=6, range 758181.7-765430.4 ns)
  758181.7 |####################
  758544.1 |
  758906.6 |
  759269.0 |
  759631.4 |
  759993.9 |
  760356.3 |####################
  760718.7 |########################################
  761081.2 |####################
  761443.6 |
  761806.1 |
  762168.5 |
  762530.9 |
  762893.4 |
  763255.8 |
  763618.2 |
  763980.7 |
  764343.1 |
  764705.5 |
  765068.0 |
  (0 below, 1 above range)

carrier_pre_madd_null (n=6, range 683005.0-694039.2 ns)
  683005.0 |########################################
  683556.7 |
  684108.4 |
  684660.1 |
  685211.8 |
  685763.5 |####################
  686315.2 |
  686867.0 |
  687418.7 |
  687970.4 |
  688522.1 |
  689073.8 |
  689625.5 |
  690177.2 |
  690728.9 |
  691280.6 |
  691832.3 |
  692384.0 |####################
  692935.7 |####################
  693487.4 |
  (0 below, 1 above range)

carrier_pre_madd_regcache (n=6, range 928348.7-1025360.8 ns)
  928348.7 |####################
  933199.3 |
  938049.9 |
  942900.5 |
  947751.1 |
  952601.7 |
  957452.3 |
  962302.9 |
  967153.5 |########################################
  972004.1 |
  976854.8 |
  981705.4 |
  986556.0 |
  991406.6 |
  996257.2 |
  1001107.8 |
  1005958.4 |####################
  1010809.0 |
  1015659.6 |
  1020510.2 |####################
  (0 below, 1 above range)

carrier_pre_madd_switch (n=6, range 758302.9-772276.9 ns)
  758302.9 |########################################
  759001.6 |
  759700.3 |
  760399.0 |########################################
  761097.7 |
  761796.4 |
  762495.1 |
  763193.8 |
  763892.5 |
  764591.2 |
  765289.9 |########################################
  765988.6 |
  766687.3 |########################################
  767386.0 |########################################
  768084.7 |
  768783.4 |
  769482.1 |
  770180.8 |
  770879.5 |
  771578.2 |
  (0 below, 1 above range)

carrier_pre_madd_threaded (n=6, range 756112.5-764882.7 ns)
  756112.5 |########################################
  756551.0 |########################################
  756989.5 |########################################
  757428.0 |
  757866.5 |
  758305.1 |
  758743.6 |
  759182.1 |########################################
  759620.6 |
  760059.1 |
  760497.6 |
  760936.1 |
  761374.6 |
  761813.1 |
  762251.6 |
  762690.1 |########################################
  763128.7 |
  763567.2 |
  764005.7 |
  764444.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_madd_direct**: bridge=106.4% of algo (FFI overhead may distort results)
- **carrier_pre_madd_fntable**: bridge=104.9% of algo (FFI overhead may distort results)
- **carrier_pre_madd_null**: bridge=105.3% of algo (FFI overhead may distort results)
- **carrier_pre_madd_regcache**: bridge=103.7% of algo (FFI overhead may distort results)
- **carrier_pre_madd_switch**: bridge=104.7% of algo (FFI overhead may distort results)
- **carrier_pre_madd_threaded**: bridge=104.8% of algo (FFI overhead may distort results)

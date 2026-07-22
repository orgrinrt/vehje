# Dispatch shape over the wire form, madd profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_madd_bittree is inconsistent: worst-20% is 2.3x its best-20%

carrier_disp_madd_bittree's best 20% of batches run at 713.65 us but its worst 20% at 1.61 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_disp_madd_nullfloor** at 650330.8 ns median (-17.0% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.37x (fastest 650330.8 ns, slowest 893270.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 1016059ns | 719009ns | 715991ns | 718704ns | 1612125ns | +28.92% |
| carrier_disp_madd_fntable | 872126ns | 871481ns | 869797ns | 871306ns | 874522ns | +10.66% |
| carrier_disp_madd_ifchain | 782357ns | 781795ns | 780603ns | 781730ns | 784175ns | -0.73% |
| carrier_disp_madd_ifchainasc | 784414ns | 785058ns | 774343ns | 781664ns | 793574ns | -0.47% |
| carrier_disp_madd_ifchainlin | 723481ns | 694950ns | 690891ns | 693657ns | 784512ns | -8.20% |
| carrier_disp_madd_nullfloor | 653309ns | 652780ns | 652375ns | 652687ns | 654710ns | -17.11% |
| carrier_disp_madd_switch | 788126ns | 786729ns | 785912ns | 786651ns | 791446ns | base |
| carrier_disp_madd_threaded | 900772ns | 896836ns | 892687ns | 895816ns | 912248ns | +14.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_madd_bittree | 1013168ns | 713648ns | 1609218ns | +29.09% | 0.016 |
| carrier_disp_madd_fntable | 869350ns | 867497ns | 871418ns | +10.76% | 0.019 |
| carrier_disp_madd_ifchain | 779509ns | 777920ns | 781073ns | -0.68% | 0.021 |
| carrier_disp_madd_ifchainasc | 781327ns | 771052ns | 790925ns | -0.45% | 0.021 |
| carrier_disp_madd_ifchainlin | 720208ns | 688102ns | 780419ns | -8.24% | 0.023 |
| carrier_disp_madd_nullfloor | 650595ns | 648874ns | 652075ns | -17.11% | 0.025 |
| carrier_disp_madd_switch | 784878ns | 782708ns | 788299ns | base | 0.021 |
| carrier_disp_madd_threaded | 897715ns | 890025ns | 909640ns | +14.38% | 0.018 |

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_disp_madd_nullfloor; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_madd_bittree | 0.023 | 90.6% |
| carrier_disp_madd_fntable | 0.019 | 74.7% |
| carrier_disp_madd_ifchain | 0.021 | 83.3% |
| carrier_disp_madd_ifchainasc | 0.021 | 83.0% |
| carrier_disp_madd_ifchainlin | 0.024 | 93.8% |
| carrier_disp_madd_nullfloor | 0.025 | 99.8% |
| carrier_disp_madd_switch | 0.021 | 82.8% |
| carrier_disp_madd_threaded | 0.018 | 72.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_madd_bittree | 1016059ns | 1016059ns | +28.92% |
| carrier_disp_madd_fntable | 872126ns | 872126ns | +10.66% |
| carrier_disp_madd_ifchain | 782357ns | 782357ns | -0.73% |
| carrier_disp_madd_ifchainasc | 784414ns | 784414ns | -0.47% |
| carrier_disp_madd_ifchainlin | 723481ns | 723481ns | -8.20% |
| carrier_disp_madd_nullfloor | 653309ns | 653309ns | -17.11% |
| carrier_disp_madd_switch | 788126ns | 788126ns | base |
| carrier_disp_madd_threaded | 900772ns | 900772ns | +14.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_madd_switch | 783578ns | base | --- | [782756, 788299] | --- | --- | --- | --- |
| carrier_disp_madd_bittree | 716229ns | no significant difference | [-72772, +825810]ns | [714058, 1609218] | no | 0.2552 | 0.2188 | 0 |
| carrier_disp_madd_fntable | 868633ns | +84686.2ns (+10.8%) | [+80319, +88412]ns | [868000, 871418] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_madd_ifchain | 779377ns | -4626.7ns (-0.6%) | [-9547, -1932]ns | [778077, 781073] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_madd_ifchainasc | 781520ns | no significant difference | [-11517, +2626]ns | [771536, 790925] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_madd_ifchainlin | 691864ns | -91841.4ns (-11.7%) | [-94414, -7753]ns | [688341, 780419] | YES (adj: no) | 0.2552 | 0.2188 | 0 |
| carrier_disp_madd_nullfloor | 650331ns | -133124.8ns (-17.0%) | [-138792, -130930]ns | [649380, 652075] | YES (adj: no) | 0.0547 | 0.0313 | 0 |
| carrier_disp_madd_threaded | 893270ns | +109691.6ns (+14.0%) | [+107480, +121341]ns | [890236, 909640] | YES (adj: no) | 0.0547 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_madd_switch | carrier_disp_madd_bittree | carrier_disp_madd_fntable | carrier_disp_madd_ifchain | carrier_disp_madd_ifchainasc | carrier_disp_madd_ifchainlin | carrier_disp_madd_nullfloor | carrier_disp_madd_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 782708ns | -7.4% | +11.6% | -0.4% | -0.0% | -12.1% | -16.5% | +13.7% |
| 2 | 783303ns | -8.6% | +11.0% | -0.1% | -1.4% | -11.7% | -16.9% | +13.8% |
| 3 | 782803ns | -8.8% | +10.8% | -0.6% | -1.5% | -12.0% | -17.0% | +13.8% |
| 4 | 792491ns | -9.6% | +9.6% | -1.7% | +0.6% | +9.5% | -18.0% | +16.4% |
| 5 | 784108ns | +218.1% | +10.8% | -0.7% | +0.1% | -11.7% | -17.0% | +14.4% |
| 6 | 783854ns | -8.9% | +10.8% | -0.6% | -0.4% | -11.6% | -17.2% | +14.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_madd_bittree | -0.235 | moderate- |
| carrier_disp_madd_fntable | 0.123 | ok |
| carrier_disp_madd_ifchain | -0.249 | moderate- |
| carrier_disp_madd_ifchainasc | -0.048 | ok |
| carrier_disp_madd_ifchainlin | -0.239 | moderate- |
| carrier_disp_madd_nullfloor | 0.072 | ok |
| carrier_disp_madd_switch | -0.199 | ok |
| carrier_disp_madd_threaded | -0.135 | ok |

**Consistency summary:**

- **carrier_disp_madd_bittree**: won 5/6, lost 1/6
- **carrier_disp_madd_fntable**: won 0/6, lost 6/6
- **carrier_disp_madd_ifchain**: won 6/6, lost 0/6
- **carrier_disp_madd_ifchainasc**: won 3/6, lost 2/6
- **carrier_disp_madd_ifchainlin**: won 5/6, lost 1/6
- **carrier_disp_madd_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_madd_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_madd_bittree | 736338.8ns | 1013168.3ns | 72.7% | HIGH |
| carrier_disp_madd_fntable | 883316.1ns | 869350.1ns | 101.6% | HIGH |
| carrier_disp_madd_ifchain | 782239.0ns | 779509.0ns | 100.4% | HIGH |
| carrier_disp_madd_ifchainasc | 784386.9ns | 781327.2ns | 100.4% | HIGH |
| carrier_disp_madd_ifchainlin | 738713.8ns | 720208.2ns | 102.6% | HIGH |
| carrier_disp_madd_nullfloor | 653429.7ns | 650595.3ns | 100.4% | HIGH |
| carrier_disp_madd_switch | 787709.3ns | 784877.8ns | 100.4% | HIGH |
| carrier_disp_madd_threaded | 901276.6ns | 897715.3ns | 100.4% | HIGH |

## Distribution (algo ns)

```
carrier_disp_madd_bittree (n=6, range 713647.5-1609218.1 ns)
  713647.5 |########################################
  758426.0 |
  803204.6 |
  847983.1 |
  892761.6 |
  937540.1 |
  982318.7 |
  1027097.2 |
  1071875.7 |
  1116654.3 |
  1161432.8 |
  1206211.3 |
  1250989.9 |
  1295768.4 |
  1340546.9 |
  1385325.4 |
  1430104.0 |
  1474882.5 |
  1519661.0 |
  1564439.6 |
  (0 below, 1 above range)

carrier_disp_madd_fntable (n=6, range 867497.1-871417.5 ns)
  867497.1 |####################
  867693.1 |
  867889.1 |
  868085.2 |
  868281.2 |
  868477.2 |########################################
  868673.2 |####################
  868869.2 |
  869065.3 |
  869261.3 |
  869457.3 |####################
  869653.3 |
  869849.3 |
  870045.4 |
  870241.4 |
  870437.4 |
  870633.4 |
  870829.4 |
  871025.5 |
  871221.5 |
  (0 below, 1 above range)

carrier_disp_madd_ifchain (n=6, range 777919.6-781073.3 ns)
  777919.6 |########################################
  778077.3 |########################################
  778235.0 |
  778392.7 |
  778550.3 |
  778708.0 |
  778865.7 |
  779023.4 |
  779181.1 |########################################
  779338.8 |########################################
  779496.5 |
  779654.2 |########################################
  779811.8 |
  779969.5 |
  780127.2 |
  780284.9 |
  780442.6 |
  780600.3 |
  780758.0 |
  780915.7 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainasc (n=6, range 771051.7-790925.0 ns)
  771051.7 |########################################
  772045.4 |
  773039.0 |
  774032.7 |
  775026.4 |
  776020.0 |
  777013.7 |
  778007.4 |
  779001.0 |
  779994.7 |####################
  780988.3 |
  781982.0 |####################
  782975.7 |
  783969.3 |####################
  784963.0 |
  785956.7 |
  786950.3 |
  787944.0 |
  788937.7 |
  789931.3 |
  (0 below, 1 above range)

carrier_disp_madd_ifchainlin (n=6, range 688101.7-780419.3 ns)
  688101.7 |########################################
  692717.6 |##########
  697333.5 |
  701949.3 |
  706565.2 |
  711181.1 |
  715797.0 |
  720412.9 |
  725028.8 |
  729644.6 |
  734260.5 |
  738876.4 |
  743492.3 |
  748108.2 |
  752724.1 |
  757339.9 |
  761955.8 |
  766571.7 |
  771187.6 |
  775803.5 |
  (0 below, 1 above range)

carrier_disp_madd_nullfloor (n=6, range 648874.2-652075.0 ns)
  648874.2 |########################################
  649034.2 |
  649194.3 |
  649354.3 |
  649514.4 |
  649674.4 |
  649834.4 |########################################
  649994.5 |########################################
  650154.5 |
  650314.6 |
  650474.6 |########################################
  650634.6 |
  650794.7 |########################################
  650954.7 |
  651114.8 |
  651274.8 |
  651434.8 |
  651594.9 |
  651754.9 |
  651915.0 |
  (0 below, 1 above range)

carrier_disp_madd_switch (n=6, range 782707.9-788299.4 ns)
  782707.9 |########################################
  782987.5 |
  783267.0 |####################
  783546.6 |
  783826.2 |####################
  784105.8 |####################
  784385.3 |
  784664.9 |
  784944.5 |
  785224.1 |
  785503.6 |
  785783.2 |
  786062.8 |
  786342.3 |
  786621.9 |
  786901.5 |
  787181.1 |
  787460.6 |
  787740.2 |
  788019.8 |
  (0 below, 1 above range)

carrier_disp_madd_threaded (n=6, range 890025.4-909640.2 ns)
  890025.4 |########################################
  891006.1 |####################
  891986.9 |
  892967.6 |
  893948.4 |####################
  894929.1 |
  895909.8 |
  896890.6 |####################
  897871.3 |
  898852.1 |
  899832.8 |
  900813.5 |
  901794.3 |
  902775.0 |
  903755.8 |
  904736.5 |
  905717.2 |
  906698.0 |
  907678.7 |
  908659.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_madd_bittree**: CV=65.4% (high variance, measurements may be unstable)
- **carrier_disp_madd_bittree**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_fntable**: bridge=101.7% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchain**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainasc**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_ifchainlin**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_disp_madd_nullfloor**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_disp_madd_switch**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_disp_madd_threaded**: bridge=100.2% of algo (FFI overhead may distort results)

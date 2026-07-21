# Strategy showdown (skew2): all strategies x tiers (native/interp) same footing

12 variants, 6 samples per variant.
Baseline: **sd_bintree_int_skew2**

## Key findings

- **Fastest: sd_bintree_nat_skew2** at 7423.1 ns median (-81.4% vs baseline)
- 8 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 6.18x (fastest 7423.1 ns, slowest 45897.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 42145ns | 42479ns | 37857ns | 42336ns | 44002ns | base |
| sd_bintree_nat_skew2 | 10211ns | 9835ns | 9467ns | 9790ns | 11214ns | -75.77% |
| sd_chain_int_skew2 | 48314ns | 48228ns | 43912ns | 47407ns | 51876ns | +14.64% |
| sd_chain_nat_skew2 | 10452ns | 10280ns | 9859ns | 10153ns | 11197ns | -75.20% |
| sd_chain_rev_int_skew2 | 41981ns | 40832ns | 39039ns | 40308ns | 45964ns | -0.39% |
| sd_chain_rev_nat_skew2 | 11502ns | 11539ns | 10987ns | 11496ns | 11767ns | -72.71% |
| sd_evalall_int_skew2 | 27146ns | 27333ns | 24458ns | 27041ns | 28647ns | -35.59% |
| sd_evalall_nat_skew2 | 28403ns | 27220ns | 24304ns | 26413ns | 33438ns | -32.61% |
| sd_jumptable_int_skew2 | 10869ns | 10736ns | 9552ns | 10440ns | 12171ns | -74.21% |
| sd_jumptable_nat_skew2 | 10858ns | 11007ns | 9811ns | 10758ns | 11532ns | -74.24% |
| sd_profiled_int_skew2 | 42516ns | 43047ns | 40620ns | 42408ns | 43627ns | +0.88% |
| sd_profiled_nat_skew2 | 11075ns | 11038ns | 10061ns | 10792ns | 12007ns | -73.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 39602ns | 35567ns | 41406ns | base | 0.103 |
| sd_bintree_nat_skew2 | 7751ns | 7047ns | 8691ns | -80.43% | 0.528 |
| sd_chain_int_skew2 | 45947ns | 41690ns | 49364ns | +16.02% | 0.089 |
| sd_chain_nat_skew2 | 8113ns | 7678ns | 8693ns | -79.51% | 0.505 |
| sd_chain_rev_int_skew2 | 39574ns | 36740ns | 43353ns | -0.07% | 0.104 |
| sd_chain_rev_nat_skew2 | 8973ns | 8564ns | 9195ns | -77.34% | 0.456 |
| sd_evalall_int_skew2 | 24684ns | 22207ns | 26037ns | -37.67% | 0.166 |
| sd_evalall_nat_skew2 | 25855ns | 22042ns | 30512ns | -34.71% | 0.158 |
| sd_jumptable_int_skew2 | 8470ns | 7450ns | 9545ns | -78.61% | 0.484 |
| sd_jumptable_nat_skew2 | 8363ns | 7565ns | 8875ns | -78.88% | 0.490 |
| sd_profiled_int_skew2 | 39944ns | 38140ns | 40959ns | +0.86% | 0.103 |
| sd_profiled_nat_skew2 | 8656ns | 7890ns | 9389ns | -78.14% | 0.473 |

## Performance model

- Peak throughput: **0.581 Gops/s** (sd_bintree_nat_skew2; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sd_bintree_int_skew2 | 0.103 | 17.7% |
| sd_bintree_nat_skew2 | 0.552 | 94.9% |
| sd_chain_int_skew2 | 0.089 | 15.4% |
| sd_chain_nat_skew2 | 0.516 | 88.7% |
| sd_chain_rev_int_skew2 | 0.106 | 18.3% |
| sd_chain_rev_nat_skew2 | 0.454 | 78.1% |
| sd_evalall_int_skew2 | 0.165 | 28.3% |
| sd_evalall_nat_skew2 | 0.166 | 28.5% |
| sd_jumptable_int_skew2 | 0.493 | 84.8% |
| sd_jumptable_nat_skew2 | 0.481 | 82.8% |
| sd_profiled_int_skew2 | 0.101 | 17.4% |
| sd_profiled_nat_skew2 | 0.476 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sd_bintree_int_skew2 | 42145ns | 42145ns | base |
| sd_bintree_nat_skew2 | 10211ns | 10211ns | -75.77% |
| sd_chain_int_skew2 | 48314ns | 48314ns | +14.64% |
| sd_chain_nat_skew2 | 10452ns | 10452ns | -75.20% |
| sd_chain_rev_int_skew2 | 41981ns | 41981ns | -0.39% |
| sd_chain_rev_nat_skew2 | 11502ns | 11502ns | -72.71% |
| sd_evalall_int_skew2 | 27146ns | 27146ns | -35.59% |
| sd_evalall_nat_skew2 | 28403ns | 28403ns | -32.61% |
| sd_jumptable_int_skew2 | 10869ns | 10869ns | -74.21% |
| sd_jumptable_nat_skew2 | 10858ns | 10858ns | -74.24% |
| sd_profiled_int_skew2 | 42516ns | 42516ns | +0.88% |
| sd_profiled_nat_skew2 | 11075ns | 11075ns | -73.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sd_bintree_int_skew2 | 39886ns | base | --- | [37513, 41406] | --- | --- | --- | --- |
| sd_bintree_nat_skew2 | 7423ns | -32245.8ns (-80.8%) | [-34143, -29164]ns | [7139, 8691] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_int_skew2 | 45897ns | +6944.5ns (+17.4%) | [+2272, +9819]ns | [42581, 49364] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_nat_skew2 | 7945ns | -31564.6ns (-79.1%) | [-33645, -29256]ns | [7702, 8693] | YES | 0.0382 | 0.0313 | 0 |
| sd_chain_rev_int_skew2 | 38500ns | no significant difference | [-2621, +2888]ns | [36869, 43353] | no | 1.0000 | 1.0000 | 0 |
| sd_chain_rev_nat_skew2 | 9024ns | -30763.6ns (-77.1%) | [-32593, -28531]ns | [8700, 9195] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_int_skew2 | 24889ns | -14655.4ns (-36.7%) | [-17373, -12725]ns | [23127, 26037] | YES | 0.0382 | 0.0313 | 0 |
| sd_evalall_nat_skew2 | 24749ns | -14270.4ns (-35.8%) | [-17681, -9289]ns | [22304, 30512] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_int_skew2 | 8313ns | -30993.0ns (-77.7%) | [-33458, -28944]ns | [7553, 9545] | YES | 0.0382 | 0.0313 | 0 |
| sd_jumptable_nat_skew2 | 8508ns | -31590.1ns (-79.2%) | [-33344, -28783]ns | [7706, 8875] | YES | 0.0382 | 0.0313 | 0 |
| sd_profiled_int_skew2 | 40461ns | no significant difference | [-1273, +2168]ns | [38413, 40959] | no | 1.0000 | 1.0000 | 0 |
| sd_profiled_nat_skew2 | 8611ns | -30908.2ns (-77.5%) | [-33019, -28910]ns | [7968, 9389] | YES | 0.0382 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sd_bintree_int_skew2 | sd_bintree_nat_skew2 | sd_chain_int_skew2 | sd_chain_nat_skew2 | sd_chain_rev_int_skew2 | sd_chain_rev_nat_skew2 | sd_evalall_int_skew2 | sd_evalall_nat_skew2 | sd_jumptable_int_skew2 | sd_jumptable_nat_skew2 | sd_profiled_int_skew2 | sd_profiled_nat_skew2 |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | 41469ns | -82.6% | +4.8% | -81.4% | +3.6% | -78.1% | -37.6% | -45.6% | -82.0% | -81.8% | -1.9% | -80.6% |
| 2 | 39460ns | -80.9% | +22.2% | -77.6% | +10.8% | -77.2% | -38.1% | -33.4% | -79.3% | -77.2% | +4.5% | -78.5% |
| 3 | 41343ns | -82.4% | +6.1% | -81.1% | -4.5% | -79.3% | -46.3% | -39.8% | -79.6% | -79.3% | -2.6% | -78.9% |
| 4 | 35567ns | -78.1% | +17.2% | -78.4% | +3.3% | -75.2% | -32.4% | -38.0% | -78.5% | -76.2% | +7.2% | -77.8% |
| 5 | 40144ns | -76.1% | +19.3% | -78.7% | -6.5% | -77.4% | -34.7% | -13.4% | -76.5% | -78.2% | -3.6% | -76.9% |
| 6 | 39629ns | -82.2% | +27.5% | -79.6% | -6.6% | -76.5% | -36.0% | -37.9% | -75.6% | -80.2% | +2.7% | -76.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sd_bintree_int_skew2 | -0.420 | moderate- |
| sd_bintree_nat_skew2 | -0.240 | moderate- |
| sd_chain_int_skew2 | -0.016 | ok |
| sd_chain_nat_skew2 | -0.494 | moderate- |
| sd_chain_rev_int_skew2 | 0.525 | HIGH+ (drift/warm-up) |
| sd_chain_rev_nat_skew2 | 0.236 | moderate+ |
| sd_evalall_int_skew2 | 0.185 | ok |
| sd_evalall_nat_skew2 | -0.401 | moderate- |
| sd_jumptable_int_skew2 | 0.165 | ok |
| sd_jumptable_nat_skew2 | -0.351 | moderate- |
| sd_profiled_int_skew2 | 0.273 | moderate+ |
| sd_profiled_nat_skew2 | 0.035 | ok |

**Consistency summary:**

- **sd_bintree_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_int_skew2**: won 0/6, lost 6/6
- **sd_chain_nat_skew2**: won 6/6, lost 0/6
- **sd_chain_rev_int_skew2**: won 3/6, lost 3/6
- **sd_chain_rev_nat_skew2**: won 6/6, lost 0/6
- **sd_evalall_int_skew2**: won 6/6, lost 0/6
- **sd_evalall_nat_skew2**: won 6/6, lost 0/6
- **sd_jumptable_int_skew2**: won 6/6, lost 0/6
- **sd_jumptable_nat_skew2**: won 6/6, lost 0/6
- **sd_profiled_int_skew2**: won 3/6, lost 3/6
- **sd_profiled_nat_skew2**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sd_bintree_int_skew2 | 3.7ns | 39601.9ns | 0.0% |  |
| sd_bintree_nat_skew2 | 3.6ns | 7751.0ns | 0.0% |  |
| sd_chain_int_skew2 | 3.8ns | 45947.4ns | 0.0% |  |
| sd_chain_nat_skew2 | 3.0ns | 8113.2ns | 0.0% |  |
| sd_chain_rev_int_skew2 | 3.7ns | 39574.0ns | 0.0% |  |
| sd_chain_rev_nat_skew2 | 3.2ns | 8972.9ns | 0.0% |  |
| sd_evalall_int_skew2 | 4.7ns | 24684.3ns | 0.0% |  |
| sd_evalall_nat_skew2 | 5.4ns | 25855.1ns | 0.0% |  |
| sd_jumptable_int_skew2 | 2.6ns | 8470.3ns | 0.0% |  |
| sd_jumptable_nat_skew2 | 3.3ns | 8362.9ns | 0.0% |  |
| sd_profiled_int_skew2 | 4.1ns | 39944.3ns | 0.0% |  |
| sd_profiled_nat_skew2 | 2.6ns | 8656.1ns | 0.0% |  |

## Distribution (algo ns)

```
sd_bintree_int_skew2 (n=6, range 35567.1-41405.9 ns)
  35567.1 |####################
  35859.0 |
  36151.0 |
  36442.9 |
  36734.8 |
  37026.8 |
  37318.7 |
  37610.7 |
  37902.6 |
  38194.5 |
  38486.5 |
  38778.4 |
  39070.4 |
  39362.3 |########################################
  39654.2 |
  39946.2 |####################
  40238.1 |
  40530.0 |
  40822.0 |
  41113.9 |####################
  (0 below, 1 above range)

sd_bintree_nat_skew2 (n=6, range 7046.7-8691.5 ns)
   7046.7 |########################################
   7128.9 |
   7211.2 |########################################
   7293.4 |########################################
   7375.6 |
   7457.9 |
   7540.1 |########################################
   7622.4 |
   7704.6 |########################################
   7786.8 |
   7869.1 |
   7951.3 |
   8033.6 |
   8115.8 |
   8198.0 |
   8280.3 |
   8362.5 |
   8444.7 |
   8527.0 |
   8609.2 |
  (0 below, 1 above range)

sd_chain_int_skew2 (n=6, range 41690.0-49363.8 ns)
  41690.0 |########################################
  42073.7 |
  42457.4 |
  42841.1 |
  43224.8 |########################################
  43608.4 |########################################
  43992.1 |
  44375.8 |
  44759.5 |
  45143.2 |
  45526.9 |
  45910.6 |
  46294.2 |
  46677.9 |
  47061.6 |
  47445.3 |
  47829.0 |########################################
  48212.7 |########################################
  48596.4 |
  48980.1 |
  (0 below, 1 above range)

sd_chain_nat_skew2 (n=6, range 7678.3-8692.7 ns)
   7678.3 |########################################
   7729.0 |
   7779.7 |####################
   7830.5 |
   7881.2 |
   7931.9 |
   7982.6 |
   8033.3 |
   8084.1 |####################
   8134.8 |
   8185.5 |
   8236.2 |
   8286.9 |
   8337.7 |
   8388.4 |
   8439.1 |
   8489.8 |
   8540.5 |####################
   8591.3 |
   8642.0 |
  (0 below, 1 above range)

sd_chain_rev_int_skew2 (n=6, range 36740.0-43352.7 ns)
  36740.0 |########################################
  37070.6 |
  37401.3 |####################
  37731.9 |
  38062.5 |
  38393.2 |
  38723.8 |
  39054.4 |
  39385.1 |####################
  39715.7 |
  40046.3 |
  40377.0 |
  40707.6 |
  41038.3 |
  41368.9 |
  41699.5 |
  42030.2 |
  42360.8 |
  42691.4 |####################
  43022.1 |
  (0 below, 1 above range)

sd_chain_rev_nat_skew2 (n=6, range 8564.2-9194.5 ns)
   8564.2 |########################################
   8595.7 |
   8627.2 |
   8658.8 |
   8690.3 |
   8721.8 |
   8753.3 |
   8784.8 |
   8816.3 |########################################
   8847.9 |
   8879.4 |
   8910.9 |
   8942.4 |
   8973.9 |########################################
   9005.4 |
   9037.0 |########################################
   9068.5 |########################################
   9100.0 |
   9131.5 |
   9163.0 |
  (0 below, 1 above range)

sd_evalall_int_skew2 (n=6, range 22206.7-26037.1 ns)
  22206.7 |########################################
  22398.2 |
  22589.7 |
  22781.3 |
  22972.8 |
  23164.3 |
  23355.8 |
  23547.3 |
  23738.9 |
  23930.4 |########################################
  24121.9 |
  24313.4 |########################################
  24504.9 |
  24696.5 |
  24888.0 |
  25079.5 |
  25271.0 |########################################
  25462.5 |
  25654.1 |
  25845.6 |########################################
  (0 below, 1 above range)

sd_evalall_nat_skew2 (n=6, range 22042.5-30512.3 ns)
  22042.5 |####################
  22466.0 |####################
  22889.5 |
  23313.0 |
  23736.5 |
  24160.0 |
  24583.4 |########################################
  25006.9 |
  25430.4 |
  25853.9 |####################
  26277.4 |
  26700.9 |
  27124.4 |
  27547.9 |
  27971.4 |
  28394.8 |
  28818.3 |
  29241.8 |
  29665.3 |
  30088.8 |
  (0 below, 1 above range)

sd_jumptable_int_skew2 (n=6, range 7450.0-9545.4 ns)
   7450.0 |########################################
   7554.8 |########################################
   7659.5 |
   7764.3 |
   7869.1 |
   7973.9 |
   8078.6 |########################################
   8183.4 |
   8288.2 |
   8392.9 |########################################
   8497.7 |
   8602.5 |
   8707.2 |
   8812.0 |
   8916.8 |
   9021.5 |
   9126.3 |
   9231.1 |
   9335.9 |########################################
   9440.6 |
  (0 below, 1 above range)

sd_jumptable_nat_skew2 (n=6, range 7565.4-8875.5 ns)
   7565.4 |########################################
   7630.9 |
   7696.4 |
   7761.9 |
   7827.4 |########################################
   7892.9 |
   7958.4 |
   8023.9 |
   8089.4 |
   8154.9 |
   8220.4 |
   8285.9 |
   8351.4 |
   8416.9 |########################################
   8482.4 |
   8547.9 |########################################
   8613.4 |
   8678.9 |
   8744.4 |########################################
   8809.9 |
  (0 below, 1 above range)

sd_profiled_int_skew2 (n=6, range 38139.6-40959.0 ns)
  38139.6 |########################################
  38280.6 |
  38421.5 |
  38562.5 |########################################
  38703.5 |
  38844.4 |
  38985.4 |
  39126.4 |
  39267.4 |
  39408.3 |
  39549.3 |
  39690.3 |
  39831.2 |
  39972.2 |
  40113.2 |
  40254.2 |########################################
  40395.1 |
  40536.1 |########################################
  40677.1 |########################################
  40818.0 |
  (0 below, 1 above range)

sd_profiled_nat_skew2 (n=6, range 7890.0-9389.2 ns)
   7890.0 |########################################
   7965.0 |
   8039.9 |########################################
   8114.9 |
   8189.8 |
   8264.8 |
   8339.8 |
   8414.7 |
   8489.7 |########################################
   8564.6 |
   8639.6 |
   8714.6 |########################################
   8789.5 |
   8864.5 |
   8939.4 |
   9014.4 |
   9089.4 |
   9164.3 |
   9239.3 |########################################
   9314.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **sd_chain_rev_int_skew2**: autocorrelation=0.53 (measurement drift or warm-up artifact)

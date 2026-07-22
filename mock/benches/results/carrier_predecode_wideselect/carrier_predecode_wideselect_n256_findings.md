# Predecoded dispatch shape, wideselect profile (carrier)

6 variants, 6 samples per variant.
Baseline: **carrier_pre_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_pre_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_pre_wideselect_null beats baseline by 32% (significant)

carrier_pre_wideselect_null is -2.90 us (32%) faster than baseline carrier_pre_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_pre_wideselect_null** at 6295.8 ns median (-31.3% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.88x (fastest 6295.8 ns, slowest 11854.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 9206ns | 9168ns | 9068ns | 9144ns | 9368ns | -21.13% |
| carrier_pre_wideselect_fntable | 14071ns | 14348ns | 12519ns | 14332ns | 14455ns | +20.54% |
| carrier_pre_wideselect_null | 8838ns | 8836ns | 8691ns | 8817ns | 8943ns | -24.29% |
| carrier_pre_wideselect_regcache | 12870ns | 12834ns | 12745ns | 12811ns | 13022ns | +10.26% |
| carrier_pre_wideselect_switch | 11673ns | 11666ns | 11607ns | 11656ns | 11732ns | base |
| carrier_pre_wideselect_threaded | 10706ns | 10691ns | 10640ns | 10674ns | 10787ns | -8.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_pre_wideselect_direct | 6670ns | 6586ns | 6718ns | -27.27% | 0.038 |
| carrier_pre_wideselect_fntable | 11614ns | 10343ns | 11896ns | +26.63% | 0.022 |
| carrier_pre_wideselect_null | 6283ns | 6211ns | 6330ns | -31.49% | 0.041 |
| carrier_pre_wideselect_regcache | 10294ns | 10252ns | 10348ns | +12.24% | 0.025 |
| carrier_pre_wideselect_switch | 9171ns | 9125ns | 9221ns | base | 0.028 |
| carrier_pre_wideselect_threaded | 8157ns | 8002ns | 8278ns | -11.06% | 0.031 |

## Performance model

- Peak throughput: **0.041 Gops/s** (carrier_pre_wideselect_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_pre_wideselect_direct | 0.038 | 93.1% |
| carrier_pre_wideselect_fntable | 0.022 | 52.4% |
| carrier_pre_wideselect_null | 0.041 | 98.7% |
| carrier_pre_wideselect_regcache | 0.025 | 60.4% |
| carrier_pre_wideselect_switch | 0.028 | 67.8% |
| carrier_pre_wideselect_threaded | 0.031 | 76.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_pre_wideselect_direct | 9206ns | 9206ns | -21.13% |
| carrier_pre_wideselect_fntable | 14071ns | 14071ns | +20.54% |
| carrier_pre_wideselect_null | 8838ns | 8838ns | -24.29% |
| carrier_pre_wideselect_regcache | 12870ns | 12870ns | +10.26% |
| carrier_pre_wideselect_switch | 11673ns | 11673ns | base |
| carrier_pre_wideselect_threaded | 10706ns | 10706ns | -8.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_pre_wideselect_switch | 9165ns | base | --- | [9128, 9221] | --- | --- | --- | --- |
| carrier_pre_wideselect_direct | 6673ns | -2475.7ns (-27.0%) | [-2572, -2456]ns | [6620, 6718] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_fntable | 11855ns | +2674.8ns (+29.2%) | [+1928, +2726]ns | [11092, 11896] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_null | 6296ns | -2898.0ns (-31.6%) | [-2941, -2826]ns | [6224, 6330] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_regcache | 10279ns | +1122.1ns (+12.2%) | [+1068, +1178]ns | [10255, 10348] | YES | 0.0313 | 0.0313 | 0 |
| carrier_pre_wideselect_threaded | 8175ns | -997.1ns (-10.9%) | [-1148, -899]ns | [8017, 8278] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_pre_wideselect_switch | carrier_pre_wideselect_direct | carrier_pre_wideselect_fntable | carrier_pre_wideselect_null | carrier_pre_wideselect_regcache | carrier_pre_wideselect_threaded |
|---|---|---|---|---|---|---|
| 1 | 9147ns | -27.1% | +13.1% | -31.8% | +13.4% | -12.2% |
| 2 | 9132ns | -26.8% | +29.9% | -30.8% | +12.4% | -10.6% |
| 3 | 9125ns | -27.1% | +29.8% | -31.1% | +12.4% | -9.1% |
| 4 | 9247ns | -28.8% | +29.0% | -31.4% | +11.7% | -10.7% |
| 5 | 9196ns | -27.0% | +28.9% | -31.4% | +11.5% | -10.9% |
| 6 | 9182ns | -26.8% | +29.1% | -32.4% | +12.1% | -12.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_pre_wideselect_direct | -0.035 | ok |
| carrier_pre_wideselect_fntable | -0.031 | ok |
| carrier_pre_wideselect_null | -0.146 | ok |
| carrier_pre_wideselect_regcache | -0.354 | moderate- |
| carrier_pre_wideselect_switch | 0.133 | ok |
| carrier_pre_wideselect_threaded | 0.177 | ok |

**Consistency summary:**

- **carrier_pre_wideselect_direct**: won 6/6, lost 0/6
- **carrier_pre_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_pre_wideselect_null**: won 6/6, lost 0/6
- **carrier_pre_wideselect_regcache**: won 0/6, lost 6/6
- **carrier_pre_wideselect_threaded**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_pre_wideselect_direct | 88091.9ns | 6670.3ns | 1320.7% | HIGH |
| carrier_pre_wideselect_fntable | 93936.6ns | 11614.2ns | 808.8% | HIGH |
| carrier_pre_wideselect_null | 87994.8ns | 6283.2ns | 1400.5% | HIGH |
| carrier_pre_wideselect_regcache | 91079.4ns | 10294.2ns | 884.8% | HIGH |
| carrier_pre_wideselect_switch | 90808.5ns | 9171.5ns | 990.1% | HIGH |
| carrier_pre_wideselect_threaded | 89840.6ns | 8156.9ns | 1101.4% | HIGH |

## Distribution (algo ns)

```
carrier_pre_wideselect_direct (n=6, range 6585.8-6718.1 ns)
   6585.8 |########################################
   6592.4 |
   6599.0 |
   6605.6 |
   6612.3 |
   6618.9 |
   6625.5 |
   6632.1 |
   6638.7 |
   6645.3 |
   6652.0 |########################################
   6658.6 |########################################
   6665.2 |
   6671.8 |
   6678.4 |########################################
   6685.0 |
   6691.6 |
   6698.3 |
   6704.9 |
   6711.5 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_fntable (n=6, range 10343.3-11895.9 ns)
  10343.3 |##########
  10420.9 |
  10498.6 |
  10576.2 |
  10653.8 |
  10731.4 |
  10809.1 |
  10886.7 |
  10964.3 |
  11041.9 |
  11119.6 |
  11197.2 |
  11274.8 |
  11352.5 |
  11430.1 |
  11507.7 |
  11585.3 |
  11663.0 |
  11740.6 |
  11818.2 |########################################
  (0 below, 1 above range)

carrier_pre_wideselect_null (n=6, range 6211.2-6330.2 ns)
   6211.2 |########################################
   6217.1 |
   6223.1 |
   6229.1 |
   6235.0 |########################################
   6240.9 |
   6246.9 |
   6252.9 |
   6258.8 |
   6264.8 |
   6270.7 |
   6276.7 |
   6282.6 |########################################
   6288.6 |
   6294.5 |
   6300.5 |########################################
   6306.4 |
   6312.4 |
   6318.3 |########################################
   6324.3 |
  (0 below, 1 above range)

carrier_pre_wideselect_regcache (n=6, range 10252.1-10348.1 ns)
  10252.1 |########################################
  10256.9 |########################################
  10261.7 |########################################
  10266.5 |
  10271.3 |
  10276.1 |
  10280.9 |
  10285.7 |
  10290.5 |########################################
  10295.3 |
  10300.1 |
  10304.9 |
  10309.7 |
  10314.5 |
  10319.3 |
  10324.1 |########################################
  10328.9 |
  10333.7 |
  10338.5 |
  10343.3 |
  (0 below, 1 above range)

carrier_pre_wideselect_switch (n=6, range 9124.6-9221.5 ns)
   9124.6 |########################################
   9129.4 |########################################
   9134.3 |
   9139.1 |
   9144.0 |########################################
   9148.8 |
   9153.7 |
   9158.5 |
   9163.3 |
   9168.2 |
   9173.0 |
   9177.9 |########################################
   9182.7 |
   9187.6 |
   9192.4 |########################################
   9197.2 |
   9202.1 |
   9206.9 |
   9211.8 |
   9216.6 |
  (0 below, 1 above range)

carrier_pre_wideselect_threaded (n=6, range 8002.5-8278.1 ns)
   8002.5 |########################################
   8016.3 |
   8030.1 |########################################
   8043.8 |
   8057.6 |
   8071.4 |
   8085.2 |
   8099.0 |
   8112.7 |
   8126.5 |
   8140.3 |
   8154.1 |########################################
   8167.9 |
   8181.6 |########################################
   8195.4 |
   8209.2 |
   8223.0 |
   8236.8 |
   8250.5 |########################################
   8264.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_pre_wideselect_direct**: bridge=1321.4% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_fntable**: bridge=792.1% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_null**: bridge=1398.1% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_regcache**: bridge=886.2% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_switch**: bridge=990.9% of algo (FFI overhead may distort results)
- **carrier_pre_wideselect_threaded**: bridge=1097.8% of algo (FFI overhead may distort results)

# Dispatch shape over the wire form, scatter profile (carrier)

8 variants, 6 samples per variant.
Baseline: **carrier_disp_scatter_switch**

## Highlights

Baseline for all deltas below: **carrier_disp_scatter_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_scatter_nullfloor dominates: 44% faster than the next best (carrier_disp_scatter_ifchainasc)

carrier_disp_scatter_nullfloor (29.19 us) leads carrier_disp_scatter_ifchainasc (42.11 us) by 44%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_scatter_nullfloor beats baseline by 32% (significant)

carrier_disp_scatter_nullfloor is -13.49 us (32%) faster than baseline carrier_disp_scatter_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_disp_scatter_ifchainlin is an outlier: 3.1x slower than the field

carrier_disp_scatter_ifchainlin (90.25 us) is 3.1x the fastest (29.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} vs {carrier_disp_scatter_ifchainlin} (62% apart)

The field splits into a fast tier {carrier_disp_scatter_nullfloor, carrier_disp_scatter_ifchainasc, carrier_disp_scatter_switch, carrier_disp_scatter_ifchain, carrier_disp_scatter_threaded, carrier_disp_scatter_bittree, carrier_disp_scatter_fntable} and a slow tier {carrier_disp_scatter_ifchainlin} with a 62% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.1x the fastest

Fastest carrier_disp_scatter_nullfloor (29.19 us) to slowest carrier_disp_scatter_ifchainlin (90.25 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_disp_scatter_nullfloor** at 29189.6 ns median (-30.9% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 3.09x (fastest 29189.6 ns, slowest 90254.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 53151ns | 52616ns | 51129ns | 52275ns | 55476ns | +18.27% |
| carrier_disp_scatter_fntable | 58160ns | 58164ns | 55439ns | 57452ns | 60584ns | +29.42% |
| carrier_disp_scatter_ifchain | 45661ns | 45346ns | 44430ns | 45190ns | 46984ns | +1.60% |
| carrier_disp_scatter_ifchainasc | 45064ns | 44381ns | 43425ns | 44094ns | 47339ns | +0.28% |
| carrier_disp_scatter_ifchainlin | 93839ns | 92663ns | 91396ns | 92371ns | 97264ns | +108.81% |
| carrier_disp_scatter_nullfloor | 31616ns | 31432ns | 30592ns | 31251ns | 32675ns | -29.65% |
| carrier_disp_scatter_switch | 44940ns | 44665ns | 43055ns | 44201ns | 46992ns | base |
| carrier_disp_scatter_threaded | 48880ns | 48471ns | 47097ns | 48336ns | 50587ns | +8.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_scatter_bittree | 50777ns | 48943ns | 52942ns | +19.36% | 0.020 |
| carrier_disp_scatter_fntable | 55785ns | 53164ns | 58196ns | +31.13% | 0.018 |
| carrier_disp_scatter_ifchain | 43272ns | 42235ns | 44436ns | +1.72% | 0.024 |
| carrier_disp_scatter_ifchainasc | 42689ns | 41086ns | 44855ns | +0.35% | 0.024 |
| carrier_disp_scatter_ifchainlin | 91393ns | 89067ns | 94725ns | +114.83% | 0.011 |
| carrier_disp_scatter_nullfloor | 29333ns | 28442ns | 30245ns | -31.05% | 0.035 |
| carrier_disp_scatter_switch | 42542ns | 40726ns | 44530ns | base | 0.024 |
| carrier_disp_scatter_threaded | 46593ns | 44978ns | 48236ns | +9.52% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 471150 | 1862798 | 0.253 | 1.02× |
| carrier_disp_scatter_fntable | 513232 | 2445391 | 0.210 | 1.11× |
| carrier_disp_scatter_ifchain | 450306 | 2041604 | 0.221 | 0.98× |
| carrier_disp_scatter_ifchainasc | 480591 | 2181645 | 0.220 | 1.04× |
| carrier_disp_scatter_ifchainlin | 567063 | 3338389 | 0.170 | 1.23× |
| carrier_disp_scatter_nullfloor | 385593 | 2168808 | 0.178 | 0.84× |
| carrier_disp_scatter_switch | 461625 | 2061919 | 0.224 | 1.00× |
| carrier_disp_scatter_threaded | 434407 | 2449559 | 0.177 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_disp_scatter_nullfloor; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_scatter_bittree | 0.020 | 56.6% |
| carrier_disp_scatter_fntable | 0.018 | 51.0% |
| carrier_disp_scatter_ifchain | 0.024 | 66.2% |
| carrier_disp_scatter_ifchainasc | 0.024 | 67.5% |
| carrier_disp_scatter_ifchainlin | 0.011 | 31.5% |
| carrier_disp_scatter_nullfloor | 0.035 | 97.4% |
| carrier_disp_scatter_switch | 0.024 | 67.3% |
| carrier_disp_scatter_threaded | 0.022 | 61.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_scatter_bittree | 53151ns | 53151ns | +18.27% |
| carrier_disp_scatter_fntable | 58160ns | 58160ns | +29.42% |
| carrier_disp_scatter_ifchain | 45661ns | 45661ns | +1.60% |
| carrier_disp_scatter_ifchainasc | 45064ns | 45064ns | +0.28% |
| carrier_disp_scatter_ifchainlin | 93839ns | 93839ns | +108.81% |
| carrier_disp_scatter_nullfloor | 31616ns | 31616ns | -29.65% |
| carrier_disp_scatter_switch | 44940ns | 44940ns | base |
| carrier_disp_scatter_threaded | 48880ns | 48880ns | +8.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_scatter_switch | 42234ns | base | --- | [40861, 44530] | --- | --- | --- | --- |
| carrier_disp_scatter_bittree | 50245ns | +8566.1ns (+20.3%) | [+6297, +9842]ns | [49144, 52942] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_fntable | 55803ns | +12185.9ns (+28.9%) | [+10396, +17149]ns | [53357, 58196] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_ifchain | 42975ns | no significant difference | [-1072, +1884]ns | [42404, 44436] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainasc | 42107ns | no significant difference | [-2014, +2279]ns | [41104, 44855] | no | 0.6875 | 0.6875 | 0 |
| carrier_disp_scatter_ifchainlin | 90255ns | +48978.8ns (+116.0%) | [+45617, +51959]ns | [89200, 94725] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_nullfloor | 29190ns | -13494.8ns (-32.0%) | [-14922, -11208]ns | [28566, 30245] | YES | 0.0438 | 0.0313 | 0 |
| carrier_disp_scatter_threaded | 46247ns | +4719.6ns (+11.2%) | [+901, +6533]ns | [45296, 48236] | YES | 0.0438 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_scatter_switch | carrier_disp_scatter_bittree | carrier_disp_scatter_fntable | carrier_disp_scatter_ifchain | carrier_disp_scatter_ifchainasc | carrier_disp_scatter_ifchainlin | carrier_disp_scatter_nullfloor | carrier_disp_scatter_threaded |
|---|---|---|---|---|---|---|---|---|
| 1 | 43126ns | +23.3% | +23.3% | +5.1% | -4.7% | +108.3% | -33.5% | +12.0% |
| 2 | 41343ns | +20.6% | +36.2% | +3.0% | +8.7% | +119.4% | -25.7% | +10.3% |
| 3 | 44538ns | +18.4% | +24.1% | -2.6% | +0.5% | +117.5% | -33.9% | +1.0% |
| 4 | 44522ns | +9.9% | +26.5% | -2.2% | -4.5% | +100.1% | -33.2% | +3.1% |
| 5 | 40996ns | +23.5% | +30.6% | +3.8% | +0.3% | +125.8% | -30.6% | +13.7% |
| 6 | 40726ns | +21.2% | +47.5% | +3.7% | +2.3% | +119.3% | -29.0% | +18.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_scatter_bittree | -0.448 | moderate- |
| carrier_disp_scatter_fntable | -0.411 | moderate- |
| carrier_disp_scatter_ifchain | -0.143 | ok |
| carrier_disp_scatter_ifchainasc | 0.166 | ok |
| carrier_disp_scatter_ifchainlin | -0.467 | moderate- |
| carrier_disp_scatter_nullfloor | -0.199 | ok |
| carrier_disp_scatter_switch | 0.039 | ok |
| carrier_disp_scatter_threaded | 0.114 | ok |

**Consistency summary:**

- **carrier_disp_scatter_bittree**: won 0/6, lost 6/6
- **carrier_disp_scatter_fntable**: won 0/6, lost 6/6
- **carrier_disp_scatter_ifchain**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainasc**: won 2/6, lost 4/6
- **carrier_disp_scatter_ifchainlin**: won 0/6, lost 6/6
- **carrier_disp_scatter_nullfloor**: won 6/6, lost 0/6
- **carrier_disp_scatter_threaded**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_scatter_bittree | 101680.1ns | 50776.8ns | 200.2% | HIGH |
| carrier_disp_scatter_fntable | 112033.0ns | 55785.2ns | 200.8% | HIGH |
| carrier_disp_scatter_ifchain | 102767.3ns | 43271.6ns | 237.5% | HIGH |
| carrier_disp_scatter_ifchainasc | 111166.2ns | 42688.8ns | 260.4% | HIGH |
| carrier_disp_scatter_ifchainlin | 91489.7ns | 91393.2ns | 100.1% | HIGH |
| carrier_disp_scatter_nullfloor | 93949.4ns | 29333.4ns | 320.3% | HIGH |
| carrier_disp_scatter_switch | 106301.2ns | 42541.7ns | 249.9% | HIGH |
| carrier_disp_scatter_threaded | 93333.1ns | 46593.0ns | 200.3% | HIGH |

## Distribution (algo ns)

```
carrier_disp_scatter_bittree (n=6, range 48943.3-52941.7 ns)
  48943.3 |########################################
  49143.2 |
  49343.1 |########################################
  49543.1 |
  49743.0 |########################################
  49942.9 |
  50142.8 |
  50342.7 |
  50542.6 |########################################
  50742.6 |
  50942.5 |
  51142.4 |
  51342.3 |
  51542.2 |
  51742.1 |
  51942.1 |
  52142.0 |
  52341.9 |
  52541.8 |########################################
  52741.7 |
  (0 below, 1 above range)

carrier_disp_scatter_fntable (n=6, range 53164.2-58195.7 ns)
  53164.2 |####################
  53415.8 |####################
  53667.3 |
  53918.9 |
  54170.5 |
  54422.1 |
  54673.6 |
  54925.2 |
  55176.8 |####################
  55428.4 |
  55679.9 |
  55931.5 |
  56183.1 |########################################
  56434.6 |
  56686.2 |
  56937.8 |
  57189.4 |
  57440.9 |
  57692.5 |
  57944.1 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchain (n=6, range 42235.0-44436.2 ns)
  42235.0 |####################
  42345.1 |
  42455.1 |
  42565.2 |########################################
  42675.2 |
  42785.3 |
  42895.4 |
  43005.4 |
  43115.5 |
  43225.6 |
  43335.6 |####################
  43445.7 |
  43555.8 |####################
  43665.8 |
  43775.9 |
  43885.9 |
  43996.0 |
  44106.1 |
  44216.1 |
  44326.2 |
  (0 below, 1 above range)

carrier_disp_scatter_ifchainasc (n=6, range 41086.2-44854.8 ns)
  41086.2 |########################################
  41274.6 |
  41463.1 |
  41651.5 |####################
  41839.9 |
  42028.3 |
  42216.8 |
  42405.2 |####################
  42593.6 |
  42782.0 |
  42970.5 |
  43158.9 |
  43347.3 |
  43535.8 |
  43724.2 |
  43912.6 |
  44101.0 |
  44289.5 |
  44477.9 |
  44666.3 |####################
  (0 below, 1 above range)

carrier_disp_scatter_ifchainlin (n=6, range 89067.1-94725.4 ns)
  89067.1 |########################################
  89350.0 |
  89632.9 |####################
  89915.8 |
  90198.8 |
  90481.7 |####################
  90764.6 |
  91047.5 |
  91330.4 |
  91613.3 |
  91896.2 |
  92179.2 |
  92462.1 |####################
  92745.0 |
  93027.9 |
  93310.8 |
  93593.7 |
  93876.7 |
  94159.6 |
  94442.5 |
  (0 below, 1 above range)

carrier_disp_scatter_nullfloor (n=6, range 28441.7-30244.6 ns)
  28441.7 |########################################
  28531.8 |
  28622.0 |########################################
  28712.1 |
  28802.3 |
  28892.4 |########################################
  28982.6 |
  29072.7 |
  29162.9 |
  29253.0 |
  29343.2 |
  29433.3 |########################################
  29523.4 |
  29613.6 |
  29703.7 |########################################
  29793.9 |
  29884.0 |
  29974.2 |
  30064.3 |
  30154.5 |
  (0 below, 1 above range)

carrier_disp_scatter_switch (n=6, range 40726.2-44529.6 ns)
  40726.2 |########################################
  40916.4 |########################################
  41106.5 |
  41296.7 |########################################
  41486.9 |
  41677.0 |
  41867.2 |
  42057.4 |
  42247.6 |
  42437.7 |
  42627.9 |
  42818.1 |
  43008.2 |########################################
  43198.4 |
  43388.6 |
  43578.8 |
  43768.9 |
  43959.1 |
  44149.3 |
  44339.4 |########################################
  (0 below, 1 above range)

carrier_disp_scatter_threaded (n=6, range 44978.3-48235.6 ns)
  44978.3 |########################################
  45141.2 |
  45304.0 |
  45466.9 |########################################
  45629.8 |
  45792.6 |########################################
  45955.5 |
  46118.4 |
  46281.2 |
  46444.1 |
  46607.0 |########################################
  46769.8 |
  46932.7 |
  47095.5 |
  47258.4 |
  47421.3 |
  47584.1 |
  47747.0 |
  47909.9 |
  48072.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_disp_scatter_bittree**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_fntable**: bridge=202.3% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchain**: bridge=232.5% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainasc**: bridge=283.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_ifchainlin**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_nullfloor**: bridge=311.6% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_switch**: bridge=251.9% of algo (FFI overhead may distort results)
- **carrier_disp_scatter_threaded**: bridge=200.3% of algo (FFI overhead may distort results)

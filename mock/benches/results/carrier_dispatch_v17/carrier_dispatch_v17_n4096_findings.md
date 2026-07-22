# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_fntable_v17 dominates: 18% faster than the next best (carrier_disp_switch_v17)

carrier_disp_fntable_v17 (458.23 us) leads carrier_disp_switch_v17 (540.88 us) by 18%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_switch_v17 shows alternating (throttle bounce) (autocorr -0.53)

carrier_disp_switch_v17's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_disp_fntable_v17** at 458227.5 ns median (-15.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.36x (fastest 458227.5 ns, slowest 622012.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 460512ns | 460716ns | 429503ns | 451211ns | 489967ns | -15.29% |
| carrier_disp_switch_v17 | 543653ns | 543593ns | 500628ns | 534594ns | 578754ns | base |
| carrier_disp_threaded_v17 | 611230ns | 624631ns | 557974ns | 604807ns | 647493ns | +12.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 458005ns | 427306ns | 487138ns | -15.35% | 0.009 |
| carrier_disp_switch_v17 | 541065ns | 498268ns | 575980ns | base | 0.008 |
| carrier_disp_threaded_v17 | 608543ns | 555738ns | 644338ns | +12.47% | 0.007 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_disp_fntable_v17; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.009 | 93.3% |
| carrier_disp_switch_v17 | 0.008 | 79.0% |
| carrier_disp_threaded_v17 | 0.007 | 68.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 460512ns | 460512ns | -15.29% |
| carrier_disp_switch_v17 | 543653ns | 543653ns | base |
| carrier_disp_threaded_v17 | 611230ns | 611230ns | +12.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 540882ns | base | --- | [506332, 575980] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 458228ns | -94796.7ns (-17.5%) | [-110731, -43653]ns | [428649, 487138] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_threaded_v17 | 622012ns | +75576.8ns (+14.0%) | [+15762, +111096]ns | [559279, 644338] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 | carrier_disp_threaded_v17 |
|---|---|---|---|
| 1 | 599873ns | -17.1% | +3.8% |
| 2 | 498268ns | -4.3% | +13.0% |
| 3 | 546932ns | -21.4% | +1.6% |
| 4 | 514396ns | -16.9% | +23.6% |
| 5 | 552088ns | -18.9% | +18.2% |
| 6 | 534833ns | -12.3% | +16.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | 0.332 | moderate+ |
| carrier_disp_switch_v17 | -0.531 | HIGH- (thermal bounce) |
| carrier_disp_threaded_v17 | 0.265 | moderate+ |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 6/6, lost 0/6
- **carrier_disp_threaded_v17**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 442.1ns | 458004.9ns | 0.1% |  |
| carrier_disp_switch_v17 | 449.9ns | 541064.8ns | 0.1% |  |
| carrier_disp_threaded_v17 | 463.2ns | 608543.2ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 427305.8-487138.3 ns)
  427305.8 |########################################
  430297.4 |
  433289.1 |
  436280.7 |
  439272.3 |
  442263.9 |
  445255.6 |####################
  448247.2 |
  451238.8 |
  454230.4 |
  457222.1 |
  460213.7 |
  463205.3 |
  466197.0 |####################
  469188.6 |
  472180.2 |
  475171.8 |####################
  478163.5 |
  481155.1 |
  484146.7 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 498267.5-575980.4 ns)
  498267.5 |########################################
  502153.1 |
  506038.8 |
  509924.4 |
  513810.1 |########################################
  517695.7 |
  521581.4 |
  525467.0 |
  529352.7 |
  533238.3 |########################################
  537123.9 |
  541009.6 |
  544895.2 |########################################
  548780.9 |########################################
  552666.5 |
  556552.2 |
  560437.8 |
  564323.5 |
  568209.1 |
  572094.8 |
  (0 below, 1 above range)

carrier_disp_threaded_v17 (n=6, range 555737.5-644337.9 ns)
  555737.5 |########################################
  560167.5 |########################################
  564597.5 |
  569027.6 |
  573457.6 |
  577887.6 |
  582317.6 |
  586747.6 |
  591177.7 |
  595607.7 |
  600037.7 |
  604467.7 |
  608897.7 |
  613327.8 |
  617757.8 |########################################
  622187.8 |########################################
  626617.8 |
  631047.8 |
  635477.9 |########################################
  639907.9 |
  (0 below, 1 above range)

```

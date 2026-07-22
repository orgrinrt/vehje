# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_disp_switch_v4) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_disp_switch_v4 has the worst median (476.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_disp_fntable_v4 at 400.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: carrier_disp_fntable_v4** at 400219.8 ns median (-16.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.19x (fastest 400219.8 ns, slowest 476711.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 401897ns | 402905ns | 384364ns | 400625ns | 412572ns | -16.23% |
| carrier_disp_switch_v4 | 479781ns | 479406ns | 467894ns | 478025ns | 488359ns | base |
| carrier_disp_threaded_v4 | 454865ns | 437652ns | 411212ns | 432178ns | 510723ns | -5.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 399239ns | 381556ns | 409841ns | -16.31% | 0.010 |
| carrier_disp_switch_v4 | 477057ns | 465621ns | 485506ns | base | 0.009 |
| carrier_disp_threaded_v4 | 452225ns | 408867ns | 507963ns | -5.21% | 0.009 |

## Performance model

- Peak throughput: **0.011 Gops/s** (carrier_disp_fntable_v4; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.010 | 95.3% |
| carrier_disp_switch_v4 | 0.009 | 80.0% |
| carrier_disp_threaded_v4 | 0.009 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 401897ns | 401897ns | -16.23% |
| carrier_disp_switch_v4 | 479781ns | 479781ns | base |
| carrier_disp_threaded_v4 | 454865ns | 454865ns | -5.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 476712ns | base | --- | [468954, 485506] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 400220ns | -74485.4ns (-15.6%) | [-95899, -63071]ns | [387656, 409841] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_disp_threaded_v4 | 434941ns | no significant difference | [-55184, +31251]ns | [413770, 507963] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 | carrier_disp_threaded_v4 |
|---|---|---|---|
| 1 | 483790ns | -17.2% | -10.6% |
| 2 | 479888ns | -17.9% | +4.3% |
| 3 | 472288ns | -12.8% | -11.4% |
| 4 | 465621ns | -14.2% | -12.2% |
| 5 | 487223ns | -21.7% | -10.2% |
| 6 | 473536ns | -13.8% | +8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | -0.405 | moderate- |
| carrier_disp_switch_v4 | -0.285 | moderate- |
| carrier_disp_threaded_v4 | -0.143 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 6/6, lost 0/6
- **carrier_disp_threaded_v4**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 431.5ns | 399239.0ns | 0.1% |  |
| carrier_disp_switch_v4 | 438.0ns | 477057.5ns | 0.1% |  |
| carrier_disp_threaded_v4 | 456.2ns | 452224.9ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 381555.8-409841.2 ns)
  381555.8 |########################################
  382970.1 |
  384384.3 |
  385798.6 |
  387212.9 |
  388627.2 |
  390041.4 |
  391455.7 |
  392870.0 |########################################
  394284.3 |
  395698.5 |
  397112.8 |
  398527.1 |########################################
  399941.3 |########################################
  401355.6 |
  402769.9 |
  404184.2 |
  405598.4 |
  407012.7 |########################################
  408427.0 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 465620.8-485506.2 ns)
  465620.8 |########################################
  466615.1 |
  467609.3 |
  468603.6 |
  469597.9 |
  470592.2 |
  471586.4 |########################################
  472580.7 |########################################
  473575.0 |
  474569.3 |
  475563.5 |
  476557.8 |
  477552.1 |
  478546.3 |
  479540.6 |########################################
  480534.9 |
  481529.2 |
  482523.4 |
  483517.7 |########################################
  484512.0 |
  (0 below, 1 above range)

carrier_disp_threaded_v4 (n=6, range 408867.1-507962.9 ns)
  408867.1 |########################################
  413821.9 |########################################
  418776.7 |
  423731.5 |
  428686.3 |########################################
  433641.0 |########################################
  438595.8 |
  443550.6 |
  448505.4 |
  453460.2 |
  458415.0 |
  463369.8 |
  468324.6 |
  473279.4 |
  478234.2 |
  483189.0 |
  488143.7 |
  493098.5 |
  498053.3 |########################################
  503008.1 |
  (0 below, 1 above range)

```

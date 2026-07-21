# Dispatch shape: switch vs fn-pointer table, op vocab v17 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v17**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v17**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_disp_switch_v17) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_disp_switch_v17 has the worst median (553.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_disp_fntable_v17 at 458.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_disp_fntable_v17 dominates: 21% faster than the next best (carrier_disp_switch_v17)

carrier_disp_fntable_v17 (458.86 us) leads carrier_disp_switch_v17 (553.60 us) by 21%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_disp_fntable_v17** at 458856.4 ns median (-17.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.21x (fastest 458856.4 ns, slowest 553602.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 462901ns | 461414ns | 442680ns | 460103ns | 477208ns | -15.83% |
| carrier_disp_switch_v17 | 549943ns | 556094ns | 514482ns | 548873ns | 569280ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v17 | 460121ns | 440206ns | 474195ns | -15.92% | 0.009 |
| carrier_disp_switch_v17 | 547218ns | 511587ns | 566553ns | base | 0.007 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_disp_fntable_v17; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v17 | 0.009 | 95.9% |
| carrier_disp_switch_v17 | 0.007 | 79.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v17 | 462901ns | 462901ns | -15.83% |
| carrier_disp_switch_v17 | 549943ns | 549943ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v17 | 553602ns | base | --- | [521498, 566553] | --- | --- | --- | --- |
| carrier_disp_fntable_v17 | 458856ns | -91465.5ns (-16.5%) | [-104591, -65234]ns | [447312, 474195] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v17 | carrier_disp_fntable_v17 |
|---|---|---|
| 1 | 546970ns | -15.9% |
| 2 | 511587ns | -14.0% |
| 3 | 560235ns | -18.3% |
| 4 | 560882ns | -19.0% |
| 5 | 531408ns | -11.1% |
| 6 | 572224ns | -16.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v17 | 0.227 | moderate+ |
| carrier_disp_switch_v17 | -0.355 | moderate- |

**Consistency summary:**

- **carrier_disp_fntable_v17**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v17 | 455.1ns | 460121.0ns | 0.1% |  |
| carrier_disp_switch_v17 | 469.6ns | 547217.7ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v17 (n=6, range 440206.2-474194.8 ns)
  440206.2 |########################################
  441905.6 |
  443605.1 |
  445304.5 |
  447003.9 |
  448703.4 |
  450402.8 |
  452102.2 |
  453801.6 |########################################
  455501.1 |
  457200.5 |########################################
  458899.9 |########################################
  460599.4 |
  462298.8 |
  463998.2 |
  465697.7 |
  467397.1 |
  469096.5 |
  470795.9 |########################################
  472495.4 |
  (0 below, 1 above range)

carrier_disp_switch_v17 (n=6, range 511587.1-566552.9 ns)
  511587.1 |####################
  514335.4 |
  517083.7 |
  519832.0 |
  522580.3 |
  525328.6 |
  528076.9 |
  530825.1 |####################
  533573.4 |
  536321.7 |
  539070.0 |
  541818.3 |
  544566.6 |####################
  547314.9 |
  550063.2 |
  552811.5 |
  555559.8 |
  558308.1 |########################################
  561056.4 |
  563804.7 |
  (0 below, 1 above range)

```

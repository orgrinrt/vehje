# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_disp_switch_v4) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_disp_switch_v4 has the worst median (474.93 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_disp_fntable_v4 at 408.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_disp_fntable_v4 dominates: 16% faster than the next best (carrier_disp_switch_v4)

carrier_disp_fntable_v4 (408.07 us) leads carrier_disp_switch_v4 (474.93 us) by 16%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_disp_fntable_v4** at 408072.7 ns median (-14.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.16x (fastest 408072.7 ns, slowest 474926.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 408510ns | 410668ns | 395922ns | 405976ns | 418604ns | -14.53% |
| carrier_disp_switch_v4 | 477934ns | 477647ns | 468643ns | 475319ns | 486501ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 405990ns | 393639ns | 415929ns | -14.57% | 0.010 |
| carrier_disp_switch_v4 | 475238ns | 466325ns | 483529ns | base | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_disp_fntable_v4; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.010 | 96.5% |
| carrier_disp_switch_v4 | 0.009 | 82.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 408510ns | 408510ns | -14.53% |
| carrier_disp_switch_v4 | 477934ns | 477934ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 474927ns | base | --- | [467259, 483529] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 408073ns | -68100.0ns (-14.3%) | [-80950, -58695]ns | [393968, 415929] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 |
|---|---|---|
| 1 | 483512ns | -18.6% |
| 2 | 466325ns | -15.4% |
| 3 | 483546ns | -14.7% |
| 4 | 480958ns | -12.8% |
| 5 | 468194ns | -12.0% |
| 6 | 468895ns | -13.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.410 | moderate+ |
| carrier_disp_switch_v4 | -0.282 | moderate- |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 491.2ns | 405989.9ns | 0.1% |  |
| carrier_disp_switch_v4 | 442.7ns | 475238.3ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 393639.2-415928.7 ns)
  393639.2 |########################################
  394753.7 |
  395868.2 |
  396982.6 |
  398097.1 |
  399211.6 |
  400326.0 |
  401440.5 |
  402555.0 |
  403669.5 |####################
  404784.0 |
  405898.4 |
  407012.9 |
  408127.4 |
  409241.9 |
  410356.3 |
  411470.8 |########################################
  412585.3 |
  413699.8 |
  414814.2 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 466324.6-483529.2 ns)
  466324.6 |####################
  467184.8 |
  468045.1 |########################################
  468905.3 |
  469765.5 |
  470625.7 |
  471486.0 |
  472346.2 |
  473206.4 |
  474066.6 |
  474926.9 |
  475787.1 |
  476647.3 |
  477507.6 |
  478367.8 |
  479228.0 |
  480088.2 |
  480948.5 |####################
  481808.7 |
  482668.9 |####################
  (0 below, 1 above range)

```

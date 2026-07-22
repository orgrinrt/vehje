# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (545.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flat at 430.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flat dominates: 27% faster than the next best (carrier_predec_wire)

carrier_predec_flat (430.91 us) leads carrier_predec_wire (545.28 us) by 27%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_predec_flat** at 430908.3 ns median (-21.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.27x (fastest 430908.3 ns, slowest 545282.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 437046ns | 433165ns | 421294ns | 429930ns | 455595ns | -19.37% |
| carrier_predec_wire | 542046ns | 548467ns | 501218ns | 540541ns | 564718ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 434733ns | 418975ns | 453233ns | -19.37% | 0.009 |
| carrier_predec_wire | 539195ns | 498855ns | 561941ns | base | 0.008 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_predec_flat; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.010 | 97.2% |
| carrier_predec_wire | 0.008 | 76.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 437046ns | 437046ns | -19.37% |
| carrier_predec_wire | 542046ns | 542046ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 545282ns | base | --- | [510361, 561941] | --- | --- | --- | --- |
| carrier_predec_flat | 430908ns | -104266.0ns (-19.1%) | [-128598, -80522]ns | [420056, 453233] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat |
|---|---|---|
| 1 | 498855ns | -16.0% |
| 2 | 544751ns | -20.6% |
| 3 | 549620ns | -14.8% |
| 4 | 521867ns | -19.3% |
| 5 | 574261ns | -25.3% |
| 6 | 545813ns | -19.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | -0.269 | moderate- |
| carrier_predec_wire | -0.216 | moderate- |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 10031.0ns | 434732.6ns | 2.3% |  |
| carrier_predec_wire | 453.2ns | 539194.6ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 418975.0-453233.1 ns)
  418975.0 |########################################
  420687.9 |########################################
  422400.8 |
  424113.7 |
  425826.6 |
  427539.5 |########################################
  429252.4 |
  430965.3 |
  432678.2 |########################################
  434391.1 |
  436104.0 |
  437817.0 |########################################
  439529.9 |
  441242.8 |
  442955.7 |
  444668.6 |
  446381.5 |
  448094.4 |
  449807.3 |
  451520.2 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 498854.6-561940.8 ns)
  498854.6 |####################
  502008.9 |
  505163.2 |
  508317.5 |
  511471.8 |
  514626.2 |
  517780.5 |
  520934.8 |####################
  524089.1 |
  527243.4 |
  530397.7 |
  533552.0 |
  536706.3 |
  539860.6 |
  543014.9 |########################################
  546169.2 |
  549323.6 |####################
  552477.9 |
  555632.2 |
  558786.5 |
  (0 below, 1 above range)

```

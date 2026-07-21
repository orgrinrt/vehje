# Reachability fixpoint: whole-column vs real semi-naive (chain32)

2 variants, 6 samples per variant.
Baseline: **r_chain32_whole**

## Highlights

Baseline for all deltas below: **r_chain32_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain32_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain32_whole has the worst median (25.84 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain32_semi at 21.63 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain32_semi dominates: 19% faster than the next best (r_chain32_whole)

r_chain32_semi (21.63 ms) leads r_chain32_whole (25.84 ms) by 19%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: r_chain32_semi** at 21632195.6 ns median (-16.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.19x (fastest 21632195.6 ns, slowest 25840203.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain32_semi | 22066471ns | 21636137ns | 21607140ns | 21630443ns | 22950179ns | -14.81% |
| r_chain32_whole | 25902657ns | 25844447ns | 25783640ns | 25839827ns | 26056409ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain32_semi | 22062383ns | 21603304ns | 22945620ns | -14.81% | 0.001 |
| r_chain32_whole | 25898373ns | 25779302ns | 26052088ns | base | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain32_semi; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain32_semi | 0.001 | 99.9% |
| r_chain32_whole | 0.001 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain32_semi | 22066471ns | 22066471ns | -14.81% |
| r_chain32_whole | 25902657ns | 25902657ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain32_whole | 25840204ns | base | --- | [25802827, 26052088] | --- | --- | --- | --- |
| r_chain32_semi | 21632196ns | -4178122.5ns (-16.2%) | [-4234374, -3095476]ns | [21609332, 22945620] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain32_whole | r_chain32_semi |
|---|---|---|
| 1 | 25826353ns | -16.3% |
| 2 | 25854178ns | -16.4% |
| 3 | 25779302ns | -16.1% |
| 4 | 26249999ns | -8.1% |
| 5 | 25832192ns | -15.8% |
| 6 | 25848216ns | -16.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain32_semi | -0.190 | ok |
| r_chain32_whole | -0.351 | moderate- |

**Consistency summary:**

- **r_chain32_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain32_semi | 8545.5ns | 22062382.6ns | 0.0% |  |
| r_chain32_whole | 7152.9ns | 25898373.2ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain32_semi (n=6, range 21603303.7-22945619.8 ns)
  21603303.7 |########################################
  21670419.5 |
  21737535.3 |##########
  21804651.1 |
  21871766.9 |
  21938882.7 |
  22005998.5 |
  22073114.3 |
  22140230.1 |
  22207345.9 |
  22274461.8 |
  22341577.6 |
  22408693.4 |
  22475809.2 |
  22542925.0 |
  22610040.8 |
  22677156.6 |
  22744272.4 |
  22811388.2 |
  22878504.0 |
  (0 below, 1 above range)

r_chain32_whole (n=6, range 25779301.7-26052088.4 ns)
  25779301.7 |####################
  25792941.0 |
  25806580.4 |
  25820219.7 |########################################
  25833859.0 |
  25847498.4 |########################################
  25861137.7 |
  25874777.0 |
  25888416.4 |
  25902055.7 |
  25915695.0 |
  25929334.4 |
  25942973.7 |
  25956613.0 |
  25970252.4 |
  25983891.7 |
  25997531.0 |
  26011170.4 |
  26024809.7 |
  26038449.0 |
  (0 below, 1 above range)

```

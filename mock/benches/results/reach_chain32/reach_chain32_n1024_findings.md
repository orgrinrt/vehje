# Reachability fixpoint: whole-column vs real semi-naive (chain32)

2 variants, 6 samples per variant.
Baseline: **r_chain32_whole**

## Highlights

Baseline for all deltas below: **r_chain32_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain32_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain32_whole has the worst median (1.61 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain32_semi at 694.88 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain32_semi dominates: 131% faster than the next best (r_chain32_whole)

r_chain32_semi (694.88 us) leads r_chain32_whole (1.61 ms) by 131%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain32_semi beats baseline by 57% (significant)

r_chain32_semi is -914.33 us (57%) faster than baseline r_chain32_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_chain32_semi** at 694884.8 ns median (-56.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.31x (fastest 694884.8 ns, slowest 1607387.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain32_semi | 701048ns | 697306ns | 671604ns | 693012ns | 727825ns | -56.47% |
| r_chain32_whole | 1610490ns | 1610653ns | 1591492ns | 1608469ns | 1623021ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain32_semi | 698710ns | 669321ns | 725495ns | -56.53% | 0.001 |
| r_chain32_whole | 1607273ns | 1587999ns | 1620061ns | base | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (r_chain32_semi; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain32_semi | 0.001 | 96.3% |
| r_chain32_whole | 0.001 | 41.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain32_semi | 701048ns | 701048ns | -56.47% |
| r_chain32_whole | 1610490ns | 1610490ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain32_whole | 1607388ns | base | --- | [1594372, 1620061] | --- | --- | --- | --- |
| r_chain32_semi | 694885ns | -914326.7ns (-56.9%) | [-931637, -879726]ns | [675751, 725495] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain32_whole | r_chain32_semi |
|---|---|---|
| 1 | 1617678ns | -56.5% |
| 2 | 1614001ns | -57.7% |
| 3 | 1622443ns | -55.8% |
| 4 | 1587999ns | -53.8% |
| 5 | 1600745ns | -57.1% |
| 6 | 1600775ns | -58.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain32_semi | 0.065 | ok |
| r_chain32_whole | 0.057 | ok |

**Consistency summary:**

- **r_chain32_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain32_semi | 966.9ns | 698710.1ns | 0.1% |  |
| r_chain32_whole | 362.2ns | 1607273.4ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain32_semi (n=6, range 669320.8-725494.6 ns)
  669320.8 |########################################
  672129.5 |
  674938.2 |
  677746.9 |
  680555.6 |########################################
  683364.2 |
  686172.9 |########################################
  688981.6 |
  691790.3 |
  694599.0 |
  697407.7 |
  700216.4 |
  703025.1 |########################################
  705833.7 |
  708642.4 |
  711451.1 |
  714259.8 |########################################
  717068.5 |
  719877.2 |
  722685.9 |
  (0 below, 1 above range)

r_chain32_whole (n=6, range 1587998.8-1620060.6 ns)
  1587998.8 |####################
  1589601.9 |
  1591205.0 |
  1592808.1 |
  1594411.2 |
  1596014.2 |
  1597617.3 |
  1599220.4 |########################################
  1600823.5 |
  1602426.6 |
  1604029.7 |
  1605632.8 |
  1607235.9 |
  1608839.0 |
  1610442.1 |
  1612045.2 |
  1613648.2 |####################
  1615251.3 |
  1616854.4 |####################
  1618457.5 |
  (0 below, 1 above range)

```

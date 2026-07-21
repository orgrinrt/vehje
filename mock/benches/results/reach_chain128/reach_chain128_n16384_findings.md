# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (105.39 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 53.37 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 97% faster than the next best (r_chain128_whole)

r_chain128_semi (53.37 ms) leads r_chain128_whole (105.39 ms) by 97%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 49% (significant)

r_chain128_semi is -52.09 ms (49%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_chain128_semi** at 53372787.1 ns median (-49.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.97x (fastest 53372787.1 ns, slowest 105393997.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 53491219ns | 53377236ns | 53142807ns | 53343972ns | 53886295ns | -49.71% |
| r_chain128_whole | 106370278ns | 105398745ns | 105195048ns | 105331196ns | 108516517ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 53486808ns | 53138636ns | 53881819ns | -49.71% | 0.000 |
| r_chain128_whole | 106365504ns | 105190359ns | 108511628ns | base | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.000 | 99.6% |
| r_chain128_whole | 0.000 | 50.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 53491219ns | 53491219ns | -49.71% |
| r_chain128_whole | 106370278ns | 106370278ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 105393997ns | base | --- | [105190885, 108511628] | --- | --- | --- | --- |
| r_chain128_semi | 53372787ns | -52087287.1ns (-49.4%) | [-55025594, -51523204]ns | [53205818, 53881819] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 105191412ns | -49.5% |
| 2 | 106687025ns | -49.9% |
| 3 | 105420000ns | -49.4% |
| 4 | 105367995ns | -49.4% |
| 5 | 105190359ns | -48.5% |
| 6 | 110336231ns | -51.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.033 | ok |
| r_chain128_whole | -0.158 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 10007.1ns | 53486808.2ns | 0.0% |  |
| r_chain128_whole | 9289.4ns | 106365503.5ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 53138635.8-53881819.3 ns)
  53138635.8 |########################################
  53175795.0 |
  53212954.2 |
  53250113.3 |########################################
  53287272.5 |
  53324431.7 |########################################
  53361590.9 |
  53398750.0 |########################################
  53435909.2 |
  53473068.4 |
  53510227.6 |
  53547386.8 |########################################
  53584545.9 |
  53621705.1 |
  53658864.3 |
  53696023.5 |
  53733182.6 |
  53770341.8 |
  53807501.0 |
  53844660.2 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 105190358.7-108511628.1 ns)
  105190358.7 |########################################
  105356422.2 |########################################
  105522485.6 |
  105688549.1 |
  105854612.6 |
  106020676.0 |
  106186739.5 |
  106352803.0 |
  106518866.5 |
  106684929.9 |####################
  106850993.4 |
  107017056.9 |
  107183120.3 |
  107349183.8 |
  107515247.3 |
  107681310.8 |
  107847374.2 |
  108013437.7 |
  108179501.2 |
  108345564.6 |
  (0 below, 1 above range)

```

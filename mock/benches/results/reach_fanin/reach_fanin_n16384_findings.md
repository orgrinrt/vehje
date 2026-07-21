# Reachability fixpoint: whole-column vs real semi-naive (fanin)

2 variants, 6 samples per variant.
Baseline: **r_fanin_whole**

## Highlights

Baseline for all deltas below: **r_fanin_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_fanin_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_fanin_whole has the worst median (286.47 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_fanin_semi at 150.37 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_fanin_semi dominates: 91% faster than the next best (r_fanin_whole)

r_fanin_semi (150.37 us) leads r_fanin_whole (286.47 us) by 91%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_fanin_semi beats baseline by 47% (significant)

r_fanin_semi is -135.42 us (47%) faster than baseline r_fanin_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_fanin_semi** at 150366.6 ns median (-47.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.91x (fastest 150366.6 ns, slowest 286472.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_fanin_semi | 153499ns | 152591ns | 149926ns | 152352ns | 157006ns | -47.43% |
| r_fanin_whole | 291999ns | 288756ns | 285223ns | 287926ns | 301498ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_fanin_semi | 151231ns | 147764ns | 154614ns | -47.79% | 0.108 |
| r_fanin_whole | 289662ns | 283027ns | 298992ns | base | 0.057 |

## Performance model

- Peak throughput: **0.111 Gops/s** (r_fanin_semi; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_fanin_semi | 0.109 | 98.3% |
| r_fanin_whole | 0.057 | 51.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_fanin_semi | 153499ns | 153499ns | -47.43% |
| r_fanin_whole | 291999ns | 291999ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_fanin_whole | 286472ns | base | --- | [283523, 298992] | --- | --- | --- | --- |
| r_fanin_semi | 150367ns | -135424.0ns (-47.3%) | [-146496, -133375]ns | [148711, 154614] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_fanin_whole | r_fanin_semi |
|---|---|---|
| 1 | 310477ns | -49.5% |
| 2 | 284019ns | -47.0% |
| 3 | 287108ns | -48.5% |
| 4 | 285836ns | -46.7% |
| 5 | 287507ns | -47.8% |
| 6 | 283027ns | -47.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_fanin_semi | -0.099 | ok |
| r_fanin_whole | -0.132 | ok |

**Consistency summary:**

- **r_fanin_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_fanin_semi | 2772.8ns | 151230.6ns | 1.8% |  |
| r_fanin_whole | 2312.0ns | 289662.2ns | 0.8% |  |

## Distribution (algo ns)

```
r_fanin_semi (n=6, range 147763.8-154614.4 ns)
  147763.8 |########################################
  148106.3 |
  148448.9 |
  148791.4 |
  149133.9 |
  149476.4 |########################################
  149819.0 |########################################
  150161.5 |
  150504.0 |########################################
  150846.6 |
  151189.1 |
  151531.6 |
  151874.2 |
  152216.7 |########################################
  152559.2 |
  152901.8 |
  153244.3 |
  153586.8 |
  153929.3 |
  154271.9 |
  (0 below, 1 above range)

r_fanin_whole (n=6, range 283026.7-298991.9 ns)
  283026.7 |####################
  283825.0 |####################
  284623.2 |
  285421.5 |####################
  286219.7 |
  287018.0 |########################################
  287816.3 |
  288614.5 |
  289412.8 |
  290211.0 |
  291009.3 |
  291807.6 |
  292605.8 |
  293404.1 |
  294202.3 |
  295000.6 |
  295798.9 |
  296597.1 |
  297395.4 |
  298193.6 |
  (0 below, 1 above range)

```

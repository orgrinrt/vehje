# Reachability fixpoint: whole-column vs real semi-naive (chain32)

2 variants, 6 samples per variant.
Baseline: **r_chain32_whole**

## Highlights

Baseline for all deltas below: **r_chain32_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain32_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain32_whole has the worst median (422.70 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain32_semi at 137.69 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain32_semi dominates: 207% faster than the next best (r_chain32_whole)

r_chain32_semi (137.69 us) leads r_chain32_whole (422.70 us) by 207%, a clear separation rather than a photo finish. CV 4.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain32_semi beats baseline by 66% (significant)

r_chain32_semi is -278.58 us (66%) faster than baseline r_chain32_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.1x the fastest

Fastest r_chain32_semi (137.69 us) to slowest r_chain32_whole (422.70 us): 3.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain32_semi** at 137689.5 ns median (-67.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.07x (fastest 137689.5 ns, slowest 422702.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain32_semi | 142674ns | 140056ns | 136822ns | 139145ns | 150894ns | -66.74% |
| r_chain32_whole | 428959ns | 425339ns | 409248ns | 420729ns | 451159ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain32_semi | 140247ns | 134495ns | 148269ns | -67.09% | 0.002 |
| r_chain32_whole | 426171ns | 406337ns | 448103ns | base | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (r_chain32_semi; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain32_semi | 0.002 | 97.7% |
| r_chain32_whole | 0.001 | 31.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain32_semi | 142674ns | 142674ns | -66.74% |
| r_chain32_whole | 428959ns | 428959ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain32_whole | 422702ns | base | --- | [407707, 448103] | --- | --- | --- | --- |
| r_chain32_semi | 137690ns | -278575.1ns (-65.9%) | [-310414, -268784]ns | [134781, 148269] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain32_whole | r_chain32_semi |
|---|---|---|
| 1 | 406337ns | -66.9% |
| 2 | 445495ns | -69.1% |
| 3 | 425030ns | -63.8% |
| 4 | 450712ns | -69.5% |
| 5 | 409078ns | -65.1% |
| 6 | 420375ns | -67.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain32_semi | -0.281 | moderate- |
| r_chain32_whole | -0.444 | moderate- |

**Consistency summary:**

- **r_chain32_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain32_semi | 323.5ns | 140246.6ns | 0.2% |  |
| r_chain32_whole | 246.4ns | 426170.9ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain32_semi (n=6, range 134495.4-148269.3 ns)
  134495.4 |########################################
  135184.1 |
  135872.8 |
  136561.5 |
  137250.2 |########################################
  137938.9 |
  138627.6 |
  139316.3 |
  140005.0 |
  140693.7 |
  141382.4 |
  142071.1 |####################
  142759.8 |
  143448.5 |
  144137.2 |
  144825.9 |
  145514.6 |
  146203.3 |
  146892.0 |
  147580.7 |
  (0 below, 1 above range)

r_chain32_whole (n=6, range 406336.7-448103.2 ns)
  406336.7 |########################################
  408425.0 |########################################
  410513.3 |
  412601.7 |
  414690.0 |
  416778.3 |
  418866.6 |########################################
  420955.0 |
  423043.3 |########################################
  425131.6 |
  427219.9 |
  429308.2 |
  431396.6 |
  433484.9 |
  435573.2 |
  437661.5 |
  439749.9 |
  441838.2 |
  443926.5 |########################################
  446014.8 |
  (0 below, 1 above range)

```

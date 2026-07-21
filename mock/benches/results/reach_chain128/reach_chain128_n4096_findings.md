# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (26.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 7.72 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 240% faster than the next best (r_chain128_whole)

r_chain128_semi (7.72 ms) leads r_chain128_whole (26.21 ms) by 240%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 71% (significant)

r_chain128_semi is -18.50 ms (71%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.4x the fastest

Fastest r_chain128_semi (7.72 ms) to slowest r_chain128_whole (26.21 ms): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain128_semi** at 7717519.2 ns median (-70.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.40x (fastest 7717519.2 ns, slowest 26205662.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 7720326ns | 7720224ns | 7695654ns | 7717236ns | 7737297ns | -70.87% |
| r_chain128_whole | 26503418ns | 26209691ns | 26146915ns | 26195402ns | 27143694ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 7717546ns | 7692805ns | 7734517ns | -70.88% | 0.001 |
| r_chain128_whole | 26499376ns | 26143027ns | 27139534ns | base | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.001 | 99.7% |
| r_chain128_whole | 0.000 | 29.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 7720326ns | 7720326ns | -70.87% |
| r_chain128_whole | 26503418ns | 26503418ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 26205663ns | base | --- | [26152932, 27139534] | --- | --- | --- | --- |
| r_chain128_semi | 7717519ns | -18498263.1ns (-70.6%) | [-19418538, -18428690]ns | [7700601, 7734517] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 26143027ns | -70.5% |
| 2 | 28029484ns | -72.5% |
| 3 | 26183792ns | -70.6% |
| 4 | 26249583ns | -70.5% |
| 5 | 26162837ns | -70.5% |
| 6 | 26227534ns | -70.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.217 | moderate- |
| r_chain128_whole | -0.275 | moderate- |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 2105.6ns | 7717545.5ns | 0.0% |  |
| r_chain128_whole | 1829.6ns | 26499376.0ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 7692805.0-7734516.8 ns)
  7692805.0 |########################################
  7694890.6 |
  7696976.2 |
  7699061.8 |
  7701147.4 |
  7703233.0 |
  7705318.6 |
  7707404.1 |########################################
  7709489.7 |########################################
  7711575.3 |
  7713660.9 |
  7715746.5 |
  7717832.1 |
  7719917.7 |
  7722003.3 |########################################
  7724088.9 |########################################
  7726174.5 |
  7728260.1 |
  7730345.7 |
  7732431.3 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 26143026.7-27139533.5 ns)
  26143026.7 |########################################
  26192852.0 |#############
  26242677.4 |#############
  26292502.7 |
  26342328.1 |
  26392153.4 |
  26441978.8 |
  26491804.1 |
  26541629.4 |
  26591454.8 |
  26641280.1 |
  26691105.5 |
  26740930.8 |
  26790756.2 |
  26840581.5 |
  26890406.8 |
  26940232.2 |
  26990057.5 |
  27039882.9 |
  27089708.2 |
  (0 below, 1 above range)

```

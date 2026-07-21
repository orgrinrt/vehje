# Reachability fixpoint: whole-column vs real semi-naive (chain32)

2 variants, 6 samples per variant.
Baseline: **r_chain32_whole**

## Highlights

Baseline for all deltas below: **r_chain32_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain32_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain32_whole has the worst median (6.42 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain32_semi at 5.25 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain32_semi dominates: 22% faster than the next best (r_chain32_whole)

r_chain32_semi (5.25 ms) leads r_chain32_whole (6.42 ms) by 22%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: r_chain32_semi** at 5251671.7 ns median (-18.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.22x (fastest 5251671.7 ns, slowest 6424202.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain32_semi | 5336755ns | 5254662ns | 5225170ns | 5246381ns | 5528108ns | -16.87% |
| r_chain32_whole | 6419720ns | 6427429ns | 6381847ns | 6417927ns | 6441345ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain32_semi | 5333722ns | 5222189ns | 5524864ns | -16.87% | 0.001 |
| r_chain32_whole | 6416408ns | 6378911ns | 6437910ns | base | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain32_semi; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain32_semi | 0.001 | 99.4% |
| r_chain32_whole | 0.001 | 81.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain32_semi | 5336755ns | 5336755ns | -16.87% |
| r_chain32_whole | 6419720ns | 6419720ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain32_whole | 6424203ns | base | --- | [6387110, 6437910] | --- | --- | --- | --- |
| r_chain32_semi | 5251672ns | -1172644.0ns (-18.3%) | [-1213168, -862247]ns | [5224630, 5524864] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain32_whole | r_chain32_semi |
|---|---|---|
| 1 | 6378911ns | -9.4% |
| 2 | 6428130ns | -18.7% |
| 3 | 6428355ns | -18.4% |
| 4 | 6447465ns | -19.0% |
| 5 | 6420276ns | -18.1% |
| 6 | 6395310ns | -17.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain32_semi | -0.063 | ok |
| r_chain32_whole | 0.035 | ok |

**Consistency summary:**

- **r_chain32_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain32_semi | 1782.6ns | 5333721.7ns | 0.0% |  |
| r_chain32_whole | 979.7ns | 6416407.8ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain32_semi (n=6, range 5222189.2-5524863.8 ns)
  5222189.2 |########################################
  5237322.9 |####################
  5252456.7 |####################
  5267590.4 |####################
  5282724.1 |
  5297857.8 |
  5312991.6 |
  5328125.3 |
  5343259.0 |
  5358392.7 |
  5373526.5 |
  5388660.2 |
  5403793.9 |
  5418927.7 |
  5434061.4 |
  5449195.1 |
  5464328.8 |
  5479462.6 |
  5494596.3 |
  5509730.0 |
  (0 below, 1 above range)

r_chain32_whole (n=6, range 6378910.8-6437910.2 ns)
  6378910.8 |####################
  6381860.8 |
  6384810.7 |
  6387760.7 |
  6390710.7 |
  6393660.7 |####################
  6396610.6 |
  6399560.6 |
  6402510.6 |
  6405460.5 |
  6408410.5 |
  6411360.5 |
  6414310.4 |
  6417260.4 |
  6420210.4 |####################
  6423160.3 |
  6426110.3 |########################################
  6429060.3 |
  6432010.3 |
  6434960.2 |
  (0 below, 1 above range)

```

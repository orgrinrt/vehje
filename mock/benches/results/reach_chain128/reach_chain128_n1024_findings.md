# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (6.51 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 1.06 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 514% faster than the next best (r_chain128_whole)

r_chain128_semi (1.06 ms) leads r_chain128_whole (6.51 ms) by 514%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 84% (significant)

r_chain128_semi is -5.45 ms (84%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 6.1x the fastest

Fastest r_chain128_semi (1.06 ms) to slowest r_chain128_whole (6.51 ms): 6.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain128_semi** at 1059123.9 ns median (-83.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 6.14x (fastest 1059123.9 ns, slowest 6506225.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 1060526ns | 1061535ns | 1040946ns | 1059641ns | 1071644ns | -83.78% |
| r_chain128_whole | 6537647ns | 6509953ns | 6487327ns | 6504353ns | 6612749ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 1058151ns | 1038686ns | 1069197ns | -83.81% | 0.001 |
| r_chain128_whole | 6534028ns | 6483531ns | 6609176ns | base | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.001 | 98.1% |
| r_chain128_whole | 0.000 | 16.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 1060526ns | 1060526ns | -83.78% |
| r_chain128_whole | 6537647ns | 6537647ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 6506226ns | base | --- | [6486683, 6609176] | --- | --- | --- | --- |
| r_chain128_semi | 1059124ns | -5447101.7ns (-83.7%) | [-5552487, -5428043]ns | [1046133, 1069197] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 6693192ns | -83.9% |
| 2 | 6525161ns | -84.1% |
| 3 | 6506752ns | -83.8% |
| 4 | 6489835ns | -83.6% |
| 5 | 6483531ns | -83.7% |
| 6 | 6505700ns | -83.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.478 | moderate- |
| r_chain128_whole | 0.118 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 753.6ns | 1058151.2ns | 0.1% |  |
| r_chain128_whole | 784.3ns | 6534028.5ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 1038685.8-1069197.0 ns)
  1038685.8 |########################################
  1040211.4 |
  1041736.9 |
  1043262.5 |
  1044788.1 |
  1046313.6 |
  1047839.2 |
  1049364.7 |
  1050890.3 |
  1052415.9 |########################################
  1053941.4 |
  1055467.0 |########################################
  1056992.5 |
  1058518.1 |
  1060043.7 |########################################
  1061569.2 |
  1063094.8 |########################################
  1064620.4 |
  1066145.9 |
  1067671.5 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 6483531.2-6609176.5 ns)
  6483531.2 |####################
  6489813.5 |####################
  6496095.7 |
  6502378.0 |########################################
  6508660.2 |
  6514942.5 |
  6521224.8 |####################
  6527507.0 |
  6533789.3 |
  6540071.6 |
  6546353.8 |
  6552636.1 |
  6558918.4 |
  6565200.6 |
  6571482.9 |
  6577765.1 |
  6584047.4 |
  6590329.7 |
  6596611.9 |
  6602894.2 |
  (0 below, 1 above range)

```

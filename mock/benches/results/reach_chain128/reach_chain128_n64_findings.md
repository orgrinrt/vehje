# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (50.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 20.61 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 146% faster than the next best (r_chain128_whole)

r_chain128_semi (20.61 us) leads r_chain128_whole (50.71 us) by 146%, a clear separation rather than a photo finish. CV 4.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 60% (significant)

r_chain128_semi is -30.18 us (60%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: r_chain128_semi** at 20605.0 ns median (-59.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.46x (fastest 20605.0 ns, slowest 50707.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 23326ns | 23600ns | 21313ns | 23104ns | 24666ns | -55.71% |
| r_chain128_whole | 52664ns | 53064ns | 49585ns | 52395ns | 54606ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 20446ns | 19128ns | 21242ns | -59.38% | 0.003 |
| r_chain128_whole | 50329ns | 47408ns | 52171ns | base | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.003 | 92.8% |
| r_chain128_whole | 0.001 | 37.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 23326ns | 23326ns | -55.71% |
| r_chain128_whole | 52664ns | 52664ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 50708ns | base | --- | [48108, 52171] | --- | --- | --- | --- |
| r_chain128_semi | 20605ns | -30181.8ns (-59.5%) | [-30936, -28530]ns | [19491, 21242] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 47408ns | -57.8% |
| 2 | 52130ns | -59.2% |
| 3 | 51705ns | -59.0% |
| 4 | 52211ns | -59.4% |
| 5 | 48808ns | -60.8% |
| 6 | 49711ns | -60.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | 0.158 | ok |
| r_chain128_whole | -0.106 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 160.0ns | 20446.0ns | 0.8% |  |
| r_chain128_whole | 61.7ns | 50328.7ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 19128.3-21242.1 ns)
  19128.3 |####################
  19234.0 |
  19339.7 |
  19445.4 |
  19551.0 |
  19656.7 |
  19762.4 |####################
  19868.1 |
  19973.8 |####################
  20079.5 |
  20185.2 |
  20290.9 |
  20396.6 |
  20502.2 |
  20607.9 |
  20713.6 |
  20819.3 |
  20925.0 |
  21030.7 |
  21136.4 |########################################
  (0 below, 1 above range)

r_chain128_whole (n=6, range 47407.9-52170.6 ns)
  47407.9 |########################################
  47646.0 |
  47884.2 |
  48122.3 |
  48360.4 |
  48598.6 |########################################
  48836.7 |
  49074.8 |
  49313.0 |
  49551.1 |########################################
  49789.2 |
  50027.4 |
  50265.5 |
  50503.7 |
  50741.8 |
  50979.9 |
  51218.1 |
  51456.2 |
  51694.3 |########################################
  51932.5 |########################################
  (0 below, 1 above range)

```

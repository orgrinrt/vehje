# Reachability fixpoint: whole-column vs real semi-naive (randomdag)

2 variants, 6 samples per variant.
Baseline: **r_randomdag_whole**

## Highlights

Baseline for all deltas below: **r_randomdag_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_randomdag_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_randomdag_whole has the worst median (31.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_randomdag_semi at 10.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_randomdag_semi dominates: 191% faster than the next best (r_randomdag_whole)

r_randomdag_semi (10.68 us) leads r_randomdag_whole (31.06 us) by 191%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_randomdag_semi beats baseline by 66% (significant)

r_randomdag_semi is -20.48 us (66%) faster than baseline r_randomdag_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_randomdag_semi shows alternating (throttle bounce) (autocorr -0.50)

r_randomdag_semi's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: r_randomdag_semi** at 10682.5 ns median (-65.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.91x (fastest 10682.5 ns, slowest 31063.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_randomdag_semi | 13091ns | 12964ns | 12805ns | 12920ns | 13492ns | -61.18% |
| r_randomdag_whole | 33720ns | 33359ns | 32135ns | 33175ns | 35329ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_randomdag_semi | 10815ns | 10567ns | 11175ns | -65.57% | 0.006 |
| r_randomdag_whole | 31410ns | 29939ns | 32898ns | base | 0.002 |

## Performance model

- Peak throughput: **0.006 Gops/s** (r_randomdag_semi; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_randomdag_semi | 0.006 | 98.9% |
| r_randomdag_whole | 0.002 | 34.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_randomdag_semi | 13091ns | 13091ns | -61.18% |
| r_randomdag_whole | 33720ns | 33720ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_randomdag_whole | 31063ns | base | --- | [30270, 32898] | --- | --- | --- | --- |
| r_randomdag_semi | 10682ns | -20475.6ns (-65.9%) | [-21918, -19392]ns | [10588, 11175] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_randomdag_whole | r_randomdag_semi |
|---|---|---|
| 1 | 30913ns | -65.7% |
| 2 | 31213ns | -66.1% |
| 3 | 33273ns | -65.9% |
| 4 | 32522ns | -67.3% |
| 5 | 29939ns | -63.2% |
| 6 | 30600ns | -64.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_randomdag_semi | -0.500 | HIGH- (thermal bounce) |
| r_randomdag_whole | 0.174 | ok |

**Consistency summary:**

- **r_randomdag_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_randomdag_semi | 147.2ns | 10815.1ns | 1.4% |  |
| r_randomdag_whole | 65.6ns | 31410.2ns | 0.2% |  |

## Distribution (algo ns)

```
r_randomdag_semi (n=6, range 10567.1-11175.0 ns)
  10567.1 |########################################
  10597.5 |########################################
  10627.9 |########################################
  10658.3 |
  10688.7 |
  10719.1 |########################################
  10749.5 |
  10779.9 |
  10810.3 |
  10840.7 |
  10871.0 |
  10901.4 |
  10931.8 |
  10962.2 |
  10992.6 |########################################
  11023.0 |
  11053.4 |
  11083.8 |
  11114.2 |
  11144.6 |
  (0 below, 1 above range)

r_randomdag_whole (n=6, range 29939.2-32897.7 ns)
  29939.2 |########################################
  30087.1 |
  30235.0 |
  30383.0 |
  30530.9 |########################################
  30678.8 |
  30826.8 |########################################
  30974.7 |
  31122.6 |########################################
  31270.5 |
  31418.4 |
  31566.4 |
  31714.3 |
  31862.2 |
  32010.1 |
  32158.1 |
  32306.0 |
  32453.9 |########################################
  32601.8 |
  32749.8 |
  (0 below, 1 above range)

```

# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (830.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 260.95 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 218% faster than the next best (r_chain128_whole)

r_chain128_semi (260.95 us) leads r_chain128_whole (830.32 us) by 218%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 68% (significant)

r_chain128_semi is -566.85 us (68%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.2x the fastest

Fastest r_chain128_semi (260.95 us) to slowest r_chain128_whole (830.32 us): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain128_semi** at 260954.8 ns median (-68.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.18x (fastest 260954.8 ns, slowest 830324.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 261990ns | 263228ns | 253024ns | 262360ns | 265919ns | -68.74% |
| r_chain128_whole | 838045ns | 832821ns | 804973ns | 827793ns | 869959ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 259730ns | 250880ns | 263608ns | -68.91% | 0.001 |
| r_chain128_whole | 835400ns | 802573ns | 867089ns | base | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.001 | 96.1% |
| r_chain128_whole | 0.000 | 30.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 261990ns | 261990ns | -68.74% |
| r_chain128_whole | 838045ns | 838045ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 830324ns | base | --- | [808786, 867089] | --- | --- | --- | --- |
| r_chain128_semi | 260955ns | -566851.5ns (-68.3%) | [-610820, -549339]ns | [254628, 263608] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 825327ns | -68.3% |
| 2 | 855710ns | -70.7% |
| 3 | 802573ns | -67.5% |
| 4 | 815000ns | -68.3% |
| 5 | 835322ns | -68.2% |
| 6 | 878469ns | -70.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.160 | ok |
| r_chain128_whole | -0.053 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 471.1ns | 259730.1ns | 0.2% |  |
| r_chain128_whole | 260.3ns | 835400.0ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 250879.6-263607.9 ns)
  250879.6 |####################
  251516.0 |
  252152.4 |
  252788.8 |
  253425.3 |
  254061.7 |
  254698.1 |
  255334.5 |
  255970.9 |
  256607.3 |
  257243.8 |
  257880.2 |####################
  258516.6 |
  259153.0 |
  259789.4 |
  260425.8 |####################
  261062.2 |########################################
  261698.7 |
  262335.1 |
  262971.5 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 802573.3-867089.2 ns)
  802573.3 |########################################
  805799.1 |
  809024.9 |
  812250.7 |########################################
  815476.5 |
  818702.3 |
  821928.1 |
  825153.9 |########################################
  828379.7 |
  831605.5 |
  834831.2 |########################################
  838057.0 |
  841282.8 |
  844508.6 |
  847734.4 |
  850960.2 |
  854186.0 |########################################
  857411.8 |
  860637.6 |
  863863.4 |
  (0 below, 1 above range)

```

# Reachability fixpoint: whole-column vs real semi-naive (randomdag)

2 variants, 6 samples per variant.
Baseline: **r_randomdag_whole**

## Highlights

Baseline for all deltas below: **r_randomdag_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_randomdag_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_randomdag_whole has the worst median (162.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_randomdag_semi at 39.82 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_randomdag_semi dominates: 308% faster than the next best (r_randomdag_whole)

r_randomdag_semi (39.82 us) leads r_randomdag_whole (162.63 us) by 308%, a clear separation rather than a photo finish. CV 18.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_randomdag_semi beats baseline by 75% (significant)

r_randomdag_semi is -121.56 us (75%) faster than baseline r_randomdag_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_randomdag_semi is fastest but the noisiest (CV 18.1%)

r_randomdag_semi wins on median (39.82 us) yet has the highest variance (CV 18.1%), while r_randomdag_whole is the steadiest (CV 4.1%, 162.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.1x the fastest

Fastest r_randomdag_semi (39.82 us) to slowest r_randomdag_whole (162.63 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_randomdag_semi** at 39818.1 ns median (-75.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.08x (fastest 39818.1 ns, slowest 162630.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_randomdag_semi | 44768ns | 42134ns | 39424ns | 41341ns | 52580ns | -73.33% |
| r_randomdag_whole | 167887ns | 165170ns | 160519ns | 164046ns | 177334ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_randomdag_semi | 42367ns | 37207ns | 49934ns | -74.38% | 0.006 |
| r_randomdag_whole | 165339ns | 158288ns | 174476ns | base | 0.002 |

## Performance model

- Peak throughput: **0.007 Gops/s** (r_randomdag_semi; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_randomdag_semi | 0.006 | 93.4% |
| r_randomdag_whole | 0.002 | 22.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_randomdag_semi | 44768ns | 44768ns | -73.33% |
| r_randomdag_whole | 167887ns | 167887ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_randomdag_whole | 162631ns | base | --- | [158912, 174476] | --- | --- | --- | --- |
| r_randomdag_semi | 39818ns | -121561.2ns (-74.7%) | [-134584, -112770]ns | [37350, 49934] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_randomdag_whole | r_randomdag_semi |
|---|---|---|
| 1 | 158288ns | -76.5% |
| 2 | 162448ns | -74.2% |
| 3 | 159536ns | -76.5% |
| 4 | 175361ns | -76.1% |
| 5 | 162813ns | -64.4% |
| 6 | 173590ns | -78.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_randomdag_semi | -0.223 | moderate- |
| r_randomdag_whole | -0.252 | moderate- |

**Consistency summary:**

- **r_randomdag_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_randomdag_semi | 286.3ns | 42367.4ns | 0.7% |  |
| r_randomdag_whole | 179.2ns | 165339.3ns | 0.1% |  |

## Distribution (algo ns)

```
r_randomdag_semi (n=6, range 37207.1-49933.8 ns)
  37207.1 |########################################
  37843.4 |
  38479.8 |
  39116.1 |
  39752.4 |
  40388.8 |
  41025.1 |
  41661.4 |##########################
  42297.8 |
  42934.1 |
  43570.4 |
  44206.8 |
  44843.1 |
  45479.4 |
  46115.8 |
  46752.1 |
  47388.4 |
  48024.8 |
  48661.1 |
  49297.4 |
  (0 below, 1 above range)

r_randomdag_whole (n=6, range 158287.5-174475.6 ns)
  158287.5 |####################
  159096.9 |####################
  159906.3 |
  160715.7 |
  161525.1 |
  162334.5 |########################################
  163143.9 |
  163953.3 |
  164762.7 |
  165572.1 |
  166381.5 |
  167191.0 |
  168000.4 |
  168809.8 |
  169619.2 |
  170428.6 |
  171238.0 |
  172047.4 |
  172856.8 |####################
  173666.2 |
  (0 below, 1 above range)

```

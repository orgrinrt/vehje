# Reachability fixpoint: whole-column vs real semi-naive (chain8)

2 variants, 6 samples per variant.
Baseline: **r_chain8_whole**

## Highlights

Baseline for all deltas below: **r_chain8_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain8_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain8_whole has the worst median (5.91 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain8_semi at 1.22 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain8_semi dominates: 386% faster than the next best (r_chain8_whole)

r_chain8_semi (1.22 ms) leads r_chain8_whole (5.91 ms) by 386%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain8_semi beats baseline by 79% (significant)

r_chain8_semi is -4.70 ms (79%) faster than baseline r_chain8_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 4.9x the fastest

Fastest r_chain8_semi (1.22 ms) to slowest r_chain8_whole (5.91 ms): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain8_semi** at 1217301.2 ns median (-79.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.86x (fastest 1217301.2 ns, slowest 5910445.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain8_semi | 1218949ns | 1219774ns | 1208014ns | 1217792ns | 1226152ns | -79.38% |
| r_chain8_whole | 5911669ns | 5914196ns | 5880441ns | 5909286ns | 5930859ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain8_semi | 1216455ns | 1205490ns | 1223608ns | -79.41% | 0.013 |
| r_chain8_whole | 5907937ns | 5876470ns | 5927252ns | base | 0.003 |

## Performance model

- Peak throughput: **0.014 Gops/s** (r_chain8_semi; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain8_semi | 0.013 | 99.0% |
| r_chain8_whole | 0.003 | 20.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain8_semi | 1218949ns | 1218949ns | -79.38% |
| r_chain8_whole | 5911669ns | 5911669ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain8_whole | 5910446ns | base | --- | [5886114, 5927252] | --- | --- | --- | --- |
| r_chain8_semi | 1217301ns | -4695130.0ns (-79.4%) | [-4711457, -4667860]ns | [1208455, 1223608] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain8_whole | r_chain8_semi |
|---|---|---|
| 1 | 5916490ns | -79.6% |
| 2 | 5876470ns | -79.3% |
| 3 | 5895758ns | -79.5% |
| 4 | 5904401ns | -79.2% |
| 5 | 5924900ns | -79.4% |
| 6 | 5929605ns | -79.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain8_semi | -0.116 | ok |
| r_chain8_whole | 0.235 | moderate+ |

**Consistency summary:**

- **r_chain8_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain8_semi | 3910.7ns | 1216454.9ns | 0.3% |  |
| r_chain8_whole | 3938.1ns | 5907937.4ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain8_semi (n=6, range 1205490.4-1223608.4 ns)
  1205490.4 |########################################
  1206396.3 |
  1207302.2 |
  1208208.1 |
  1209114.0 |
  1210019.9 |
  1210925.8 |########################################
  1211831.7 |
  1212737.6 |
  1213643.5 |
  1214549.4 |
  1215455.3 |
  1216361.2 |########################################
  1217267.1 |########################################
  1218173.0 |########################################
  1219078.9 |
  1219984.8 |
  1220890.7 |
  1221796.6 |
  1222702.5 |
  (0 below, 1 above range)

r_chain8_whole (n=6, range 5876470.4-5927252.3 ns)
  5876470.4 |########################################
  5879009.5 |
  5881548.6 |
  5884087.7 |
  5886626.8 |
  5889165.9 |
  5891705.0 |
  5894244.1 |########################################
  5896783.2 |
  5899322.3 |
  5901861.3 |
  5904400.4 |########################################
  5906939.5 |
  5909478.6 |
  5912017.7 |
  5914556.8 |########################################
  5917095.9 |
  5919635.0 |
  5922174.1 |
  5924713.2 |########################################
  (0 below, 1 above range)

```

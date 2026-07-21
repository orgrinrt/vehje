# Reachability fixpoint: whole-column vs real semi-naive (randomdag)

2 variants, 6 samples per variant.
Baseline: **r_randomdag_whole**

## Highlights

Baseline for all deltas below: **r_randomdag_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_randomdag_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_randomdag_whole has the worst median (851.37 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_randomdag_semi at 183.78 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_randomdag_semi dominates: 363% faster than the next best (r_randomdag_whole)

r_randomdag_semi (183.78 us) leads r_randomdag_whole (851.37 us) by 363%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_randomdag_semi beats baseline by 78% (significant)

r_randomdag_semi is -660.07 us (78%) faster than baseline r_randomdag_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_randomdag_semi is fastest but the noisiest (CV 10.8%)

r_randomdag_semi wins on median (183.78 us) yet has the highest variance (CV 10.8%), while r_randomdag_whole is the steadiest (CV 4.6%, 851.37 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.6x the fastest

Fastest r_randomdag_semi (183.78 us) to slowest r_randomdag_whole (851.37 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_randomdag_semi** at 183784.0 ns median (-78.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 4.63x (fastest 183784.0 ns, slowest 851370.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_randomdag_semi | 193078ns | 186102ns | 172285ns | 182127ns | 219900ns | -77.62% |
| r_randomdag_whole | 862813ns | 853860ns | 823616ns | 844218ns | 910304ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_randomdag_semi | 190713ns | 170078ns | 217326ns | -77.83% | 0.005 |
| r_randomdag_whole | 860220ns | 821371ns | 907246ns | base | 0.001 |

## Performance model

- Peak throughput: **0.006 Gops/s** (r_randomdag_semi; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_randomdag_semi | 0.006 | 92.5% |
| r_randomdag_whole | 0.001 | 20.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_randomdag_semi | 193078ns | 193078ns | -77.62% |
| r_randomdag_whole | 862813ns | 862813ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_randomdag_whole | 851370ns | base | --- | [822043, 907246] | --- | --- | --- | --- |
| r_randomdag_semi | 183784ns | -660067.3ns (-77.5%) | [-704164, -644290]ns | [171028, 217326] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_randomdag_whole | r_randomdag_semi |
|---|---|---|
| 1 | 882243ns | -79.4% |
| 2 | 932249ns | -76.0% |
| 3 | 864742ns | -75.6% |
| 4 | 821371ns | -77.4% |
| 5 | 837998ns | -79.5% |
| 6 | 822715ns | -79.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_randomdag_semi | 0.318 | moderate+ |
| r_randomdag_whole | 0.377 | moderate+ |

**Consistency summary:**

- **r_randomdag_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_randomdag_semi | 437.4ns | 190712.6ns | 0.2% |  |
| r_randomdag_whole | 371.6ns | 860219.6ns | 0.0% |  |

## Distribution (algo ns)

```
r_randomdag_semi (n=6, range 170077.9-217326.0 ns)
  170077.9 |########################################
  172440.3 |
  174802.7 |
  177165.1 |
  179527.5 |
  181889.9 |####################
  184252.3 |####################
  186614.8 |
  188977.2 |
  191339.6 |
  193702.0 |
  196064.4 |
  198426.8 |
  200789.2 |
  203151.6 |
  205514.0 |
  207876.4 |
  210238.8 |####################
  212601.2 |
  214963.6 |
  (0 below, 1 above range)

r_randomdag_whole (n=6, range 821370.8-907246.1 ns)
  821370.8 |########################################
  825664.6 |
  829958.3 |
  834252.1 |####################
  838545.9 |
  842839.6 |
  847133.4 |
  851427.1 |
  855720.9 |
  860014.7 |
  864308.4 |####################
  868602.2 |
  872896.0 |
  877189.7 |
  881483.5 |####################
  885777.2 |
  890071.0 |
  894364.8 |
  898658.5 |
  902952.3 |
  (0 below, 1 above range)

```

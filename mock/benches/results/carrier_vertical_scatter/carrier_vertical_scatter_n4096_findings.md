# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (4.34 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 843.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 58% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (843.42 us) leads carrier_vert_scatter_vert4 (1.34 ms) by 58%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 81% (significant)

carrier_vert_scatter_vert8 is -3.50 ms (81%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 5.1x slower than the field

carrier_vert_scatter_scalar (4.34 ms) is 5.1x the fastest (843.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.1x the fastest

Fastest carrier_vert_scatter_vert8 (843.42 us) to slowest carrier_vert_scatter_scalar (4.34 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 843417.1 ns median (-80.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.15x (fastest 843417.1 ns, slowest 4343521.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 4315215ns | 4347900ns | 4042520ns | 4268022ns | 4522352ns | base |
| carrier_vert_scatter_vert4 | 1334151ns | 1340352ns | 1309741ns | 1331877ns | 1349768ns | -69.08% |
| carrier_vert_scatter_vert8 | 841290ns | 847321ns | 807828ns | 846263ns | 850561ns | -80.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 4310973ns | 4038132ns | 4518065ns | base | 0.001 |
| carrier_vert_scatter_vert4 | 1330375ns | 1305736ns | 1346327ns | -69.14% | 0.003 |
| carrier_vert_scatter_vert8 | 837648ns | 804177ns | 846516ns | -80.57% | 0.005 |

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.001 | 18.5% |
| carrier_vert_scatter_vert4 | 0.003 | 60.2% |
| carrier_vert_scatter_vert8 | 0.005 | 95.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 4315215ns | 4315215ns | base |
| carrier_vert_scatter_vert4 | 1334151ns | 1334151ns | -69.08% |
| carrier_vert_scatter_vert8 | 841290ns | 841290ns | -80.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 4343522ns | base | --- | [4071331, 4518065] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 1336456ns | -3003823.3ns (-69.2%) | [-3174981, -2762989]ns | [1308342, 1346327] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 843417ns | -3497005.6ns (-80.5%) | [-3675854, -3247115]ns | [823011, 846516] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 4527925ns | -70.2% | -81.4% |
| 2 | 4508205ns | -70.4% | -81.3% |
| 3 | 4412927ns | -69.7% | -80.9% |
| 4 | 4104531ns | -68.1% | -80.4% |
| 5 | 4274117ns | -68.6% | -80.2% |
| 6 | 4038132ns | -67.7% | -79.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | 0.277 | moderate+ |
| carrier_vert_scatter_vert4 | -0.323 | moderate- |
| carrier_vert_scatter_vert8 | -0.345 | moderate- |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 4314164.1ns | 4310972.8ns | 100.1% | HIGH |
| carrier_vert_scatter_vert4 | 1330618.9ns | 1330375.0ns | 100.0% | HIGH |
| carrier_vert_scatter_vert8 | 829763.2ns | 837648.1ns | 99.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 4038131.7-4518065.0 ns)
  4038131.7 |########################################
  4062128.4 |
  4086125.0 |########################################
  4110121.7 |
  4134118.4 |
  4158115.0 |
  4182111.7 |
  4206108.4 |
  4230105.0 |
  4254101.7 |########################################
  4278098.3 |
  4302095.0 |
  4326091.7 |
  4350088.3 |
  4374085.0 |
  4398081.7 |########################################
  4422078.3 |
  4446075.0 |
  4470071.7 |
  4494068.3 |########################################
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 1305735.8-1346326.6 ns)
  1305735.8 |####################
  1307765.3 |
  1309794.9 |####################
  1311824.4 |
  1313854.0 |
  1315883.5 |
  1317913.1 |
  1319942.6 |
  1321972.1 |
  1324001.7 |
  1326031.2 |
  1328060.8 |
  1330090.3 |
  1332119.9 |
  1334149.4 |
  1336178.9 |########################################
  1338208.5 |
  1340238.0 |
  1342267.6 |####################
  1344297.1 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 804176.7-846516.2 ns)
  804176.7 |####################
  806293.7 |
  808410.7 |
  810527.6 |
  812644.6 |
  814761.6 |
  816878.6 |
  818995.5 |
  821112.5 |
  823229.5 |
  825346.5 |
  827463.5 |
  829580.4 |
  831697.4 |
  833814.4 |
  835931.4 |
  838048.3 |
  840165.3 |####################
  842282.3 |########################################
  844399.3 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=97.2% of algo (FFI overhead may distort results)

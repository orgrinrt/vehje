# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (4.30 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 828.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 62% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (828.48 us) leads carrier_vert_real_vert4 (1.34 ms) by 62%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 81% (significant)

carrier_vert_real_vert8 is -3.47 ms (81%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 5.2x slower than the field

carrier_vert_real_scalar (4.30 ms) is 5.2x the fastest (828.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.2x the fastest

Fastest carrier_vert_real_vert8 (828.48 us) to slowest carrier_vert_real_scalar (4.30 ms): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 828479.1 ns median (-80.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.20x (fastest 828479.1 ns, slowest 4304222.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 4320179ns | 4308234ns | 4042542ns | 4263246ns | 4544398ns | base |
| carrier_vert_real_vert4 | 1344697ns | 1347943ns | 1325707ns | 1343069ns | 1356634ns | -68.87% |
| carrier_vert_real_vert8 | 833007ns | 831458ns | 808718ns | 828370ns | 852109ns | -80.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 4316385ns | 4038968ns | 4540428ns | base | 0.001 |
| carrier_vert_real_vert4 | 1340896ns | 1322435ns | 1352600ns | -68.93% | 0.003 |
| carrier_vert_real_vert8 | 829848ns | 806464ns | 848512ns | -80.77% | 0.005 |

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.001 | 18.7% |
| carrier_vert_real_vert4 | 0.003 | 60.0% |
| carrier_vert_real_vert8 | 0.005 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 4320179ns | 4320179ns | base |
| carrier_vert_real_vert4 | 1344697ns | 1344697ns | -68.87% |
| carrier_vert_real_vert8 | 833007ns | 833007ns | -80.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 4304222ns | base | --- | [4104506, 4540428] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 1343983ns | -2960601.9ns (-68.8%) | [-3196445, -2769420]ns | [1326106, 1352600] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 828479ns | -3467201.7ns (-80.6%) | [-3702282, -3290128]ns | [812553, 848512] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 4170043ns | -68.3% | -80.7% |
| 2 | 4331802ns | -68.7% | -80.6% |
| 3 | 4704541ns | -71.4% | -81.8% |
| 4 | 4038968ns | -66.6% | -79.6% |
| 5 | 4276642ns | -68.9% | -80.5% |
| 6 | 4376315ns | -69.3% | -81.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.375 | moderate- |
| carrier_vert_real_vert4 | -0.419 | moderate- |
| carrier_vert_real_vert8 | -0.160 | ok |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 4315285.4ns | 4316385.3ns | 100.0% | HIGH |
| carrier_vert_real_vert4 | 1337340.5ns | 1340896.3ns | 99.7% | HIGH |
| carrier_vert_real_vert8 | 842488.3ns | 829848.3ns | 101.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 4038968.3-4540428.1 ns)
  4038968.3 |########################################
  4064041.3 |
  4089114.3 |
  4114187.3 |
  4139260.3 |
  4164333.2 |########################################
  4189406.2 |
  4214479.2 |
  4239552.2 |
  4264625.2 |########################################
  4289698.2 |
  4314771.2 |########################################
  4339844.2 |
  4364917.2 |########################################
  4389990.2 |
  4415063.1 |
  4440136.1 |
  4465209.1 |
  4490282.1 |
  4515355.1 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 1322434.6-1352600.0 ns)
  1322434.6 |####################
  1323942.9 |
  1325451.1 |
  1326959.4 |
  1328467.7 |####################
  1329976.0 |
  1331484.2 |
  1332992.5 |
  1334500.8 |
  1336009.0 |
  1337517.3 |
  1339025.6 |
  1340533.8 |
  1342042.1 |
  1343550.4 |########################################
  1345058.6 |
  1346566.9 |####################
  1348075.2 |
  1349583.5 |
  1351091.7 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 806464.2-848512.5 ns)
  806464.2 |########################################
  808566.6 |
  810669.0 |
  812771.4 |
  814873.9 |
  816976.3 |########################################
  819078.7 |
  821181.1 |########################################
  823283.5 |
  825385.9 |
  827488.3 |
  829590.8 |
  831693.2 |
  833795.6 |########################################
  835898.0 |
  838000.4 |########################################
  840102.8 |
  842205.3 |
  844307.7 |
  846410.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=101.3% of algo (FFI overhead may distort results)

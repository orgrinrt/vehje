# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (20.83 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 3.52 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 56% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (3.52 ms) leads carrier_vert_real_vert4 (5.48 ms) by 56%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 83% (significant)

carrier_vert_real_vert8 is -17.31 ms (83%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 5.9x slower than the field

carrier_vert_real_scalar (20.83 ms) is 5.9x the fastest (3.52 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_real_vert8 is fastest but the noisiest (CV 6.3%)

carrier_vert_real_vert8 wins on median (3.52 ms) yet has the highest variance (CV 6.3%), while carrier_vert_real_scalar is the steadiest (CV 1.5%, 20.83 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.9x the fastest

Fastest carrier_vert_real_vert8 (3.52 ms) to slowest carrier_vert_real_scalar (20.83 ms): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 3515894.4 ns median (-83.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.92x (fastest 3515894.4 ns, slowest 20825628.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 20941040ns | 20829933ns | 20722982ns | 20804475ns | 21254917ns | base |
| carrier_vert_real_vert4 | 5678305ns | 5481833ns | 5470298ns | 5478176ns | 6082502ns | -72.88% |
| carrier_vert_real_vert8 | 3614312ns | 3519835ns | 3502562ns | 3516119ns | 3817477ns | -82.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 20936757ns | 20719025ns | 21250665ns | base | 0.001 |
| carrier_vert_real_vert4 | 5674530ns | 5466905ns | 6078964ns | -72.90% | 0.003 |
| carrier_vert_real_vert8 | 3610339ns | 3499520ns | 3812850ns | -82.76% | 0.005 |

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.001 | 16.8% |
| carrier_vert_real_vert4 | 0.003 | 63.9% |
| carrier_vert_real_vert8 | 0.005 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 20941040ns | 20941040ns | base |
| carrier_vert_real_vert4 | 5678305ns | 5678305ns | -72.88% |
| carrier_vert_real_vert8 | 3614312ns | 3614312ns | -82.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 20825629ns | base | --- | [20733978, 21250665] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 5477491ns | -15304251.4ns (-73.5%) | [-15549690, -14932742]ns | [5467134, 6078964] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 3515894ns | -17314898.4ns (-83.1%) | [-17440451, -17223906]ns | [3502273, 3812850] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 20836704ns | -70.1% | -83.2% |
| 2 | 20864300ns | -73.8% | -83.1% |
| 3 | 20814554ns | -73.7% | -83.1% |
| 4 | 20748931ns | -73.6% | -83.1% |
| 5 | 20719025ns | -73.6% | -83.1% |
| 6 | 21637031ns | -72.6% | -81.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.120 | ok |
| carrier_vert_real_vert4 | -0.089 | ok |
| carrier_vert_real_vert8 | -0.051 | ok |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 20814073.5ns | 20936757.4ns | 99.4% | HIGH |
| carrier_vert_real_vert4 | 5513503.7ns | 5674529.6ns | 97.2% | HIGH |
| carrier_vert_real_vert8 | 3679752.8ns | 3610339.0ns | 101.9% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 20719024.6-21250665.4 ns)
  20719024.6 |########################################
  20745606.6 |########################################
  20772188.7 |
  20798770.7 |########################################
  20825352.8 |########################################
  20851934.8 |########################################
  20878516.8 |
  20905098.9 |
  20931680.9 |
  20958263.0 |
  20984845.0 |
  21011427.0 |
  21038009.1 |
  21064591.1 |
  21091173.2 |
  21117755.2 |
  21144337.2 |
  21170919.3 |
  21197501.3 |
  21224083.4 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 5466905.4-6078963.6 ns)
  5466905.4 |########################################
  5497508.3 |
  5528111.2 |
  5558714.1 |
  5589317.0 |
  5619919.9 |
  5650522.8 |
  5681125.8 |
  5711728.7 |
  5742331.6 |
  5772934.5 |
  5803537.4 |
  5834140.3 |
  5864743.2 |
  5895346.1 |
  5925949.0 |##########
  5956551.9 |
  5987154.8 |
  6017757.7 |
  6048360.6 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 3499520.4-3812849.5 ns)
  3499520.4 |########################################
  3515186.9 |##########################
  3530853.3 |
  3546519.8 |
  3562186.2 |
  3577852.7 |
  3593519.1 |
  3609185.6 |
  3624852.1 |
  3640518.5 |
  3656185.0 |
  3671851.4 |
  3687517.9 |
  3703184.3 |
  3718850.8 |
  3734517.3 |
  3750183.7 |
  3765850.2 |
  3781516.6 |
  3797183.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=102.1% of algo (FFI overhead may distort results)

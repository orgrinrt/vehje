# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (6.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 2.38 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 45% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (2.38 ms) leads carrier_vert_madd_vert4 (3.46 ms) by 45%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 62% (significant)

carrier_vert_madd_vert8 is -3.83 ms (62%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.6x slower than the field

carrier_vert_madd_scalar (6.21 ms) is 2.6x the fastest (2.38 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 2381323.4 ns median (-61.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.61x (fastest 2381323.4 ns, slowest 6211944.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 6216508ns | 6215418ns | 6205860ns | 6214209ns | 6225280ns | base |
| carrier_vert_madd_vert4 | 3463087ns | 3464518ns | 3413026ns | 3457358ns | 3496713ns | -44.29% |
| carrier_vert_madd_vert8 | 2380313ns | 2384603ns | 2355292ns | 2381245ns | 2391425ns | -61.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 6213034ns | 6202432ns | 6221626ns | base | 0.003 |
| carrier_vert_madd_vert4 | 3459726ns | 3410150ns | 3493009ns | -44.32% | 0.005 |
| carrier_vert_madd_vert8 | 2377255ns | 2352610ns | 2388461ns | -61.74% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 38718296 | 84639848 | 0.457 | 1.00× |
| carrier_vert_madd_vert4 | 21364490 | 34587050 | 0.618 | 0.55× |
| carrier_vert_madd_vert8 | 14796354 | 25161035 | 0.588 | 0.38× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 37.9% |
| carrier_vert_madd_vert4 | 0.005 | 68.0% |
| carrier_vert_madd_vert8 | 0.007 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 6216508ns | 6216508ns | base |
| carrier_vert_madd_vert4 | 3463087ns | 3463087ns | -44.29% |
| carrier_vert_madd_vert8 | 2380313ns | 2380313ns | -61.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 6211944ns | base | --- | [6205531, 6221626] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 3460954ns | -2746111.9ns (-44.2%) | [-2786072, -2727738]ns | [3425216, 3493009] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 2381323ns | -3831624.6ns (-61.7%) | [-3851377, -3824336]ns | [2361979, 2388461] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 6208630ns | -44.6% | -62.1% |
| 2 | 6218553ns | -43.6% | -61.6% |
| 3 | 6202432ns | -44.3% | -61.8% |
| 4 | 6213948ns | -45.1% | -61.5% |
| 5 | 6209940ns | -44.2% | -61.6% |
| 6 | 6224700ns | -44.1% | -61.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.426 | moderate- |
| carrier_vert_madd_vert4 | -0.198 | ok |
| carrier_vert_madd_vert8 | -0.265 | moderate- |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 6276846.5ns | 6213033.7ns | 101.0% | HIGH |
| carrier_vert_madd_vert4 | 3461157.4ns | 3459726.4ns | 100.0% | HIGH |
| carrier_vert_madd_vert8 | 2389967.8ns | 2377254.6ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 6202431.7-6221626.4 ns)
  6202431.7 |########################################
  6203391.4 |
  6204351.2 |
  6205310.9 |
  6206270.7 |
  6207230.4 |
  6208190.1 |########################################
  6209149.9 |########################################
  6210109.6 |
  6211069.3 |
  6212029.1 |
  6212988.8 |########################################
  6213948.5 |
  6214908.3 |
  6215868.0 |
  6216827.8 |
  6217787.5 |########################################
  6218747.2 |
  6219707.0 |
  6220666.7 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 3410150.0-3493009.4 ns)
  3410150.0 |########################################
  3414293.0 |
  3418435.9 |
  3422578.9 |
  3426721.9 |
  3430864.9 |
  3435007.8 |
  3439150.8 |########################################
  3443293.8 |
  3447436.7 |
  3451579.7 |
  3455722.7 |########################################
  3459865.6 |
  3464008.6 |########################################
  3468151.6 |
  3472294.5 |
  3476437.5 |########################################
  3480580.5 |
  3484723.5 |
  3488866.4 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 2352610.0-2388461.2 ns)
  2352610.0 |########################################
  2354402.6 |
  2356195.1 |
  2357987.7 |
  2359780.2 |
  2361572.8 |
  2363365.4 |
  2365157.9 |
  2366950.5 |
  2368743.1 |
  2370535.6 |########################################
  2372328.2 |
  2374120.8 |
  2375913.3 |
  2377705.9 |########################################
  2379498.4 |
  2381291.0 |
  2383083.6 |########################################
  2384876.1 |########################################
  2386668.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=101.1% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=100.4% of algo (FFI overhead may distort results)

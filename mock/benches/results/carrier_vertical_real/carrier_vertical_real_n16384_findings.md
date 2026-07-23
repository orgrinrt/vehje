# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (17.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 3.60 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 55% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (3.60 ms) leads carrier_vert_real_vert4 (5.57 ms) by 55%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 79% (significant)

carrier_vert_real_vert8 is -13.59 ms (79%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 4.8x slower than the field

carrier_vert_real_scalar (17.18 ms) is 4.8x the fastest (3.60 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_vert_real_vert8 (3.60 ms) to slowest carrier_vert_real_scalar (17.18 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 3597711.7 ns median (-79.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.78x (fastest 3597711.7 ns, slowest 17183446.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 17296735ns | 17187691ns | 17033072ns | 17156208ns | 17639357ns | base |
| carrier_vert_real_vert4 | 5571228ns | 5570329ns | 5548557ns | 5568370ns | 5586851ns | -67.79% |
| carrier_vert_real_vert8 | 3600148ns | 3601612ns | 3561879ns | 3597236ns | 3623650ns | -79.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 17292506ns | 17029127ns | 17634883ns | base | 0.001 |
| carrier_vert_real_vert4 | 5567229ns | 5544768ns | 5582736ns | -67.81% | 0.003 |
| carrier_vert_real_vert8 | 3596312ns | 3558311ns | 3619755ns | -79.20% | 0.005 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_real_scalar | 106381741 | 84178697 | 1.264 | 1.00× |
| carrier_vert_real_vert4 | 34515008 | 31791810 | 1.086 | 0.32× |
| carrier_vert_real_vert8 | 22398835 | 22344736 | 1.002 | 0.21× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.001 | 20.7% |
| carrier_vert_real_vert4 | 0.003 | 63.9% |
| carrier_vert_real_vert8 | 0.005 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 17296735ns | 17296735ns | base |
| carrier_vert_real_vert4 | 5571228ns | 5571228ns | -67.79% |
| carrier_vert_real_vert8 | 3600148ns | 3600148ns | -79.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 17183447ns | base | --- | [17059188, 17634883] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 5566305ns | -11615368.8ns (-67.6%) | [-12067579, -11492883]ns | [5552647, 5582736] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 3597712ns | -13585735.2ns (-79.1%) | [-14015128, -13487719]ns | [3571469, 3619755] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 17224558ns | -67.5% | -79.1% |
| 2 | 17089249ns | -67.4% | -79.2% |
| 3 | 17029127ns | -67.3% | -79.0% |
| 4 | 17740950ns | -68.7% | -79.5% |
| 5 | 17528816ns | -68.2% | -79.4% |
| 6 | 17142335ns | -67.7% | -79.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | 0.050 | ok |
| carrier_vert_real_vert4 | -0.091 | ok |
| carrier_vert_real_vert8 | 0.003 | ok |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 17391005.1ns | 17292506.0ns | 100.6% | HIGH |
| carrier_vert_real_vert4 | 5574234.0ns | 5567229.3ns | 100.1% | HIGH |
| carrier_vert_real_vert8 | 3682358.4ns | 3596311.9ns | 102.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 17029126.7-17634883.1 ns)
  17029126.7 |########################################
  17059414.5 |########################################
  17089702.3 |
  17119990.2 |########################################
  17150278.0 |
  17180565.8 |
  17210853.6 |########################################
  17241141.4 |
  17271429.3 |
  17301717.1 |
  17332004.9 |
  17362292.7 |
  17392580.5 |
  17422868.4 |
  17453156.2 |
  17483444.0 |
  17513731.8 |########################################
  17544019.6 |
  17574307.5 |
  17604595.3 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 5544768.3-5582735.6 ns)
  5544768.3 |########################################
  5546666.7 |
  5548565.0 |
  5550463.4 |
  5552361.8 |
  5554260.1 |
  5556158.5 |
  5558056.9 |
  5559955.2 |########################################
  5561853.6 |########################################
  5563751.9 |
  5565650.3 |
  5567548.7 |
  5569447.0 |########################################
  5571345.4 |
  5573243.8 |########################################
  5575142.1 |
  5577040.5 |
  5578938.9 |
  5580837.2 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 3558310.8-3619754.8 ns)
  3558310.8 |####################
  3561383.0 |
  3564455.2 |
  3567527.4 |
  3570599.6 |
  3573671.8 |
  3576744.0 |
  3579816.2 |
  3582888.4 |####################
  3585960.6 |
  3589032.8 |
  3592105.0 |####################
  3595177.2 |
  3598249.4 |
  3601321.6 |########################################
  3604393.8 |
  3607466.0 |
  3610538.2 |
  3613610.4 |
  3616682.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=102.2% of algo (FFI overhead may distort results)

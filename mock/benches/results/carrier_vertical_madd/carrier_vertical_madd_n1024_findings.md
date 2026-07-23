# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (378.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 140.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 50% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (140.55 us) leads carrier_vert_madd_vert4 (211.26 us) by 50%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 63% (significant)

carrier_vert_madd_vert8 is -238.64 us (63%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.7x slower than the field

carrier_vert_madd_scalar (378.40 us) is 2.7x the fastest (140.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_madd_scalar shows alternating (throttle bounce) (autocorr -0.67)

carrier_vert_madd_scalar's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 140546.0 ns median (-62.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.69x (fastest 140546.0 ns, slowest 378399.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 381980ns | 380615ns | 378219ns | 380120ns | 386652ns | base |
| carrier_vert_madd_vert4 | 213339ns | 213785ns | 209562ns | 213253ns | 215356ns | -44.15% |
| carrier_vert_madd_vert8 | 142788ns | 143036ns | 141041ns | 142536ns | 144038ns | -62.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 379635ns | 375707ns | 384172ns | base | 0.003 |
| carrier_vert_madd_vert4 | 210808ns | 206651ns | 212871ns | -44.47% | 0.005 |
| carrier_vert_madd_vert8 | 140352ns | 138670ns | 141741ns | -63.03% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 2382320 | 5312689 | 0.448 | 1.00× |
| carrier_vert_madd_vert4 | 1310599 | 2181797 | 0.601 | 0.55× |
| carrier_vert_madd_vert8 | 885253 | 1594226 | 0.555 | 0.37× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 36.6% |
| carrier_vert_madd_vert4 | 0.005 | 65.6% |
| carrier_vert_madd_vert8 | 0.007 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 381980ns | 381980ns | base |
| carrier_vert_madd_vert4 | 213339ns | 213339ns | -44.15% |
| carrier_vert_madd_vert8 | 142788ns | 142788ns | -62.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 378400ns | base | --- | [376335, 384172] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 211265ns | -167745.6ns (-44.3%) | [-172577, -166158]ns | [208290, 212871] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 140546ns | -238637.9ns (-63.1%) | [-242466, -236745]ns | [138770, 141741] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 376963ns | -45.2% | -62.8% |
| 2 | 381567ns | -44.0% | -62.6% |
| 3 | 375707ns | -43.9% | -63.0% |
| 4 | 386777ns | -45.2% | -63.6% |
| 5 | 377567ns | -44.4% | -63.3% |
| 6 | 379232ns | -44.1% | -62.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.670 | HIGH- (thermal bounce) |
| carrier_vert_madd_vert4 | -0.512 | HIGH- (thermal bounce) |
| carrier_vert_madd_vert8 | -0.533 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 384164.7ns | 379635.4ns | 101.2% | HIGH |
| carrier_vert_madd_vert4 | 211551.7ns | 210808.5ns | 100.4% | HIGH |
| carrier_vert_madd_vert8 | 141784.4ns | 140352.4ns | 101.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 375706.7-384171.7 ns)
  375706.7 |########################################
  376130.0 |
  376553.2 |########################################
  376976.5 |
  377399.7 |########################################
  377823.0 |
  378246.2 |
  378669.5 |
  379092.7 |########################################
  379516.0 |
  379939.2 |
  380362.5 |
  380785.7 |
  381209.0 |########################################
  381632.2 |
  382055.5 |
  382478.7 |
  382902.0 |
  383325.2 |
  383748.5 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 206651.2-212870.6 ns)
  206651.2 |########################################
  206962.2 |
  207273.1 |
  207584.1 |
  207895.1 |
  208206.0 |
  208517.0 |
  208828.0 |
  209139.0 |
  209449.9 |
  209760.9 |########################################
  210071.9 |
  210382.8 |########################################
  210693.8 |
  211004.8 |
  211315.8 |
  211626.7 |########################################
  211937.7 |########################################
  212248.7 |
  212559.6 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 138670.0-141741.2 ns)
  138670.0 |########################################
  138823.6 |########################################
  138977.1 |
  139130.7 |
  139284.2 |
  139437.8 |
  139591.4 |
  139744.9 |
  139898.5 |
  140052.1 |
  140205.6 |########################################
  140359.2 |
  140512.8 |
  140666.3 |########################################
  140819.9 |########################################
  140973.4 |
  141127.0 |
  141280.6 |
  141434.1 |
  141587.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=101.2% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=100.9% of algo (FFI overhead may distort results)

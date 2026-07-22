# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (91.47 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 34.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 50% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (34.53 us) leads carrier_vert_madd_vert4 (51.93 us) by 50%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 61% (significant)

carrier_vert_madd_vert8 is -55.80 us (61%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.6x slower than the field

carrier_vert_madd_scalar (91.47 us) is 2.6x the fastest (34.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 34528.8 ns median (-62.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.65x (fastest 34528.8 ns, slowest 91474.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 93854ns | 93888ns | 92266ns | 93738ns | 94823ns | base |
| carrier_vert_madd_vert4 | 53681ns | 54484ns | 50454ns | 53565ns | 55469ns | -42.80% |
| carrier_vert_madd_vert8 | 37685ns | 36962ns | 36692ns | 36878ns | 39391ns | -59.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 91394ns | 89852ns | 92384ns | base | 0.003 |
| carrier_vert_madd_vert4 | 51198ns | 48058ns | 52956ns | -43.98% | 0.005 |
| carrier_vert_madd_vert8 | 35251ns | 34264ns | 36898ns | -61.43% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 37.5% |
| carrier_vert_madd_vert4 | 0.005 | 66.0% |
| carrier_vert_madd_vert8 | 0.007 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 93854ns | 93854ns | base |
| carrier_vert_madd_vert4 | 53681ns | 53681ns | -42.80% |
| carrier_vert_madd_vert8 | 37685ns | 37685ns | -59.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 91475ns | base | --- | [90324, 92384] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 51930ns | -39269.9ns (-42.9%) | [-43120, -38197]ns | [48709, 52956] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 34529ns | -55795.0ns (-61.0%) | [-58058, -54577]ns | [34326, 36898] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 92164ns | -47.9% | -62.7% |
| 2 | 89852ns | -42.6% | -61.4% |
| 3 | 91455ns | -41.6% | -59.7% |
| 4 | 90795ns | -42.4% | -62.1% |
| 5 | 92603ns | -43.3% | -63.0% |
| 6 | 91495ns | -46.1% | -59.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.400 | moderate- |
| carrier_vert_madd_vert4 | 0.057 | ok |
| carrier_vert_madd_vert8 | -0.318 | moderate- |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 91844.8ns | 91394.0ns | 100.5% | HIGH |
| carrier_vert_madd_vert4 | 102466.1ns | 51198.5ns | 200.1% | HIGH |
| carrier_vert_madd_vert8 | 106039.5ns | 35250.6ns | 300.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 89852.1-92383.5 ns)
  89852.1 |####################
  89978.7 |
  90105.2 |
  90231.8 |
  90358.4 |
  90485.0 |
  90611.5 |
  90738.1 |####################
  90864.7 |
  90991.2 |
  91117.8 |
  91244.4 |
  91370.9 |########################################
  91497.5 |
  91624.1 |
  91750.6 |
  91877.2 |
  92003.8 |
  92130.4 |####################
  92256.9 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 48057.5-52956.1 ns)
  48057.5 |########################################
  48302.4 |
  48547.4 |
  48792.3 |
  49037.2 |
  49282.1 |########################################
  49527.1 |
  49772.0 |
  50016.9 |
  50261.8 |
  50506.8 |
  50751.7 |
  50996.6 |
  51241.6 |
  51486.5 |########################################
  51731.4 |
  51976.3 |
  52221.3 |########################################
  52466.2 |########################################
  52711.1 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 34264.2-36897.5 ns)
  34264.2 |########################################
  34395.9 |####################
  34527.5 |####################
  34659.2 |
  34790.9 |
  34922.5 |
  35054.2 |
  35185.9 |
  35317.5 |
  35449.2 |
  35580.8 |
  35712.5 |
  35844.2 |
  35975.8 |
  36107.5 |
  36239.2 |
  36370.8 |
  36502.5 |
  36634.2 |
  36765.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=199.9% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=300.7% of algo (FFI overhead may distort results)

# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (6.29 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 2.35 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 44% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (2.35 ms) leads carrier_vert_madd_vert4 (3.39 ms) by 44%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 63% (significant)

carrier_vert_madd_vert8 is -3.94 ms (63%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.7x slower than the field

carrier_vert_madd_scalar (6.29 ms) is 2.7x the fastest (2.35 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 2346559.0 ns median (-62.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.68x (fastest 2346559.0 ns, slowest 6289040.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 6317845ns | 6292758ns | 6278162ns | 6291922ns | 6376570ns | base |
| carrier_vert_madd_vert4 | 3400478ns | 3394265ns | 3386215ns | 3393028ns | 3418785ns | -46.18% |
| carrier_vert_madd_vert8 | 2351861ns | 2350420ns | 2346084ns | 2349630ns | 2358096ns | -62.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 6314225ns | 6273913ns | 6373332ns | base | 0.003 |
| carrier_vert_madd_vert4 | 3397054ns | 3383100ns | 3415745ns | -46.20% | 0.005 |
| carrier_vert_madd_vert8 | 2348440ns | 2342431ns | 2355055ns | -62.81% | 0.007 |

## Performance model

- Peak throughput: **0.007 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 37.2% |
| carrier_vert_madd_vert4 | 0.005 | 69.1% |
| carrier_vert_madd_vert8 | 0.007 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 6317845ns | 6317845ns | base |
| carrier_vert_madd_vert4 | 3400478ns | 3400478ns | -46.18% |
| carrier_vert_madd_vert8 | 2351861ns | 2351861ns | -62.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 6289041ns | base | --- | [6280303, 6373332] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 3390678ns | -2899955.8ns (-46.1%) | [-2975017, -2876541]ns | [3384739, 3415745] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 2346559ns | -3944193.5ns (-62.7%) | [-4018277, -3934884]ns | [2343707, 2355055] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 6273913ns | -45.9% | -62.6% |
| 2 | 6309852ns | -45.5% | -62.6% |
| 3 | 6436812ns | -47.3% | -63.5% |
| 4 | 6289108ns | -46.2% | -62.7% |
| 5 | 6288973ns | -46.1% | -62.7% |
| 6 | 6286692ns | -46.1% | -62.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.113 | ok |
| carrier_vert_madd_vert4 | -0.065 | ok |
| carrier_vert_madd_vert8 | 0.128 | ok |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 6507585.3ns | 6314225.2ns | 103.1% | HIGH |
| carrier_vert_madd_vert4 | 3396098.0ns | 3397054.1ns | 100.0% | HIGH |
| carrier_vert_madd_vert8 | 2360089.9ns | 2348440.1ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 6273913.3-6373331.9 ns)
  6273913.3 |####################
  6278884.2 |
  6283855.2 |####################
  6288826.1 |########################################
  6293797.0 |
  6298768.0 |
  6303738.9 |
  6308709.8 |####################
  6313680.7 |
  6318651.7 |
  6323622.6 |
  6328593.5 |
  6333564.5 |
  6338535.4 |
  6343506.3 |
  6348477.2 |
  6353448.2 |
  6358419.1 |
  6363390.0 |
  6368361.0 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 3383099.6-3415745.0 ns)
  3383099.6 |####################
  3384731.9 |
  3386364.1 |####################
  3387996.4 |####################
  3389628.7 |
  3391261.0 |########################################
  3392893.2 |
  3394525.5 |
  3396157.8 |
  3397790.0 |
  3399422.3 |
  3401054.6 |
  3402686.8 |
  3404319.1 |
  3405951.4 |
  3407583.6 |
  3409215.9 |
  3410848.2 |
  3412480.5 |
  3414112.7 |
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 2342430.8-2355054.6 ns)
  2342430.8 |########################################
  2343062.0 |
  2343693.2 |
  2344324.4 |
  2344955.6 |########################################
  2345586.8 |########################################
  2346217.9 |
  2346849.1 |########################################
  2347480.3 |
  2348111.5 |
  2348742.7 |
  2349373.9 |
  2350005.1 |
  2350636.3 |########################################
  2351267.5 |
  2351898.6 |
  2352529.8 |
  2353161.0 |
  2353792.2 |
  2354423.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=100.5% of algo (FFI overhead may distort results)

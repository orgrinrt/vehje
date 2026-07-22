# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (20.77 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 3.51 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 56% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (3.51 ms) leads carrier_vert_scatter_vert4 (5.49 ms) by 56%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 83% (significant)

carrier_vert_scatter_vert8 is -17.25 ms (83%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 5.9x slower than the field

carrier_vert_scatter_scalar (20.77 ms) is 5.9x the fastest (3.51 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_scatter_vert8 shows warm-up / thermal drift (autocorr +0.57)

carrier_vert_scatter_vert8's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.9x the fastest

Fastest carrier_vert_scatter_vert8 (3.51 ms) to slowest carrier_vert_scatter_scalar (20.77 ms): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 3512692.1 ns median (-83.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.91x (fastest 3512692.1 ns, slowest 20773526.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 20773847ns | 20777295ns | 20696920ns | 20766512ns | 20823313ns | base |
| carrier_vert_scatter_vert4 | 5485968ns | 5491562ns | 5446832ns | 5489469ns | 5500285ns | -73.59% |
| carrier_vert_scatter_vert8 | 3516102ns | 3515898ns | 3507473ns | 3514350ns | 3523044ns | -83.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 20769951ns | 20693674ns | 20819008ns | base | 0.001 |
| carrier_vert_scatter_vert4 | 5482676ns | 5443764ns | 5496805ns | -73.60% | 0.003 |
| carrier_vert_scatter_vert8 | 3512884ns | 3504529ns | 3519519ns | -83.09% | 0.005 |

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.001 | 16.9% |
| carrier_vert_scatter_vert4 | 0.003 | 63.9% |
| carrier_vert_scatter_vert8 | 0.005 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 20773847ns | 20773847ns | base |
| carrier_vert_scatter_vert4 | 5485968ns | 5485968ns | -73.59% |
| carrier_vert_scatter_vert8 | 3516102ns | 3516102ns | -83.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 20773526ns | base | --- | [20717320, 20819008] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 5488395ns | -15299632.1ns (-73.6%) | [-15333268, -15228925]ns | [5462829, 5496805] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 3512692ns | -17254006.9ns (-83.1%) | [-17307655, -17209541]ns | [3506440, 3519519] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 20768005ns | -73.8% | -83.1% |
| 2 | 20779047ns | -73.5% | -83.1% |
| 3 | 20829533ns | -73.6% | -83.1% |
| 4 | 20740966ns | -73.5% | -83.1% |
| 5 | 20693674ns | -73.5% | -83.1% |
| 6 | 20808483ns | -73.7% | -83.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | -0.164 | ok |
| carrier_vert_scatter_vert4 | -0.296 | moderate- |
| carrier_vert_scatter_vert8 | 0.568 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 20781124.2ns | 20769951.4ns | 100.1% | HIGH |
| carrier_vert_scatter_vert4 | 5500753.4ns | 5482676.4ns | 100.3% | HIGH |
| carrier_vert_scatter_vert8 | 3592850.8ns | 3512883.8ns | 102.3% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 20693674.2-20819008.1 ns)
  20693674.2 |########################################
  20699940.9 |
  20706207.6 |
  20712474.3 |
  20718741.0 |
  20725007.7 |
  20731274.4 |
  20737541.1 |########################################
  20743807.8 |
  20750074.5 |
  20756341.1 |
  20762607.8 |########################################
  20768874.5 |
  20775141.2 |########################################
  20781407.9 |
  20787674.6 |
  20793941.3 |
  20800208.0 |
  20806474.7 |########################################
  20812741.4 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 5443763.8-5496804.6 ns)
  5443763.8 |####################
  5446415.8 |
  5449067.9 |
  5451719.9 |
  5454372.0 |
  5457024.0 |
  5459676.0 |
  5462328.1 |
  5464980.1 |
  5467632.2 |
  5470284.2 |
  5472936.2 |
  5475588.3 |
  5478240.3 |
  5480892.4 |####################
  5483544.4 |
  5486196.4 |########################################
  5488848.5 |####################
  5491500.5 |
  5494152.6 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 3504529.2-3519519.3 ns)
  3504529.2 |########################################
  3505278.7 |
  3506028.2 |
  3506777.7 |
  3507527.2 |
  3508276.7 |########################################
  3509026.2 |
  3509775.8 |
  3510525.3 |########################################
  3511274.8 |
  3512024.3 |
  3512773.8 |
  3513523.3 |
  3514272.8 |########################################
  3515022.3 |
  3515771.8 |
  3516521.3 |
  3517270.8 |
  3518020.3 |
  3518769.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: autocorrelation=0.57 (measurement drift or warm-up artifact)
- **carrier_vert_scatter_vert8**: bridge=102.3% of algo (FFI overhead may distort results)

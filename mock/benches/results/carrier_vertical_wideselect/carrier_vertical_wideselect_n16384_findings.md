# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (17.12 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 3.19 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 dominates: 46% faster than the next best (carrier_vert_wideselect_vert4)

carrier_vert_wideselect_vert8 (3.19 ms) leads carrier_vert_wideselect_vert4 (4.67 ms) by 46%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_wideselect_vert8 beats baseline by 81% (significant)

carrier_vert_wideselect_vert8 is -13.93 ms (81%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 5.4x slower than the field

carrier_vert_wideselect_scalar (17.12 ms) is 5.4x the fastest (3.19 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 5.4x the fastest

Fastest carrier_vert_wideselect_vert8 (3.19 ms) to slowest carrier_vert_wideselect_scalar (17.12 ms): 5.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 3194867.1 ns median (-81.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.36x (fastest 3194867.1 ns, slowest 17121636.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 17133913ns | 17125812ns | 16974982ns | 17082643ns | 17290283ns | base |
| carrier_vert_wideselect_vert4 | 4701675ns | 4673422ns | 4662803ns | 4671476ns | 4766408ns | -72.56% |
| carrier_vert_wideselect_vert8 | 3197020ns | 3198437ns | 3185112ns | 3196938ns | 3203097ns | -81.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 17129607ns | 16970752ns | 17285663ns | base | 0.001 |
| carrier_vert_wideselect_vert4 | 4698093ns | 4658297ns | 4763187ns | -72.57% | 0.003 |
| carrier_vert_wideselect_vert8 | 3193284ns | 3180659ns | 3199260ns | -81.36% | 0.005 |

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.001 | 18.6% |
| carrier_vert_wideselect_vert4 | 0.004 | 68.1% |
| carrier_vert_wideselect_vert8 | 0.005 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 17133913ns | 17133913ns | base |
| carrier_vert_wideselect_vert4 | 4701675ns | 4701675ns | -72.56% |
| carrier_vert_wideselect_vert8 | 3197020ns | 3197020ns | -81.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 17121637ns | base | --- | [16981522, 17285663] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 4669668ns | -12448560.0ns (-72.7%) | [-12531458, -12314523]ns | [4661425, 4763187] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 3194867ns | -13931107.7ns (-81.4%) | [-14089995, -13787865]ns | [3185726, 3199260] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 16992292ns | -72.5% | -81.2% |
| 2 | 17143158ns | -72.7% | -81.3% |
| 3 | 17100115ns | -72.8% | -81.4% |
| 4 | 17212582ns | -72.9% | -81.4% |
| 5 | 17358744ns | -72.1% | -81.6% |
| 6 | 16970752ns | -72.5% | -81.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | -0.211 | moderate- |
| carrier_vert_wideselect_vert4 | -0.284 | moderate- |
| carrier_vert_wideselect_vert8 | -0.365 | moderate- |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 17606149.7ns | 17129607.1ns | 102.8% | HIGH |
| carrier_vert_wideselect_vert4 | 4703291.8ns | 4698093.4ns | 100.1% | HIGH |
| carrier_vert_wideselect_vert8 | 3260768.9ns | 3193284.3ns | 102.1% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 16970752.1-17285662.9 ns)
  16970752.1 |########################################
  16986497.6 |########################################
  17002243.2 |
  17017988.7 |
  17033734.3 |
  17049479.8 |
  17065225.3 |
  17080970.9 |
  17096716.4 |########################################
  17112462.0 |
  17128207.5 |########################################
  17143953.0 |
  17159698.6 |
  17175444.1 |
  17191189.7 |
  17206935.2 |########################################
  17222680.7 |
  17238426.3 |
  17254171.8 |
  17269917.4 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 4658297.1-4763187.2 ns)
  4658297.1 |####################
  4663541.6 |####################
  4668786.1 |########################################
  4674030.6 |
  4679275.1 |
  4684519.6 |####################
  4689764.1 |
  4695008.7 |
  4700253.2 |
  4705497.7 |
  4710742.2 |
  4715986.7 |
  4721231.2 |
  4726475.7 |
  4731720.2 |
  4736964.7 |
  4742209.2 |
  4747453.7 |
  4752698.2 |
  4757942.7 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 3180658.7-3199259.8 ns)
  3180658.7 |########################################
  3181588.8 |
  3182518.8 |
  3183448.9 |
  3184378.9 |
  3185309.0 |
  3186239.0 |
  3187169.1 |
  3188099.1 |
  3189029.2 |
  3189959.2 |########################################
  3190889.3 |
  3191819.4 |
  3192749.4 |########################################
  3193679.5 |
  3194609.5 |
  3195539.6 |
  3196469.6 |########################################
  3197399.7 |########################################
  3198329.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=102.1% of algo (FFI overhead may distort results)

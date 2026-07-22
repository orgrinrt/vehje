# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (314.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 120.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 dominates: 45% faster than the next best (carrier_vert_wideselect_vert4)

carrier_vert_wideselect_vert8 (120.25 us) leads carrier_vert_wideselect_vert4 (174.18 us) by 45%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_wideselect_vert8 beats baseline by 62% (significant)

carrier_vert_wideselect_vert8 is -194.47 us (62%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 2.6x slower than the field

carrier_vert_wideselect_scalar (314.59 us) is 2.6x the fastest (120.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_wideselect_vert4 shows alternating (throttle bounce) (autocorr -0.56)

carrier_vert_wideselect_vert4's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 120252.5 ns median (-61.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.62x (fastest 120252.5 ns, slowest 314587.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 317030ns | 316838ns | 315139ns | 316437ns | 318865ns | base |
| carrier_vert_wideselect_vert4 | 176829ns | 176390ns | 174678ns | 176020ns | 179118ns | -44.22% |
| carrier_vert_wideselect_vert8 | 123038ns | 122487ns | 121172ns | 122286ns | 125098ns | -61.19% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 314788ns | 312832ns | 316646ns | base | 0.003 |
| carrier_vert_wideselect_vert4 | 174611ns | 172465ns | 176870ns | -44.53% | 0.006 |
| carrier_vert_wideselect_vert8 | 120795ns | 118948ns | 122819ns | -61.63% | 0.008 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.003 | 37.8% |
| carrier_vert_wideselect_vert4 | 0.006 | 68.3% |
| carrier_vert_wideselect_vert8 | 0.009 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 317030ns | 317030ns | base |
| carrier_vert_wideselect_vert4 | 176829ns | 176829ns | -44.22% |
| carrier_vert_wideselect_vert8 | 123038ns | 123038ns | -61.19% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 314588ns | base | --- | [313130, 316646] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 174182ns | -139466.0ns (-44.3%) | [-143866, -137199]ns | [172780, 176870] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 120252ns | -194467.2ns (-61.8%) | [-195709, -191803]ns | [119313, 122819] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 314971ns | -45.2% | -62.2% |
| 2 | 314711ns | -43.8% | -62.0% |
| 3 | 318320ns | -45.6% | -61.4% |
| 4 | 314465ns | -44.3% | -61.7% |
| 5 | 313427ns | -43.6% | -61.7% |
| 6 | 312832ns | -44.6% | -60.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.091 | ok |
| carrier_vert_wideselect_vert4 | -0.564 | HIGH- (thermal bounce) |
| carrier_vert_wideselect_vert8 | -0.165 | ok |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 314728.4ns | 314787.7ns | 100.0% | HIGH |
| carrier_vert_wideselect_vert4 | 189750.8ns | 174610.8ns | 108.7% | HIGH |
| carrier_vert_wideselect_vert8 | 121977.4ns | 120794.9ns | 101.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 312832.5-316645.8 ns)
  312832.5 |########################################
  313023.2 |
  313213.8 |
  313404.5 |########################################
  313595.2 |
  313785.8 |
  313976.5 |
  314167.2 |
  314357.8 |########################################
  314548.5 |########################################
  314739.2 |
  314929.8 |########################################
  315120.5 |
  315311.1 |
  315501.8 |
  315692.5 |
  315883.1 |
  316073.8 |
  316264.5 |
  316455.1 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 172465.4-176869.8 ns)
  172465.4 |########################################
  172685.6 |
  172905.8 |########################################
  173126.1 |########################################
  173346.3 |
  173566.5 |
  173786.7 |
  174006.9 |
  174227.2 |
  174447.4 |
  174667.6 |
  174887.8 |########################################
  175108.0 |
  175328.3 |
  175548.5 |
  175768.7 |
  175988.9 |
  176209.1 |
  176429.4 |
  176649.6 |########################################
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 118948.3-122819.4 ns)
  118948.3 |########################################
  119141.9 |
  119335.4 |
  119529.0 |########################################
  119722.5 |
  119916.1 |########################################
  120109.6 |
  120303.2 |
  120496.7 |########################################
  120690.3 |
  120883.8 |
  121077.4 |
  121270.9 |
  121464.5 |
  121658.0 |
  121851.6 |
  122045.1 |
  122238.7 |
  122432.2 |
  122625.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=108.8% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=101.0% of algo (FFI overhead may distort results)

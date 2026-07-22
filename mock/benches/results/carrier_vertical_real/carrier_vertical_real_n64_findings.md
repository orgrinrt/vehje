# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (19.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 7.32 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 beats baseline by 63% (significant)

carrier_vert_real_vert8 is -12.29 us (63%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 2.7x slower than the field

carrier_vert_real_scalar (19.44 us) is 2.7x the fastest (7.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 7323.8 ns median (-62.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.65x (fastest 7323.8 ns, slowest 19437.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 22114ns | 22023ns | 21368ns | 21988ns | 22675ns | base |
| carrier_vert_real_vert4 | 10338ns | 10559ns | 9274ns | 10507ns | 10616ns | -53.25% |
| carrier_vert_real_vert8 | 9901ns | 9915ns | 9486ns | 9892ns | 10122ns | -55.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 19502ns | 18862ns | 19971ns | base | 0.003 |
| carrier_vert_real_vert4 | 7803ns | 7013ns | 7978ns | -59.99% | 0.008 |
| carrier_vert_real_vert8 | 7286ns | 6940ns | 7435ns | -62.64% | 0.009 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.003 | 35.7% |
| carrier_vert_real_vert4 | 0.008 | 87.3% |
| carrier_vert_real_vert8 | 0.009 | 94.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 22114ns | 22114ns | base |
| carrier_vert_real_vert4 | 10338ns | 10338ns | -53.25% |
| carrier_vert_real_vert8 | 9901ns | 9901ns | -55.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 19438ns | base | --- | [19097, 19971] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 7951ns | -11694.4ns (-60.2%) | [-11995, -11406]ns | [7480, 7978] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 7324ns | -12285.2ns (-63.2%) | [-12624, -11739]ns | [7099, 7435] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 18862ns | -62.8% | -61.5% |
| 2 | 19331ns | -58.9% | -64.1% |
| 3 | 19492ns | -59.2% | -62.5% |
| 4 | 19383ns | -59.0% | -61.3% |
| 5 | 19905ns | -59.8% | -63.2% |
| 6 | 20037ns | -60.3% | -63.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | 0.310 | moderate+ |
| carrier_vert_real_vert4 | -0.020 | ok |
| carrier_vert_real_vert8 | 0.116 | ok |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 97613.2ns | 19501.9ns | 500.5% | HIGH |
| carrier_vert_real_vert4 | 88489.7ns | 7803.3ns | 1134.0% | HIGH |
| carrier_vert_real_vert8 | 88394.6ns | 7286.0ns | 1213.2% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 18862.5-19970.8 ns)
  18862.5 |########################################
  18917.9 |
  18973.3 |
  19028.8 |
  19084.2 |
  19139.6 |
  19195.0 |
  19250.4 |
  19305.8 |########################################
  19361.3 |########################################
  19416.7 |
  19472.1 |########################################
  19527.5 |
  19582.9 |
  19638.3 |
  19693.8 |
  19749.2 |
  19804.6 |
  19860.0 |########################################
  19915.4 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 7012.9-7978.3 ns)
   7012.9 |##########
   7061.2 |
   7109.4 |
   7157.7 |
   7206.0 |
   7254.2 |
   7302.5 |
   7350.8 |
   7399.1 |
   7447.3 |
   7495.6 |
   7543.9 |
   7592.1 |
   7640.4 |
   7688.7 |
   7736.9 |
   7785.2 |
   7833.5 |
   7881.8 |
   7930.0 |########################################
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 6940.0-7435.4 ns)
   6940.0 |####################
   6964.8 |
   6989.5 |
   7014.3 |
   7039.1 |
   7063.9 |
   7088.6 |
   7113.4 |
   7138.2 |
   7162.9 |
   7187.7 |
   7212.5 |
   7237.2 |####################
   7262.0 |
   7286.8 |
   7311.5 |########################################
   7336.3 |####################
   7361.1 |
   7385.9 |
   7410.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=501.1% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=1105.2% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=1203.6% of algo (FFI overhead may distort results)

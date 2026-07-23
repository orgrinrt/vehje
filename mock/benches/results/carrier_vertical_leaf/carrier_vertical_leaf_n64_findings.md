# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (10.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 6.42 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 beats baseline by 35% (significant)

carrier_vert_leaf_vert8 is -3.63 us (35%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_vert8 is fastest but the noisiest (CV 11.3%)

carrier_vert_leaf_vert8 wins on median (6.42 us) yet has the highest variance (CV 11.3%), while carrier_vert_leaf_vert4 is the steadiest (CV 1.9%, 6.82 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader carrier_vert_leaf_vert8 vs stability leader carrier_vert_leaf_vert4 (+6% speed for 5.9x steadier)

carrier_vert_leaf_vert8 is fastest (6.42 us, CV 11.3%); carrier_vert_leaf_vert4 gives up 6.2% median for 5.9x lower variance (CV 1.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 6423.3 ns median (-37.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.60x (fastest 6423.3 ns, slowest 10255.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 12811ns | 12636ns | 11915ns | 12556ns | 13640ns | base |
| carrier_vert_leaf_vert4 | 9174ns | 9164ns | 8949ns | 9146ns | 9330ns | -28.39% |
| carrier_vert_leaf_vert8 | 9031ns | 8689ns | 8420ns | 8638ns | 9927ns | -29.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 10397ns | 9689ns | 11098ns | base | 0.006 |
| carrier_vert_leaf_vert4 | 6857ns | 6701ns | 7027ns | -34.04% | 0.009 |
| carrier_vert_leaf_vert8 | 6692ns | 6200ns | 7422ns | -35.63% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 311291 | 1522376 | 0.204 | 1.00× |
| carrier_vert_leaf_vert4 | 304002 | 857118 | 0.355 | 0.98× |
| carrier_vert_leaf_vert8 | 299364 | 621050 | 0.482 | 0.96× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.006 | 60.5% |
| carrier_vert_leaf_vert4 | 0.009 | 90.9% |
| carrier_vert_leaf_vert8 | 0.010 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 12811ns | 12811ns | base |
| carrier_vert_leaf_vert4 | 9174ns | 9174ns | -28.39% |
| carrier_vert_leaf_vert8 | 9031ns | 9031ns | -29.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 10255ns | base | --- | [9838, 11098] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 6823ns | -3464.9ns (-33.8%) | [-4098, -3056]ns | [6722, 7027] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 6423ns | -3632.8ns (-35.4%) | [-4123, -3358]ns | [6231, 7422] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 9986ns | -32.9% | -34.4% |
| 2 | 10039ns | -32.8% | -37.6% |
| 3 | 9689ns | -29.2% | -36.0% |
| 4 | 10471ns | -35.2% | -38.8% |
| 5 | 11574ns | -38.9% | -28.3% |
| 6 | 10622ns | -34.2% | -39.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | 0.312 | moderate+ |
| carrier_vert_leaf_vert4 | 0.277 | moderate+ |
| carrier_vert_leaf_vert8 | -0.142 | ok |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 91890.0ns | 10396.8ns | 883.8% | HIGH |
| carrier_vert_leaf_vert4 | 90051.9ns | 6857.3ns | 1313.2% | HIGH |
| carrier_vert_leaf_vert8 | 93585.0ns | 6692.3ns | 1398.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 9689.2-11097.7 ns)
   9689.2 |####################
   9759.6 |
   9830.1 |
   9900.5 |
   9970.9 |########################################
  10041.3 |
  10111.8 |
  10182.2 |
  10252.6 |
  10323.0 |
  10393.5 |
  10463.9 |####################
  10534.3 |
  10604.7 |####################
  10675.2 |
  10745.6 |
  10816.0 |
  10886.4 |
  10956.9 |
  11027.3 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 6700.8-7027.0 ns)
   6700.8 |########################################
   6717.1 |
   6733.4 |########################################
   6749.7 |
   6766.1 |
   6782.4 |########################################
   6798.7 |
   6815.0 |
   6831.3 |
   6847.6 |########################################
   6863.9 |
   6880.2 |
   6896.5 |
   6912.9 |
   6929.2 |
   6945.5 |
   6961.8 |
   6978.1 |########################################
   6994.4 |
   7010.7 |
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 6199.6-7422.3 ns)
   6199.6 |####################
   6260.7 |####################
   6321.9 |
   6383.0 |########################################
   6444.1 |
   6505.3 |####################
   6566.4 |
   6627.5 |
   6688.7 |
   6749.8 |
   6811.0 |
   6872.1 |
   6933.2 |
   6994.4 |
   7055.5 |
   7116.6 |
   7177.8 |
   7238.9 |
   7300.0 |
   7361.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=888.7% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=1317.6% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=1403.9% of algo (FFI overhead may distort results)

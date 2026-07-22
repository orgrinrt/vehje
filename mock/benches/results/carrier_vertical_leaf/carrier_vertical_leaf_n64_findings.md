# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (16.12 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 6.79 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 beats baseline by 58% (significant)

carrier_vert_leaf_vert8 is -9.40 us (58%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 2.4x slower than the field

carrier_vert_leaf_scalar (16.12 us) is 2.4x the fastest (6.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_leaf_vert4 shows alternating (throttle bounce) (autocorr -0.54)

carrier_vert_leaf_vert4's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_vert_leaf_vert8 vs stability leader carrier_vert_leaf_vert4 (+8% speed for 1.4x steadier)

carrier_vert_leaf_vert8 is fastest (6.79 us, CV 4.4%); carrier_vert_leaf_vert4 gives up 8.1% median for 1.4x lower variance (CV 3.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 6787.7 ns median (-57.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.38x (fastest 6787.7 ns, slowest 16122.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 19854ns | 18433ns | 17639ns | 18213ns | 23424ns | base |
| carrier_vert_leaf_vert4 | 9776ns | 9794ns | 9421ns | 9687ns | 10086ns | -50.76% |
| carrier_vert_leaf_vert8 | 9287ns | 9350ns | 8738ns | 9150ns | 9769ns | -53.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 17361ns | 15452ns | 20467ns | base | 0.004 |
| carrier_vert_leaf_vert4 | 7339ns | 7094ns | 7583ns | -57.73% | 0.009 |
| carrier_vert_leaf_vert8 | 6754ns | 6336ns | 7089ns | -61.09% | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.004 | 39.3% |
| carrier_vert_leaf_vert4 | 0.009 | 86.4% |
| carrier_vert_leaf_vert8 | 0.009 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 19854ns | 19854ns | base |
| carrier_vert_leaf_vert4 | 9776ns | 9776ns | -50.76% |
| carrier_vert_leaf_vert8 | 9287ns | 9287ns | -53.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 16122ns | base | --- | [15493, 20467] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 7336ns | -8782.8ns (-54.5%) | [-13131, -8152]ns | [7097, 7583] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 6788ns | -9397.4ns (-58.3%) | [-13679, -8742]ns | [6387, 7089] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 15534ns | -54.3% | -58.6% |
| 2 | 16682ns | -54.6% | -58.0% |
| 3 | 15563ns | -54.4% | -53.9% |
| 4 | 15452ns | -50.9% | -59.0% |
| 5 | 18093ns | -58.4% | -61.8% |
| 6 | 22841ns | -68.7% | -70.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | 0.206 | moderate+ |
| carrier_vert_leaf_vert4 | -0.535 | HIGH- (thermal bounce) |
| carrier_vert_leaf_vert8 | -0.418 | moderate- |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 96434.1ns | 17360.7ns | 555.5% | HIGH |
| carrier_vert_leaf_vert4 | 90098.1ns | 7338.7ns | 1227.7% | HIGH |
| carrier_vert_leaf_vert8 | 90442.8ns | 6754.4ns | 1339.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 15451.7-20466.8 ns)
  15451.7 |########################################
  15702.5 |
  15953.2 |
  16204.0 |
  16454.7 |#############
  16705.5 |
  16956.2 |
  17207.0 |
  17457.8 |
  17708.5 |
  17959.3 |#############
  18210.0 |
  18460.8 |
  18711.5 |
  18962.3 |
  19213.1 |
  19463.8 |
  19714.6 |
  19965.3 |
  20216.1 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 7094.2-7582.7 ns)
   7094.2 |########################################
   7118.6 |
   7143.1 |####################
   7167.5 |
   7191.9 |
   7216.3 |
   7240.8 |
   7265.2 |
   7289.6 |
   7314.0 |
   7338.4 |
   7362.9 |
   7387.3 |
   7411.7 |
   7436.1 |
   7460.6 |
   7485.0 |
   7509.4 |####################
   7533.8 |
   7558.3 |####################
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 6336.2-7088.8 ns)
   6336.2 |########################################
   6373.8 |
   6411.5 |########################################
   6449.1 |
   6486.7 |
   6524.3 |
   6562.0 |
   6599.6 |
   6637.2 |########################################
   6674.8 |
   6712.5 |
   6750.1 |
   6787.7 |
   6825.4 |
   6863.0 |
   6900.6 |########################################
   6938.2 |
   6975.9 |########################################
   7013.5 |
   7051.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=593.8% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=1228.6% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=1329.9% of algo (FFI overhead may distort results)

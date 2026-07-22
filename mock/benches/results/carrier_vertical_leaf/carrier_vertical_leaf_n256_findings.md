# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_leaf_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_leaf_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_leaf_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_leaf_scalar has the worst median (71.03 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_leaf_vert8 at 27.95 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_leaf_vert8 beats baseline by 62% (significant)

carrier_vert_leaf_vert8 is -43.77 us (62%) faster than baseline carrier_vert_leaf_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_leaf_scalar is an outlier: 2.5x slower than the field

carrier_vert_leaf_scalar (71.03 us) is 2.5x the fastest (27.95 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_leaf_vert8 is fastest but the noisiest (CV 6.6%)

carrier_vert_leaf_vert8 wins on median (27.95 us) yet has the highest variance (CV 6.6%), while carrier_vert_leaf_scalar is the steadiest (CV 2.3%, 71.03 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_vert_leaf_vert8** at 27949.4 ns median (-60.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.54x (fastest 27949.4 ns, slowest 71027.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 73502ns | 73547ns | 70580ns | 73218ns | 75389ns | base |
| carrier_vert_leaf_vert4 | 32411ns | 32266ns | 31160ns | 32009ns | 33639ns | -55.91% |
| carrier_vert_leaf_vert8 | 30524ns | 30429ns | 28251ns | 29792ns | 32759ns | -58.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 70973ns | 68368ns | 72716ns | base | 0.004 |
| carrier_vert_leaf_vert4 | 30032ns | 28870ns | 31163ns | -57.69% | 0.009 |
| carrier_vert_leaf_vert8 | 27994ns | 25828ns | 30095ns | -60.56% | 0.009 |

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_leaf_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_leaf_scalar | 0.004 | 36.4% |
| carrier_vert_leaf_vert4 | 0.009 | 86.4% |
| carrier_vert_leaf_vert8 | 0.009 | 92.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_leaf_scalar | 73502ns | 73502ns | base |
| carrier_vert_leaf_vert4 | 32411ns | 32411ns | -55.91% |
| carrier_vert_leaf_vert8 | 30524ns | 30524ns | -58.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_leaf_scalar | 71028ns | base | --- | [69177, 72716] | --- | --- | --- | --- |
| carrier_vert_leaf_vert4 | 29901ns | -41518.7ns (-58.5%) | [-43223, -38083]ns | [29031, 31163] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_leaf_vert8 | 27949ns | -43773.5ns (-61.6%) | [-44762, -40403]ns | [25936, 30095] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_leaf_scalar | carrier_vert_leaf_vert4 | carrier_vert_leaf_vert8 |
|---|---|---|---|
| 1 | 71931ns | -58.9% | -56.4% |
| 2 | 72975ns | -60.4% | -62.0% |
| 3 | 69986ns | -58.3% | -62.8% |
| 4 | 68368ns | -54.4% | -58.8% |
| 5 | 70124ns | -55.6% | -63.2% |
| 6 | 72456ns | -58.3% | -60.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_leaf_scalar | 0.222 | moderate+ |
| carrier_vert_leaf_vert4 | 0.414 | moderate+ |
| carrier_vert_leaf_vert8 | -0.139 | ok |

**Consistency summary:**

- **carrier_vert_leaf_vert4**: won 6/6, lost 0/6
- **carrier_vert_leaf_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_leaf_scalar | 141716.0ns | 70973.4ns | 199.7% | HIGH |
| carrier_vert_leaf_vert4 | 95875.6ns | 30031.7ns | 319.2% | HIGH |
| carrier_vert_leaf_vert8 | 102879.8ns | 27993.8ns | 367.5% | HIGH |

## Distribution (algo ns)

```
carrier_vert_leaf_scalar (n=6, range 68367.9-72715.6 ns)
  68367.9 |########################################
  68585.3 |
  68802.7 |
  69020.1 |
  69237.4 |
  69454.8 |
  69672.2 |
  69889.6 |########################################
  70107.0 |########################################
  70324.4 |
  70541.8 |
  70759.1 |
  70976.5 |
  71193.9 |
  71411.3 |
  71628.7 |
  71846.1 |########################################
  72063.4 |
  72280.8 |########################################
  72498.2 |
  (0 below, 1 above range)

carrier_vert_leaf_vert4 (n=6, range 28870.0-31162.7 ns)
  28870.0 |########################################
  28984.6 |
  29099.3 |########################################
  29213.9 |
  29328.5 |
  29443.2 |
  29557.8 |########################################
  29672.4 |
  29787.1 |
  29901.7 |
  30016.3 |
  30131.0 |########################################
  30245.6 |
  30360.3 |
  30474.9 |
  30589.5 |
  30704.2 |
  30818.8 |
  30933.4 |
  31048.1 |########################################
  (0 below, 1 above range)

carrier_vert_leaf_vert8 (n=6, range 25827.9-30095.4 ns)
  25827.9 |########################################
  26041.3 |########################################
  26254.7 |
  26468.0 |
  26681.4 |
  26894.8 |
  27108.2 |
  27321.5 |
  27534.9 |########################################
  27748.3 |
  27961.7 |########################################
  28175.0 |
  28388.4 |
  28601.8 |
  28815.2 |########################################
  29028.5 |
  29241.9 |
  29455.3 |
  29668.7 |
  29882.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_leaf_scalar**: bridge=199.7% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert4**: bridge=312.5% of algo (FFI overhead may distort results)
- **carrier_vert_leaf_vert8**: bridge=372.2% of algo (FFI overhead may distort results)

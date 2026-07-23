# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (3.02 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 751.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 dominates: 54% faster than the next best (carrier_vert_wideselect_vert4)

carrier_vert_wideselect_vert8 (751.70 us) leads carrier_vert_wideselect_vert4 (1.16 ms) by 54%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_wideselect_vert8 beats baseline by 75% (significant)

carrier_vert_wideselect_vert8 is -2.26 ms (75%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 4.0x slower than the field

carrier_vert_wideselect_scalar (3.02 ms) is 4.0x the fastest (751.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.0x the fastest

Fastest carrier_vert_wideselect_vert8 (751.70 us) to slowest carrier_vert_wideselect_scalar (3.02 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 751700.0 ns median (-75.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.02x (fastest 751700.0 ns, slowest 3022340.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3011932ns | 3025196ns | 2967473ns | 3007696ns | 3040515ns | base |
| carrier_vert_wideselect_vert4 | 1166177ns | 1162158ns | 1153510ns | 1159796ns | 1182082ns | -61.28% |
| carrier_vert_wideselect_vert8 | 762258ns | 754871ns | 735721ns | 751547ns | 791591ns | -74.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3008950ns | 2964592ns | 3037372ns | base | 0.001 |
| carrier_vert_wideselect_vert4 | 1163092ns | 1150156ns | 1179044ns | -61.35% | 0.004 |
| carrier_vert_wideselect_vert8 | 759062ns | 732170ns | 788507ns | -74.77% | 0.005 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 18701714 | 21208534 | 0.882 | 1.00× |
| carrier_vert_wideselect_vert4 | 7189864 | 8584277 | 0.838 | 0.38× |
| carrier_vert_wideselect_vert8 | 4742909 | 6085642 | 0.779 | 0.25× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.001 | 24.2% |
| carrier_vert_wideselect_vert4 | 0.004 | 63.2% |
| carrier_vert_wideselect_vert8 | 0.005 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 3011932ns | 3011932ns | base |
| carrier_vert_wideselect_vert4 | 1166177ns | 1166177ns | -61.28% |
| carrier_vert_wideselect_vert8 | 762258ns | 762258ns | -74.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3022341ns | base | --- | [2967138, 3037372] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 1159015ns | -1858537.4ns (-61.5%) | [-1886154, -1792882]ns | [1151218, 1179044] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 751700ns | -2255684.8ns (-74.6%) | [-2298202, -2195777]ns | [736979, 788507] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 2969682ns | -60.0% | -74.7% |
| 2 | 2964592ns | -60.8% | -73.3% |
| 3 | 3028963ns | -61.3% | -75.5% |
| 4 | 3033344ns | -62.0% | -75.2% |
| 5 | 3041400ns | -62.2% | -75.9% |
| 6 | 3015718ns | -61.7% | -73.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.420 | moderate+ |
| carrier_vert_wideselect_vert4 | 0.092 | ok |
| carrier_vert_wideselect_vert8 | -0.406 | moderate- |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 3023831.1ns | 3008950.0ns | 100.5% | HIGH |
| carrier_vert_wideselect_vert4 | 1166487.0ns | 1163092.2ns | 100.3% | HIGH |
| carrier_vert_wideselect_vert8 | 784915.9ns | 759062.1ns | 103.4% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 2964592.5-3037371.9 ns)
  2964592.5 |########################################
  2968231.5 |########################################
  2971870.4 |
  2975509.4 |
  2979148.4 |
  2982787.4 |
  2986426.3 |
  2990065.3 |
  2993704.3 |
  2997343.2 |
  3000982.2 |
  3004621.2 |
  3008260.1 |
  3011899.1 |
  3015538.1 |########################################
  3019177.1 |
  3022816.0 |
  3026455.0 |########################################
  3030094.0 |########################################
  3033732.9 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 1150155.8-1179044.2 ns)
  1150155.8 |########################################
  1151600.2 |########################################
  1153044.6 |
  1154489.1 |
  1155933.5 |########################################
  1157377.9 |
  1158822.3 |
  1160266.7 |########################################
  1161711.2 |
  1163155.6 |
  1164600.0 |
  1166044.4 |
  1167488.8 |
  1168933.3 |
  1170377.7 |########################################
  1171822.1 |
  1173266.5 |
  1174710.9 |
  1176155.4 |
  1177599.8 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 732169.6-788506.9 ns)
  732169.6 |########################################
  734986.5 |
  737803.3 |
  740620.2 |########################################
  743437.1 |
  746253.9 |
  749070.8 |########################################
  751887.6 |########################################
  754704.5 |
  757521.4 |
  760338.2 |
  763155.1 |
  765972.0 |
  768788.8 |
  771605.7 |
  774422.5 |
  777239.4 |
  780056.3 |
  782873.1 |
  785690.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=99.6% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=103.5% of algo (FFI overhead may distort results)

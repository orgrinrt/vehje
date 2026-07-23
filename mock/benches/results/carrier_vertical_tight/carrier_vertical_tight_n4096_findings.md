# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (1.20 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 519.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 26% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (519.58 us) leads carrier_vert_tight_vert4 (653.47 us) by 26%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 57% (significant)

carrier_vert_tight_vert8 is -683.94 us (57%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.3x slower than the field

carrier_vert_tight_scalar (1.20 ms) is 2.3x the fastest (519.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_tight_scalar shows alternating (throttle bounce) (autocorr -0.66)

carrier_vert_tight_scalar's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 519584.8 ns median (-56.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.31x (fastest 519584.8 ns, slowest 1200709.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 1205630ns | 1203161ns | 1196022ns | 1201152ns | 1217152ns | base |
| carrier_vert_tight_vert4 | 654697ns | 655911ns | 645988ns | 654592ns | 659209ns | -45.70% |
| carrier_vert_tight_vert8 | 522856ns | 522869ns | 519637ns | 521797ns | 526053ns | -56.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 1203111ns | 1193692ns | 1214391ns | base | 0.003 |
| carrier_vert_tight_vert4 | 652066ns | 643376ns | 656450ns | -45.80% | 0.006 |
| carrier_vert_tight_vert8 | 519779ns | 516470ns | 523182ns | -56.80% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 7552974 | 21171043 | 0.357 | 1.00× |
| carrier_vert_tight_vert4 | 4044341 | 7953743 | 0.508 | 0.54× |
| carrier_vert_tight_vert8 | 3262795 | 5601442 | 0.582 | 0.43× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.003 | 43.0% |
| carrier_vert_tight_vert4 | 0.006 | 79.0% |
| carrier_vert_tight_vert8 | 0.008 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 1205630ns | 1205630ns | base |
| carrier_vert_tight_vert4 | 654697ns | 654697ns | -45.70% |
| carrier_vert_tight_vert8 | 522856ns | 522856ns | -56.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 1200709ns | base | --- | [1194232, 1214391] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 653466ns | -548994.6ns (-45.7%) | [-565680, -538460]ns | [646281, 656450] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 519585ns | -683936.4ns (-57.0%) | [-694206, -671851]ns | [516571, 523182] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 1199880ns | -45.5% | -57.0% |
| 2 | 1206078ns | -46.7% | -56.8% |
| 3 | 1193692ns | -44.8% | -56.0% |
| 4 | 1222704ns | -46.5% | -57.5% |
| 5 | 1194772ns | -45.4% | -56.5% |
| 6 | 1201538ns | -46.0% | -57.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | -0.660 | HIGH- (thermal bounce) |
| carrier_vert_tight_vert4 | -0.468 | moderate- |
| carrier_vert_tight_vert8 | -0.019 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 1225548.8ns | 1203110.7ns | 101.9% | HIGH |
| carrier_vert_tight_vert4 | 656443.0ns | 652065.8ns | 100.7% | HIGH |
| carrier_vert_tight_vert8 | 524828.7ns | 519779.2ns | 101.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 1193692.1-1214390.9 ns)
  1193692.1 |########################################
  1194727.0 |########################################
  1195762.0 |
  1196796.9 |
  1197831.9 |
  1198866.8 |########################################
  1199901.7 |
  1200936.7 |########################################
  1201971.6 |
  1203006.5 |
  1204041.5 |
  1205076.4 |########################################
  1206111.4 |
  1207146.3 |
  1208181.2 |
  1209216.2 |
  1210251.1 |
  1211286.0 |
  1212321.0 |
  1213355.9 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 643376.2-656449.8 ns)
  643376.2 |####################
  644029.9 |
  644683.6 |
  645337.2 |
  645990.9 |
  646644.6 |
  647298.3 |
  647952.0 |
  648605.6 |####################
  649259.3 |
  649913.0 |
  650566.7 |
  651220.4 |
  651874.0 |
  652527.7 |####################
  653181.4 |
  653835.1 |########################################
  654488.8 |
  655142.4 |
  655796.1 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 516470.4-523181.5 ns)
  516470.4 |########################################
  516806.0 |
  517141.5 |
  517477.1 |
  517812.6 |
  518148.2 |
  518483.7 |
  518819.3 |
  519154.8 |####################
  519490.4 |
  519826.0 |####################
  520161.5 |
  520497.1 |
  520832.6 |
  521168.2 |
  521503.7 |####################
  521839.3 |
  522174.8 |
  522510.4 |
  522845.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=101.7% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=100.9% of algo (FFI overhead may distort results)

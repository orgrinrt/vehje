# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (13.86 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 3.23 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 dominates: 47% faster than the next best (carrier_vert_wideselect_vert4)

carrier_vert_wideselect_vert8 (3.23 ms) leads carrier_vert_wideselect_vert4 (4.73 ms) by 47%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_wideselect_vert8 beats baseline by 77% (significant)

carrier_vert_wideselect_vert8 is -10.63 ms (77%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 4.3x slower than the field

carrier_vert_wideselect_scalar (13.86 ms) is 4.3x the fastest (3.23 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_wideselect_vert4 shows alternating (throttle bounce) (autocorr -0.51)

carrier_vert_wideselect_vert4's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest carrier_vert_wideselect_vert8 (3.23 ms) to slowest carrier_vert_wideselect_scalar (13.86 ms): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 3229418.3 ns median (-76.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.29x (fastest 3229418.3 ns, slowest 13862386.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 13874972ns | 13866509ns | 13841180ns | 13858374ns | 13916764ns | base |
| carrier_vert_wideselect_vert4 | 4739316ns | 4737992ns | 4719882ns | 4734946ns | 4755588ns | -65.84% |
| carrier_vert_wideselect_vert8 | 3235672ns | 3232416ns | 3209730ns | 3226802ns | 3261946ns | -76.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 13870836ns | 13837047ns | 13912580ns | base | 0.001 |
| carrier_vert_wideselect_vert4 | 4735930ns | 4716707ns | 4752109ns | -65.86% | 0.003 |
| carrier_vert_wideselect_vert8 | 3232627ns | 3206875ns | 3258590ns | -76.69% | 0.005 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 85867583 | 84729402 | 1.013 | 1.00× |
| carrier_vert_wideselect_vert4 | 29367782 | 34248434 | 0.857 | 0.34× |
| carrier_vert_wideselect_vert8 | 20275481 | 24238304 | 0.837 | 0.24× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.001 | 23.1% |
| carrier_vert_wideselect_vert4 | 0.003 | 67.7% |
| carrier_vert_wideselect_vert8 | 0.005 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 13874972ns | 13874972ns | base |
| carrier_vert_wideselect_vert4 | 4739316ns | 4739316ns | -65.84% |
| carrier_vert_wideselect_vert8 | 3235672ns | 3235672ns | -76.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 13862386ns | base | --- | [13837543, 13912580] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 4734493ns | -9139436.1ns (-65.9%) | [-9179850, -9085434]ns | [4721187, 4752109] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 3229418ns | -10632482.1ns (-76.7%) | [-10684421, -10597725]ns | [3209874, 3258590] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 13894467ns | -65.9% | -76.9% |
| 2 | 13837047ns | -65.7% | -76.7% |
| 3 | 13930694ns | -66.1% | -76.7% |
| 4 | 13855688ns | -65.9% | -76.4% |
| 5 | 13838039ns | -65.6% | -76.8% |
| 6 | 13869085ns | -66.0% | -76.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | -0.481 | moderate- |
| carrier_vert_wideselect_vert4 | -0.510 | HIGH- (thermal bounce) |
| carrier_vert_wideselect_vert8 | -0.004 | ok |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 13931466.2ns | 13870836.5ns | 100.4% | HIGH |
| carrier_vert_wideselect_vert4 | 4735068.8ns | 4735929.7ns | 100.0% | HIGH |
| carrier_vert_wideselect_vert8 | 3296259.6ns | 3232627.4ns | 102.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 13837046.7-13912580.2 ns)
  13837046.7 |########################################
  13840823.4 |
  13844600.1 |
  13848376.7 |
  13852153.4 |####################
  13855930.1 |
  13859706.8 |
  13863483.4 |
  13867260.1 |####################
  13871036.8 |
  13874813.5 |
  13878590.2 |
  13882366.8 |
  13886143.5 |
  13889920.2 |
  13893696.9 |####################
  13897473.5 |
  13901250.2 |
  13905026.9 |
  13908803.6 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 4716707.1-4752109.0 ns)
  4716707.1 |########################################
  4718477.2 |
  4720247.3 |
  4722017.4 |
  4723787.5 |
  4725557.6 |########################################
  4727327.7 |
  4729097.7 |########################################
  4730867.8 |
  4732637.9 |
  4734408.0 |
  4736178.1 |
  4737948.2 |
  4739718.3 |########################################
  4741488.4 |
  4743258.5 |
  4745028.6 |########################################
  4746798.7 |
  4748568.8 |
  4750338.9 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 3206874.6-3258589.8 ns)
  3206874.6 |####################
  3209460.4 |
  3212046.1 |####################
  3214631.9 |
  3217217.6 |
  3219803.4 |
  3222389.2 |
  3224974.9 |
  3227560.7 |########################################
  3230146.4 |
  3232732.2 |
  3235318.0 |
  3237903.7 |
  3240489.5 |
  3243075.2 |
  3245661.0 |
  3248246.8 |####################
  3250832.5 |
  3253418.3 |
  3256004.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=102.0% of algo (FFI overhead may distort results)

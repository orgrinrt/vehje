# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_wideselect_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_wideselect_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_wideselect_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_wideselect_scalar has the worst median (279.66 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_wideselect_vert8 at 119.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_wideselect_vert8 dominates: 51% faster than the next best (carrier_vert_wideselect_vert4)

carrier_vert_wideselect_vert8 (119.43 us) leads carrier_vert_wideselect_vert4 (180.53 us) by 51%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_wideselect_vert8 beats baseline by 57% (significant)

carrier_vert_wideselect_vert8 is -158.95 us (57%) faster than baseline carrier_vert_wideselect_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_wideselect_scalar is an outlier: 2.3x slower than the field

carrier_vert_wideselect_scalar (279.66 us) is 2.3x the fastest (119.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_wideselect_scalar shows alternating (throttle bounce) (autocorr -0.67)

carrier_vert_wideselect_scalar's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_wideselect_vert8** at 119425.2 ns median (-57.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.34x (fastest 119425.2 ns, slowest 279660.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 283231ns | 282117ns | 277875ns | 281431ns | 288610ns | base |
| carrier_vert_wideselect_vert4 | 183095ns | 183014ns | 180919ns | 182551ns | 184998ns | -35.35% |
| carrier_vert_wideselect_vert8 | 122194ns | 121966ns | 118309ns | 121327ns | 125437ns | -56.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 280790ns | 275011ns | 286306ns | base | 0.004 |
| carrier_vert_wideselect_vert4 | 180483ns | 178385ns | 182304ns | -35.72% | 0.006 |
| carrier_vert_wideselect_vert8 | 119687ns | 116046ns | 122874ns | -57.37% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 1752430 | 5311074 | 0.330 | 1.00× |
| carrier_vert_wideselect_vert4 | 1182376 | 2160428 | 0.547 | 0.67× |
| carrier_vert_wideselect_vert8 | 744722 | 1535041 | 0.485 | 0.42× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_wideselect_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_wideselect_scalar | 0.004 | 41.5% |
| carrier_vert_wideselect_vert4 | 0.006 | 64.3% |
| carrier_vert_wideselect_vert8 | 0.009 | 97.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_wideselect_scalar | 283231ns | 283231ns | base |
| carrier_vert_wideselect_vert4 | 183095ns | 183095ns | -35.35% |
| carrier_vert_wideselect_vert8 | 122194ns | 122194ns | -56.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 279661ns | base | --- | [276404, 286306] | --- | --- | --- | --- |
| carrier_vert_wideselect_vert4 | 180534ns | -99906.9ns (-35.7%) | [-105871, -95144]ns | [178611, 182304] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_wideselect_vert8 | 119425ns | -158945.2ns (-56.8%) | [-167577, -156786]ns | [116763, 122874] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_wideselect_scalar | carrier_vert_wideselect_vert4 | carrier_vert_wideselect_vert8 |
|---|---|---|---|
| 1 | 278621ns | -34.3% | -55.7% |
| 2 | 280700ns | -36.5% | -56.5% |
| 3 | 282422ns | -35.7% | -58.4% |
| 4 | 275011ns | -34.4% | -57.8% |
| 5 | 290190ns | -37.7% | -58.7% |
| 6 | 277797ns | -35.6% | -57.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_wideselect_scalar | -0.665 | HIGH- (thermal bounce) |
| carrier_vert_wideselect_vert4 | -0.559 | HIGH- (thermal bounce) |
| carrier_vert_wideselect_vert8 | 0.272 | moderate+ |

**Consistency summary:**

- **carrier_vert_wideselect_vert4**: won 6/6, lost 0/6
- **carrier_vert_wideselect_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_wideselect_scalar | 285935.9ns | 280790.3ns | 101.8% | HIGH |
| carrier_vert_wideselect_vert4 | 197519.8ns | 180483.1ns | 109.4% | HIGH |
| carrier_vert_wideselect_vert8 | 120941.0ns | 119687.3ns | 101.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_wideselect_scalar (n=6, range 275011.2-286306.2 ns)
  275011.2 |########################################
  275576.0 |
  276140.7 |
  276705.5 |
  277270.2 |########################################
  277835.0 |
  278399.7 |########################################
  278964.5 |
  279529.2 |
  280094.0 |
  280658.7 |########################################
  281223.5 |
  281788.2 |
  282353.0 |########################################
  282917.7 |
  283482.5 |
  284047.2 |
  284612.0 |
  285176.7 |
  285741.5 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert4 (n=6, range 178385.0-182304.3 ns)
  178385.0 |########################################
  178581.0 |
  178776.9 |########################################
  178972.9 |
  179168.9 |
  179364.8 |
  179560.8 |
  179756.8 |
  179952.7 |
  180148.7 |########################################
  180344.7 |
  180540.6 |
  180736.6 |########################################
  180932.6 |
  181128.5 |
  181324.5 |
  181520.5 |########################################
  181716.4 |
  181912.4 |
  182108.4 |
  (0 below, 1 above range)

carrier_vert_wideselect_vert8 (n=6, range 116046.2-122874.1 ns)
  116046.2 |########################################
  116387.6 |
  116729.0 |
  117070.4 |
  117411.8 |########################################
  117753.2 |
  118094.6 |
  118436.0 |
  118777.4 |########################################
  119118.8 |
  119460.2 |
  119801.6 |########################################
  120143.0 |
  120484.4 |
  120825.8 |
  121167.2 |
  121508.6 |
  121850.0 |########################################
  122191.4 |
  122532.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_wideselect_scalar**: bridge=101.9% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert4**: bridge=109.0% of algo (FFI overhead may distort results)
- **carrier_vert_wideselect_vert8**: bridge=101.1% of algo (FFI overhead may distort results)

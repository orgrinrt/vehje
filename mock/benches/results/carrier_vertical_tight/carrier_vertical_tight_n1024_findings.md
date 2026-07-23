# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (319.91 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 122.73 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 33% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (122.73 us) leads carrier_vert_tight_vert4 (162.79 us) by 33%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 62% (significant)

carrier_vert_tight_vert8 is -197.69 us (62%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.6x slower than the field

carrier_vert_tight_scalar (319.91 us) is 2.6x the fastest (122.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_tight_scalar shows alternating (throttle bounce) (autocorr -0.70)

carrier_vert_tight_scalar's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 122727.8 ns median (-61.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.61x (fastest 122727.8 ns, slowest 319911.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 323318ns | 322251ns | 316138ns | 320742ns | 330772ns | base |
| carrier_vert_tight_vert4 | 165763ns | 165382ns | 163575ns | 165057ns | 167915ns | -48.73% |
| carrier_vert_tight_vert8 | 125410ns | 125056ns | 123947ns | 124814ns | 127035ns | -61.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 320994ns | 313960ns | 328338ns | base | 0.003 |
| carrier_vert_tight_vert4 | 163160ns | 161086ns | 165404ns | -49.17% | 0.006 |
| carrier_vert_tight_vert8 | 123128ns | 121636ns | 124776ns | -61.64% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 1995130 | 5309235 | 0.376 | 1.00× |
| carrier_vert_tight_vert4 | 1022521 | 2036217 | 0.502 | 0.51× |
| carrier_vert_tight_vert8 | 775422 | 1448121 | 0.535 | 0.39× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.003 | 38.0% |
| carrier_vert_tight_vert4 | 0.006 | 74.7% |
| carrier_vert_tight_vert8 | 0.008 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 323318ns | 323318ns | base |
| carrier_vert_tight_vert4 | 165763ns | 165763ns | -48.73% |
| carrier_vert_tight_vert8 | 125410ns | 125410ns | -61.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 319912ns | base | --- | [314731, 328338] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 162794ns | -157858.1ns (-49.3%) | [-164874, -150770]ns | [161281, 165404] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 122728ns | -197690.6ns (-61.8%) | [-204212, -191693]ns | [121882, 124776] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 318181ns | -49.1% | -61.8% |
| 2 | 321642ns | -49.8% | -61.8% |
| 3 | 323499ns | -49.3% | -62.1% |
| 4 | 313960ns | -46.9% | -61.1% |
| 5 | 333178ns | -50.9% | -62.3% |
| 6 | 315502ns | -48.9% | -60.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | -0.702 | HIGH- (thermal bounce) |
| carrier_vert_tight_vert4 | 0.188 | ok |
| carrier_vert_tight_vert8 | 0.064 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 323887.1ns | 320993.7ns | 100.9% | HIGH |
| carrier_vert_tight_vert4 | 163731.8ns | 163159.5ns | 100.4% | HIGH |
| carrier_vert_tight_vert8 | 124419.3ns | 123128.4ns | 101.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 313960.0-328338.3 ns)
  313960.0 |########################################
  314678.9 |
  315397.8 |########################################
  316116.8 |
  316835.7 |
  317554.6 |########################################
  318273.5 |
  318992.4 |
  319711.3 |
  320430.3 |
  321149.2 |########################################
  321868.1 |
  322587.0 |
  323305.9 |########################################
  324024.8 |
  324743.8 |
  325462.7 |
  326181.6 |
  326900.5 |
  327619.4 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 161086.2-165403.5 ns)
  161086.2 |########################################
  161302.1 |########################################
  161517.9 |
  161733.8 |
  161949.7 |########################################
  162165.5 |
  162381.4 |
  162597.3 |
  162813.1 |
  163029.0 |
  163244.9 |
  163460.7 |########################################
  163676.6 |
  163892.5 |########################################
  164108.3 |
  164324.2 |
  164540.1 |
  164755.9 |
  164971.8 |
  165187.7 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 121635.8-124775.8 ns)
  121635.8 |########################################
  121792.8 |
  121949.8 |
  122106.8 |########################################
  122263.8 |
  122420.8 |
  122577.8 |########################################
  122734.8 |########################################
  122891.8 |
  123048.8 |
  123205.8 |
  123362.8 |
  123519.8 |
  123676.8 |
  123833.8 |########################################
  123990.8 |
  124147.8 |
  124304.8 |
  124461.8 |
  124618.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=100.9% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=101.2% of algo (FFI overhead may distort results)

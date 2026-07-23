# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), tight profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_tight_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_tight_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_tight_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_tight_scalar has the worst median (64.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_tight_vert8 at 30.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_tight_vert8 dominates: 26% faster than the next best (carrier_vert_tight_vert4)

carrier_vert_tight_vert8 (30.27 us) leads carrier_vert_tight_vert4 (38.29 us) by 26%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_tight_vert8 beats baseline by 53% (significant)

carrier_vert_tight_vert8 is -33.78 us (53%) faster than baseline carrier_vert_tight_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_tight_scalar is an outlier: 2.1x slower than the field

carrier_vert_tight_scalar (64.06 us) is 2.1x the fastest (30.27 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_tight_vert8** at 30274.0 ns median (-52.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.12x (fastest 30274.0 ns, slowest 64062.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 66942ns | 66438ns | 65405ns | 66373ns | 68563ns | base |
| carrier_vert_tight_vert4 | 40649ns | 40566ns | 40342ns | 40549ns | 40953ns | -39.28% |
| carrier_vert_tight_vert8 | 32799ns | 32570ns | 32434ns | 32528ns | 33389ns | -51.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 64575ns | 62988ns | 66219ns | base | 0.004 |
| carrier_vert_tight_vert4 | 38318ns | 37953ns | 38585ns | -40.66% | 0.007 |
| carrier_vert_tight_vert8 | 30452ns | 30053ns | 30930ns | -52.84% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 607050 | 1996029 | 0.304 | 1.00× |
| carrier_vert_tight_vert4 | 477551 | 1029274 | 0.464 | 0.79× |
| carrier_vert_tight_vert8 | 385786 | 738100 | 0.523 | 0.64× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_vert_tight_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_tight_scalar | 0.004 | 46.9% |
| carrier_vert_tight_vert4 | 0.007 | 78.5% |
| carrier_vert_tight_vert8 | 0.008 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_tight_scalar | 66942ns | 66942ns | base |
| carrier_vert_tight_vert4 | 40649ns | 40649ns | -39.28% |
| carrier_vert_tight_vert8 | 32799ns | 32799ns | -51.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_tight_scalar | 64062ns | base | --- | [63442, 66219] | --- | --- | --- | --- |
| carrier_vert_tight_vert4 | 38295ns | -25744.0ns (-40.2%) | [-27780, -25245]ns | [38075, 38585] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_tight_vert8 | 30274ns | -33783.1ns (-52.7%) | [-36050, -32534]ns | [30152, 30930] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_tight_scalar | carrier_vert_tight_vert4 | carrier_vert_tight_vert8 |
|---|---|---|---|
| 1 | 62988ns | -39.7% | -52.0% |
| 2 | 63896ns | -40.2% | -50.6% |
| 3 | 64048ns | -40.3% | -52.8% |
| 4 | 64700ns | -40.8% | -53.2% |
| 5 | 67739ns | -43.1% | -55.6% |
| 6 | 64077ns | -39.7% | -52.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_tight_scalar | 0.014 | ok |
| carrier_vert_tight_vert4 | 0.417 | moderate+ |
| carrier_vert_tight_vert8 | -0.181 | ok |

**Consistency summary:**

- **carrier_vert_tight_vert4**: won 6/6, lost 0/6
- **carrier_vert_tight_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_tight_scalar | 131063.0ns | 64574.5ns | 203.0% | HIGH |
| carrier_vert_tight_vert4 | 114578.5ns | 38318.2ns | 299.0% | HIGH |
| carrier_vert_tight_vert8 | 91614.1ns | 30452.2ns | 300.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_tight_scalar (n=6, range 62987.5-66219.4 ns)
  62987.5 |####################
  63149.1 |
  63310.7 |
  63472.3 |
  63633.9 |
  63795.5 |####################
  63957.1 |########################################
  64118.7 |
  64280.3 |
  64441.9 |
  64603.4 |####################
  64765.0 |
  64926.6 |
  65088.2 |
  65249.8 |
  65411.4 |
  65573.0 |
  65734.6 |
  65896.2 |
  66057.8 |
  (0 below, 1 above range)

carrier_vert_tight_vert4 (n=6, range 37952.9-38585.4 ns)
  37952.9 |########################################
  37984.5 |
  38016.2 |
  38047.8 |
  38079.4 |
  38111.0 |
  38142.7 |
  38174.3 |########################################
  38205.9 |
  38237.5 |########################################
  38269.2 |
  38300.8 |########################################
  38332.4 |
  38364.0 |
  38395.7 |
  38427.3 |
  38458.9 |
  38490.5 |
  38522.2 |########################################
  38553.8 |
  (0 below, 1 above range)

carrier_vert_tight_vert8 (n=6, range 30052.9-30930.2 ns)
  30052.9 |####################
  30096.8 |
  30140.6 |
  30184.5 |
  30228.4 |########################################
  30272.2 |########################################
  30316.1 |
  30360.0 |
  30403.8 |
  30447.7 |
  30491.6 |
  30535.4 |
  30579.3 |
  30623.1 |
  30667.0 |
  30710.9 |
  30754.7 |
  30798.6 |
  30842.5 |
  30886.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_tight_scalar**: bridge=203.3% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert4**: bridge=299.6% of algo (FFI overhead may distort results)
- **carrier_vert_tight_vert8**: bridge=300.7% of algo (FFI overhead may distort results)

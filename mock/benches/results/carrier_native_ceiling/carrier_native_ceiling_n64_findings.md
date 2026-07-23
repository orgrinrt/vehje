# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (175.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 109.20 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 61% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (109.20 us) leads carrier_ceil_interp (175.63 us) by 61%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 38% (significant)

carrier_ceil_native is -66.74 us (38%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 109196.7 ns median (-37.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.61x (fastest 109196.7 ns, slowest 175628.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 180563ns | 178016ns | 176257ns | 177726ns | 186972ns | base |
| carrier_ceil_native | 112226ns | 111543ns | 109940ns | 111041ns | 115146ns | -37.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 177932ns | 173954ns | 183760ns | base | 0.000 |
| carrier_ceil_native | 109779ns | 107526ns | 112519ns | -38.30% | 0.001 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ceil_interp | 1063910 | 4708486 | 0.226 | 1.00× |
| carrier_ceil_native | 657382 | 3472453 | 0.189 | 0.62× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.001 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 61.2% |
| carrier_ceil_native | 0.001 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 180563ns | 180563ns | base |
| carrier_ceil_native | 112226ns | 112226ns | -37.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 175629ns | base | --- | [174406, 183760] | --- | --- | --- | --- |
| carrier_ceil_native | 109197ns | -66743.8ns (-38.0%) | [-75285, -62429]ns | [107621, 112519] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 173954ns | -37.2% |
| 2 | 175894ns | -34.2% |
| 3 | 185175ns | -41.8% |
| 4 | 182346ns | -40.1% |
| 5 | 175363ns | -37.7% |
| 6 | 174858ns | -38.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | 0.203 | moderate+ |
| carrier_ceil_native | -0.285 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 178590.0ns | 177931.5ns | 100.4% | HIGH |
| carrier_ceil_native | 109284.2ns | 109778.8ns | 99.5% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 173954.2-183760.2 ns)
  173954.2 |########################################
  174444.5 |########################################
  174934.8 |########################################
  175425.1 |########################################
  175915.4 |
  176405.7 |
  176896.0 |
  177386.3 |
  177876.6 |
  178366.9 |
  178857.2 |
  179347.5 |
  179837.8 |
  180328.1 |
  180818.4 |
  181308.7 |
  181799.0 |
  182289.3 |########################################
  182779.6 |
  183269.9 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 107526.2-112518.6 ns)
  107526.2 |##########################
  107775.8 |
  108025.4 |
  108275.1 |
  108524.7 |
  108774.3 |
  109023.9 |########################################
  109273.5 |
  109523.1 |
  109772.8 |
  110022.4 |
  110272.0 |
  110521.6 |
  110771.2 |
  111020.8 |
  111270.5 |
  111520.1 |
  111769.7 |
  112019.3 |
  112268.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.7% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=99.6% of algo (FFI overhead may distort results)

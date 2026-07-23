# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (43.07 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 28.88 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 49% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (28.88 ms) leads carrier_ceil_interp (43.07 ms) by 49%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 33% (significant)

carrier_ceil_native is -14.18 ms (33%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 28883421.0 ns median (-32.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.49x (fastest 28883421.0 ns, slowest 43066212.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 43092664ns | 43070591ns | 43064922ns | 43069473ns | 43141321ns | base |
| carrier_ceil_native | 28905051ns | 28887762ns | 28845431ns | 28876035ns | 28978384ns | -32.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 43088233ns | 43060537ns | 43136846ns | base | 0.000 |
| carrier_ceil_native | 28900694ns | 28841155ns | 28973765ns | -32.93% | 0.000 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ceil_interp | 267802395 | 1176128238 | 0.228 | 1.00× |
| carrier_ceil_native | 179379007 | 873155847 | 0.205 | 0.67× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 67.0% |
| carrier_ceil_native | 0.000 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 43092664ns | 43092664ns | base |
| carrier_ceil_native | 28905051ns | 28905051ns | -32.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 43066213ns | base | --- | [43061640, 43136846] | --- | --- | --- | --- |
| carrier_ceil_native | 28883421ns | -14181946.2ns (-32.9%) | [-14245006, -14135665]ns | [28844895, 28973765] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 43117060ns | -33.1% |
| 2 | 43156632ns | -32.7% |
| 3 | 43069295ns | -32.9% |
| 4 | 43060537ns | -32.9% |
| 5 | 43062742ns | -33.0% |
| 6 | 43063131ns | -32.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | 0.322 | moderate+ |
| carrier_ceil_native | -0.275 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 43044620.6ns | 43088232.9ns | 99.9% | HIGH |
| carrier_ceil_native | 28898174.6ns | 28900693.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 43060537.1-43136846.2 ns)
  43060537.1 |########################################
  43064352.6 |
  43068168.0 |#############
  43071983.5 |
  43075798.9 |
  43079614.4 |
  43083429.8 |
  43087245.3 |
  43091060.8 |
  43094876.2 |
  43098691.7 |
  43102507.1 |
  43106322.6 |
  43110138.0 |
  43113953.5 |#############
  43117769.0 |
  43121584.4 |
  43125399.9 |
  43129215.3 |
  43133030.8 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 28841154.6-28973765.4 ns)
  28841154.6 |####################
  28847785.1 |####################
  28854415.7 |
  28861046.2 |
  28867676.8 |
  28874307.3 |####################
  28880937.8 |
  28887568.4 |########################################
  28894198.9 |
  28900829.5 |
  28907460.0 |
  28914090.5 |
  28920721.1 |
  28927351.6 |
  28933982.2 |
  28940612.7 |
  28947243.2 |
  28953873.8 |
  28960504.3 |
  28967134.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.0% of algo (FFI overhead may distort results)

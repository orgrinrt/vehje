# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (42.48 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 28.39 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 50% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (28.39 ms) leads carrier_ceil_interp (42.48 ms) by 50%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 33% (significant)

carrier_ceil_native is -14.01 ms (33%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 28389917.1 ns median (-33.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.50x (fastest 28389917.1 ns, slowest 42476070.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 42568913ns | 42480527ns | 42332505ns | 42434593ns | 42888598ns | base |
| carrier_ceil_native | 28520804ns | 28394318ns | 28285656ns | 28369851ns | 28864807ns | -33.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 42564395ns | 42327988ns | 42884028ns | base | 0.000 |
| carrier_ceil_native | 28516229ns | 28280300ns | 28860167ns | -33.00% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 66.6% |
| carrier_ceil_native | 0.000 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 42568913ns | 42568913ns | base |
| carrier_ceil_native | 28520804ns | 28520804ns | -33.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 42476070ns | base | --- | [42333086, 42884028] | --- | --- | --- | --- |
| carrier_ceil_native | 28389917ns | -14014149.3ns (-33.0%) | [-14200087, -13930262]ns | [28298602, 28860167] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 42959320ns | -32.9% |
| 2 | 42808737ns | -32.5% |
| 3 | 42327988ns | -32.9% |
| 4 | 42389716ns | -33.0% |
| 5 | 42562424ns | -33.6% |
| 6 | 42338183ns | -33.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | 0.229 | moderate+ |
| carrier_ceil_native | 0.464 | moderate+ |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 42522706.0ns | 42564394.6ns | 99.9% | HIGH |
| carrier_ceil_native | 28530389.7ns | 28516228.6ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 42327987.9-42884028.2 ns)
  42327987.9 |########################################
  42355789.9 |
  42383591.9 |####################
  42411393.9 |
  42439196.0 |
  42466998.0 |
  42494800.0 |
  42522602.0 |
  42550404.0 |####################
  42578206.0 |
  42606008.0 |
  42633810.0 |
  42661612.1 |
  42689414.1 |
  42717216.1 |
  42745018.1 |
  42772820.1 |
  42800622.1 |####################
  42828424.1 |
  42856226.1 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 28280300.4-28860166.9 ns)
  28280300.4 |########################################
  28309293.7 |########################################
  28338287.0 |
  28367280.4 |########################################
  28396273.7 |########################################
  28425267.0 |
  28454260.3 |
  28483253.7 |
  28512247.0 |
  28541240.3 |
  28570233.6 |
  28599227.0 |
  28628220.3 |
  28657213.6 |
  28686206.9 |
  28715200.3 |
  28744193.6 |
  28773186.9 |
  28802180.2 |
  28831173.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.1% of algo (FFI overhead may distort results)

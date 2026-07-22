# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (42.48 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 28.41 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 50% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (28.41 ms) leads carrier_ceil_interp (42.48 ms) by 50%, a clear separation rather than a photo finish. CV 0.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 33% (significant)

carrier_ceil_native is -13.97 ms (33%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_ceil_native** at 28406378.6 ns median (-33.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.50x (fastest 28406378.6 ns, slowest 42480190.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 42894445ns | 42484364ns | 42294045ns | 42426350ns | 43896788ns | base |
| carrier_ceil_native | 28438009ns | 28411021ns | 28324323ns | 28387395ns | 28570772ns | -33.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 42889750ns | 42289198ns | 43892156ns | base | 0.000 |
| carrier_ceil_native | 28433837ns | 28320033ns | 28566752ns | -33.70% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 66.7% |
| carrier_ceil_native | 0.000 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 42894445ns | 42894445ns | base |
| carrier_ceil_native | 28438009ns | 28438009ns | -33.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 42480190ns | base | --- | [42296905, 43892156] | --- | --- | --- | --- |
| carrier_ceil_native | 28406379ns | -13965205.2ns (-32.9%) | [-15514826, -13887708]ns | [28328380, 28566752] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 42491451ns | -32.6% |
| 2 | 43051003ns | -34.2% |
| 3 | 42468929ns | -32.9% |
| 4 | 44733308ns | -36.5% |
| 5 | 42304611ns | -32.9% |
| 6 | 42289198ns | -33.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.366 | moderate- |
| carrier_ceil_native | -0.330 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 42572674.4ns | 42889750.1ns | 99.3% | HIGH |
| carrier_ceil_native | 28429409.2ns | 28433837.0ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 42289198.3-43892155.6 ns)
  42289198.3 |########################################
  42369346.2 |
  42449494.0 |########################################
  42529641.9 |
  42609789.8 |
  42689937.6 |
  42770085.5 |
  42850233.4 |
  42930381.2 |
  43010529.1 |####################
  43090676.9 |
  43170824.8 |
  43250972.7 |
  43331120.5 |
  43411268.4 |
  43491416.3 |
  43571564.1 |
  43651712.0 |
  43731859.9 |
  43812007.7 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 28320032.9-28566752.3 ns)
  28320032.9 |########################################
  28332368.9 |########################################
  28344704.8 |
  28357040.8 |
  28369376.8 |
  28381712.8 |
  28394048.7 |########################################
  28406384.7 |########################################
  28418720.7 |
  28431056.6 |
  28443392.6 |
  28455728.6 |
  28468064.5 |
  28480400.5 |
  28492736.5 |
  28505072.4 |########################################
  28517408.4 |
  28529744.4 |
  28542080.4 |
  28554416.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.0% of algo (FFI overhead may distort results)

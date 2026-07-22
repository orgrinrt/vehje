# Native ceiling: interpreter vs shape-specialized native madd loop (THROUGHPUT over a byte stream, O(N^2), not comparable to sibling per-execution numbers)

2 variants, 6 samples per variant.
Baseline: **carrier_ceil_interp**

## Highlights

Baseline for all deltas below: **carrier_ceil_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_ceil_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_ceil_interp has the worst median (43.17 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_ceil_native at 28.96 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_ceil_native dominates: 49% faster than the next best (carrier_ceil_interp)

carrier_ceil_native (28.96 ms) leads carrier_ceil_interp (43.17 ms) by 49%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceil_native beats baseline by 33% (significant)

carrier_ceil_native is -14.24 ms (33%) faster than baseline carrier_ceil_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ceil_interp shows alternating (throttle bounce) (autocorr -0.65)

carrier_ceil_interp's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_ceil_native** at 28958194.8 ns median (-32.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.49x (fastest 28958194.8 ns, slowest 43169200.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceil_interp | 43197114ns | 43174124ns | 43138197ns | 43168326ns | 43269755ns | base |
| carrier_ceil_native | 28978627ns | 28962479ns | 28887894ns | 28939830ns | 29082189ns | -32.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceil_interp | 43192181ns | 43133395ns | 43264781ns | base | 0.000 |
| carrier_ceil_native | 28974091ns | 28883553ns | 29077436ns | -32.92% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (carrier_ceil_native; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceil_interp | 0.000 | 66.9% |
| carrier_ceil_native | 0.000 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceil_interp | 43197114ns | 43197114ns | base |
| carrier_ceil_native | 28978627ns | 28978627ns | -32.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceil_interp | 43169200ns | base | --- | [43142561, 43264781] | --- | --- | --- | --- |
| carrier_ceil_native | 28958195ns | -14240346.3ns (-33.0%) | [-14341291, -14072632]ns | [28886642, 29077436] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceil_interp | carrier_ceil_native |
|---|---|---|
| 1 | 43171660ns | -32.9% |
| 2 | 43151727ns | -33.1% |
| 3 | 43305674ns | -33.1% |
| 4 | 43133395ns | -32.6% |
| 5 | 43223887ns | -33.2% |
| 6 | 43166740ns | -32.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceil_interp | -0.654 | HIGH- (thermal bounce) |
| carrier_ceil_native | -0.460 | moderate- |

**Consistency summary:**

- **carrier_ceil_native**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceil_interp | 43143351.7ns | 43192180.6ns | 99.9% | HIGH |
| carrier_ceil_native | 29172077.2ns | 28974090.8ns | 100.7% | HIGH |

## Distribution (algo ns)

```
carrier_ceil_interp (n=6, range 43133395.0-43264780.7 ns)
  43133395.0 |####################
  43139964.3 |
  43146533.6 |####################
  43153102.8 |
  43159672.1 |
  43166241.4 |########################################
  43172810.7 |
  43179380.0 |
  43185949.3 |
  43192518.5 |
  43199087.8 |
  43205657.1 |
  43212226.4 |
  43218795.7 |####################
  43225365.0 |
  43231934.2 |
  43238503.5 |
  43245072.8 |
  43251642.1 |
  43258211.4 |
  (0 below, 1 above range)

carrier_ceil_native (n=6, range 28883552.9-29077435.6 ns)
  28883552.9 |########################################
  28893247.0 |
  28902941.2 |
  28912635.3 |
  28922329.4 |
  28932023.6 |
  28941717.7 |
  28951411.8 |####################
  28961106.0 |####################
  28970800.1 |
  28980494.2 |
  28990188.4 |
  28999882.5 |
  29009576.7 |
  29019270.8 |
  29028964.9 |
  29038659.1 |
  29048353.2 |
  29058047.3 |####################
  29067741.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ceil_interp**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_ceil_native**: bridge=100.0% of algo (FFI overhead may distort results)

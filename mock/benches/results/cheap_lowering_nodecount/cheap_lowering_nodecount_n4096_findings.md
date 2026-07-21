# Cheap lowering node-count reduction: fold+CSE vs fold-only (metric = node count, not time)

2 variants, 6 samples per variant.
Baseline: **cl_foldonly**

## Highlights

Baseline for all deltas below: **cl_foldonly**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cl_foldonly dominates: 170% faster than the next best (cl_foldcse)

cl_foldonly (6.77 us) leads cl_foldcse (18.30 us) by 170%, a clear separation rather than a photo finish. CV 7.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cl_foldonly is fastest but the noisiest (CV 7.4%)

cl_foldonly wins on median (6.77 us) yet has the highest variance (CV 7.4%), while cl_foldcse is the steadiest (CV 2.3%, 18.30 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (cl_foldonly)

The baseline cl_foldonly is the fastest (6.77 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cl_foldonly) is the fastest** at 6770.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.70x (fastest 6770.2 ns, slowest 18298.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cl_foldcse | 20592ns | 20749ns | 19805ns | 20602ns | 20969ns | +126.20% |
| cl_foldonly | 9103ns | 9208ns | 7851ns | 9175ns | 9622ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cl_foldcse | 18126ns | 17348ns | 18484ns | +169.93% | 0.226 |
| cl_foldonly | 6715ns | 5716ns | 7181ns | base | 0.610 |

## Performance model

- Peak throughput: **0.717 Gops/s** (cl_foldonly; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cl_foldcse | 0.224 | 31.2% |
| cl_foldonly | 0.605 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cl_foldcse | 20592ns | 20592ns | +126.20% |
| cl_foldonly | 9103ns | 9103ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cl_foldonly | 6770ns | base | --- | [6194, 7181] | --- | --- | --- | --- |
| cl_foldcse | 18299ns | +11409.0ns (+168.5%) | [+10906, +11918]ns | [17597, 18484] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | cl_foldonly | cl_foldcse |
|---|---|---|
| 1 | 5716ns | +212.2% |
| 2 | 6984ns | +162.2% |
| 3 | 6745ns | +157.2% |
| 4 | 7378ns | +151.9% |
| 5 | 6795ns | +169.1% |
| 6 | 6673ns | +175.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cl_foldcse | -0.426 | moderate- |
| cl_foldonly | -0.126 | ok |

**Consistency summary:**

- **cl_foldcse**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cl_foldcse | 10808.8ns | 18126.3ns | 59.6% | HIGH |
| cl_foldonly | 6241.4ns | 6715.2ns | 92.9% | HIGH |

## Distribution (algo ns)

```
cl_foldcse (n=6, range 17348.3-18483.6 ns)
  17348.3 |####################
  17405.1 |
  17461.8 |
  17518.6 |
  17575.3 |
  17632.1 |
  17688.9 |
  17745.6 |
  17802.4 |####################
  17859.2 |
  17915.9 |
  17972.7 |
  18029.5 |
  18086.2 |
  18143.0 |
  18199.7 |
  18256.5 |########################################
  18313.3 |
  18370.0 |####################
  18426.8 |
  (0 below, 1 above range)

cl_foldonly (n=6, range 5715.8-7181.1 ns)
   5715.8 |####################
   5789.1 |
   5862.3 |
   5935.6 |
   6008.9 |
   6082.1 |
   6155.4 |
   6228.6 |
   6301.9 |
   6375.2 |
   6448.4 |
   6521.7 |
   6595.0 |
   6668.2 |####################
   6741.5 |########################################
   6814.7 |
   6888.0 |
   6961.3 |####################
   7034.5 |
   7107.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **cl_foldcse**: bridge=59.2% of algo (FFI overhead may distort results)
- **cl_foldonly**: bridge=94.2% of algo (FFI overhead may distort results)

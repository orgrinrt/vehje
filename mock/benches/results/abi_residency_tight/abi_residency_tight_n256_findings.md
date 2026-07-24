# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 62684% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (3.18 us) leads abi_residency_tight_reused_buffer (2.00 ms) by 62684%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -1.99 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 630.1x slower than the field

abi_residency_tight_fresh_alloc (2.00 ms) is 630.1x the fastest (3.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 630.1x the fastest

Fastest abi_residency_tight_null_entry (3.18 us) to slowest abi_residency_tight_fresh_alloc (2.00 ms): 630.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 3178.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 630.13x (fastest 3178.6 ns, slowest 2002909.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2004900ns | 2005308ns | 1989360ns | 2004518ns | 2013244ns | +0.30% |
| abi_residency_tight_null_entry | 5522ns | 5525ns | 5295ns | 5479ns | 5700ns | -99.72% |
| abi_residency_tight_reused_buffer | 1998955ns | 1998219ns | 1992469ns | 1997663ns | 2004135ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2002392ns | 1987014ns | 2010689ns | +0.30% | 0.000 |
| abi_residency_tight_null_entry | 3196ns | 3087ns | 3321ns | -99.84% | 0.080 |
| abi_residency_tight_reused_buffer | 1996444ns | 1990077ns | 2001650ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 37249.8 | 2003593.9 | 2002391.9 | n/a |
| abi_residency_tight_null_entry | 27564.7 | 3206.2 | 3195.6 | n/a |
| abi_residency_tight_reused_buffer | 36535.3 | 1998369.2 | 1996443.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.2% |
| abi_residency_tight_null_entry | 0.081 | 97.1% |
| abi_residency_tight_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2004900ns | 2004900ns | +0.30% |
| abi_residency_tight_null_entry | 5522ns | 5522ns | -99.72% |
| abi_residency_tight_reused_buffer | 1998955ns | 1998955ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 1995622ns | base | --- | [1992059, 2001650] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2002910ns | no significant difference | [-6234, +14263]ns | [1993576, 2010689] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_tight_null_entry | 3179ns | -1992409.8ns (-99.8%) | [-1998444, -1988891]ns | [3087, 3321] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2005483ns | -0.2% | -99.8% |
| 2 | 1995036ns | +0.7% | -99.8% |
| 3 | 1997817ns | +0.7% | -99.8% |
| 4 | 1994041ns | +0.3% | -99.8% |
| 5 | 1990077ns | +0.7% | -99.8% |
| 6 | 1996208ns | -0.5% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.057 | ok |
| abi_residency_tight_null_entry | -0.212 | moderate- |
| abi_residency_tight_reused_buffer | -0.009 | ok |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6045526.0ns | 2002391.9ns | 301.9% | HIGH |
| abi_residency_tight_null_entry | 121412.2ns | 3195.6ns | 3799.4% | HIGH |
| abi_residency_tight_reused_buffer | 6034196.3ns | 1996443.6ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1987014.2-2010689.4 ns)
  1987014.2 |########################################
  1988198.0 |
  1989381.7 |
  1990565.5 |
  1991749.2 |
  1992933.0 |
  1994116.7 |
  1995300.5 |
  1996484.3 |
  1997668.0 |
  1998851.8 |
  2000035.5 |########################################
  2001219.3 |########################################
  2002403.0 |
  2003586.8 |########################################
  2004770.6 |
  2005954.3 |
  2007138.1 |
  2008321.8 |########################################
  2009505.6 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 3086.7-3321.2 ns)
   3086.7 |########################################
   3098.4 |####################
   3110.2 |
   3121.9 |
   3133.6 |
   3145.3 |
   3157.1 |
   3168.8 |
   3180.5 |
   3192.2 |
   3204.0 |
   3215.7 |
   3227.4 |
   3239.2 |####################
   3250.9 |
   3262.6 |
   3274.3 |
   3286.1 |
   3297.8 |
   3309.5 |####################
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1990077.1-2001650.0 ns)
  1990077.1 |########################################
  1990655.7 |
  1991234.4 |
  1991813.0 |
  1992391.7 |
  1992970.3 |
  1993549.0 |########################################
  1994127.6 |
  1994706.3 |########################################
  1995284.9 |
  1995863.6 |########################################
  1996442.2 |
  1997020.8 |
  1997599.5 |########################################
  1998178.1 |
  1998756.8 |
  1999335.4 |
  1999914.1 |
  2000492.7 |
  2001071.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=3833.4% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.3% of algo (FFI overhead may distort results)

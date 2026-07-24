# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 55749% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (3.60 us) leads abi_residency_tight_reused_buffer (2.01 ms) by 55749%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.01 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 561.4x slower than the field

abi_residency_tight_fresh_alloc (2.02 ms) is 561.4x the fastest (3.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 561.4x the fastest

Fastest abi_residency_tight_null_entry (3.60 us) to slowest abi_residency_tight_fresh_alloc (2.02 ms): 561.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 3600.2 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 561.42x (fastest 3600.2 ns, slowest 2021224.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2032480ns | 2023894ns | 2016693ns | 2022849ns | 2054821ns | -3.59% |
| abi_residency_tight_null_entry | 5897ns | 5951ns | 5561ns | 5942ns | 5998ns | -99.72% |
| abi_residency_tight_reused_buffer | 2108224ns | 2013227ns | 2007965ns | 2012358ns | 2302152ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2029782ns | 2014160ns | 2051850ns | -3.60% | 0.000 |
| abi_residency_tight_null_entry | 3563ns | 3357ns | 3616ns | -99.83% | 0.001 |
| abi_residency_tight_reused_buffer | 2105534ns | 2005230ns | 2299278ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 44462.4 | 2031193.9 | 2029781.8 | 0 |
| abi_residency_tight_null_entry | 28142.9 | 3635.1 | 3562.6 | n/a |
| abi_residency_tight_reused_buffer | 42621.2 | 2049592.9 | 2105533.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.2% |
| abi_residency_tight_null_entry | 0.001 | 93.2% |
| abi_residency_tight_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2032480ns | 2032480ns | -3.59% |
| abi_residency_tight_null_entry | 5897ns | 5897ns | -99.72% |
| abi_residency_tight_reused_buffer | 2108224ns | 2108224ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2010661ns | base | --- | [2006663, 2299278] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2021225ns | no significant difference | [-249741, +13594]ns | [2016270, 2051850] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_tight_null_entry | 3600ns | -2007046.0ns (-99.8%) | [-2295791, -2003076]ns | [3471, 3616] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2011351ns | +0.4% | -99.8% |
| 2 | 2582932ns | -19.4% | -99.9% |
| 3 | 2009970ns | +0.6% | -99.8% |
| 4 | 2015622ns | +0.1% | -99.8% |
| 5 | 2005230ns | +0.4% | -99.8% |
| 6 | 2008095ns | +0.7% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.145 | ok |
| abi_residency_tight_null_entry | -0.324 | moderate- |
| abi_residency_tight_reused_buffer | -0.231 | moderate- |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6139566.6ns | 2029781.8ns | 302.5% | HIGH |
| abi_residency_tight_null_entry | 121928.7ns | 3562.6ns | 3422.5% | HIGH |
| abi_residency_tight_reused_buffer | 6133410.2ns | 2105533.7ns | 291.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 2014160.0-2051850.5 ns)
  2014160.0 |####################
  2016044.5 |
  2017929.0 |####################
  2019813.6 |####################
  2021698.1 |########################################
  2023582.6 |
  2025467.1 |
  2027351.7 |
  2029236.2 |
  2031120.7 |
  2033005.2 |
  2034889.7 |
  2036774.3 |
  2038658.8 |
  2040543.3 |
  2042427.8 |
  2044312.4 |
  2046196.9 |
  2048081.4 |
  2049965.9 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 3356.7-3616.4 ns)
   3356.7 |####################
   3369.7 |
   3382.7 |
   3395.7 |
   3408.6 |
   3421.6 |
   3434.6 |
   3447.6 |
   3460.6 |
   3473.6 |
   3486.6 |
   3499.6 |
   3512.5 |
   3525.5 |
   3538.5 |
   3551.5 |
   3564.5 |
   3577.5 |########################################
   3590.5 |
   3603.5 |########################################
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 2005230.4-2299277.5 ns)
  2005230.4 |########################################
  2019932.8 |
  2034635.1 |
  2049337.5 |
  2064039.8 |
  2078742.2 |
  2093444.5 |
  2108146.9 |
  2122849.2 |
  2137551.6 |
  2152254.0 |
  2166956.3 |
  2181658.7 |
  2196361.0 |
  2211063.4 |
  2225765.7 |
  2240468.1 |
  2255170.4 |
  2269872.8 |
  2284575.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=3413.5% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)

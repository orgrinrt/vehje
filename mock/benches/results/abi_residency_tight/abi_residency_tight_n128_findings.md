# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 74823% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (2.67 us) leads abi_residency_tight_reused_buffer (2.00 ms) by 74823%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.00 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 751.3x slower than the field

abi_residency_tight_fresh_alloc (2.00 ms) is 751.3x the fastest (2.67 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 751.3x the fastest

Fastest abi_residency_tight_null_entry (2.67 us) to slowest abi_residency_tight_fresh_alloc (2.00 ms): 751.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 2668.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 751.28x (fastest 2668.2 ns, slowest 2004538.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2009810ns | 2007132ns | 1995359ns | 2006432ns | 2022101ns | -0.20% |
| abi_residency_tight_null_entry | 4930ns | 4894ns | 4764ns | 4869ns | 5105ns | -99.76% |
| abi_residency_tight_reused_buffer | 2013783ns | 2001635ns | 1991078ns | 1998816ns | 2047586ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2007219ns | 1992890ns | 2019454ns | -0.19% | 0.000 |
| abi_residency_tight_null_entry | 2690ns | 2611ns | 2790ns | -99.87% | 0.048 |
| abi_residency_tight_reused_buffer | 2011041ns | 1988477ns | 2044553ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 40225.8 | 2085879.3 | 2007218.5 | n/a |
| abi_residency_tight_null_entry | 26770.2 | 2721.4 | 2690.1 | n/a |
| abi_residency_tight_reused_buffer | 42707.9 | 2011579.5 | 2011041.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.1% |
| abi_residency_tight_null_entry | 0.048 | 97.9% |
| abi_residency_tight_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2009810ns | 2009810ns | -0.20% |
| abi_residency_tight_null_entry | 4930ns | 4930ns | -99.76% |
| abi_residency_tight_reused_buffer | 2013783ns | 2013783ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 1999063ns | base | --- | [1989508, 2044553] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2004538ns | no significant difference | [-25809, +10235]ns | [1997664, 2019454] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_tight_null_entry | 2668ns | -1996426.9ns (-99.9%) | [-2041860, -1986767]ns | [2612, 2790] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2004762ns | +0.2% | -99.9% |
| 2 | 1990538ns | +0.1% | -99.9% |
| 3 | 1999148ns | +0.2% | -99.9% |
| 4 | 1988477ns | +0.7% | -99.9% |
| 5 | 2084344ns | -2.6% | -99.9% |
| 6 | 1998978ns | +0.3% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.128 | ok |
| abi_residency_tight_null_entry | 0.031 | ok |
| abi_residency_tight_reused_buffer | -0.286 | moderate- |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6199584.6ns | 2007218.5ns | 308.9% | HIGH |
| abi_residency_tight_null_entry | 119397.2ns | 2690.1ns | 4438.3% | HIGH |
| abi_residency_tight_reused_buffer | 6084765.1ns | 2011041.3ns | 302.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1992889.6-2019453.5 ns)
  1992889.6 |########################################
  1994217.8 |
  1995546.0 |
  1996874.2 |
  1998202.4 |
  1999530.6 |
  2000858.8 |
  2002187.0 |########################################
  2003515.2 |########################################
  2004843.4 |########################################
  2006171.6 |
  2007499.7 |########################################
  2008827.9 |
  2010156.1 |
  2011484.3 |
  2012812.5 |
  2014140.7 |
  2015468.9 |
  2016797.1 |
  2018125.3 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 2611.2-2789.8 ns)
   2611.2 |########################################
   2620.1 |
   2629.1 |
   2638.0 |
   2646.9 |
   2655.8 |####################
   2664.8 |
   2673.7 |####################
   2682.6 |
   2691.6 |
   2700.5 |
   2709.4 |
   2718.4 |
   2727.3 |
   2736.2 |
   2745.2 |
   2754.1 |
   2763.0 |
   2771.9 |####################
   2780.9 |
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1988477.1-2044553.1 ns)
  1988477.1 |########################################
  1991280.9 |
  1994084.7 |
  1996888.5 |########################################
  1999692.3 |
  2002496.1 |####################
  2005299.9 |
  2008103.7 |
  2010907.5 |
  2013711.3 |
  2016515.1 |
  2019318.9 |
  2022122.7 |
  2024926.5 |
  2027730.3 |
  2030534.1 |
  2033337.9 |
  2036141.7 |
  2038945.5 |
  2041749.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=4467.3% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)

# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_tight_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_tight_reused_buffer has the worst median (2.00 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_tight_null_entry at 3.05 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_tight_null_entry dominates: 65377% faster than the next best (abi_residency_tight_fresh_alloc)

abi_residency_tight_null_entry (3.05 us) leads abi_residency_tight_fresh_alloc (2.00 ms) by 65377%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.00 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_reused_buffer is an outlier: 655.3x slower than the field

abi_residency_tight_reused_buffer (2.00 ms) is 655.3x the fastest (3.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_tight_null_entry shows alternating (throttle bounce) (autocorr -0.73)

abi_residency_tight_null_entry's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 655.3x the fastest

Fastest abi_residency_tight_null_entry (3.05 us) to slowest abi_residency_tight_reused_buffer (2.00 ms): 655.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 3054.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 655.31x (fastest 3054.8 ns, slowest 2001841.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2002888ns | 2002709ns | 1994686ns | 2001691ns | 2008783ns | -1.01% |
| abi_residency_tight_null_entry | 5326ns | 5323ns | 5094ns | 5259ns | 5542ns | -99.74% |
| abi_residency_tight_reused_buffer | 2023322ns | 2004315ns | 1999182ns | 2003589ns | 2064991ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2000361ns | 1992136ns | 2006201ns | -1.00% | 0.000 |
| abi_residency_tight_null_entry | 3050ns | 2921ns | 3160ns | -99.85% | 0.003 |
| abi_residency_tight_reused_buffer | 2020530ns | 1996570ns | 2061766ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 39119.7 | 1999766.8 | 2000361.0 | n/a |
| abi_residency_tight_null_entry | 27114.4 | 3137.8 | 3049.6 | n/a |
| abi_residency_tight_reused_buffer | 45834.7 | 2073372.5 | 2020530.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.1% |
| abi_residency_tight_null_entry | 0.003 | 95.6% |
| abi_residency_tight_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2002888ns | 2002888ns | -1.01% |
| abi_residency_tight_null_entry | 5326ns | 5326ns | -99.74% |
| abi_residency_tight_reused_buffer | 2023322ns | 2023322ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2001841ns | base | --- | [1997982, 2061766] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2000193ns | no significant difference | [-58355, +2211]ns | [1994689, 2006201] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_tight_null_entry | 3055ns | -1998768.8ns (-99.8%) | [-2058766, -1994906]ns | [2934, 3160] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2002295ns | +0.0% | -99.8% |
| 2 | 2001388ns | -0.5% | -99.8% |
| 3 | 2035730ns | -1.9% | -99.8% |
| 4 | 2087802ns | -3.7% | -99.9% |
| 5 | 1999395ns | +0.1% | -99.8% |
| 6 | 1996570ns | +0.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.061 | ok |
| abi_residency_tight_null_entry | -0.734 | HIGH- (thermal bounce) |
| abi_residency_tight_reused_buffer | 0.026 | ok |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 3/6, lost 1/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6038563.1ns | 2000361.0ns | 301.9% | HIGH |
| abi_residency_tight_null_entry | 119195.8ns | 3049.6ns | 3908.5% | HIGH |
| abi_residency_tight_reused_buffer | 6240236.1ns | 2020530.0ns | 308.8% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1992135.8-2006201.2 ns)
  1992135.8 |########################################
  1992839.1 |
  1993542.3 |
  1994245.6 |
  1994948.9 |
  1995652.2 |
  1996355.4 |
  1997058.7 |########################################
  1997762.0 |########################################
  1998465.3 |
  1999168.5 |
  1999871.8 |
  2000575.1 |
  2001278.3 |
  2001981.6 |########################################
  2002684.9 |########################################
  2003388.2 |
  2004091.4 |
  2004794.7 |
  2005498.0 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 2920.8-3160.0 ns)
   2920.8 |########################################
   2932.8 |
   2944.7 |########################################
   2956.7 |
   2968.6 |
   2980.6 |
   2992.6 |
   3004.5 |
   3016.5 |
   3028.4 |########################################
   3040.4 |
   3052.4 |
   3064.3 |
   3076.3 |########################################
   3088.2 |
   3100.2 |
   3112.2 |########################################
   3124.1 |
   3136.1 |
   3148.0 |
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1996569.6-2061766.4 ns)
  1996569.6 |########################################
  1999829.4 |########################################
  2003089.3 |
  2006349.1 |
  2009609.0 |
  2012868.8 |
  2016128.7 |
  2019388.5 |
  2022648.3 |
  2025908.2 |
  2029168.0 |
  2032427.9 |
  2035687.7 |####################
  2038947.6 |
  2042207.4 |
  2045467.2 |
  2048727.1 |
  2051986.9 |
  2055246.8 |
  2058506.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=3898.1% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)

# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 49869% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (4.01 us) leads abi_residency_tight_reused_buffer (2.00 ms) by 49869%, a clear separation rather than a photo finish. CV 3.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.00 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 501.0x slower than the field

abi_residency_tight_fresh_alloc (2.01 ms) is 501.0x the fastest (4.01 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 501.0x the fastest

Fastest abi_residency_tight_null_entry (4.01 us) to slowest abi_residency_tight_fresh_alloc (2.01 ms): 501.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 4005.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 501.05x (fastest 4005.6 ns, slowest 2006989.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2008659ns | 2009570ns | 2001339ns | 2008778ns | 2012141ns | +0.25% |
| abi_residency_tight_null_entry | 6287ns | 6248ns | 6085ns | 6201ns | 6518ns | -99.69% |
| abi_residency_tight_reused_buffer | 2003629ns | 2004190ns | 2000298ns | 2003497ns | 2005493ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2006071ns | 1998841ns | 2009489ns | +0.25% | 0.000 |
| abi_residency_tight_null_entry | 4012ns | 3878ns | 4141ns | -99.80% | 0.001 |
| abi_residency_tight_reused_buffer | 2001027ns | 1997725ns | 2002961ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 40863.1 | 2006856.3 | 2006071.3 | n/a |
| abi_residency_tight_null_entry | 27007.2 | 4184.7 | 4012.0 | n/a |
| abi_residency_tight_reused_buffer | 38855.8 | 2000925.8 | 2001026.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.2% |
| abi_residency_tight_null_entry | 0.001 | 96.8% |
| abi_residency_tight_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2008659ns | 2008659ns | +0.25% |
| abi_residency_tight_null_entry | 6287ns | 6287ns | -99.69% |
| abi_residency_tight_reused_buffer | 2003629ns | 2003629ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2001566ns | base | --- | [1998554, 2002961] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2006989ns | +5295.8ns (+0.3%) | [+803, +9035]ns | [2001735, 2009489] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_residency_tight_null_entry | 4006ns | -1997577.6ns (-99.8%) | [-1998955, -1994512]ns | [3889, 4141] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2003440ns | +0.3% | -99.8% |
| 2 | 2002482ns | +0.1% | -99.8% |
| 3 | 2001119ns | +0.3% | -99.8% |
| 4 | 2002012ns | +0.3% | -99.8% |
| 5 | 1997725ns | +0.6% | -99.8% |
| 6 | 1999382ns | -0.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.313 | moderate- |
| abi_residency_tight_null_entry | -0.430 | moderate- |
| abi_residency_tight_reused_buffer | 0.263 | moderate+ |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 0/6, lost 5/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6060860.2ns | 2006071.3ns | 302.1% | HIGH |
| abi_residency_tight_null_entry | 121748.0ns | 4012.0ns | 3034.6% | HIGH |
| abi_residency_tight_reused_buffer | 6045509.0ns | 2001026.8ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1998841.2-2009489.2 ns)
  1998841.2 |########################################
  1999373.6 |
  1999906.0 |
  2000438.4 |
  2000970.8 |
  2001503.2 |
  2002035.6 |
  2002568.0 |
  2003100.4 |
  2003632.8 |
  2004165.2 |########################################
  2004697.6 |
  2005230.0 |
  2005762.4 |########################################
  2006294.8 |
  2006827.2 |
  2007359.6 |########################################
  2007892.0 |
  2008424.4 |
  2008956.8 |########################################
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 3878.3-4141.2 ns)
   3878.3 |####################
   3891.4 |####################
   3904.6 |####################
   3917.7 |
   3930.9 |
   3944.0 |
   3957.2 |
   3970.3 |
   3983.5 |
   3996.6 |
   4009.8 |
   4022.9 |
   4036.1 |
   4049.2 |
   4062.4 |
   4075.5 |
   4088.7 |########################################
   4101.8 |
   4115.0 |
   4128.1 |
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1997725.0-2002961.1 ns)
  1997725.0 |########################################
  1997986.8 |
  1998248.6 |
  1998510.4 |
  1998772.2 |
  1999034.0 |
  1999295.8 |########################################
  1999557.6 |
  1999819.4 |
  2000081.2 |
  2000343.0 |
  2000604.8 |
  2000866.6 |########################################
  2001128.4 |
  2001390.2 |
  2001652.0 |
  2001913.8 |########################################
  2002175.6 |
  2002437.4 |########################################
  2002699.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=3037.6% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)

# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 41040% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (4.90 us) leads abi_residency_tight_reused_buffer (2.01 ms) by 41040%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.01 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 412.7x slower than the field

abi_residency_tight_fresh_alloc (2.02 ms) is 412.7x the fastest (4.90 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 412.7x the fastest

Fastest abi_residency_tight_null_entry (4.90 us) to slowest abi_residency_tight_fresh_alloc (2.02 ms): 412.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 4896.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 412.68x (fastest 4896.1 ns, slowest 2020516.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2023792ns | 2023015ns | 2017696ns | 2021491ns | 2030292ns | +0.33% |
| abi_residency_tight_null_entry | 7181ns | 7172ns | 6861ns | 7131ns | 7416ns | -99.64% |
| abi_residency_tight_reused_buffer | 2017109ns | 2016802ns | 2008452ns | 2015327ns | 2024111ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2021258ns | 2014995ns | 2027743ns | +0.33% | 0.000 |
| abi_residency_tight_null_entry | 4907ns | 4691ns | 5079ns | -99.76% | 0.000 |
| abi_residency_tight_reused_buffer | 2014536ns | 2005917ns | 2021542ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 39467.1 | 2020905.1 | 2021258.4 | 0 |
| abi_residency_tight_null_entry | 26919.4 | 4962.0 | 4906.9 | n/a |
| abi_residency_tight_reused_buffer | 40287.7 | 2014851.3 | 2014536.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.2% |
| abi_residency_tight_null_entry | 0.000 | 95.8% |
| abi_residency_tight_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2023792ns | 2023792ns | +0.33% |
| abi_residency_tight_null_entry | 7181ns | 7181ns | -99.64% |
| abi_residency_tight_reused_buffer | 2017109ns | 2017109ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2014225ns | base | --- | [2007842, 2021542] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2020516ns | no significant difference | [-1512, +19902]ns | [2015516, 2027743] | no | 0.2188 | 0.2188 | 0 |
| abi_residency_tight_null_entry | 4896ns | -2009146.1ns (-99.7%) | [-2016797, -2002946]ns | [4746, 5079] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2013192ns | +0.1% | -99.8% |
| 2 | 2015258ns | +0.0% | -99.7% |
| 3 | 2009766ns | +1.2% | -99.8% |
| 4 | 2018717ns | +0.1% | -99.8% |
| 5 | 2024368ns | -0.2% | -99.8% |
| 6 | 2005917ns | +0.8% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | -0.187 | ok |
| abi_residency_tight_null_entry | 0.444 | moderate+ |
| abi_residency_tight_reused_buffer | -0.318 | moderate- |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 1/6, lost 2/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6103837.8ns | 2021258.4ns | 302.0% | HIGH |
| abi_residency_tight_null_entry | 123885.4ns | 4906.9ns | 2524.7% | HIGH |
| abi_residency_tight_reused_buffer | 6090021.6ns | 2014536.5ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 2014994.6-2027743.4 ns)
  2014994.6 |####################
  2015632.0 |####################
  2016269.5 |
  2016906.9 |
  2017544.4 |
  2018181.8 |
  2018819.2 |
  2019456.7 |
  2020094.1 |########################################
  2020731.5 |
  2021369.0 |
  2022006.4 |####################
  2022643.9 |
  2023281.3 |
  2023918.7 |
  2024556.2 |
  2025193.6 |
  2025831.0 |
  2026468.5 |
  2027105.9 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 4690.8-5079.1 ns)
   4690.8 |########################################
   4710.2 |
   4729.6 |
   4749.1 |
   4768.5 |
   4787.9 |########################################
   4807.3 |########################################
   4826.7 |
   4846.1 |
   4865.6 |
   4885.0 |
   4904.4 |
   4923.8 |
   4943.2 |
   4962.6 |########################################
   4982.1 |
   5001.5 |
   5020.9 |########################################
   5040.3 |
   5059.7 |
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 2005917.1-2021542.5 ns)
  2005917.1 |########################################
  2006698.4 |
  2007479.6 |
  2008260.9 |
  2009042.2 |########################################
  2009823.5 |
  2010604.7 |
  2011386.0 |
  2012167.3 |
  2012948.5 |########################################
  2013729.8 |
  2014511.1 |########################################
  2015292.3 |
  2016073.6 |
  2016854.9 |
  2017636.1 |
  2018417.4 |########################################
  2019198.7 |
  2019980.0 |
  2020761.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=2534.3% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)

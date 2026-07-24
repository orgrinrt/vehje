# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 44547% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (4.80 us) leads abi_residency_real_reused_buffer (2.14 ms) by 44547%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.14 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 447.9x slower than the field

abi_residency_real_fresh_alloc (2.15 ms) is 447.9x the fastest (4.80 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 447.9x the fastest

Fastest abi_residency_real_null_entry (4.80 us) to slowest abi_residency_real_fresh_alloc (2.15 ms): 447.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 4800.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 447.90x (fastest 4800.8 ns, slowest 2150279.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2152076ns | 2152870ns | 2139662ns | 2149187ns | 2162617ns | +0.08% |
| abi_residency_real_null_entry | 7138ns | 7043ns | 6888ns | 7021ns | 7439ns | -99.67% |
| abi_residency_real_reused_buffer | 2150286ns | 2146040ns | 2143688ns | 2145517ns | 2160739ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2149541ns | 2137068ns | 2160118ns | +0.09% | 0.000 |
| abi_residency_real_null_entry | 4866ns | 4722ns | 5045ns | -99.77% | 0.000 |
| abi_residency_real_reused_buffer | 2147642ns | 2141068ns | 2158015ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 41172.3 | 2148672.6 | 2149541.0 | 1 |
| abi_residency_real_null_entry | 28249.2 | 4960.3 | 4865.6 | n/a |
| abi_residency_real_reused_buffer | 43381.9 | 2148882.4 | 2147641.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.2% |
| abi_residency_real_null_entry | 0.000 | 98.4% |
| abi_residency_real_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2152076ns | 2152076ns | +0.08% |
| abi_residency_real_null_entry | 7138ns | 7138ns | -99.67% |
| abi_residency_real_reused_buffer | 2150286ns | 2150286ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2143421ns | base | --- | [2141489, 2158015] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2150280ns | no significant difference | [-16019, +13840]ns | [2138225, 2160118] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_real_null_entry | 4801ns | -2138522.9ns (-99.8%) | [-2153102, -2136703]ns | [4751, 5045] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2141910ns | +0.1% | -99.8% |
| 2 | 2141068ns | -0.2% | -99.8% |
| 3 | 2142896ns | +0.6% | -99.8% |
| 4 | 2148609ns | +0.7% | -99.8% |
| 5 | 2167421ns | -1.3% | -99.8% |
| 6 | 2143947ns | +0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | -0.255 | moderate- |
| abi_residency_real_null_entry | 0.211 | moderate+ |
| abi_residency_real_reused_buffer | 0.020 | ok |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6488189.6ns | 2149541.0ns | 301.8% | HIGH |
| abi_residency_real_null_entry | 124987.9ns | 4865.6ns | 2568.8% | HIGH |
| abi_residency_real_reused_buffer | 6492325.1ns | 2147641.7ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2137067.5-2160117.9 ns)
  2137067.5 |########################################
  2138220.0 |
  2139372.5 |########################################
  2140525.1 |
  2141677.6 |
  2142830.1 |
  2143982.6 |########################################
  2145135.1 |
  2146287.7 |
  2147440.2 |
  2148592.7 |
  2149745.2 |
  2150897.7 |
  2152050.3 |
  2153202.8 |
  2154355.3 |
  2155507.8 |########################################
  2156660.3 |########################################
  2157812.9 |
  2158965.4 |
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 4721.7-5044.8 ns)
   4721.7 |########################################
   4737.9 |
   4754.0 |
   4770.2 |########################################
   4786.3 |########################################
   4802.5 |########################################
   4818.6 |
   4834.8 |
   4850.9 |
   4867.1 |
   4883.2 |
   4899.4 |
   4915.6 |
   4931.7 |
   4947.9 |
   4964.0 |
   4980.2 |
   4996.3 |
   5012.5 |########################################
   5028.6 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2141067.5-2158015.0 ns)
  2141067.5 |########################################
  2141914.9 |
  2142762.2 |####################
  2143609.6 |####################
  2144457.0 |
  2145304.4 |
  2146151.8 |
  2146999.1 |
  2147846.5 |####################
  2148693.9 |
  2149541.2 |
  2150388.6 |
  2151236.0 |
  2152083.4 |
  2152930.8 |
  2153778.1 |
  2154625.5 |
  2155472.9 |
  2156320.2 |
  2157167.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=2603.3% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=302.5% of algo (FFI overhead may distort results)

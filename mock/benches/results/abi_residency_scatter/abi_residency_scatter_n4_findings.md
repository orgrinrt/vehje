# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 52896% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (4.02 us) leads abi_residency_scatter_reused_buffer (2.13 ms) by 52896%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 531.0x slower than the field

abi_residency_scatter_fresh_alloc (2.13 ms) is 531.0x the fastest (4.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 531.0x the fastest

Fastest abi_residency_scatter_null_entry (4.02 us) to slowest abi_residency_scatter_fresh_alloc (2.13 ms): 531.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 4020.8 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 530.95x (fastest 4020.8 ns, slowest 2134861.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2151983ns | 2137450ns | 2128814ns | 2135097ns | 2188897ns | -0.17% |
| abi_residency_scatter_null_entry | 6292ns | 6288ns | 6074ns | 6236ns | 6484ns | -99.71% |
| abi_residency_scatter_reused_buffer | 2155707ns | 2133559ns | 2127502ns | 2131639ns | 2205911ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2149357ns | 2126290ns | 2186078ns | -0.17% | 0.000 |
| abi_residency_scatter_null_entry | 4024ns | 3879ns | 4151ns | -99.81% | 0.001 |
| abi_residency_scatter_reused_buffer | 2152961ns | 2124837ns | 2202999ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 43527.7 | 2147314.2 | 2149356.9 | 0 |
| abi_residency_scatter_null_entry | 25703.2 | 4164.7 | 4023.6 | n/a |
| abi_residency_scatter_reused_buffer | 43251.4 | 2142179.4 | 2152960.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.2% |
| abi_residency_scatter_null_entry | 0.001 | 96.5% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2151983ns | 2151983ns | -0.17% |
| abi_residency_scatter_null_entry | 6292ns | 6292ns | -99.71% |
| abi_residency_scatter_reused_buffer | 2155707ns | 2155707ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2130866ns | base | --- | [2125018, 2202999] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2134862ns | no significant difference | [-22831, +12356]ns | [2127130, 2186078] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_scatter_null_entry | 4021ns | -2126769.8ns (-99.8%) | [-2198959, -2121083]ns | [3899, 4151] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2257101ns | -1.1% | -99.8% |
| 2 | 2124837ns | +0.5% | -99.8% |
| 3 | 2125198ns | +0.7% | -99.8% |
| 4 | 2148898ns | -1.0% | -99.8% |
| 5 | 2132233ns | +0.1% | -99.8% |
| 6 | 2129498ns | -0.2% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.024 | ok |
| abi_residency_scatter_null_entry | 0.237 | moderate+ |
| abi_residency_scatter_reused_buffer | -0.109 | ok |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 3/6, lost 3/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6529218.0ns | 2149356.9ns | 303.8% | HIGH |
| abi_residency_scatter_null_entry | 120294.5ns | 4023.6ns | 2989.7% | HIGH |
| abi_residency_scatter_reused_buffer | 6481650.5ns | 2152960.8ns | 301.1% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2126290.0-2186078.3 ns)
  2126290.0 |########################################
  2129279.4 |
  2132268.8 |########################################
  2135258.2 |
  2138247.7 |####################
  2141237.1 |
  2144226.5 |
  2147215.9 |
  2150205.3 |
  2153194.7 |
  2156184.1 |
  2159173.6 |
  2162163.0 |
  2165152.4 |
  2168141.8 |
  2171131.2 |
  2174120.6 |
  2177110.1 |
  2180099.5 |
  2183088.9 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 3878.8-4150.6 ns)
   3878.8 |########################################
   3892.4 |
   3906.0 |
   3919.6 |########################################
   3933.2 |
   3946.8 |
   3960.3 |
   3973.9 |
   3987.5 |########################################
   4001.1 |
   4014.7 |
   4028.3 |
   4041.9 |########################################
   4055.5 |
   4069.1 |
   4082.7 |
   4096.2 |
   4109.8 |
   4123.4 |
   4137.0 |########################################
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2124837.1-2202999.1 ns)
  2124837.1 |########################################
  2128745.2 |########################################
  2132653.3 |
  2136561.4 |
  2140469.5 |
  2144377.6 |
  2148285.7 |####################
  2152193.8 |
  2156101.9 |
  2160010.0 |
  2163918.1 |
  2167826.2 |
  2171734.3 |
  2175642.4 |
  2179550.5 |
  2183458.6 |
  2187366.7 |
  2191274.8 |
  2195182.9 |
  2199091.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=2997.5% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=302.4% of algo (FFI overhead may distort results)

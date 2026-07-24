# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 43031% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (4.94 us) leads abi_residency_scatter_reused_buffer (2.13 ms) by 43031%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 432.6x slower than the field

abi_residency_scatter_fresh_alloc (2.14 ms) is 432.6x the fastest (4.94 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 432.6x the fastest

Fastest abi_residency_scatter_null_entry (4.94 us) to slowest abi_residency_scatter_fresh_alloc (2.14 ms): 432.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 4942.0 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 432.62x (fastest 4942.0 ns, slowest 2138033.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2166900ns | 2140552ns | 2133396ns | 2139098ns | 2225356ns | +1.16% |
| abi_residency_scatter_null_entry | 7236ns | 7247ns | 6918ns | 7198ns | 7452ns | -99.66% |
| abi_residency_scatter_reused_buffer | 2142082ns | 2134032ns | 2122061ns | 2130118ns | 2170039ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2164262ns | 2130893ns | 2222464ns | +1.16% | 0.000 |
| abi_residency_scatter_null_entry | 4950ns | 4757ns | 5103ns | -99.77% | 0.000 |
| abi_residency_scatter_reused_buffer | 2139453ns | 2119318ns | 2167187ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 42518.3 | 2162677.1 | 2164262.4 | n/a |
| abi_residency_scatter_null_entry | 26067.0 | 5009.3 | 4950.1 | n/a |
| abi_residency_scatter_reused_buffer | 40360.0 | 2137722.0 | 2139453.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.2% |
| abi_residency_scatter_null_entry | 0.000 | 96.2% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2166900ns | 2166900ns | +1.16% |
| abi_residency_scatter_null_entry | 7236ns | 7236ns | -99.66% |
| abi_residency_scatter_reused_buffer | 2142082ns | 2142082ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2131569ns | base | --- | [2119604, 2167187] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2138034ns | +14130.0ns (+0.7%) | [+1377, +58920]ns | [2132290, 2222464] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_residency_scatter_null_entry | 4942ns | -2126610.4ns (-99.8%) | [-2162234, -2114666]ns | [4805, 5103] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2193114ns | +4.5% | -99.8% |
| 2 | 2119890ns | +0.5% | -99.8% |
| 3 | 2119318ns | +0.9% | -99.8% |
| 4 | 2141261ns | -0.2% | -99.8% |
| 5 | 2127291ns | +0.3% | -99.8% |
| 6 | 2135848ns | +0.8% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.078 | ok |
| abi_residency_scatter_null_entry | -0.446 | moderate- |
| abi_residency_scatter_reused_buffer | -0.175 | ok |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 1/6, lost 5/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6555752.1ns | 2164262.4ns | 302.9% | HIGH |
| abi_residency_scatter_null_entry | 122474.0ns | 4950.1ns | 2474.2% | HIGH |
| abi_residency_scatter_reused_buffer | 6459046.6ns | 2139453.4ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2130893.3-2222463.5 ns)
  2130893.3 |########################################
  2135471.8 |########################################
  2140050.3 |
  2144628.8 |
  2149207.3 |####################
  2153785.9 |
  2158364.4 |
  2162942.9 |
  2167521.4 |
  2172099.9 |
  2176678.4 |
  2181256.9 |
  2185835.4 |
  2190414.0 |
  2194992.5 |
  2199571.0 |
  2204149.5 |
  2208728.0 |
  2213306.5 |
  2217885.0 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 4756.7-5103.3 ns)
   4756.7 |########################################
   4774.0 |
   4791.4 |
   4808.7 |
   4826.0 |
   4843.3 |########################################
   4860.7 |########################################
   4878.0 |
   4895.3 |
   4912.7 |
   4930.0 |
   4947.3 |
   4964.7 |
   4982.0 |
   4999.3 |
   5016.6 |########################################
   5034.0 |########################################
   5051.3 |
   5068.6 |
   5086.0 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2119317.5-2167187.3 ns)
  2119317.5 |########################################
  2121711.0 |
  2124104.5 |
  2126498.0 |####################
  2128891.5 |
  2131285.0 |
  2133678.4 |####################
  2136071.9 |
  2138465.4 |
  2140858.9 |####################
  2143252.4 |
  2145645.9 |
  2148039.4 |
  2150432.9 |
  2152826.4 |
  2155219.8 |
  2157613.3 |
  2160006.8 |
  2162400.3 |
  2164793.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=2482.7% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)

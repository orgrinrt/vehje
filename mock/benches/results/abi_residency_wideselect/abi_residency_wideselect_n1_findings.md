# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 41383% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (4.96 us) leads abi_residency_wideselect_reused_buffer (2.06 ms) by 41383%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 415.3x slower than the field

abi_residency_wideselect_fresh_alloc (2.06 ms) is 415.3x the fastest (4.96 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 415.3x the fastest

Fastest abi_residency_wideselect_null_entry (4.96 us) to slowest abi_residency_wideselect_fresh_alloc (2.06 ms): 415.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 4961.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 415.32x (fastest 4961.9 ns, slowest 2060786.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2070764ns | 2063240ns | 2050363ns | 2062751ns | 2092986ns | -3.63% |
| abi_residency_wideselect_null_entry | 7249ns | 7251ns | 6975ns | 7219ns | 7429ns | -99.66% |
| abi_residency_wideselect_reused_buffer | 2148855ns | 2060927ns | 2048589ns | 2058322ns | 2334786ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2068196ns | 2047776ns | 2090286ns | -3.63% | 0.000 |
| abi_residency_wideselect_null_entry | 4956ns | 4763ns | 5085ns | -99.77% | 0.000 |
| abi_residency_wideselect_reused_buffer | 2146176ns | 2046068ns | 2331874ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 41696.0 | 2174045.3 | 2068195.9 | n/a |
| abi_residency_wideselect_null_entry | 28156.0 | 5020.1 | 4955.8 | n/a |
| abi_residency_wideselect_reused_buffer | 43485.8 | 2072011.5 | 2146175.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.2% |
| abi_residency_wideselect_null_entry | 0.000 | 96.0% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2070764ns | 2070764ns | -3.63% |
| abi_residency_wideselect_null_entry | 7249ns | 7249ns | -99.66% |
| abi_residency_wideselect_reused_buffer | 2148855ns | 2148855ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2058327ns | base | --- | [2048327, 2331874] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2060787ns | no significant difference | [-251749, +11721]ns | [2053515, 2090286] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_wideselect_null_entry | 4962ns | -2053393.1ns (-99.8%) | [-2327025, -2043242]ns | [4821, 5085] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2064050ns | -0.8% | -99.8% |
| 2 | 2050586ns | +0.5% | -99.8% |
| 3 | 2059326ns | +0.4% | -99.8% |
| 4 | 2046068ns | +0.6% | -99.8% |
| 5 | 2057328ns | +0.2% | -99.8% |
| 6 | 2599698ns | -18.7% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | -0.044 | ok |
| abi_residency_wideselect_null_entry | -0.328 | moderate- |
| abi_residency_wideselect_reused_buffer | -0.027 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 2/6, lost 4/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6354697.4ns | 2068195.9ns | 307.3% | HIGH |
| abi_residency_wideselect_null_entry | 124927.9ns | 4955.8ns | 2520.8% | HIGH |
| abi_residency_wideselect_reused_buffer | 6271381.2ns | 2146175.9ns | 292.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2047776.2-2090286.3 ns)
  2047776.2 |####################
  2049901.7 |
  2052027.2 |
  2054152.7 |
  2056278.2 |
  2058403.7 |####################
  2060529.2 |########################################
  2062654.7 |
  2064780.2 |
  2066905.7 |####################
  2069031.2 |
  2071156.8 |
  2073282.3 |
  2075407.8 |
  2077533.3 |
  2079658.8 |
  2081784.3 |
  2083909.8 |
  2086035.3 |
  2088160.8 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 4762.9-5084.8 ns)
   4762.9 |########################################
   4779.0 |
   4795.1 |
   4811.2 |
   4827.3 |
   4843.4 |
   4859.5 |
   4875.6 |########################################
   4891.7 |
   4907.8 |
   4923.9 |########################################
   4939.9 |
   4956.0 |
   4972.1 |
   4988.2 |########################################
   5004.3 |
   5020.4 |
   5036.5 |
   5052.6 |
   5068.7 |########################################
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2046067.9-2331873.8 ns)
  2046067.9 |########################################
  2060358.2 |##########
  2074648.5 |
  2088938.8 |
  2103229.1 |
  2117519.4 |
  2131809.7 |
  2146099.9 |
  2160390.2 |
  2174680.5 |
  2188970.8 |
  2203261.1 |
  2217551.4 |
  2231841.7 |
  2246132.0 |
  2260422.3 |
  2274712.6 |
  2289002.9 |
  2303293.2 |
  2317583.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=2518.1% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.2% of algo (FFI overhead may distort results)

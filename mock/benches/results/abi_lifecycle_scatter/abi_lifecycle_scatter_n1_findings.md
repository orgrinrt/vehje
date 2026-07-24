# abi_lifecycle (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_scatter_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_scatter_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_scatter_null_entry dominates: 42967% faster than the next best (abi_lifecycle_scatter_held_handle)

abi_lifecycle_scatter_null_entry (4.98 us) leads abi_lifecycle_scatter_held_handle (2.14 ms) by 42967%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_scatter_null_entry beats baseline by 100% (significant)

abi_lifecycle_scatter_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_scatter_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_scatter_fresh_per_batch is an outlier: 1060.4x slower than the field

abi_lifecycle_scatter_fresh_per_batch (5.28 ms) is 1060.4x the fastest (4.98 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_scatter_null_entry} vs {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} (42967% apart)

The field splits into a fast tier {abi_lifecycle_scatter_null_entry} and a slow tier {abi_lifecycle_scatter_held_handle, abi_lifecycle_scatter_fresh_per_column, abi_lifecycle_scatter_fresh_per_batch} with a 42967% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1060.4x the fastest

Fastest abi_lifecycle_scatter_null_entry (4.98 us) to slowest abi_lifecycle_scatter_fresh_per_batch (5.28 ms): 1060.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_scatter_null_entry** at 4977.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1060.41x (fastest 4977.1 ns, slowest 5277755.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 5313895ns | 5280662ns | 5266218ns | 5277246ns | 5392707ns | +144.53% |
| abi_lifecycle_scatter_fresh_per_column | 2189021ns | 2162946ns | 2160923ns | 2162303ns | 2243146ns | +0.73% |
| abi_lifecycle_scatter_held_handle | 2173100ns | 2146069ns | 2136567ns | 2143708ns | 2235454ns | base |
| abi_lifecycle_scatter_null_entry | 7317ns | 7330ns | 7054ns | 7287ns | 7492ns | -99.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 5310955ns | 5263330ns | 5389609ns | +144.69% | 0.000 |
| abi_lifecycle_scatter_fresh_per_column | 2186277ns | 2158200ns | 2240000ns | +0.73% | 0.000 |
| abi_lifecycle_scatter_held_handle | 2170502ns | 2134060ns | 2232736ns | base | 0.000 |
| abi_lifecycle_scatter_null_entry | 4998ns | 4816ns | 5138ns | -99.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 54316.5 | 5303224.8 | 5310955.5 | n/a |
| abi_lifecycle_scatter_fresh_per_column | 45594.6 | 2184422.6 | 2186277.0 | n/a |
| abi_lifecycle_scatter_held_handle | 41352.1 | 2169937.5 | 2170502.3 | n/a |
| abi_lifecycle_scatter_null_entry | 27736.4 | 5109.5 | 4997.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_lifecycle_scatter_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_scatter_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_scatter_held_handle | 0.000 | 0.2% |
| abi_lifecycle_scatter_null_entry | 0.000 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 5313895ns | 5313895ns | +144.53% |
| abi_lifecycle_scatter_fresh_per_column | 2189021ns | 2189021ns | +0.73% |
| abi_lifecycle_scatter_held_handle | 2173100ns | 2173100ns | base |
| abi_lifecycle_scatter_null_entry | 7317ns | 7317ns | -99.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_scatter_held_handle | 2143506ns | base | --- | [2135265, 2232736] | --- | --- | --- | --- |
| abi_lifecycle_scatter_fresh_per_batch | 5277755ns | +3138554.8ns (+146.4%) | [+3125932, +3156873]ns | [5265502, 5389609] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_scatter_fresh_per_column | 2160432ns | +17433.2ns (+0.8%) | [+6758, +23134]ns | [2158398, 2240000] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_scatter_null_entry | 4977ns | -2138478.8ns (-99.8%) | [-2227648, -2130387]ns | [4878, 5138] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_scatter_held_handle | abi_lifecycle_scatter_fresh_per_batch | abi_lifecycle_scatter_fresh_per_column | abi_lifecycle_scatter_null_entry |
|---|---|---|---|---|
| 1 | 2317964ns | +136.9% | -0.0% | -99.8% |
| 2 | 2147508ns | +146.3% | +0.7% | -99.8% |
| 3 | 2134060ns | +147.0% | +1.1% | -99.8% |
| 4 | 2136469ns | +146.6% | +1.0% | -99.8% |
| 5 | 2142672ns | +145.6% | +0.8% | -99.8% |
| 6 | 2144340ns | +146.5% | +0.9% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 0.050 | ok |
| abi_lifecycle_scatter_fresh_per_column | -0.016 | ok |
| abi_lifecycle_scatter_held_handle | 0.014 | ok |
| abi_lifecycle_scatter_null_entry | -0.296 | moderate- |

**Consistency summary:**

- **abi_lifecycle_scatter_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_scatter_fresh_per_column**: won 0/6, lost 5/6
- **abi_lifecycle_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_scatter_fresh_per_batch | 16041956.4ns | 5310955.5ns | 302.1% | HIGH |
| abi_lifecycle_scatter_fresh_per_column | 6636763.9ns | 2186277.0ns | 303.6% | HIGH |
| abi_lifecycle_scatter_held_handle | 6543223.3ns | 2170502.3ns | 301.5% | HIGH |
| abi_lifecycle_scatter_null_entry | 124256.9ns | 4997.7ns | 2486.3% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_scatter_fresh_per_batch (n=6, range 5263329.6-5389608.9 ns)
  5263329.6 |########################################
  5269643.6 |####################
  5275957.5 |
  5282271.5 |####################
  5288585.5 |####################
  5294899.4 |
  5301213.4 |
  5307527.4 |
  5313841.3 |
  5320155.3 |
  5326469.3 |
  5332783.2 |
  5339097.2 |
  5345411.2 |
  5351725.1 |
  5358039.1 |
  5364353.1 |
  5370667.0 |
  5376981.0 |
  5383295.0 |
  (0 below, 1 above range)

abi_lifecycle_scatter_fresh_per_column (n=6, range 2158200.0-2240000.5 ns)
  2158200.0 |########################################
  2162290.0 |##########
  2166380.0 |
  2170470.1 |
  2174560.1 |
  2178650.1 |
  2182740.1 |
  2186830.2 |
  2190920.2 |
  2195010.2 |
  2199100.2 |
  2203190.2 |
  2207280.3 |
  2211370.3 |
  2215460.3 |
  2219550.3 |
  2223640.4 |
  2227730.4 |
  2231820.4 |
  2235910.4 |
  (0 below, 1 above range)

abi_lifecycle_scatter_held_handle (n=6, range 2134060.0-2232736.0 ns)
  2134060.0 |########################################
  2138993.8 |####################
  2143927.6 |########################################
  2148861.4 |
  2153795.2 |
  2158729.0 |
  2163662.8 |
  2168596.6 |
  2173530.4 |
  2178464.2 |
  2183398.0 |
  2188331.8 |
  2193265.6 |
  2198199.4 |
  2203133.2 |
  2208067.0 |
  2213000.8 |
  2217934.6 |
  2222868.4 |
  2227802.2 |
  (0 below, 1 above range)

abi_lifecycle_scatter_null_entry (n=6, range 4816.2-5138.1 ns)
   4816.2 |####################
   4832.3 |
   4848.4 |
   4864.5 |
   4880.6 |
   4896.7 |
   4912.8 |
   4928.9 |########################################
   4945.0 |
   4961.1 |
   4977.1 |
   4993.2 |
   5009.3 |####################
   5025.4 |
   5041.5 |
   5057.6 |
   5073.7 |
   5089.8 |
   5105.9 |####################
   5122.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_scatter_fresh_per_batch**: bridge=300.9% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_fresh_per_column**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_held_handle**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_scatter_null_entry**: bridge=2487.8% of algo (FFI overhead may distort results)

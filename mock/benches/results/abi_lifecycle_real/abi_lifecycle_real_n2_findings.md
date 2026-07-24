# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 63277% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (3.41 us) leads abi_lifecycle_real_held_handle (2.16 ms) by 63277%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.16 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 1090.0x slower than the field

abi_lifecycle_real_fresh_per_batch (3.72 ms) is 1090.0x the fastest (3.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (63277% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 63277% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1090.0x the fastest

Fastest abi_lifecycle_real_null_entry (3.41 us) to slowest abi_lifecycle_real_fresh_per_batch (3.72 ms): 1090.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 3411.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1090.01x (fastest 3411.9 ns, slowest 3719015.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 3769541ns | 3721667ns | 3699792ns | 3718368ns | 3881174ns | +72.62% |
| abi_lifecycle_real_fresh_per_column | 2189506ns | 2176869ns | 2174331ns | 2176132ns | 2217156ns | +0.26% |
| abi_lifecycle_real_held_handle | 2183735ns | 2165032ns | 2149153ns | 2162528ns | 2232836ns | base |
| abi_lifecycle_real_null_entry | 5720ns | 5683ns | 5508ns | 5634ns | 5957ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 3766732ns | 3697036ns | 3878118ns | +72.70% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2186891ns | 2171860ns | 2214345ns | +0.27% | 0.000 |
| abi_lifecycle_real_held_handle | 2181039ns | 2146577ns | 2229945ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 3447ns | 3335ns | 3592ns | -99.84% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 49466.7 | 3788978.8 | 3766732.1 | n/a |
| abi_lifecycle_real_fresh_per_column | 42485.3 | 2300217.6 | 2186891.3 | 0 |
| abi_lifecycle_real_held_handle | 43172.0 | 2169747.6 | 2181039.0 | n/a |
| abi_lifecycle_real_null_entry | 27129.1 | 3515.3 | 3447.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.2% |
| abi_lifecycle_real_held_handle | 0.000 | 0.2% |
| abi_lifecycle_real_null_entry | 0.001 | 97.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 3769541ns | 3769541ns | +72.62% |
| abi_lifecycle_real_fresh_per_column | 2189506ns | 2189506ns | +0.26% |
| abi_lifecycle_real_held_handle | 2183735ns | 2183735ns | base |
| abi_lifecycle_real_null_entry | 5720ns | 5720ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2162345ns | base | --- | [2150827, 2229945] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 3719015ns | +1558288.7ns (+72.1%) | [+1543028, +1655762]ns | [3703064, 3878118] | YES | 0.0469 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2174386ns | no significant difference | [-20539, +26054]ns | [2171943, 2214345] | no | 0.2188 | 0.2188 | 0 |
| abi_lifecycle_real_null_entry | 3412ns | -2158837.5ns (-99.8%) | [-2226523, -2147415]ns | [3337, 3592] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2170258ns | +71.6% | +0.1% | -99.8% |
| 2 | 2155078ns | +73.6% | +0.8% | -99.8% |
| 3 | 2289632ns | +75.4% | -1.9% | -99.8% |
| 4 | 2146577ns | +72.8% | +1.6% | -99.8% |
| 5 | 2162313ns | +71.7% | +0.5% | -99.8% |
| 6 | 2162376ns | +71.0% | +0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.172 | ok |
| abi_lifecycle_real_fresh_per_column | -0.172 | ok |
| abi_lifecycle_real_held_handle | -0.365 | moderate- |
| abi_lifecycle_real_null_entry | -0.265 | moderate- |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 1/6, lost 4/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 11455078.0ns | 3766732.1ns | 304.1% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6721222.2ns | 2186891.3ns | 307.3% | HIGH |
| abi_lifecycle_real_held_handle | 6544732.9ns | 2181039.0ns | 300.1% | HIGH |
| abi_lifecycle_real_null_entry | 119642.6ns | 3447.1ns | 3470.8% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 3697036.2-3878117.5 ns)
  3697036.2 |####################
  3706090.3 |########################################
  3715144.3 |
  3724198.4 |####################
  3733252.5 |####################
  3742306.5 |
  3751360.6 |
  3760414.7 |
  3769468.7 |
  3778522.8 |
  3787576.9 |
  3796630.9 |
  3805685.0 |
  3814739.0 |
  3823793.1 |
  3832847.2 |
  3841901.2 |
  3850955.3 |
  3860009.4 |
  3869063.4 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2171860.4-2214344.8 ns)
  2171860.4 |########################################
  2173984.6 |#############
  2176108.8 |
  2178233.1 |
  2180357.3 |#############
  2182481.5 |
  2184605.7 |
  2186729.9 |
  2188854.2 |
  2190978.4 |
  2193102.6 |
  2195226.8 |
  2197351.0 |
  2199475.3 |
  2201599.5 |
  2203723.7 |
  2205847.9 |
  2207972.1 |
  2210096.4 |
  2212220.6 |
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2146576.7-2229945.2 ns)
  2146576.7 |####################
  2150745.1 |
  2154913.6 |####################
  2159082.0 |########################################
  2163250.4 |
  2167418.8 |####################
  2171587.2 |
  2175755.7 |
  2179924.1 |
  2184092.5 |
  2188261.0 |
  2192429.4 |
  2196597.8 |
  2200766.2 |
  2204934.7 |
  2209103.1 |
  2213271.5 |
  2217439.9 |
  2221608.4 |
  2225776.8 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 3334.6-3592.5 ns)
   3334.6 |########################################
   3347.5 |
   3360.4 |
   3373.3 |
   3386.2 |####################
   3399.1 |
   3412.0 |
   3424.9 |####################
   3437.8 |
   3450.7 |
   3463.6 |
   3476.4 |
   3489.3 |
   3502.2 |####################
   3515.1 |
   3528.0 |
   3540.9 |
   3553.8 |
   3566.7 |
   3579.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=3516.6% of algo (FFI overhead may distort results)

# abi_lifecycle (real)

4 variants, 6 samples per variant.
Baseline: **abi_lifecycle_real_held_handle**

## Highlights

Baseline for all deltas below: **abi_lifecycle_real_held_handle**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_lifecycle_real_null_entry dominates: 83146% faster than the next best (abi_lifecycle_real_held_handle)

abi_lifecycle_real_null_entry (2.57 us) leads abi_lifecycle_real_held_handle (2.14 ms) by 83146%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_lifecycle_real_null_entry beats baseline by 100% (significant)

abi_lifecycle_real_null_entry is -2.14 ms (100%) faster than baseline abi_lifecycle_real_held_handle, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_lifecycle_real_fresh_per_batch is an outlier: 911.8x slower than the field

abi_lifecycle_real_fresh_per_batch (2.35 ms) is 911.8x the fastest (2.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_lifecycle_real_null_entry shows alternating (throttle bounce) (autocorr -0.66)

abi_lifecycle_real_null_entry's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_lifecycle_real_null_entry} vs {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} (83146% apart)

The field splits into a fast tier {abi_lifecycle_real_null_entry} and a slow tier {abi_lifecycle_real_held_handle, abi_lifecycle_real_fresh_per_column, abi_lifecycle_real_fresh_per_batch} with a 83146% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 911.8x the fastest

Fastest abi_lifecycle_real_null_entry (2.57 us) to slowest abi_lifecycle_real_fresh_per_batch (2.35 ms): 911.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_lifecycle_real_null_entry** at 2571.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 911.81x (fastest 2571.9 ns, slowest 2345079.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2347102ns | 2347669ns | 2334369ns | 2345960ns | 2355181ns | +9.48% |
| abi_lifecycle_real_fresh_per_column | 2163116ns | 2163732ns | 2154861ns | 2161952ns | 2168991ns | +0.90% |
| abi_lifecycle_real_held_handle | 2143815ns | 2143578ns | 2139546ns | 2142254ns | 2148292ns | base |
| abi_lifecycle_real_null_entry | 4904ns | 4904ns | 4735ns | 4865ns | 5046ns | -99.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2344440ns | 2331789ns | 2352381ns | +9.49% | 0.000 |
| abi_lifecycle_real_fresh_per_column | 2160475ns | 2152336ns | 2166365ns | +0.90% | 0.000 |
| abi_lifecycle_real_held_handle | 2141191ns | 2136827ns | 2145671ns | base | 0.000 |
| abi_lifecycle_real_null_entry | 2582ns | 2526ns | 2642ns | -99.88% | 0.006 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 42075.9 | 2342856.5 | 2344440.4 | n/a |
| abi_lifecycle_real_fresh_per_column | 40794.4 | 2158529.6 | 2160475.0 | 0 |
| abi_lifecycle_real_held_handle | 41459.7 | 2143551.3 | 2141191.4 | n/a |
| abi_lifecycle_real_null_entry | 28326.3 | 2693.3 | 2581.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_lifecycle_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 0.000 | 0.1% |
| abi_lifecycle_real_fresh_per_column | 0.000 | 0.1% |
| abi_lifecycle_real_held_handle | 0.000 | 0.1% |
| abi_lifecycle_real_null_entry | 0.006 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 2347102ns | 2347102ns | +9.48% |
| abi_lifecycle_real_fresh_per_column | 2163116ns | 2163116ns | +0.90% |
| abi_lifecycle_real_held_handle | 2143815ns | 2143815ns | base |
| abi_lifecycle_real_null_entry | 4904ns | 4904ns | -99.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_lifecycle_real_held_handle | 2141007ns | base | --- | [2136897, 2145671] | --- | --- | --- | --- |
| abi_lifecycle_real_fresh_per_batch | 2345080ns | +202212.1ns (+9.4%) | [+194727, +212807]ns | [2335860, 2352381] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_fresh_per_column | 2161078ns | +17531.5ns (+0.8%) | [+15859, +24460]ns | [2153982, 2166365] | YES | 0.0313 | 0.0313 | 0 |
| abi_lifecycle_real_null_entry | 2572ns | -2138436.8ns (-99.9%) | [-2143070, -2134322]ns | [2532, 2642] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_lifecycle_real_held_handle | abi_lifecycle_real_fresh_per_batch | abi_lifecycle_real_fresh_per_column | abi_lifecycle_real_null_entry |
|---|---|---|---|---|
| 1 | 2148132ns | +9.4% | +0.9% | -99.9% |
| 2 | 2139055ns | +9.0% | +0.8% | -99.9% |
| 3 | 2142958ns | +9.4% | +0.8% | -99.9% |
| 4 | 2143209ns | +9.2% | +1.1% | -99.9% |
| 5 | 2136966ns | +10.1% | +0.7% | -99.9% |
| 6 | 2136827ns | +9.8% | +1.2% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_lifecycle_real_fresh_per_batch | -0.372 | moderate- |
| abi_lifecycle_real_fresh_per_column | -0.577 | HIGH- (thermal bounce) |
| abi_lifecycle_real_held_handle | -0.053 | ok |
| abi_lifecycle_real_null_entry | -0.658 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_lifecycle_real_fresh_per_batch**: won 0/6, lost 6/6
- **abi_lifecycle_real_fresh_per_column**: won 0/6, lost 6/6
- **abi_lifecycle_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_lifecycle_real_fresh_per_batch | 7070984.7ns | 2344440.4ns | 301.6% | HIGH |
| abi_lifecycle_real_fresh_per_column | 6519694.4ns | 2160475.0ns | 301.8% | HIGH |
| abi_lifecycle_real_held_handle | 6474262.2ns | 2141191.4ns | 302.4% | HIGH |
| abi_lifecycle_real_null_entry | 119233.5ns | 2581.9ns | 4618.1% | HIGH |

## Distribution (algo ns)

```
abi_lifecycle_real_fresh_per_batch (n=6, range 2331788.8-2352381.5 ns)
  2331788.8 |########################################
  2332818.4 |
  2333848.1 |
  2334877.7 |
  2335907.3 |
  2336937.0 |
  2337966.6 |
  2338996.2 |########################################
  2340025.9 |
  2341055.5 |
  2342085.1 |
  2343114.8 |
  2344144.4 |########################################
  2345174.0 |########################################
  2346203.7 |
  2347233.3 |
  2348262.9 |
  2349292.6 |
  2350322.2 |########################################
  2351351.8 |
  (0 below, 1 above range)

abi_lifecycle_real_fresh_per_column (n=6, range 2152336.2-2166365.2 ns)
  2152336.2 |########################################
  2153037.7 |
  2153739.1 |
  2154440.6 |
  2155142.0 |########################################
  2155843.5 |
  2156544.9 |
  2157246.4 |
  2157947.8 |
  2158649.3 |########################################
  2159350.7 |
  2160052.2 |
  2160753.6 |
  2161455.1 |
  2162156.5 |########################################
  2162858.0 |
  2163559.4 |
  2164260.9 |
  2164962.3 |
  2165663.8 |########################################
  (0 below, 1 above range)

abi_lifecycle_real_held_handle (n=6, range 2136827.1-2145670.9 ns)
  2136827.1 |########################################
  2137269.3 |
  2137711.5 |
  2138153.7 |
  2138595.9 |
  2139038.0 |####################
  2139480.2 |
  2139922.4 |
  2140364.6 |
  2140806.8 |
  2141249.0 |
  2141691.2 |
  2142133.4 |
  2142575.5 |####################
  2143017.7 |####################
  2143459.9 |
  2143902.1 |
  2144344.3 |
  2144786.5 |
  2145228.7 |
  (0 below, 1 above range)

abi_lifecycle_real_null_entry (n=6, range 2525.8-2641.9 ns)
   2525.8 |####################
   2531.6 |
   2537.4 |########################################
   2543.2 |
   2549.0 |
   2554.8 |
   2560.6 |
   2566.4 |
   2572.2 |
   2578.0 |
   2583.8 |
   2589.6 |
   2595.4 |
   2601.2 |####################
   2607.0 |####################
   2612.8 |
   2618.6 |
   2624.4 |
   2630.2 |
   2636.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_lifecycle_real_fresh_per_batch**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_fresh_per_column**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_held_handle**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_lifecycle_real_null_entry**: bridge=4638.2% of algo (FFI overhead may distort results)

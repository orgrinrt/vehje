# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_real_null_entry dominates: 83112% faster than the next best (abi_residency_real_reused_buffer)

abi_residency_real_null_entry (2.56 us) leads abi_residency_real_reused_buffer (2.13 ms) by 83112%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.13 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_fresh_alloc is an outlier: 835.7x slower than the field

abi_residency_real_fresh_alloc (2.14 ms) is 835.7x the fastest (2.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 835.7x the fastest

Fastest abi_residency_real_null_entry (2.56 us) to slowest abi_residency_real_fresh_alloc (2.14 ms): 835.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 2560.2 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 835.73x (fastest 2560.2 ns, slowest 2139632.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2142025ns | 2142128ns | 2136044ns | 2140861ns | 2146763ns | +0.47% |
| abi_residency_real_null_entry | 4829ns | 4845ns | 4610ns | 4832ns | 4934ns | -99.77% |
| abi_residency_real_reused_buffer | 2131970ns | 2133096ns | 2115844ns | 2129419ns | 2143859ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2139543ns | 2133608ns | 2144228ns | +0.48% | 0.000 |
| abi_residency_real_null_entry | 2540ns | 2443ns | 2575ns | -99.88% | 0.025 |
| abi_residency_real_reused_buffer | 2129311ns | 2113312ns | 2141154ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 39957.2 | 2138800.5 | 2139543.1 | n/a |
| abi_residency_real_null_entry | 27505.1 | 2783.1 | 2539.5 | n/a |
| abi_residency_real_reused_buffer | 39596.4 | 2128121.8 | 2129310.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.026 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.1% |
| abi_residency_real_null_entry | 0.025 | 95.4% |
| abi_residency_real_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2142025ns | 2142025ns | +0.47% |
| abi_residency_real_null_entry | 4829ns | 4829ns | -99.77% |
| abi_residency_real_reused_buffer | 2131970ns | 2131970ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2130392ns | base | --- | [2116387, 2141154] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2139633ns | no significant difference | [-4767, +27841]ns | [2134769, 2144228] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_real_null_entry | 2560ns | -2127895.2ns (-99.9%) | [-2138606, -2113812]ns | [2483, 2575] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2135024ns | -0.1% | -99.9% |
| 2 | 2113312ns | +1.5% | -99.9% |
| 3 | 2144047ns | -0.4% | -99.9% |
| 4 | 2138260ns | -0.0% | -99.9% |
| 5 | 2119462ns | +1.1% | -99.9% |
| 6 | 2125759ns | +0.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | -0.402 | moderate- |
| abi_residency_real_null_entry | -0.228 | moderate- |
| abi_residency_real_reused_buffer | -0.357 | moderate- |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 1/6, lost 3/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6456605.1ns | 2139543.1ns | 301.8% | HIGH |
| abi_residency_real_null_entry | 113508.1ns | 2539.5ns | 4469.7% | HIGH |
| abi_residency_real_reused_buffer | 6427467.0ns | 2129310.7ns | 301.9% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2133608.3-2144228.1 ns)
  2133608.3 |########################################
  2134139.3 |
  2134670.3 |
  2135201.3 |
  2135732.3 |########################################
  2136263.2 |
  2136794.2 |########################################
  2137325.2 |
  2137856.2 |
  2138387.2 |
  2138918.2 |
  2139449.2 |
  2139980.2 |
  2140511.2 |
  2141042.2 |
  2141573.1 |########################################
  2142104.1 |
  2142635.1 |
  2143166.1 |########################################
  2143697.1 |
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 2442.9-2575.0 ns)
   2442.9 |########################################
   2449.5 |
   2456.1 |
   2462.7 |
   2469.3 |
   2475.9 |
   2482.5 |
   2489.1 |
   2495.7 |
   2502.3 |
   2508.9 |
   2515.6 |
   2522.2 |########################################
   2528.8 |
   2535.4 |
   2542.0 |
   2548.6 |########################################
   2555.2 |
   2561.8 |
   2568.4 |########################################
  (0 below, 2 above range)

abi_residency_real_reused_buffer (n=6, range 2113311.7-2141153.8 ns)
  2113311.7 |########################################
  2114703.8 |
  2116095.9 |
  2117488.0 |
  2118880.1 |########################################
  2120272.2 |
  2121664.3 |
  2123056.4 |
  2124448.5 |########################################
  2125840.6 |
  2127232.7 |
  2128624.8 |
  2130016.9 |
  2131409.0 |
  2132801.1 |
  2134193.2 |########################################
  2135585.3 |
  2136977.4 |########################################
  2138369.5 |
  2139761.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=4434.9% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=301.7% of algo (FFI overhead may distort results)

# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 63123% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (3.23 us) leads abi_residency_wideselect_reused_buffer (2.04 ms) by 63123%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.04 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 632.9x slower than the field

abi_residency_wideselect_fresh_alloc (2.05 ms) is 632.9x the fastest (3.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_wideselect_fresh_alloc shows alternating (throttle bounce) (autocorr -0.61)

abi_residency_wideselect_fresh_alloc's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 632.9x the fastest

Fastest abi_residency_wideselect_null_entry (3.23 us) to slowest abi_residency_wideselect_fresh_alloc (2.05 ms): 632.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 3233.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 632.87x (fastest 3233.3 ns, slowest 2046302.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2050858ns | 2048789ns | 2045807ns | 2047878ns | 2057853ns | -0.02% |
| abi_residency_wideselect_null_entry | 5603ns | 5549ns | 5424ns | 5524ns | 5811ns | -99.73% |
| abi_residency_wideselect_reused_buffer | 2051316ns | 2046785ns | 2038671ns | 2044964ns | 2067166ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2048338ns | 2043250ns | 2055288ns | -0.02% | 0.000 |
| abi_residency_wideselect_null_entry | 3252ns | 3172ns | 3338ns | -99.84% | 0.079 |
| abi_residency_wideselect_reused_buffer | 2048816ns | 2036271ns | 2064652ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 39595.0 | 2048556.4 | 2048338.3 | 4 |
| abi_residency_wideselect_null_entry | 28316.5 | 3266.9 | 3251.9 | n/a |
| abi_residency_wideselect_reused_buffer | 37736.0 | 2046914.2 | 2048816.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.081 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.2% |
| abi_residency_wideselect_null_entry | 0.079 | 98.1% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2050858ns | 2050858ns | -0.02% |
| abi_residency_wideselect_null_entry | 5603ns | 5603ns | -99.73% |
| abi_residency_wideselect_reused_buffer | 2051316ns | 2051316ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2044213ns | base | --- | [2037584, 2064652] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2046302ns | no significant difference | [-21047, +17705]ns | [2043425, 2055288] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_wideselect_null_entry | 3233ns | -2040953.6ns (-99.8%) | [-2061360, -2034379]ns | [3184, 3338] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2041790ns | +0.1% | -99.8% |
| 2 | 2038896ns | +1.0% | -99.8% |
| 3 | 2054968ns | -0.6% | -99.8% |
| 4 | 2036271ns | +0.7% | -99.8% |
| 5 | 2046636ns | +0.1% | -99.8% |
| 6 | 2074335ns | -1.5% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | -0.612 | HIGH- (thermal bounce) |
| abi_residency_wideselect_null_entry | -0.009 | ok |
| abi_residency_wideselect_reused_buffer | -0.097 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6184677.0ns | 2048338.3ns | 301.9% | HIGH |
| abi_residency_wideselect_null_entry | 121976.0ns | 3251.9ns | 3751.0% | HIGH |
| abi_residency_wideselect_reused_buffer | 6178638.9ns | 2048816.1ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2043250.4-2055288.1 ns)
  2043250.4 |########################################
  2043852.3 |
  2044454.2 |
  2045056.1 |
  2045657.9 |
  2046259.8 |
  2046861.7 |
  2047463.6 |
  2048065.5 |
  2048667.4 |#############
  2049269.3 |
  2049871.2 |
  2050473.0 |
  2051074.9 |#############
  2051676.8 |
  2052278.7 |
  2052880.6 |
  2053482.5 |
  2054084.4 |
  2054686.3 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 3172.5-3338.5 ns)
   3172.5 |########################################
   3180.8 |
   3189.1 |########################################
   3197.4 |
   3205.7 |
   3214.0 |
   3222.3 |########################################
   3230.6 |########################################
   3238.9 |
   3247.2 |
   3255.5 |
   3263.8 |
   3272.1 |
   3280.4 |
   3288.7 |
   3297.0 |
   3305.3 |
   3313.6 |
   3321.9 |########################################
   3330.2 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2036270.8-2064651.9 ns)
  2036270.8 |########################################
  2037689.9 |########################################
  2039108.9 |
  2040528.0 |########################################
  2041947.0 |
  2043366.1 |
  2044785.1 |
  2046204.2 |########################################
  2047623.2 |
  2049042.3 |
  2050461.3 |
  2051880.4 |
  2053299.4 |
  2054718.5 |########################################
  2056137.5 |
  2057556.6 |
  2058975.6 |
  2060394.7 |
  2061813.7 |
  2063232.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=3777.4% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.6% of algo (FFI overhead may distort results)

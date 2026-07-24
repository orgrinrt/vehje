# abi_residency (scatter)

3 variants, 6 samples per variant.
Baseline: **abi_residency_scatter_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_scatter_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_scatter_null_entry dominates: 62737% faster than the next best (abi_residency_scatter_reused_buffer)

abi_residency_scatter_null_entry (3.40 us) leads abi_residency_scatter_reused_buffer (2.14 ms) by 62737%, a clear separation rather than a photo finish. CV 4.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_scatter_null_entry beats baseline by 100% (significant)

abi_residency_scatter_null_entry is -2.13 ms (100%) faster than baseline abi_residency_scatter_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_scatter_fresh_alloc is an outlier: 628.4x slower than the field

abi_residency_scatter_fresh_alloc (2.14 ms) is 628.4x the fastest (3.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_residency_scatter_reused_buffer shows alternating (throttle bounce) (autocorr -0.76)

abi_residency_scatter_reused_buffer's per-pass series has lag-1 autocorrelation -0.76, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 628.4x the fastest

Fastest abi_residency_scatter_null_entry (3.40 us) to slowest abi_residency_scatter_fresh_alloc (2.14 ms): 628.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_scatter_null_entry** at 3402.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 628.40x (fastest 3402.3 ns, slowest 2137989.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2140523ns | 2140776ns | 2129650ns | 2138814ns | 2148522ns | -0.03% |
| abi_residency_scatter_null_entry | 5737ns | 5661ns | 5409ns | 5626ns | 6068ns | -99.73% |
| abi_residency_scatter_reused_buffer | 2141109ns | 2140570ns | 2132986ns | 2139128ns | 2148142ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2137815ns | 2126969ns | 2145795ns | -0.03% | 0.000 |
| abi_residency_scatter_null_entry | 3455ns | 3268ns | 3664ns | -99.84% | 0.001 |
| abi_residency_scatter_reused_buffer | 2138441ns | 2130505ns | 2145352ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 42950.3 | 2139913.4 | 2137815.4 | 3 |
| abi_residency_scatter_null_entry | 26096.7 | 3526.9 | 3454.9 | n/a |
| abi_residency_scatter_reused_buffer | 40918.1 | 2137397.1 | 2138441.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_scatter_fresh_alloc | 0.000 | 0.2% |
| abi_residency_scatter_null_entry | 0.001 | 96.1% |
| abi_residency_scatter_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 2140523ns | 2140523ns | -0.03% |
| abi_residency_scatter_null_entry | 5737ns | 5737ns | -99.73% |
| abi_residency_scatter_reused_buffer | 2141109ns | 2141109ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_scatter_reused_buffer | 2137896ns | base | --- | [2132074, 2145352] | --- | --- | --- | --- |
| abi_residency_scatter_fresh_alloc | 2137989ns | no significant difference | [-10299, +11702]ns | [2129662, 2145795] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_scatter_null_entry | 3402ns | -2134572.9ns (-99.8%) | [-2141817, -2128568]ns | [3298, 3664] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_scatter_reused_buffer | abi_residency_scatter_fresh_alloc | abi_residency_scatter_null_entry |
|---|---|---|---|
| 1 | 2134542ns | +0.6% | -99.8% |
| 2 | 2145992ns | -0.4% | -99.8% |
| 3 | 2130505ns | -0.2% | -99.8% |
| 4 | 2141251ns | -0.1% | -99.8% |
| 5 | 2133644ns | +0.5% | -99.8% |
| 6 | 2144712ns | -0.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_scatter_fresh_alloc | -0.130 | ok |
| abi_residency_scatter_null_entry | -0.326 | moderate- |
| abi_residency_scatter_reused_buffer | -0.756 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_residency_scatter_fresh_alloc**: won 4/6, lost 2/6
- **abi_residency_scatter_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_scatter_fresh_alloc | 6461767.8ns | 2137815.4ns | 302.3% | HIGH |
| abi_residency_scatter_null_entry | 119226.6ns | 3454.9ns | 3451.0% | HIGH |
| abi_residency_scatter_reused_buffer | 6454323.3ns | 2138441.0ns | 301.8% | HIGH |

## Distribution (algo ns)

```
abi_residency_scatter_fresh_alloc (n=6, range 2126969.2-2145795.0 ns)
  2126969.2 |####################
  2127910.5 |
  2128851.8 |
  2129793.1 |
  2130734.4 |
  2131675.7 |####################
  2132616.9 |
  2133558.2 |
  2134499.5 |
  2135440.8 |
  2136382.1 |
  2137323.4 |########################################
  2138264.7 |
  2139206.0 |
  2140147.3 |
  2141088.5 |
  2142029.8 |
  2142971.1 |
  2143912.4 |####################
  2144853.7 |
  (0 below, 1 above range)

abi_residency_scatter_null_entry (n=6, range 3268.3-3664.2 ns)
   3268.3 |########################################
   3288.1 |
   3307.9 |
   3327.7 |########################################
   3347.5 |
   3367.3 |########################################
   3387.1 |
   3406.9 |########################################
   3426.7 |
   3446.5 |
   3466.2 |
   3486.0 |
   3505.8 |
   3525.6 |
   3545.4 |
   3565.2 |
   3585.0 |
   3604.8 |
   3624.6 |########################################
   3644.4 |
  (0 below, 1 above range)

abi_residency_scatter_reused_buffer (n=6, range 2130505.0-2145352.3 ns)
  2130505.0 |########################################
  2131247.4 |
  2131989.7 |
  2132732.1 |
  2133474.5 |########################################
  2134216.8 |########################################
  2134959.2 |
  2135701.6 |
  2136443.9 |
  2137186.3 |
  2137928.6 |
  2138671.0 |
  2139413.4 |
  2140155.7 |
  2140898.1 |########################################
  2141640.5 |
  2142382.8 |
  2143125.2 |
  2143867.6 |
  2144609.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_scatter_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_scatter_null_entry**: bridge=3499.2% of algo (FFI overhead may distort results)
- **abi_residency_scatter_reused_buffer**: bridge=301.8% of algo (FFI overhead may distort results)

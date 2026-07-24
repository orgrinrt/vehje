# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.37 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 37162% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.37 us) leads abi_soa_win_real_soa_payload (882.03 us) by 37162%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.15 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 908.5x slower than the field

abi_soa_win_real_scalar_payload (2.15 ms) is 908.5x the fastest (2.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry shows alternating (throttle bounce) (autocorr -0.75)

abi_soa_win_real_null_entry's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 908.5x the fastest

Fastest abi_soa_win_real_null_entry (2.37 us) to slowest abi_soa_win_real_scalar_payload (2.15 ms): 908.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2367.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 908.51x (fastest 2367.1 ns, slowest 2150536.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4725ns | 4744ns | 4538ns | 4726ns | 4816ns | -99.78% |
| abi_soa_win_real_scalar_payload | 2151826ns | 2153229ns | 2146336ns | 2150932ns | 2155912ns | base |
| abi_soa_win_real_soa_payload | 884735ns | 884429ns | 883859ns | 884343ns | 885760ns | -58.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2357ns | 2290ns | 2398ns | -99.89% | 0.014 |
| abi_soa_win_real_scalar_payload | 2149112ns | 2143629ns | 2153089ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 882362ns | 881451ns | 883398ns | -58.94% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27715.6 | 2456.9 | 2357.5 | n/a |
| abi_soa_win_real_scalar_payload | 43040.0 | 2147745.8 | 2149112.5 | n/a |
| abi_soa_win_real_soa_payload | 32359.1 | 882135.9 | 882361.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.014 | 96.7% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 4725ns | 4725ns | -99.78% |
| abi_soa_win_real_scalar_payload | 2151826ns | 2151826ns | base |
| abi_soa_win_real_soa_payload | 884735ns | 884735ns | -58.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2150536ns | base | --- | [2143713, 2153089] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2367ns | -2148175.2ns (-99.9%) | [-2150775, -2141315]ns | [2307, 2398] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 882034ns | -1268110.9ns (-59.0%) | [-1270125, -1262017]ns | [881653, 883398] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2151970ns | -99.9% | -59.0% |
| 2 | 2149102ns | -99.9% | -58.9% |
| 3 | 2154060ns | -99.9% | -59.0% |
| 4 | 2143797ns | -99.9% | -58.9% |
| 5 | 2152118ns | -99.9% | -59.0% |
| 6 | 2143629ns | -99.9% | -58.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.747 | HIGH- (thermal bounce) |
| abi_soa_win_real_scalar_payload | -0.588 | HIGH- (thermal bounce) |
| abi_soa_win_real_soa_payload | -0.080 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 117056.6ns | 2357.5ns | 4965.4% | HIGH |
| abi_soa_win_real_scalar_payload | 6491775.6ns | 2149112.5ns | 302.1% | HIGH |
| abi_soa_win_real_soa_payload | 2679250.6ns | 882361.7ns | 303.6% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2289.6-2398.1 ns)
   2289.6 |####################
   2295.0 |
   2300.5 |
   2305.9 |
   2311.3 |
   2316.7 |
   2322.2 |####################
   2327.6 |
   2333.0 |
   2338.4 |####################
   2343.9 |
   2349.3 |
   2354.7 |
   2360.2 |
   2365.6 |
   2371.0 |
   2376.4 |
   2381.9 |
   2387.3 |
   2392.7 |########################################
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2143628.7-2153088.8 ns)
  2143628.7 |########################################
  2144101.7 |
  2144574.7 |
  2145047.7 |
  2145520.7 |
  2145993.7 |
  2146466.7 |
  2146939.7 |
  2147412.7 |
  2147885.7 |
  2148358.7 |
  2148831.7 |####################
  2149304.7 |
  2149777.7 |
  2150250.7 |
  2150723.7 |
  2151196.7 |
  2151669.7 |########################################
  2152142.7 |
  2152615.7 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 881451.2-883397.9 ns)
  881451.2 |########################################
  881548.5 |
  881645.9 |
  881743.2 |
  881840.5 |########################################
  881937.9 |########################################
  882035.2 |########################################
  882132.5 |
  882229.9 |
  882327.2 |
  882424.6 |
  882521.9 |
  882619.2 |
  882716.6 |########################################
  882813.9 |
  882911.2 |
  883008.6 |
  883105.9 |
  883203.2 |
  883300.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4938.1% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=303.7% of algo (FFI overhead may distort results)

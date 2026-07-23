# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.41 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 3.22 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 29963% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (3.22 us) leads abi_soa_win_real_soa_payload (969.11 us) by 29963%, a clear separation rather than a photo finish. CV 4.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.41 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 749.0x slower than the field

abi_soa_win_real_scalar_payload (2.41 ms) is 749.0x the fastest (3.22 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_scalar_payload shows alternating (throttle bounce) (autocorr -0.55)

abi_soa_win_real_scalar_payload's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 749.0x the fastest

Fastest abi_soa_win_real_null_entry (3.22 us) to slowest abi_soa_win_real_scalar_payload (2.41 ms): 749.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 3223.6 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 748.99x (fastest 3223.6 ns, slowest 2414398.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5803ns | 5802ns | 5407ns | 5690ns | 6170ns | -99.76% |
| abi_soa_win_real_scalar_payload | 2435262ns | 2419066ns | 2309374ns | 2384217ns | 2574774ns | base |
| abi_soa_win_real_soa_payload | 975688ns | 972602ns | 944652ns | 966238ns | 1005381ns | -59.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 3262ns | 3084ns | 3456ns | -99.87% | 0.002 |
| abi_soa_win_real_scalar_payload | 2430711ns | 2305503ns | 2569825ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 972085ns | 940893ns | 1001534ns | -60.01% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 34703.6 | 3478.5 | 3261.6 | n/a |
| abi_soa_win_real_scalar_payload | 95936.1 | 2447081.9 | 2430711.1 | n/a |
| abi_soa_win_real_soa_payload | 66161.0 | 967254.8 | 972085.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.002 | 95.7% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5803ns | 5803ns | -99.76% |
| abi_soa_win_real_scalar_payload | 2435262ns | 2435262ns | base |
| abi_soa_win_real_soa_payload | 975688ns | 975688ns | -59.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2414399ns | base | --- | [2307910, 2569825] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 3224ns | -2411293.3ns (-99.9%) | [-2566415, -2304640]ns | [3105, 3456] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 969106ns | -1454285.1ns (-60.2%) | [-1576723, -1344869]ns | [945617, 1001534] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2489487ns | -99.9% | -62.2% |
| 2 | 2310317ns | -99.9% | -58.9% |
| 3 | 2615526ns | -99.9% | -61.0% |
| 4 | 2339310ns | -99.9% | -58.0% |
| 5 | 2305503ns | -99.9% | -57.9% |
| 6 | 2524123ns | -99.9% | -61.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.520 | HIGH- (thermal bounce) |
| abi_soa_win_real_scalar_payload | -0.548 | HIGH- (thermal bounce) |
| abi_soa_win_real_soa_payload | 0.046 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 126221.9ns | 3261.6ns | 3869.9% | HIGH |
| abi_soa_win_real_scalar_payload | 7428226.3ns | 2430711.1ns | 305.6% | HIGH |
| abi_soa_win_real_soa_payload | 2968908.9ns | 972085.4ns | 305.4% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 3084.2-3456.1 ns)
   3084.2 |########################################
   3102.8 |
   3121.4 |########################################
   3140.0 |########################################
   3158.6 |
   3177.2 |
   3195.8 |
   3214.3 |
   3232.9 |
   3251.5 |
   3270.1 |
   3288.7 |########################################
   3307.3 |
   3325.9 |
   3344.5 |
   3363.1 |
   3381.7 |########################################
   3400.3 |
   3418.9 |
   3437.5 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2305503.3-2569824.5 ns)
  2305503.3 |########################################
  2318719.4 |
  2331935.4 |####################
  2345151.5 |
  2358367.5 |
  2371583.6 |
  2384799.7 |
  2398015.7 |
  2411231.8 |
  2424447.9 |
  2437663.9 |
  2450880.0 |
  2464096.0 |
  2477312.1 |####################
  2490528.2 |
  2503744.2 |
  2516960.3 |####################
  2530176.4 |
  2543392.4 |
  2556608.5 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 940892.9-1001533.8 ns)
  940892.9 |########################################
  943924.9 |
  946957.0 |
  949989.0 |########################################
  953021.1 |
  956053.1 |
  959085.2 |
  962117.2 |
  965149.2 |########################################
  968181.3 |
  971213.3 |########################################
  974245.4 |
  977277.4 |
  980309.5 |########################################
  983341.5 |
  986373.5 |
  989405.6 |
  992437.6 |
  995469.7 |
  998501.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=3875.7% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=307.2% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=305.0% of algo (FFI overhead may distort results)

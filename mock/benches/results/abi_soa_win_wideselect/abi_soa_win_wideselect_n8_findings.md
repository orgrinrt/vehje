# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 3.15 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 29330% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (3.15 us) leads abi_soa_win_wideselect_soa_payload (927.10 us) by 29330%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 653.6x slower than the field

abi_soa_win_wideselect_scalar_payload (2.06 ms) is 653.6x the fastest (3.15 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_wideselect_scalar_payload shows alternating (throttle bounce) (autocorr -0.63)

abi_soa_win_wideselect_scalar_payload's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 653.6x the fastest

Fastest abi_soa_win_wideselect_null_entry (3.15 us) to slowest abi_soa_win_wideselect_scalar_payload (2.06 ms): 653.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 3150.2 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 653.62x (fastest 3150.2 ns, slowest 2059040.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5508ns | 5529ns | 5326ns | 5495ns | 5619ns | -99.73% |
| abi_soa_win_wideselect_scalar_payload | 2062244ns | 2061596ns | 2055802ns | 2060159ns | 2068594ns | base |
| abi_soa_win_wideselect_soa_payload | 928564ns | 929430ns | 921572ns | 927660ns | 933415ns | -54.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 3148ns | 3058ns | 3211ns | -99.85% | 0.003 |
| abi_soa_win_wideselect_scalar_payload | 2059658ns | 2053331ns | 2065920ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 926146ns | 919231ns | 930874ns | -55.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 28678.5 | 3185.6 | 3147.8 | n/a |
| abi_soa_win_wideselect_scalar_payload | 41828.4 | 2062004.2 | 2059657.6 | n/a |
| abi_soa_win_wideselect_soa_payload | 34102.0 | 924272.6 | 926145.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.003 | 97.1% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 5508ns | 5508ns | -99.73% |
| abi_soa_win_wideselect_scalar_payload | 2062244ns | 2062244ns | base |
| abi_soa_win_wideselect_soa_payload | 928564ns | 928564ns | -54.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2059041ns | base | --- | [2054012, 2065920] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 3150ns | -2055855.6ns (-99.8%) | [-2062824, -2050850]ns | [3082, 3211] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 927097ns | -1136038.1ns (-55.2%) | [-1138910, -1125588]ns | [920466, 930874] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2053331ns | -99.8% | -55.2% |
| 2 | 2070163ns | -99.9% | -55.1% |
| 3 | 2054693ns | -99.8% | -54.7% |
| 4 | 2058231ns | -99.8% | -54.8% |
| 5 | 2061678ns | -99.8% | -55.2% |
| 6 | 2059850ns | -99.8% | -55.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | -0.122 | ok |
| abi_soa_win_wideselect_scalar_payload | -0.629 | HIGH- (thermal bounce) |
| abi_soa_win_wideselect_soa_payload | 0.092 | ok |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 120938.4ns | 3147.8ns | 3842.0% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6225882.9ns | 2059657.6ns | 302.3% | HIGH |
| abi_soa_win_wideselect_soa_payload | 2808054.2ns | 926145.7ns | 303.2% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 3057.9-3211.2 ns)
   3057.9 |########################################
   3065.6 |
   3073.2 |
   3080.9 |
   3088.6 |
   3096.2 |
   3103.9 |########################################
   3111.6 |
   3119.2 |
   3126.9 |
   3134.6 |########################################
   3142.2 |
   3149.9 |
   3157.6 |########################################
   3165.2 |
   3172.9 |
   3180.6 |
   3188.2 |
   3195.9 |
   3203.6 |########################################
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2053330.8-2065920.4 ns)
  2053330.8 |########################################
  2053960.3 |
  2054589.8 |########################################
  2055219.2 |
  2055848.7 |
  2056478.2 |
  2057107.7 |
  2057737.2 |########################################
  2058366.6 |
  2058996.1 |
  2059625.6 |########################################
  2060255.1 |
  2060884.6 |
  2061514.0 |########################################
  2062143.5 |
  2062773.0 |
  2063402.5 |
  2064032.0 |
  2064661.4 |
  2065290.9 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 919230.8-930873.7 ns)
  919230.8 |####################
  919812.9 |
  920395.1 |
  920977.2 |
  921559.4 |####################
  922141.5 |
  922723.7 |
  923305.8 |####################
  923888.0 |
  924470.1 |
  925052.2 |
  925634.4 |
  926216.5 |
  926798.7 |
  927380.8 |
  927963.0 |
  928545.1 |
  929127.3 |
  929709.4 |
  930291.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=3840.3% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=302.9% of algo (FFI overhead may distort results)

# abi_native_cross (leaf)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_leaf_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_leaf_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_leaf_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_leaf_native_ffi_w has the worst median (12.01 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_leaf_null_entry at 2.46 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_leaf_null_entry dominates: 370% faster than the next best (abi_native_cross_leaf_inproc_native)

abi_native_cross_leaf_null_entry (2.46 us) leads abi_native_cross_leaf_inproc_native (11.58 us) by 370%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_leaf_null_entry beats baseline by 79% (significant)

abi_native_cross_leaf_null_entry is -9.53 us (79%) faster than baseline abi_native_cross_leaf_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_leaf_native_ffi_w is an outlier: 4.9x slower than the field

abi_native_cross_leaf_native_ffi_w (12.01 us) is 4.9x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_leaf_inproc_native shows alternating (throttle bounce) (autocorr -0.70)

abi_native_cross_leaf_inproc_native's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.9x the fastest

Fastest abi_native_cross_leaf_null_entry (2.46 us) to slowest abi_native_cross_leaf_native_ffi_w (12.01 us): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_leaf_null_entry** at 2463.8 ns median (-79.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.87x (fastest 2463.8 ns, slowest 12005.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13879ns | 13938ns | 13480ns | 13865ns | 14100ns | -3.45% |
| abi_native_cross_leaf_native_ffi_w | 14375ns | 14316ns | 14198ns | 14285ns | 14600ns | base |
| abi_native_cross_leaf_null_entry | 4755ns | 4718ns | 4643ns | 4714ns | 4871ns | -66.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 11560ns | 11231ns | 11786ns | -4.24% | 0.006 |
| abi_native_cross_leaf_native_ffi_w | 12072ns | 11955ns | 12250ns | base | 0.005 |
| abi_native_cross_leaf_null_entry | 2469ns | 2406ns | 2523ns | -79.55% | 0.026 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 5638.2 | 11595.4 | 11559.6 | n/a |
| abi_native_cross_leaf_native_ffi_w | 25199.4 | 12248.3 | 12072.1 | n/a |
| abi_native_cross_leaf_null_entry | 27172.4 | 2709.0 | 2468.7 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_native_cross_leaf_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_leaf_inproc_native | 0.006 | 20.8% |
| abi_native_cross_leaf_native_ffi_w | 0.005 | 20.0% |
| abi_native_cross_leaf_null_entry | 0.026 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 13879ns | 13879ns | -3.45% |
| abi_native_cross_leaf_native_ffi_w | 14375ns | 14375ns | base |
| abi_native_cross_leaf_null_entry | 4755ns | 4755ns | -66.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_leaf_native_ffi_w | 12005ns | base | --- | [11961, 12250] | --- | --- | --- | --- |
| abi_native_cross_leaf_inproc_native | 11580ns | -440.7ns (-3.7%) | [-843, -253]ns | [11313, 11786] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_leaf_null_entry | 2464ns | -9530.2ns (-79.4%) | [-9776, -9504]ns | [2420, 2523] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_leaf_native_ffi_w | abi_native_cross_leaf_inproc_native | abi_native_cross_leaf_null_entry |
|---|---|---|---|
| 1 | 11955ns | -2.2% | -79.6% |
| 2 | 12377ns | -7.1% | -80.6% |
| 3 | 11967ns | -4.8% | -79.3% |
| 4 | 12123ns | -2.0% | -79.0% |
| 5 | 12035ns | -6.7% | -79.2% |
| 6 | 11976ns | -2.6% | -79.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_leaf_inproc_native | -0.704 | HIGH- (thermal bounce) |
| abi_native_cross_leaf_native_ffi_w | -0.545 | HIGH- (thermal bounce) |
| abi_native_cross_leaf_null_entry | 0.334 | moderate+ |

**Consistency summary:**

- **abi_native_cross_leaf_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_leaf_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_leaf_inproc_native | 121255.5ns | 11559.6ns | 1049.0% | HIGH |
| abi_native_cross_leaf_native_ffi_w | 140199.1ns | 12072.1ns | 1161.3% | HIGH |
| abi_native_cross_leaf_null_entry | 112380.3ns | 2468.7ns | 4552.3% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_leaf_inproc_native (n=6, range 11230.8-11786.0 ns)
  11230.8 |########################################
  11258.6 |
  11286.3 |
  11314.1 |
  11341.8 |
  11369.6 |########################################
  11397.4 |
  11425.1 |
  11452.9 |
  11480.7 |########################################
  11508.4 |
  11536.2 |
  11563.9 |
  11591.7 |
  11619.5 |
  11647.2 |########################################
  11675.0 |########################################
  11702.8 |
  11730.5 |
  11758.3 |
  (0 below, 1 above range)

abi_native_cross_leaf_native_ffi_w (n=6, range 11955.4-12250.0 ns)
  11955.4 |########################################
  11970.1 |####################
  11984.9 |
  11999.6 |
  12014.3 |
  12029.0 |####################
  12043.8 |
  12058.5 |
  12073.2 |
  12088.0 |
  12102.7 |
  12117.4 |####################
  12132.2 |
  12146.9 |
  12161.6 |
  12176.4 |
  12191.1 |
  12205.8 |
  12220.5 |
  12235.3 |
  (0 below, 1 above range)

abi_native_cross_leaf_null_entry (n=6, range 2405.8-2522.7 ns)
   2405.8 |########################################
   2411.6 |
   2417.5 |
   2423.3 |
   2429.2 |########################################
   2435.0 |
   2440.9 |
   2446.7 |########################################
   2452.6 |
   2458.4 |
   2464.2 |
   2470.1 |
   2475.9 |########################################
   2481.8 |
   2487.6 |
   2493.5 |
   2499.3 |########################################
   2505.2 |
   2511.0 |
   2516.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_leaf_inproc_native**: bridge=1059.2% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_native_ffi_w**: bridge=1160.5% of algo (FFI overhead may distort results)
- **abi_native_cross_leaf_null_entry**: bridge=4558.8% of algo (FFI overhead may distort results)

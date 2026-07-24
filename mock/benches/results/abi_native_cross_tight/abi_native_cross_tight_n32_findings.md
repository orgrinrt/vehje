# abi_native_cross (tight)

3 variants, 6 samples per variant.
Baseline: **abi_native_cross_tight_native_ffi_w**

## Highlights

Baseline for all deltas below: **abi_native_cross_tight_native_ffi_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_native_cross_tight_native_ffi_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_native_cross_tight_native_ffi_w has the worst median (12.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_native_cross_tight_null_entry at 2.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_native_cross_tight_null_entry dominates: 412% faster than the next best (abi_native_cross_tight_inproc_native)

abi_native_cross_tight_null_entry (2.25 us) leads abi_native_cross_tight_inproc_native (11.51 us) by 412%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_native_cross_tight_null_entry beats baseline by 82% (significant)

abi_native_cross_tight_null_entry is -10.17 us (82%) faster than baseline abi_native_cross_tight_native_ffi_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_native_cross_tight_native_ffi_w is an outlier: 5.5x slower than the field

abi_native_cross_tight_native_ffi_w (12.40 us) is 5.5x the fastest (2.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_native_cross_tight_inproc_native shows alternating (throttle bounce) (autocorr -0.59)

abi_native_cross_tight_inproc_native's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest abi_native_cross_tight_null_entry (2.25 us) to slowest abi_native_cross_tight_native_ffi_w (12.40 us): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_native_cross_tight_null_entry** at 2250.7 ns median (-81.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.51x (fastest 2250.7 ns, slowest 12399.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13783ns | 13808ns | 13424ns | 13746ns | 14019ns | -6.07% |
| abi_native_cross_tight_native_ffi_w | 14674ns | 14669ns | 14295ns | 14638ns | 14917ns | base |
| abi_native_cross_tight_null_entry | 4478ns | 4469ns | 4386ns | 4456ns | 4557ns | -69.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 11488ns | 11150ns | 11719ns | -7.21% | 0.003 |
| abi_native_cross_tight_native_ffi_w | 12380ns | 12088ns | 12553ns | base | 0.003 |
| abi_native_cross_tight_null_entry | 2257ns | 2200ns | 2312ns | -81.77% | 0.014 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 5416.9 | 11538.7 | 11487.5 | n/a |
| abi_native_cross_tight_native_ffi_w | 25698.8 | 12560.9 | 12380.0 | n/a |
| abi_native_cross_tight_null_entry | 26274.1 | 2374.0 | 2257.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.015 Gops/s** (abi_native_cross_tight_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_native_cross_tight_inproc_native | 0.003 | 19.1% |
| abi_native_cross_tight_native_ffi_w | 0.003 | 17.7% |
| abi_native_cross_tight_null_entry | 0.014 | 97.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_native_cross_tight_inproc_native | 13783ns | 13783ns | -6.07% |
| abi_native_cross_tight_native_ffi_w | 14674ns | 14674ns | base |
| abi_native_cross_tight_null_entry | 4478ns | 4478ns | -69.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_native_cross_tight_native_ffi_w | 12400ns | base | --- | [12187, 12553] | --- | --- | --- | --- |
| abi_native_cross_tight_inproc_native | 11513ns | -859.1ns (-6.9%) | [-1186, -633]ns | [11231, 11719] | YES | 0.0313 | 0.0313 | 0 |
| abi_native_cross_tight_null_entry | 2251ns | -10171.0ns (-82.0%) | [-10322, -9875]ns | [2209, 2312] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_native_cross_tight_native_ffi_w | abi_native_cross_tight_inproc_native | abi_native_cross_tight_null_entry |
|---|---|---|---|
| 1 | 12383ns | -6.3% | -82.1% |
| 2 | 12680ns | -9.9% | -82.2% |
| 3 | 12088ns | -7.8% | -80.7% |
| 4 | 12286ns | -4.3% | -81.4% |
| 5 | 12426ns | -9.0% | -81.9% |
| 6 | 12417ns | -6.0% | -82.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_native_cross_tight_inproc_native | -0.586 | HIGH- (thermal bounce) |
| abi_native_cross_tight_native_ffi_w | -0.331 | moderate- |
| abi_native_cross_tight_null_entry | 0.212 | moderate+ |

**Consistency summary:**

- **abi_native_cross_tight_inproc_native**: won 6/6, lost 0/6
- **abi_native_cross_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_native_cross_tight_inproc_native | 121795.6ns | 11487.5ns | 1060.2% | HIGH |
| abi_native_cross_tight_native_ffi_w | 142235.4ns | 12380.0ns | 1148.9% | HIGH |
| abi_native_cross_tight_null_entry | 114496.3ns | 2257.4ns | 5072.1% | HIGH |

## Distribution (algo ns)

```
abi_native_cross_tight_inproc_native (n=6, range 11150.0-11719.0 ns)
  11150.0 |########################################
  11178.4 |
  11206.9 |
  11235.3 |
  11263.8 |
  11292.2 |########################################
  11320.7 |
  11349.1 |
  11377.6 |
  11406.0 |########################################
  11434.5 |
  11462.9 |
  11491.4 |
  11519.8 |
  11548.3 |
  11576.7 |########################################
  11605.2 |
  11633.6 |
  11662.1 |########################################
  11690.5 |
  (0 below, 1 above range)

abi_native_cross_tight_native_ffi_w (n=6, range 12087.5-12553.3 ns)
  12087.5 |####################
  12110.8 |
  12134.1 |
  12157.4 |
  12180.7 |
  12204.0 |
  12227.2 |
  12250.5 |
  12273.8 |####################
  12297.1 |
  12320.4 |
  12343.7 |
  12367.0 |####################
  12390.3 |
  12413.6 |########################################
  12436.8 |
  12460.1 |
  12483.4 |
  12506.7 |
  12530.0 |
  (0 below, 1 above range)

abi_native_cross_tight_null_entry (n=6, range 2200.4-2312.1 ns)
   2200.4 |########################################
   2206.0 |
   2211.6 |
   2217.2 |########################################
   2222.7 |
   2228.3 |
   2233.9 |
   2239.5 |
   2245.1 |########################################
   2250.7 |########################################
   2256.2 |
   2261.8 |
   2267.4 |
   2273.0 |
   2278.6 |
   2284.2 |########################################
   2289.8 |
   2295.3 |
   2300.9 |
   2306.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_native_cross_tight_inproc_native**: bridge=1060.7% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_native_ffi_w**: bridge=1142.0% of algo (FFI overhead may distort results)
- **abi_native_cross_tight_null_entry**: bridge=5061.9% of algo (FFI overhead may distort results)

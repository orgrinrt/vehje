# abi_soa_win (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_wideselect_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_wideselect_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_wideselect_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_wideselect_scalar_payload has the worst median (2.07 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_wideselect_null_entry at 4.04 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_wideselect_null_entry dominates: 50851% faster than the next best (abi_soa_win_wideselect_soa_payload)

abi_soa_win_wideselect_null_entry (4.04 us) leads abi_soa_win_wideselect_soa_payload (2.06 ms) by 50851%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_wideselect_null_entry beats baseline by 100% (significant)

abi_soa_win_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_soa_win_wideselect_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_wideselect_scalar_payload is an outlier: 511.5x slower than the field

abi_soa_win_wideselect_scalar_payload (2.07 ms) is 511.5x the fastest (4.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_wideselect_soa_payload shows alternating (throttle bounce) (autocorr -0.56)

abi_soa_win_wideselect_soa_payload's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 511.5x the fastest

Fastest abi_soa_win_wideselect_null_entry (4.04 us) to slowest abi_soa_win_wideselect_scalar_payload (2.07 ms): 511.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_wideselect_null_entry** at 4041.4 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 511.52x (fastest 4041.4 ns, slowest 2067269.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 6425ns | 6365ns | 6282ns | 6354ns | 6605ns | -99.70% |
| abi_soa_win_wideselect_scalar_payload | 2114711ns | 2070016ns | 2061434ns | 2067311ns | 2212449ns | base |
| abi_soa_win_wideselect_soa_payload | 2060990ns | 2061673ns | 2055427ns | 2061321ns | 2063275ns | -2.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 4094ns | 3992ns | 4232ns | -99.81% | 0.001 |
| abi_soa_win_wideselect_scalar_payload | 2111891ns | 2058916ns | 2209242ns | base | 0.000 |
| abi_soa_win_wideselect_soa_payload | 2058433ns | 2052854ns | 2060632ns | -2.53% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 29072.1 | 4229.9 | 4093.5 | n/a |
| abi_soa_win_wideselect_scalar_payload | 48551.7 | 2164012.8 | 2111891.2 | n/a |
| abi_soa_win_wideselect_soa_payload | 39884.5 | 2058424.6 | 2058433.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_soa_win_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_wideselect_null_entry | 0.001 | 98.8% |
| abi_soa_win_wideselect_scalar_payload | 0.000 | 0.2% |
| abi_soa_win_wideselect_soa_payload | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 6425ns | 6425ns | -99.70% |
| abi_soa_win_wideselect_scalar_payload | 2114711ns | 2114711ns | base |
| abi_soa_win_wideselect_soa_payload | 2060990ns | 2060990ns | -2.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_wideselect_scalar_payload | 2067269ns | base | --- | [2059163, 2209242] | --- | --- | --- | --- |
| abi_soa_win_wideselect_null_entry | 4041ns | -2063235.4ns (-99.8%) | [-2205140, -2055018]ns | [4007, 4232] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_soa_win_wideselect_soa_payload | 2059147ns | -7309.0ns (-0.4%) | [-150919, -2146]ns | [2055522, 2060632] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_wideselect_scalar_payload | abi_soa_win_wideselect_null_entry | abi_soa_win_wideselect_soa_payload |
|---|---|---|---|
| 1 | 2066987ns | -99.8% | -0.3% |
| 2 | 2343774ns | -99.8% | -12.2% |
| 3 | 2058916ns | -99.8% | +0.1% |
| 4 | 2059410ns | -99.8% | -0.3% |
| 5 | 2067551ns | -99.8% | -0.4% |
| 6 | 2074710ns | -99.8% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_wideselect_null_entry | -0.346 | moderate- |
| abi_soa_win_wideselect_scalar_payload | -0.246 | moderate- |
| abi_soa_win_wideselect_soa_payload | -0.557 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_soa_win_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_wideselect_soa_payload**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_wideselect_null_entry | 123401.9ns | 4093.5ns | 3014.5% | HIGH |
| abi_soa_win_wideselect_scalar_payload | 6449521.9ns | 2111891.2ns | 305.4% | HIGH |
| abi_soa_win_wideselect_soa_payload | 6222365.7ns | 2058433.3ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_wideselect_null_entry (n=6, range 3991.7-4232.3 ns)
   3991.7 |########################################
   4003.7 |
   4015.8 |########################################
   4027.8 |########################################
   4039.8 |########################################
   4051.8 |
   4063.9 |
   4075.9 |
   4087.9 |
   4100.0 |
   4112.0 |
   4124.0 |
   4136.1 |
   4148.1 |
   4160.1 |
   4172.1 |
   4184.2 |
   4196.2 |
   4208.2 |########################################
   4220.3 |
  (0 below, 1 above range)

abi_soa_win_wideselect_scalar_payload (n=6, range 2058916.2-2209241.7 ns)
  2058916.2 |########################################
  2066432.5 |########################################
  2073948.8 |####################
  2081465.0 |
  2088981.3 |
  2096497.6 |
  2104013.9 |
  2111530.1 |
  2119046.4 |
  2126562.7 |
  2134079.0 |
  2141595.2 |
  2149111.5 |
  2156627.8 |
  2164144.1 |
  2171660.3 |
  2179176.6 |
  2186692.9 |
  2194209.2 |
  2201725.4 |
  (0 below, 1 above range)

abi_soa_win_wideselect_soa_payload (n=6, range 2052854.2-2060631.6 ns)
  2052854.2 |########################################
  2053243.1 |
  2053631.9 |
  2054020.8 |
  2054409.7 |
  2054798.6 |
  2055187.4 |
  2055576.3 |
  2055965.2 |
  2056354.1 |
  2056742.9 |
  2057131.8 |
  2057520.7 |
  2057909.5 |########################################
  2058298.4 |########################################
  2058687.3 |
  2059076.2 |
  2059465.0 |########################################
  2059853.9 |########################################
  2060242.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_wideselect_null_entry**: bridge=3060.5% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_scalar_payload**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_soa_win_wideselect_soa_payload**: bridge=302.2% of algo (FFI overhead may distort results)

# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_wideselect_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_wideselect_reused_buffer has the worst median (2.05 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_wideselect_null_entry at 2.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_wideselect_null_entry dominates: 79592% faster than the next best (abi_residency_wideselect_fresh_alloc)

abi_residency_wideselect_null_entry (2.58 us) leads abi_residency_wideselect_fresh_alloc (2.05 ms) by 79592%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_reused_buffer is an outlier: 797.3x slower than the field

abi_residency_wideselect_reused_buffer (2.05 ms) is 797.3x the fastest (2.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 797.3x the fastest

Fastest abi_residency_wideselect_null_entry (2.58 us) to slowest abi_residency_wideselect_reused_buffer (2.05 ms): 797.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 2577.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 797.32x (fastest 2577.1 ns, slowest 2054740.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2056024ns | 2056400ns | 2047931ns | 2055777ns | 2060440ns | +0.08% |
| abi_residency_wideselect_null_entry | 4895ns | 4907ns | 4671ns | 4887ns | 5019ns | -99.76% |
| abi_residency_wideselect_reused_buffer | 2054330ns | 2057434ns | 2038081ns | 2056178ns | 2059684ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2053387ns | 2045108ns | 2057905ns | +0.08% | 0.000 |
| abi_residency_wideselect_null_entry | 2579ns | 2460ns | 2644ns | -99.87% | 0.006 |
| abi_residency_wideselect_reused_buffer | 2051740ns | 2035615ns | 2057057ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 40781.3 | 2053770.2 | 2053387.4 | 0 |
| abi_residency_wideselect_null_entry | 28443.2 | 2686.0 | 2578.9 | n/a |
| abi_residency_wideselect_reused_buffer | 41577.2 | 2050359.5 | 2051740.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.007 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.1% |
| abi_residency_wideselect_null_entry | 0.006 | 95.5% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2056024ns | 2056024ns | +0.08% |
| abi_residency_wideselect_null_entry | 4895ns | 4895ns | -99.76% |
| abi_residency_wideselect_reused_buffer | 2054330ns | 2054330ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2054740ns | base | --- | [2043425, 2057057] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2053696ns | no significant difference | [-6893, +10960]ns | [2048561, 2057905] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_wideselect_null_entry | 2577ns | -2052224.6ns (-99.9%) | [-2054412, -2040848]ns | [2515, 2644] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2053718ns | -0.4% | -99.9% |
| 2 | 2057191ns | -0.3% | -99.9% |
| 3 | 2035615ns | +0.9% | -99.9% |
| 4 | 2056922ns | -0.0% | -99.9% |
| 5 | 2055762ns | +0.2% | -99.9% |
| 6 | 2051234ns | +0.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.266 | moderate+ |
| abi_residency_wideselect_null_entry | -0.336 | moderate- |
| abi_residency_wideselect_reused_buffer | -0.421 | moderate- |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6202702.8ns | 2053387.4ns | 302.1% | HIGH |
| abi_residency_wideselect_null_entry | 117971.9ns | 2578.9ns | 4574.5% | HIGH |
| abi_residency_wideselect_reused_buffer | 6197702.0ns | 2051740.5ns | 302.1% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2045107.5-2057905.4 ns)
  2045107.5 |####################
  2045747.4 |
  2046387.3 |
  2047027.2 |
  2047667.1 |
  2048307.0 |
  2048946.9 |
  2049586.8 |
  2050226.7 |
  2050866.6 |
  2051506.5 |####################
  2052146.4 |
  2052786.3 |
  2053426.2 |########################################
  2054066.1 |
  2054706.0 |
  2055345.9 |
  2055985.8 |####################
  2056625.7 |
  2057265.6 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 2460.4-2644.4 ns)
   2460.4 |####################
   2469.6 |
   2478.8 |
   2488.0 |
   2497.2 |
   2506.4 |
   2515.6 |
   2524.8 |
   2534.0 |
   2543.2 |
   2552.4 |
   2561.6 |########################################
   2570.8 |
   2580.0 |####################
   2589.2 |
   2598.4 |
   2607.6 |
   2616.8 |####################
   2626.0 |
   2635.2 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2035615.4-2057056.6 ns)
  2035615.4 |########################################
  2036687.5 |
  2037759.5 |
  2038831.6 |
  2039903.6 |
  2040975.7 |
  2042047.8 |
  2043119.8 |
  2044191.9 |
  2045264.0 |
  2046336.0 |
  2047408.1 |
  2048480.1 |
  2049552.2 |
  2050624.3 |########################################
  2051696.3 |
  2052768.4 |########################################
  2053840.5 |
  2054912.5 |########################################
  2055984.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=4568.7% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=301.9% of algo (FFI overhead may distort results)

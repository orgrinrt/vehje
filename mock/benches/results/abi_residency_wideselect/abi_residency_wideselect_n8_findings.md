# abi_residency (wideselect)

3 variants, 6 samples per variant.
Baseline: **abi_residency_wideselect_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_wideselect_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_wideselect_null_entry dominates: 67162% faster than the next best (abi_residency_wideselect_reused_buffer)

abi_residency_wideselect_null_entry (3.05 us) leads abi_residency_wideselect_reused_buffer (2.05 ms) by 67162%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_wideselect_null_entry beats baseline by 100% (significant)

abi_residency_wideselect_null_entry is -2.05 ms (100%) faster than baseline abi_residency_wideselect_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_wideselect_fresh_alloc is an outlier: 674.4x slower than the field

abi_residency_wideselect_fresh_alloc (2.06 ms) is 674.4x the fastest (3.05 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 674.4x the fastest

Fastest abi_residency_wideselect_null_entry (3.05 us) to slowest abi_residency_wideselect_fresh_alloc (2.06 ms): 674.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_wideselect_null_entry** at 3048.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 674.36x (fastest 3048.8 ns, slowest 2055965.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2057860ns | 2058560ns | 2047580ns | 2055491ns | 2066555ns | +0.26% |
| abi_residency_wideselect_null_entry | 5370ns | 5359ns | 5198ns | 5316ns | 5537ns | -99.74% |
| abi_residency_wideselect_reused_buffer | 2052522ns | 2053282ns | 2045301ns | 2051998ns | 2056919ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2055269ns | 2045130ns | 2063930ns | +0.26% | 0.000 |
| abi_residency_wideselect_null_entry | 3076ns | 2968ns | 3184ns | -99.85% | 0.003 |
| abi_residency_wideselect_reused_buffer | 2049928ns | 2042728ns | 2054322ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 42860.2 | 2053916.0 | 2055269.0 | n/a |
| abi_residency_wideselect_null_entry | 27730.5 | 3146.8 | 3075.6 | n/a |
| abi_residency_wideselect_reused_buffer | 41485.9 | 2054340.7 | 2049927.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.003 Gops/s** (abi_residency_wideselect_null_entry; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.000 | 0.1% |
| abi_residency_wideselect_null_entry | 0.003 | 97.3% |
| abi_residency_wideselect_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 2057860ns | 2057860ns | +0.26% |
| abi_residency_wideselect_null_entry | 5370ns | 5370ns | -99.74% |
| abi_residency_wideselect_reused_buffer | 2052522ns | 2052522ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_wideselect_reused_buffer | 2050659ns | base | --- | [2044802, 2054322] | --- | --- | --- | --- |
| abi_residency_wideselect_fresh_alloc | 2055965ns | no significant difference | [-3621, +13153]ns | [2045912, 2063930] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_wideselect_null_entry | 3049ns | -2047665.2ns (-99.9%) | [-2051193, -2041698]ns | [2994, 3184] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_wideselect_reused_buffer | abi_residency_wideselect_fresh_alloc | abi_residency_wideselect_null_entry |
|---|---|---|---|
| 1 | 2042728ns | +0.6% | -99.9% |
| 2 | 2050760ns | +0.3% | -99.9% |
| 3 | 2056455ns | +0.7% | -99.9% |
| 4 | 2050558ns | +0.3% | -99.9% |
| 5 | 2052189ns | -0.3% | -99.8% |
| 6 | 2046877ns | -0.0% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_wideselect_fresh_alloc | 0.295 | moderate+ |
| abi_residency_wideselect_null_entry | 0.283 | moderate+ |
| abi_residency_wideselect_reused_buffer | -0.017 | ok |

**Consistency summary:**

- **abi_residency_wideselect_fresh_alloc**: won 1/6, lost 4/6
- **abi_residency_wideselect_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_wideselect_fresh_alloc | 6210873.8ns | 2055269.0ns | 302.2% | HIGH |
| abi_residency_wideselect_null_entry | 119496.9ns | 3075.6ns | 3885.4% | HIGH |
| abi_residency_wideselect_reused_buffer | 6198432.0ns | 2049927.9ns | 302.4% | HIGH |

## Distribution (algo ns)

```
abi_residency_wideselect_fresh_alloc (n=6, range 2045129.6-2063929.8 ns)
  2045129.6 |####################
  2046069.6 |####################
  2047009.6 |
  2047949.6 |
  2048889.6 |
  2049829.6 |
  2050769.7 |
  2051709.7 |
  2052649.7 |
  2053589.7 |
  2054529.7 |####################
  2055469.7 |
  2056409.7 |########################################
  2057349.7 |
  2058289.7 |
  2059229.8 |
  2060169.8 |
  2061109.8 |
  2062049.8 |
  2062989.8 |
  (0 below, 1 above range)

abi_residency_wideselect_null_entry (n=6, range 2967.9-3183.9 ns)
   2967.9 |########################################
   2978.7 |
   2989.5 |
   3000.3 |
   3011.1 |########################################
   3021.9 |
   3032.7 |########################################
   3043.5 |
   3054.3 |########################################
   3065.1 |
   3075.9 |
   3086.7 |
   3097.5 |
   3108.3 |
   3119.1 |
   3129.9 |
   3140.7 |
   3151.5 |
   3162.3 |########################################
   3173.1 |
  (0 below, 1 above range)

abi_residency_wideselect_reused_buffer (n=6, range 2042728.3-2054322.1 ns)
  2042728.3 |####################
  2043308.0 |
  2043887.7 |
  2044467.4 |
  2045047.1 |
  2045626.8 |
  2046206.4 |
  2046786.1 |####################
  2047365.8 |
  2047945.5 |
  2048525.2 |
  2049104.9 |
  2049684.6 |
  2050264.3 |########################################
  2050844.0 |
  2051423.7 |
  2052003.3 |####################
  2052583.0 |
  2053162.7 |
  2053742.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_wideselect_fresh_alloc**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_null_entry**: bridge=3925.6% of algo (FFI overhead may distort results)
- **abi_residency_wideselect_reused_buffer**: bridge=302.5% of algo (FFI overhead may distort results)

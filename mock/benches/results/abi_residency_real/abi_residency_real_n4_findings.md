# abi_residency (real)

3 variants, 6 samples per variant.
Baseline: **abi_residency_real_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_real_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_residency_real_reused_buffer) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_residency_real_reused_buffer has the worst median (2.14 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_residency_real_null_entry at 3.99 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_residency_real_null_entry dominates: 53459% faster than the next best (abi_residency_real_fresh_alloc)

abi_residency_real_null_entry (3.99 us) leads abi_residency_real_fresh_alloc (2.14 ms) by 53459%, a clear separation rather than a photo finish. CV 4.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_real_null_entry beats baseline by 100% (significant)

abi_residency_real_null_entry is -2.14 ms (100%) faster than baseline abi_residency_real_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_real_reused_buffer is an outlier: 536.4x slower than the field

abi_residency_real_reused_buffer (2.14 ms) is 536.4x the fastest (3.99 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 536.4x the fastest

Fastest abi_residency_real_null_entry (3.99 us) to slowest abi_residency_real_reused_buffer (2.14 ms): 536.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_real_null_entry** at 3993.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 536.38x (fastest 3993.9 ns, slowest 2142275.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2143362ns | 2141781ns | 2138008ns | 2140534ns | 2150281ns | +0.06% |
| abi_residency_real_null_entry | 6269ns | 6212ns | 5958ns | 6148ns | 6604ns | -99.71% |
| abi_residency_real_reused_buffer | 2142033ns | 2144792ns | 2129371ns | 2141698ns | 2148867ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 2140790ns | 2135538ns | 2147712ns | +0.06% | 0.000 |
| abi_residency_real_null_entry | 4011ns | 3809ns | 4226ns | -99.81% | 0.001 |
| abi_residency_real_reused_buffer | 2139517ns | 2126916ns | 2146275ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 39598.5 | 2142780.8 | 2140790.3 | n/a |
| abi_residency_real_null_entry | 26986.0 | 4148.8 | 4010.7 | n/a |
| abi_residency_real_reused_buffer | 38280.0 | 2140385.4 | 2139517.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_real_fresh_alloc | 0.000 | 0.2% |
| abi_residency_real_null_entry | 0.001 | 95.4% |
| abi_residency_real_reused_buffer | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_real_fresh_alloc | 2143362ns | 2143362ns | +0.06% |
| abi_residency_real_null_entry | 6269ns | 6269ns | -99.71% |
| abi_residency_real_reused_buffer | 2142033ns | 2142033ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_real_reused_buffer | 2142275ns | base | --- | [2130002, 2146275] | --- | --- | --- | --- |
| abi_residency_real_fresh_alloc | 2139103ns | no significant difference | [-8629, +13297]ns | [2135556, 2147712] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_real_null_entry | 3994ns | -2138049.8ns (-99.8%) | [-2142366, -2126105]ns | [3812, 4226] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_real_reused_buffer | abi_residency_real_fresh_alloc | abi_residency_real_null_entry |
|---|---|---|---|
| 1 | 2126916ns | +0.5% | -99.8% |
| 2 | 2146180ns | -0.5% | -99.8% |
| 3 | 2133088ns | +0.7% | -99.8% |
| 4 | 2146369ns | -0.3% | -99.8% |
| 5 | 2141893ns | -0.3% | -99.8% |
| 6 | 2142658ns | +0.2% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_real_fresh_alloc | -0.398 | moderate- |
| abi_residency_real_null_entry | 0.338 | moderate+ |
| abi_residency_real_reused_buffer | -0.479 | moderate- |

**Consistency summary:**

- **abi_residency_real_fresh_alloc**: won 3/6, lost 3/6
- **abi_residency_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_real_fresh_alloc | 6465790.3ns | 2140790.3ns | 302.0% | HIGH |
| abi_residency_real_null_entry | 121327.1ns | 4010.7ns | 3025.1% | HIGH |
| abi_residency_real_reused_buffer | 6462117.2ns | 2139517.5ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_residency_real_fresh_alloc (n=6, range 2135537.9-2147712.0 ns)
  2135537.9 |########################################
  2136146.6 |
  2136755.3 |
  2137364.0 |
  2137972.7 |####################
  2138581.4 |
  2139190.1 |####################
  2139798.9 |
  2140407.6 |
  2141016.3 |
  2141625.0 |
  2142233.7 |
  2142842.4 |
  2143451.1 |
  2144059.8 |
  2144668.5 |
  2145277.2 |
  2145885.9 |
  2146494.6 |
  2147103.3 |####################
  (0 below, 1 above range)

abi_residency_real_null_entry (n=6, range 3809.2-4225.6 ns)
   3809.2 |########################################
   3830.0 |
   3850.8 |
   3871.7 |
   3892.5 |
   3913.3 |
   3934.1 |
   3955.0 |
   3975.8 |####################
   3996.6 |####################
   4017.4 |
   4038.2 |
   4059.1 |
   4079.9 |####################
   4100.7 |
   4121.5 |
   4142.4 |
   4163.2 |
   4184.0 |
   4204.8 |
  (0 below, 1 above range)

abi_residency_real_reused_buffer (n=6, range 2126916.2-2146274.8 ns)
  2126916.2 |########################################
  2127884.1 |
  2128852.1 |
  2129820.0 |
  2130787.9 |
  2131755.9 |
  2132723.8 |########################################
  2133691.7 |
  2134659.6 |
  2135627.6 |
  2136595.5 |
  2137563.4 |
  2138531.4 |
  2139499.3 |
  2140467.2 |
  2141435.1 |########################################
  2142403.1 |########################################
  2143371.0 |
  2144338.9 |
  2145306.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_real_fresh_alloc**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_residency_real_null_entry**: bridge=3031.8% of algo (FFI overhead may distort results)
- **abi_residency_real_reused_buffer**: bridge=301.9% of algo (FFI overhead may distort results)

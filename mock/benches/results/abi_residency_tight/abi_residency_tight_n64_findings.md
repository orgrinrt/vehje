# abi_residency (tight)

3 variants, 6 samples per variant.
Baseline: **abi_residency_tight_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_tight_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_tight_null_entry dominates: 80491% faster than the next best (abi_residency_tight_reused_buffer)

abi_residency_tight_null_entry (2.48 us) leads abi_residency_tight_reused_buffer (2.00 ms) by 80491%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_tight_null_entry beats baseline by 100% (significant)

abi_residency_tight_null_entry is -2.00 ms (100%) faster than baseline abi_residency_tight_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_tight_fresh_alloc is an outlier: 806.1x slower than the field

abi_residency_tight_fresh_alloc (2.00 ms) is 806.1x the fastest (2.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 806.1x the fastest

Fastest abi_residency_tight_null_entry (2.48 us) to slowest abi_residency_tight_fresh_alloc (2.00 ms): 806.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_tight_null_entry** at 2482.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 806.11x (fastest 2482.9 ns, slowest 2001489.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2004105ns | 2004025ns | 1999342ns | 2003107ns | 2007983ns | +0.04% |
| abi_residency_tight_null_entry | 4759ns | 4754ns | 4637ns | 4718ns | 4880ns | -99.76% |
| abi_residency_tight_reused_buffer | 2003383ns | 2003710ns | 2000305ns | 2003216ns | 2005173ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2001537ns | 1996594ns | 2005376ns | +0.04% | 0.000 |
| abi_residency_tight_null_entry | 2491ns | 2413ns | 2567ns | -99.88% | 0.026 |
| abi_residency_tight_reused_buffer | 2000784ns | 1997714ns | 2002594ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 40564.0 | 2001988.6 | 2001536.6 | n/a |
| abi_residency_tight_null_entry | 27102.9 | 2735.1 | 2491.0 | n/a |
| abi_residency_tight_reused_buffer | 39767.5 | 2001862.1 | 2000784.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_residency_tight_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.000 | 0.1% |
| abi_residency_tight_null_entry | 0.026 | 97.2% |
| abi_residency_tight_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_tight_fresh_alloc | 2004105ns | 2004105ns | +0.04% |
| abi_residency_tight_null_entry | 4759ns | 4759ns | -99.76% |
| abi_residency_tight_reused_buffer | 2003383ns | 2003383ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_tight_reused_buffer | 2000991ns | base | --- | [1998767, 2002594] | --- | --- | --- | --- |
| abi_residency_tight_fresh_alloc | 2001489ns | no significant difference | [-4172, +4340]ns | [1997745, 2005376] | no | 0.6875 | 0.6875 | 0 |
| abi_residency_tight_null_entry | 2483ns | -1998508.8ns (-99.9%) | [-2000095, -1996275]ns | [2423, 2567] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_tight_reused_buffer | abi_residency_tight_fresh_alloc | abi_residency_tight_null_entry |
|---|---|---|---|
| 1 | 2002419ns | +0.1% | -99.9% |
| 2 | 1997714ns | +0.1% | -99.9% |
| 3 | 1999819ns | +0.1% | -99.9% |
| 4 | 2000918ns | +0.3% | -99.9% |
| 5 | 2001065ns | -0.1% | -99.9% |
| 6 | 2002769ns | -0.3% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_tight_fresh_alloc | 0.009 | ok |
| abi_residency_tight_null_entry | -0.214 | moderate- |
| abi_residency_tight_reused_buffer | -0.093 | ok |

**Consistency summary:**

- **abi_residency_tight_fresh_alloc**: won 2/6, lost 3/6
- **abi_residency_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_tight_fresh_alloc | 6047416.2ns | 2001536.6ns | 302.1% | HIGH |
| abi_residency_tight_null_entry | 112627.5ns | 2491.0ns | 4521.4% | HIGH |
| abi_residency_tight_reused_buffer | 6045529.7ns | 2000784.1ns | 302.2% | HIGH |

## Distribution (algo ns)

```
abi_residency_tight_fresh_alloc (n=6, range 1996594.2-2005376.1 ns)
  1996594.2 |########################################
  1997033.3 |
  1997472.4 |
  1997911.5 |
  1998350.6 |
  1998789.7 |########################################
  1999228.8 |
  1999667.8 |
  2000106.9 |
  2000546.0 |########################################
  2000985.1 |
  2001424.2 |
  2001863.3 |########################################
  2002302.4 |
  2002741.5 |
  2003180.6 |
  2003619.7 |
  2004058.8 |########################################
  2004497.9 |
  2004937.0 |
  (0 below, 1 above range)

abi_residency_tight_null_entry (n=6, range 2413.3-2566.9 ns)
   2413.3 |####################
   2421.0 |
   2428.7 |########################################
   2436.3 |
   2444.0 |
   2451.7 |
   2459.4 |
   2467.1 |
   2474.7 |
   2482.4 |
   2490.1 |
   2497.8 |
   2505.5 |
   2513.1 |
   2520.8 |
   2528.5 |####################
   2536.2 |
   2543.9 |
   2551.5 |
   2559.2 |####################
  (0 below, 1 above range)

abi_residency_tight_reused_buffer (n=6, range 1997714.2-2002594.0 ns)
  1997714.2 |####################
  1997958.2 |
  1998202.2 |
  1998446.2 |
  1998690.2 |
  1998934.1 |
  1999178.1 |
  1999422.1 |
  1999666.1 |####################
  1999910.1 |
  2000154.1 |
  2000398.1 |
  2000642.1 |
  2000886.1 |########################################
  2001130.1 |
  2001374.1 |
  2001618.0 |
  2001862.0 |
  2002106.0 |
  2002350.0 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_tight_fresh_alloc**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_residency_tight_null_entry**: bridge=4516.6% of algo (FFI overhead may distort results)
- **abi_residency_tight_reused_buffer**: bridge=302.1% of algo (FFI overhead may distort results)

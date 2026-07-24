# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_madd_null_entry dominates: 114073% faster than the next best (abi_residency_madd_reused_buffer)

abi_residency_madd_null_entry (2.34 us) leads abi_residency_madd_reused_buffer (2.67 ms) by 114073%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.67 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_fresh_alloc is an outlier: 1146.0x slower than the field

abi_residency_madd_fresh_alloc (2.68 ms) is 1146.0x the fastest (2.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 1146.0x the fastest

Fastest abi_residency_madd_null_entry (2.34 us) to slowest abi_residency_madd_fresh_alloc (2.68 ms): 1146.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 2341.4 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1145.98x (fastest 2341.4 ns, slowest 2683262.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2695810ns | 2685854ns | 2671638ns | 2685028ns | 2724070ns | +0.71% |
| abi_residency_madd_null_entry | 4632ns | 4651ns | 4405ns | 4616ns | 4769ns | -99.83% |
| abi_residency_madd_reused_buffer | 2676682ns | 2675769ns | 2665150ns | 2672921ns | 2688090ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2693145ns | 2669241ns | 2721101ns | +0.71% | 0.000 |
| abi_residency_madd_null_entry | 2330ns | 2226ns | 2389ns | -99.91% | 0.014 |
| abi_residency_madd_reused_buffer | 2674147ns | 2662775ns | 2685459ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 44680.7 | 2696712.0 | 2693145.1 | n/a |
| abi_residency_madd_null_entry | 27007.5 | 2431.4 | 2329.8 | n/a |
| abi_residency_madd_reused_buffer | 40352.9 | 2674395.1 | 2674146.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.014 | 95.1% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2695810ns | 2695810ns | +0.71% |
| abi_residency_madd_null_entry | 4632ns | 4632ns | -99.83% |
| abi_residency_madd_reused_buffer | 2676682ns | 2676682ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2673312ns | base | --- | [2663670, 2685459] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2683263ns | +14797.7ns (+0.6%) | [+2198, +39999]ns | [2675071, 2721101] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_residency_madd_null_entry | 2341ns | -2670924.8ns (-99.9%) | [-2683190, -2661336]ns | [2259, 2389] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2676136ns | +1.3% | -99.9% |
| 2 | 2684850ns | -0.1% | -99.9% |
| 3 | 2686068ns | +1.6% | -99.9% |
| 4 | 2662775ns | +0.2% | -99.9% |
| 5 | 2670488ns | +0.5% | -99.9% |
| 6 | 2664565ns | +0.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.425 | moderate- |
| abi_residency_madd_null_entry | -0.283 | moderate- |
| abi_residency_madd_reused_buffer | 0.182 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 0/6, lost 5/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8132916.7ns | 2693145.1ns | 302.0% | HIGH |
| abi_residency_madd_null_entry | 116278.1ns | 2329.8ns | 4990.8% | HIGH |
| abi_residency_madd_reused_buffer | 8064200.9ns | 2674146.9ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2669241.2-2721100.8 ns)
  2669241.2 |####################
  2671834.2 |
  2674427.2 |
  2677020.1 |
  2679613.1 |####################
  2682206.1 |########################################
  2684799.1 |
  2687392.1 |
  2689985.1 |
  2692578.0 |
  2695171.0 |
  2697764.0 |
  2700357.0 |
  2702950.0 |
  2705543.0 |
  2708135.9 |
  2710728.9 |####################
  2713321.9 |
  2715914.9 |
  2718507.9 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 2225.8-2388.8 ns)
   2225.8 |########################################
   2233.9 |
   2242.1 |
   2250.2 |
   2258.4 |
   2266.5 |
   2274.7 |
   2282.8 |
   2291.0 |########################################
   2299.1 |
   2307.3 |########################################
   2315.4 |
   2323.6 |
   2331.7 |
   2339.9 |
   2348.0 |
   2356.2 |
   2364.3 |########################################
   2372.5 |########################################
   2380.6 |
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2662775.4-2685458.8 ns)
  2662775.4 |########################################
  2663909.6 |########################################
  2665043.7 |
  2666177.9 |
  2667312.1 |
  2668446.2 |
  2669580.4 |########################################
  2670714.6 |
  2671848.7 |
  2672982.9 |
  2674117.1 |
  2675251.2 |########################################
  2676385.4 |
  2677519.6 |
  2678653.7 |
  2679787.9 |
  2680922.1 |
  2682056.2 |
  2683190.4 |
  2684324.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=4957.4% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.6% of algo (FFI overhead may distort results)

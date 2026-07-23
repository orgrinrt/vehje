# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 36019% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.50 us) leads abi_soa_win_real_soa_payload (902.76 us) by 36019%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.19 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 877.2x slower than the field

abi_soa_win_real_scalar_payload (2.19 ms) is 877.2x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 877.2x the fastest

Fastest abi_soa_win_real_null_entry (2.50 us) to slowest abi_soa_win_real_scalar_payload (2.19 ms): 877.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2499.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 877.23x (fastest 2499.4 ns, slowest 2192536.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 4741ns | 4720ns | 4631ns | 4712ns | 4840ns | -99.79% |
| abi_soa_win_real_scalar_payload | 2216650ns | 2195807ns | 2178148ns | 2193152ns | 2271148ns | base |
| abi_soa_win_real_soa_payload | 909611ns | 905528ns | 896371ns | 904726ns | 923559ns | -58.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2510ns | 2463ns | 2558ns | -99.89% | 0.006 |
| abi_soa_win_real_scalar_payload | 2213205ns | 2175156ns | 2267245ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 906715ns | 893501ns | 920456ns | -59.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 27941.6 | 2603.4 | 2509.6 | n/a |
| abi_soa_win_real_scalar_payload | 67342.7 | 2213121.4 | 2213205.1 | n/a |
| abi_soa_win_real_soa_payload | 48521.9 | 904913.7 | 906715.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.006 | 98.6% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 4741ns | 4741ns | -99.79% |
| abi_soa_win_real_scalar_payload | 2216650ns | 2216650ns | base |
| abi_soa_win_real_soa_payload | 909611ns | 909611ns | -58.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2192536ns | base | --- | [2179834, 2267245] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2499ns | -2190002.0ns (-99.9%) | [-2264753, -2177331]ns | [2471, 2558] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 902761ns | -1291734.2ns (-58.9%) | [-1350390, -1277345]ns | [896929, 920456] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2226163ns | -99.9% | -58.1% |
| 2 | 2188326ns | -99.9% | -58.5% |
| 3 | 2175156ns | -99.9% | -58.6% |
| 4 | 2196747ns | -99.9% | -58.8% |
| 5 | 2308326ns | -99.9% | -61.0% |
| 6 | 2184512ns | -99.9% | -59.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.007 | ok |
| abi_soa_win_real_scalar_payload | -0.246 | moderate- |
| abi_soa_win_real_soa_payload | 0.147 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 117992.6ns | 2509.6ns | 4701.6% | HIGH |
| abi_soa_win_real_scalar_payload | 6721493.8ns | 2213205.1ns | 303.7% | HIGH |
| abi_soa_win_real_soa_payload | 2764607.5ns | 906715.5ns | 304.9% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2463.3-2558.1 ns)
   2463.3 |########################################
   2468.0 |
   2472.8 |
   2477.5 |########################################
   2482.3 |
   2487.0 |
   2491.8 |########################################
   2496.5 |
   2501.2 |########################################
   2506.0 |
   2510.7 |
   2515.5 |
   2520.2 |
   2525.0 |
   2529.7 |
   2534.4 |
   2539.2 |########################################
   2543.9 |
   2548.7 |
   2553.4 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2175155.8-2267244.8 ns)
  2175155.8 |####################
  2179760.2 |
  2184364.7 |########################################
  2188969.1 |
  2193573.6 |####################
  2198178.0 |
  2202782.5 |
  2207386.9 |
  2211991.4 |
  2216595.8 |
  2221200.3 |
  2225804.7 |####################
  2230409.2 |
  2235013.6 |
  2239618.1 |
  2244222.5 |
  2248827.0 |
  2253431.4 |
  2258035.9 |
  2262640.3 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 893500.8-920456.1 ns)
  893500.8 |####################
  894848.6 |
  896196.3 |
  897544.1 |
  898891.9 |
  900239.6 |########################################
  901587.4 |
  902935.1 |
  904282.9 |####################
  905630.7 |
  906978.4 |
  908326.2 |####################
  909674.0 |
  911021.7 |
  912369.5 |
  913717.2 |
  915065.0 |
  916412.8 |
  917760.5 |
  919108.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4723.0% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=305.1% of algo (FFI overhead may distort results)

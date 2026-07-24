# abi_residency (madd)

3 variants, 6 samples per variant.
Baseline: **abi_residency_madd_reused_buffer**

## Highlights

Baseline for all deltas below: **abi_residency_madd_reused_buffer**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_residency_madd_null_entry dominates: 66423% faster than the next best (abi_residency_madd_reused_buffer)

abi_residency_madd_null_entry (4.04 us) leads abi_residency_madd_reused_buffer (2.69 ms) by 66423%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_residency_madd_null_entry beats baseline by 100% (significant)

abi_residency_madd_null_entry is -2.68 ms (100%) faster than baseline abi_residency_madd_reused_buffer, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_residency_madd_fresh_alloc is an outlier: 666.0x slower than the field

abi_residency_madd_fresh_alloc (2.69 ms) is 666.0x the fastest (4.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 666.0x the fastest

Fastest abi_residency_madd_null_entry (4.04 us) to slowest abi_residency_madd_fresh_alloc (2.69 ms): 666.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_residency_madd_null_entry** at 4040.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 666.01x (fastest 4040.6 ns, slowest 2691089.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2696698ns | 2693686ns | 2683248ns | 2692702ns | 2709416ns | +0.20% |
| abi_residency_madd_null_entry | 6288ns | 6334ns | 6098ns | 6276ns | 6403ns | -99.77% |
| abi_residency_madd_reused_buffer | 2691361ns | 2690471ns | 2686230ns | 2689421ns | 2696838ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2694108ns | 2680705ns | 2706788ns | +0.20% | 0.000 |
| abi_residency_madd_null_entry | 4023ns | 3908ns | 4101ns | -99.85% | 0.001 |
| abi_residency_madd_reused_buffer | 2688762ns | 2683701ns | 2694143ns | base | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 42845.1 | 2695727.9 | 2694108.1 | n/a |
| abi_residency_madd_null_entry | 26971.9 | 4142.3 | 4022.8 | n/a |
| abi_residency_madd_reused_buffer | 41301.6 | 2688641.7 | 2688762.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_residency_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_residency_madd_fresh_alloc | 0.000 | 0.1% |
| abi_residency_madd_null_entry | 0.001 | 96.7% |
| abi_residency_madd_reused_buffer | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_residency_madd_fresh_alloc | 2696698ns | 2696698ns | +0.20% |
| abi_residency_madd_null_entry | 6288ns | 6288ns | -99.77% |
| abi_residency_madd_reused_buffer | 2691361ns | 2691361ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_residency_madd_reused_buffer | 2687916ns | base | --- | [2684228, 2694143] | --- | --- | --- | --- |
| abi_residency_madd_fresh_alloc | 2691090ns | no significant difference | [-3087, +19540]ns | [2684446, 2706788] | no | 1.0000 | 1.0000 | 0 |
| abi_residency_madd_null_entry | 4041ns | -2683876.9ns (-99.8%) | [-2690151, -2680191]ns | [3926, 4101] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_residency_madd_reused_buffer | abi_residency_madd_fresh_alloc | abi_residency_madd_null_entry |
|---|---|---|---|
| 1 | 2684862ns | +1.1% | -99.8% |
| 2 | 2684755ns | +0.3% | -99.9% |
| 3 | 2691726ns | -0.1% | -99.9% |
| 4 | 2690971ns | -0.1% | -99.9% |
| 5 | 2696559ns | +0.1% | -99.8% |
| 6 | 2683701ns | -0.1% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_residency_madd_fresh_alloc | -0.084 | ok |
| abi_residency_madd_null_entry | 0.344 | moderate+ |
| abi_residency_madd_reused_buffer | -0.091 | ok |

**Consistency summary:**

- **abi_residency_madd_fresh_alloc**: won 3/6, lost 2/6
- **abi_residency_madd_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_residency_madd_fresh_alloc | 8131233.1ns | 2694108.1ns | 301.8% | HIGH |
| abi_residency_madd_null_entry | 121847.2ns | 4022.8ns | 3028.9% | HIGH |
| abi_residency_madd_reused_buffer | 8108623.6ns | 2688762.5ns | 301.6% | HIGH |

## Distribution (algo ns)

```
abi_residency_madd_fresh_alloc (n=6, range 2680704.6-2706788.1 ns)
  2680704.6 |########################################
  2682008.8 |
  2683313.0 |
  2684617.1 |
  2685921.3 |
  2687225.5 |########################################
  2688529.7 |########################################
  2689833.8 |
  2691138.0 |
  2692442.2 |########################################
  2693746.4 |
  2695050.6 |
  2696354.7 |
  2697658.9 |########################################
  2698963.1 |
  2700267.3 |
  2701571.4 |
  2702875.6 |
  2704179.8 |
  2705484.0 |
  (0 below, 1 above range)

abi_residency_madd_null_entry (n=6, range 3908.3-4101.2 ns)
   3908.3 |########################################
   3917.9 |
   3927.6 |
   3937.2 |########################################
   3946.9 |
   3956.5 |
   3966.2 |
   3975.8 |
   3985.5 |
   3995.1 |
   4004.8 |
   4014.4 |
   4024.1 |########################################
   4033.7 |
   4043.4 |########################################
   4053.0 |
   4062.7 |
   4072.3 |########################################
   4082.0 |
   4091.6 |
  (0 below, 1 above range)

abi_residency_madd_reused_buffer (n=6, range 2683701.2-2694142.7 ns)
  2683701.2 |####################
  2684223.3 |
  2684745.4 |########################################
  2685267.4 |
  2685789.5 |
  2686311.6 |
  2686833.7 |
  2687355.7 |
  2687877.8 |
  2688399.9 |
  2688922.0 |
  2689444.0 |
  2689966.1 |
  2690488.2 |####################
  2691010.2 |
  2691532.3 |####################
  2692054.4 |
  2692576.5 |
  2693098.6 |
  2693620.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_residency_madd_fresh_alloc**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_residency_madd_null_entry**: bridge=3006.4% of algo (FFI overhead may distort results)
- **abi_residency_madd_reused_buffer**: bridge=301.6% of algo (FFI overhead may distort results)

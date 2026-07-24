# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_madd_null_entry dominates: 52118% faster than the next best (abi_boundary_w_madd_scalar_per_w)

abi_boundary_w_madd_null_entry (5.11 us) leads abi_boundary_w_madd_scalar_per_w (2.67 ms) by 52118%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.68 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_soa_runtime_w is an outlier: 526.4x slower than the field

abi_boundary_w_madd_soa_runtime_w (2.69 ms) is 526.4x the fastest (5.11 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_soa_dispatch shows alternating (throttle bounce) (autocorr -0.68)

abi_boundary_w_madd_soa_dispatch's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_soa_runtime_w} (52118% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_runtime_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_soa_runtime_w} with a 52118% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 526.4x the fastest

Fastest abi_boundary_w_madd_null_entry (5.11 us) to slowest abi_boundary_w_madd_soa_runtime_w (2.69 ms): 526.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 5108.9 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 526.41x (fastest 5108.9 ns, slowest 2689406.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 7457ns | 7463ns | 7270ns | 7441ns | 7573ns | -99.72% |
| abi_boundary_w_madd_scalar_anchor | 2682300ns | 2682894ns | 2665845ns | 2680826ns | 2692738ns | -0.25% |
| abi_boundary_w_madd_scalar_dispatch | 2689475ns | 2691101ns | 2684409ns | 2689174ns | 2692459ns | +0.02% |
| abi_boundary_w_madd_scalar_per_w | 2673782ns | 2670171ns | 2665027ns | 2669128ns | 2685142ns | -0.56% |
| abi_boundary_w_madd_scalar_runtime_w | 2688887ns | 2690249ns | 2681648ns | 2687402ns | 2694735ns | base |
| abi_boundary_w_madd_soa_dispatch | 2688543ns | 2688197ns | 2685062ns | 2688101ns | 2690947ns | -0.01% |
| abi_boundary_w_madd_soa_per_w | 2678736ns | 2680168ns | 2668500ns | 2679536ns | 2682656ns | -0.38% |
| abi_boundary_w_madd_soa_runtime_w | 2696728ns | 2691957ns | 2687752ns | 2690826ns | 2710069ns | +0.29% |
| abi_boundary_w_madd_zig_runtime_w | 2681179ns | 2680765ns | 2672795ns | 2680625ns | 2686203ns | -0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 5111ns | 5001ns | 5184ns | -99.81% | 0.000 |
| abi_boundary_w_madd_scalar_anchor | 2679654ns | 2663340ns | 2690018ns | -0.25% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2686886ns | 2681818ns | 2689923ns | +0.02% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2671347ns | 2662559ns | 2682658ns | -0.56% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2686380ns | 2679130ns | 2692251ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 2686003ns | 2682644ns | 2688295ns | -0.01% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 2676156ns | 2665981ns | 2680057ns | -0.38% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 2694024ns | 2684939ns | 2707304ns | +0.28% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2678673ns | 2670264ns | 2683666ns | -0.29% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 27303.5 | 5137.9 | 5110.8 | n/a |
| abi_boundary_w_madd_scalar_anchor | 38769.2 | 2684035.9 | 2679653.7 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 39114.2 | 2689017.3 | 2686885.6 | n/a |
| abi_boundary_w_madd_scalar_per_w | 38972.0 | 2671301.5 | 2671347.1 | 0 |
| abi_boundary_w_madd_scalar_runtime_w | 37822.2 | 2686130.8 | 2686379.6 | n/a |
| abi_boundary_w_madd_soa_dispatch | 38471.0 | 2686699.4 | 2686003.1 | 2 |
| abi_boundary_w_madd_soa_per_w | 38049.4 | 2675408.4 | 2676156.4 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 39759.9 | 2694802.7 | 2694023.8 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 178910.6 | 2677900.4 | 2678673.0 | 18 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.000 | 97.9% |
| abi_boundary_w_madd_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_madd_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_madd_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_madd_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_madd_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_madd_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_madd_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_madd_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_madd_null_entry | 7457ns | 7457ns | -99.72% |
| abi_boundary_w_madd_scalar_anchor | 2682300ns | 2682300ns | -0.25% |
| abi_boundary_w_madd_scalar_dispatch | 2689475ns | 2689475ns | +0.02% |
| abi_boundary_w_madd_scalar_per_w | 2673782ns | 2673782ns | -0.56% |
| abi_boundary_w_madd_scalar_runtime_w | 2688887ns | 2688887ns | base |
| abi_boundary_w_madd_soa_dispatch | 2688543ns | 2688543ns | -0.01% |
| abi_boundary_w_madd_soa_per_w | 2678736ns | 2678736ns | -0.38% |
| abi_boundary_w_madd_soa_runtime_w | 2696728ns | 2696728ns | +0.29% |
| abi_boundary_w_madd_zig_runtime_w | 2681179ns | 2681179ns | -0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2687722ns | base | --- | [2679166, 2692251] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 5109ns | -2682537.5ns (-99.8%) | [-2687142, -2674126]ns | [5039, 5184] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2680170ns | no significant difference | [-14889, +3252]ns | [2668773, 2690018] | no | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2688443ns | no significant difference | [-6455, +9277]ns | [2682291, 2689923] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2667798ns | -13728.9ns (-0.5%) | [-22765, -8604]ns | [2663585, 2682658] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_dispatch | 2685760ns | no significant difference | [-7038, +6325]ns | [2683955, 2688295] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_madd_soa_per_w | 2677507ns | no significant difference | [-18913, +244]ns | [2670905, 2680057] | no | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 2689407ns | no significant difference | [-2361, +20232]ns | [2685360, 2707304] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2678296ns | no significant difference | [-14379, +440]ns | [2674057, 2683666] | no | 0.3500 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2694331ns | -99.8% | -0.2% | -0.2% | -0.4% | -0.3% | -0.6% | +1.1% | -0.6% |
| 2 | 2679201ns | -99.8% | +0.4% | +0.4% | -0.5% | +0.2% | +0.1% | +0.4% | +0.2% |
| 3 | 2687250ns | -99.8% | -0.4% | -0.2% | -0.9% | +0.1% | -0.8% | -0.1% | -0.1% |
| 4 | 2688193ns | -99.8% | -0.5% | +0.1% | -0.3% | -0.2% | -0.4% | -0.1% | -0.4% |
| 5 | 2690172ns | -99.8% | -0.3% | -0.3% | -0.8% | -0.1% | -0.5% | +0.0% | -0.5% |
| 6 | 2679130ns | -99.8% | -0.6% | +0.3% | -0.5% | +0.2% | -0.1% | +0.4% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | -0.220 | moderate- |
| abi_boundary_w_madd_scalar_anchor | 0.033 | ok |
| abi_boundary_w_madd_scalar_dispatch | -0.554 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_scalar_per_w | -0.265 | moderate- |
| abi_boundary_w_madd_scalar_runtime_w | -0.443 | moderate- |
| abi_boundary_w_madd_soa_dispatch | -0.679 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_per_w | -0.504 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_soa_runtime_w | 0.014 | ok |
| abi_boundary_w_madd_zig_runtime_w | 0.230 | moderate+ |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 5/6, lost 1/6
- **abi_boundary_w_madd_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_madd_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_madd_soa_per_w**: won 4/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 1/6, lost 3/6
- **abi_boundary_w_madd_zig_runtime_w**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 124368.9ns | 5110.8ns | 2433.4% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8089038.9ns | 2679653.7ns | 301.9% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8104664.6ns | 2686885.6ns | 301.6% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8053583.2ns | 2671347.1ns | 301.5% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8100828.2ns | 2686379.6ns | 301.6% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 8101822.0ns | 2686003.1ns | 301.6% | HIGH |
| abi_boundary_w_madd_soa_per_w | 8068331.8ns | 2676156.4ns | 301.5% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 8128749.5ns | 2694023.8ns | 301.7% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8286611.3ns | 2678673.0ns | 309.4% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 5000.8-5184.1 ns)
   5000.8 |####################
   5010.0 |
   5019.1 |
   5028.3 |
   5037.5 |
   5046.6 |
   5055.8 |
   5065.0 |
   5074.1 |########################################
   5083.3 |
   5092.5 |
   5101.6 |
   5110.8 |
   5120.0 |
   5129.1 |####################
   5138.3 |
   5147.5 |
   5156.6 |
   5165.8 |####################
   5175.0 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2663340.4-2690018.4 ns)
  2663340.4 |########################################
  2664674.3 |
  2666008.2 |
  2667342.1 |
  2668676.0 |
  2670009.9 |
  2671343.8 |
  2672677.7 |
  2674011.6 |########################################
  2675345.5 |
  2676679.4 |########################################
  2678013.3 |
  2679347.2 |
  2680681.1 |
  2682015.0 |########################################
  2683348.9 |
  2684682.8 |
  2686016.7 |
  2687350.6 |
  2688684.5 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2681818.3-2689923.3 ns)
  2681818.3 |########################################
  2682223.5 |
  2682628.8 |########################################
  2683034.0 |
  2683439.3 |
  2683844.5 |
  2684249.8 |
  2684655.0 |
  2685060.3 |
  2685465.5 |
  2685870.8 |
  2686276.0 |
  2686681.3 |
  2687086.5 |
  2687491.8 |########################################
  2687897.0 |
  2688302.3 |
  2688707.5 |########################################
  2689112.8 |
  2689518.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2662558.8-2682658.1 ns)
  2662558.8 |########################################
  2663563.8 |
  2664568.7 |########################################
  2665573.7 |########################################
  2666578.7 |
  2667583.6 |
  2668588.6 |########################################
  2669593.6 |
  2670598.5 |
  2671603.5 |
  2672608.4 |
  2673613.4 |
  2674618.4 |
  2675623.3 |
  2676628.3 |
  2677633.3 |
  2678638.2 |
  2679643.2 |
  2680648.2 |########################################
  2681653.1 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2679130.4-2692251.5 ns)
  2679130.4 |########################################
  2679786.5 |
  2680442.5 |
  2681098.6 |
  2681754.6 |
  2682410.7 |
  2683066.7 |
  2683722.8 |
  2684378.8 |
  2685034.9 |
  2685690.9 |
  2686347.0 |
  2687003.0 |####################
  2687659.1 |####################
  2688315.1 |
  2688971.2 |
  2689627.2 |####################
  2690283.3 |
  2690939.3 |
  2691595.4 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 2682644.2-2688294.8 ns)
  2682644.2 |########################################
  2682926.7 |
  2683209.3 |
  2683491.8 |
  2683774.3 |
  2684056.8 |
  2684339.4 |
  2684621.9 |
  2684904.4 |
  2685186.9 |########################################
  2685469.5 |########################################
  2685752.0 |########################################
  2686034.5 |
  2686317.1 |
  2686599.6 |########################################
  2686882.1 |
  2687164.6 |
  2687447.2 |
  2687729.7 |
  2688012.2 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 2665981.2-2680057.3 ns)
  2665981.2 |########################################
  2666685.0 |
  2667388.8 |
  2668092.6 |
  2668796.4 |
  2669500.2 |
  2670204.0 |
  2670907.8 |
  2671611.6 |
  2672315.4 |
  2673019.2 |
  2673723.1 |
  2674426.9 |
  2675130.7 |########################################
  2675834.5 |
  2676538.3 |########################################
  2677242.1 |########################################
  2677945.9 |########################################
  2678649.7 |
  2679353.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 2684939.2-2707304.0 ns)
  2684939.2 |########################################
  2686057.4 |
  2687175.7 |
  2688293.9 |####################
  2689412.2 |########################################
  2690530.4 |
  2691648.6 |
  2692766.9 |
  2693885.1 |
  2695003.4 |
  2696121.6 |
  2697239.8 |
  2698358.1 |
  2699476.3 |
  2700594.6 |
  2701712.8 |
  2702831.0 |
  2703949.3 |
  2705067.5 |
  2706185.8 |
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2670263.8-2683666.0 ns)
  2670263.8 |####################
  2670933.9 |
  2671604.0 |
  2672274.1 |
  2672944.2 |
  2673614.4 |
  2674284.5 |
  2674954.6 |
  2675624.7 |
  2676294.8 |
  2676964.9 |
  2677635.0 |########################################
  2678305.1 |####################
  2678975.3 |
  2679645.4 |
  2680315.5 |
  2680985.6 |
  2681655.7 |
  2682325.8 |
  2682995.9 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=2433.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=301.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.4% of algo (FFI overhead may distort results)

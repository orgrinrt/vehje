# abi_boundary_w (madd)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_madd_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_madd_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_madd_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_madd_scalar_runtime_w has the worst median (2.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_madd_null_entry at 3.92 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_madd_null_entry dominates: 67976% faster than the next best (abi_boundary_w_madd_scalar_anchor)

abi_boundary_w_madd_null_entry (3.92 us) leads abi_boundary_w_madd_scalar_anchor (2.67 ms) by 67976%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_madd_null_entry beats baseline by 100% (significant)

abi_boundary_w_madd_null_entry is -2.70 ms (100%) faster than baseline abi_boundary_w_madd_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_madd_scalar_runtime_w is an outlier: 690.3x slower than the field

abi_boundary_w_madd_scalar_runtime_w (2.70 ms) is 690.3x the fastest (3.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_madd_soa_runtime_w shows alternating (throttle bounce) (autocorr -0.65)

abi_boundary_w_madd_soa_runtime_w's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_madd_null_entry} vs {abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_runtime_w} (67976% apart)

The field splits into a fast tier {abi_boundary_w_madd_null_entry} and a slow tier {abi_boundary_w_madd_scalar_anchor, abi_boundary_w_madd_scalar_per_w, abi_boundary_w_madd_soa_per_w, abi_boundary_w_madd_scalar_dispatch, abi_boundary_w_madd_zig_runtime_w, abi_boundary_w_madd_soa_runtime_w, abi_boundary_w_madd_soa_dispatch, abi_boundary_w_madd_scalar_runtime_w} with a 67976% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 690.3x the fastest

Fastest abi_boundary_w_madd_null_entry (3.92 us) to slowest abi_boundary_w_madd_scalar_runtime_w (2.70 ms): 690.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_madd_null_entry** at 3916.1 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 690.25x (fastest 3916.1 ns, slowest 2703066.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 6202ns | 6126ns | 5998ns | 6087ns | 6477ns | -99.77% |
| abi_boundary_w_madd_scalar_anchor | 2666701ns | 2668381ns | 2654236ns | 2664197ns | 2676690ns | -1.29% |
| abi_boundary_w_madd_scalar_dispatch | 2676025ns | 2678560ns | 2658418ns | 2672691ns | 2689831ns | -0.95% |
| abi_boundary_w_madd_scalar_per_w | 2669048ns | 2668961ns | 2657884ns | 2668759ns | 2675063ns | -1.21% |
| abi_boundary_w_madd_scalar_runtime_w | 2701629ns | 2705692ns | 2680765ns | 2701834ns | 2711755ns | base |
| abi_boundary_w_madd_soa_dispatch | 2694399ns | 2694464ns | 2692360ns | 2694177ns | 2695750ns | -0.27% |
| abi_boundary_w_madd_soa_per_w | 2669680ns | 2669527ns | 2660492ns | 2667391ns | 2677708ns | -1.18% |
| abi_boundary_w_madd_soa_runtime_w | 2690585ns | 2691386ns | 2673391ns | 2688593ns | 2702170ns | -0.41% |
| abi_boundary_w_madd_zig_runtime_w | 2687804ns | 2684659ns | 2681721ns | 2684200ns | 2696253ns | -0.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 3970ns | 3841ns | 4153ns | -99.85% | 0.001 |
| abi_boundary_w_madd_scalar_anchor | 2664061ns | 2651727ns | 2673835ns | -1.30% | 0.000 |
| abi_boundary_w_madd_scalar_dispatch | 2673440ns | 2656058ns | 2687201ns | -0.95% | 0.000 |
| abi_boundary_w_madd_scalar_per_w | 2666552ns | 2655557ns | 2672601ns | -1.20% | 0.000 |
| abi_boundary_w_madd_scalar_runtime_w | 2699047ns | 2678451ns | 2709081ns | base | 0.000 |
| abi_boundary_w_madd_soa_dispatch | 2691846ns | 2689625ns | 2693119ns | -0.27% | 0.000 |
| abi_boundary_w_madd_soa_per_w | 2667133ns | 2658178ns | 2675284ns | -1.18% | 0.000 |
| abi_boundary_w_madd_soa_runtime_w | 2688028ns | 2670662ns | 2699557ns | -0.41% | 0.000 |
| abi_boundary_w_madd_zig_runtime_w | 2685175ns | 2679105ns | 2693646ns | -0.51% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 26085.1 | 4063.0 | 3970.1 | n/a |
| abi_boundary_w_madd_scalar_anchor | 35229.2 | 2667851.6 | 2664061.0 | n/a |
| abi_boundary_w_madd_scalar_dispatch | 38203.3 | 2673249.1 | 2673439.7 | n/a |
| abi_boundary_w_madd_scalar_per_w | 37285.5 | 2666915.4 | 2666551.8 | n/a |
| abi_boundary_w_madd_scalar_runtime_w | 39515.5 | 2700017.2 | 2699046.6 | n/a |
| abi_boundary_w_madd_soa_dispatch | 38679.3 | 2692392.7 | 2691845.7 | n/a |
| abi_boundary_w_madd_soa_per_w | 36423.1 | 2668432.0 | 2667132.8 | n/a |
| abi_boundary_w_madd_soa_runtime_w | 37324.3 | 2687533.9 | 2688027.9 | n/a |
| abi_boundary_w_madd_zig_runtime_w | 179374.7 | 2688099.9 | 2685175.4 | 10 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.001 | 98.1% |
| abi_boundary_w_madd_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_madd_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_dispatch | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_per_w | 0.000 | 0.1% |
| abi_boundary_w_madd_soa_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_madd_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_madd_null_entry | 6202ns | 6202ns | -99.77% |
| abi_boundary_w_madd_scalar_anchor | 2666701ns | 2666701ns | -1.29% |
| abi_boundary_w_madd_scalar_dispatch | 2676025ns | 2676025ns | -0.95% |
| abi_boundary_w_madd_scalar_per_w | 2669048ns | 2669048ns | -1.21% |
| abi_boundary_w_madd_scalar_runtime_w | 2701629ns | 2701629ns | base |
| abi_boundary_w_madd_soa_dispatch | 2694399ns | 2694399ns | -0.27% |
| abi_boundary_w_madd_soa_per_w | 2669680ns | 2669680ns | -1.18% |
| abi_boundary_w_madd_soa_runtime_w | 2690585ns | 2690585ns | -0.41% |
| abi_boundary_w_madd_zig_runtime_w | 2687804ns | 2687804ns | -0.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_madd_scalar_runtime_w | 2703066ns | base | --- | [2684993, 2709081] | --- | --- | --- | --- |
| abi_boundary_w_madd_null_entry | 3916ns | -2699028.8ns (-99.9%) | [-2705088, -2681113]ns | [3841, 4153] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_anchor | 2665878ns | -32224.8ns (-1.2%) | [-56611, -16121]ns | [2652470, 2673835] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_dispatch | 2675891ns | -17682.1ns (-0.7%) | [-50037, -9101]ns | [2657226, 2687201] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_scalar_per_w | 2666349ns | -30464.8ns (-1.1%) | [-48059, -18961]ns | [2660705, 2672601] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_dispatch | 2691972ns | no significant difference | [-17965, +6874]ns | [2690445, 2693119] | no | 0.6875 | 0.6875 | 0 |
| abi_boundary_w_madd_soa_per_w | 2666739ns | -31364.0ns (-1.2%) | [-49705, -14672]ns | [2659376, 2675284] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_madd_soa_runtime_w | 2688859ns | no significant difference | [-24252, +7995]ns | [2675668, 2699557] | no | 0.2500 | 0.2188 | 0 |
| abi_boundary_w_madd_zig_runtime_w | 2681984ns | no significant difference | [-27987, +2085]ns | [2679896, 2693646] | no | 0.2500 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_madd_scalar_runtime_w | abi_boundary_w_madd_null_entry | abi_boundary_w_madd_scalar_anchor | abi_boundary_w_madd_scalar_dispatch | abi_boundary_w_madd_scalar_per_w | abi_boundary_w_madd_soa_dispatch | abi_boundary_w_madd_soa_per_w | abi_boundary_w_madd_soa_runtime_w | abi_boundary_w_madd_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2708306ns | -99.9% | -2.0% | -0.8% | -1.5% | -0.7% | -1.8% | -1.0% | -1.1% |
| 2 | 2678451ns | -99.9% | -0.2% | -0.3% | -0.5% | +0.5% | -0.4% | +0.9% | +0.5% |
| 3 | 2701460ns | -99.8% | -1.0% | -0.5% | -1.0% | -0.4% | -0.7% | -0.7% | -0.8% |
| 4 | 2709855ns | -99.8% | -2.1% | -2.0% | -2.0% | -0.6% | -1.9% | -0.6% | -1.0% |
| 5 | 2691535ns | -99.9% | -1.1% | -0.4% | -0.9% | +0.0% | -0.9% | -0.8% | -0.4% |
| 6 | 2704672ns | -99.9% | -1.3% | -1.7% | -1.3% | -0.4% | -1.4% | -0.3% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_madd_null_entry | 0.182 | ok |
| abi_boundary_w_madd_scalar_anchor | -0.221 | moderate- |
| abi_boundary_w_madd_scalar_dispatch | -0.551 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_scalar_per_w | -0.472 | moderate- |
| abi_boundary_w_madd_scalar_runtime_w | -0.469 | moderate- |
| abi_boundary_w_madd_soa_dispatch | 0.294 | moderate+ |
| abi_boundary_w_madd_soa_per_w | -0.358 | moderate- |
| abi_boundary_w_madd_soa_runtime_w | -0.646 | HIGH- (thermal bounce) |
| abi_boundary_w_madd_zig_runtime_w | -0.427 | moderate- |

**Consistency summary:**

- **abi_boundary_w_madd_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_anchor**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_madd_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_madd_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_madd_soa_runtime_w**: won 5/6, lost 1/6
- **abi_boundary_w_madd_zig_runtime_w**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_madd_null_entry | 120811.3ns | 3970.1ns | 3043.1% | HIGH |
| abi_boundary_w_madd_scalar_anchor | 8038079.4ns | 2664061.0ns | 301.7% | HIGH |
| abi_boundary_w_madd_scalar_dispatch | 8059460.1ns | 2673439.7ns | 301.5% | HIGH |
| abi_boundary_w_madd_scalar_per_w | 8039172.4ns | 2666551.8ns | 301.5% | HIGH |
| abi_boundary_w_madd_scalar_runtime_w | 8141804.6ns | 2699046.6ns | 301.7% | HIGH |
| abi_boundary_w_madd_soa_dispatch | 8118622.9ns | 2691845.7ns | 301.6% | HIGH |
| abi_boundary_w_madd_soa_per_w | 8044864.2ns | 2667132.8ns | 301.6% | HIGH |
| abi_boundary_w_madd_soa_runtime_w | 8104029.9ns | 2688027.9ns | 301.5% | HIGH |
| abi_boundary_w_madd_zig_runtime_w | 8312228.6ns | 2685175.4ns | 309.6% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_madd_null_entry (n=6, range 3840.8-4152.7 ns)
   3840.8 |########################################
   3856.4 |
   3872.0 |
   3887.6 |
   3903.2 |########################################
   3918.8 |
   3934.4 |
   3950.0 |
   3965.6 |
   3981.2 |
   3996.8 |
   4012.3 |
   4027.9 |
   4043.5 |
   4059.1 |
   4074.7 |
   4090.3 |
   4105.9 |
   4121.5 |
   4137.1 |####################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_anchor (n=6, range 2651727.1-2673835.0 ns)
  2651727.1 |########################################
  2652832.5 |########################################
  2653937.9 |
  2655043.3 |
  2656148.7 |
  2657254.1 |
  2658359.5 |
  2659464.9 |
  2660570.3 |########################################
  2661675.7 |
  2662781.0 |
  2663886.4 |
  2664991.8 |
  2666097.2 |
  2667202.6 |
  2668308.0 |
  2669413.4 |
  2670518.8 |########################################
  2671624.2 |
  2672729.6 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_dispatch (n=6, range 2656057.9-2687201.2 ns)
  2656057.9 |########################################
  2657615.1 |########################################
  2659172.2 |
  2660729.4 |
  2662286.6 |
  2663843.7 |
  2665400.9 |
  2666958.1 |
  2668515.2 |
  2670072.4 |########################################
  2671629.6 |
  2673186.7 |
  2674743.9 |
  2676301.1 |
  2677858.2 |
  2679415.4 |########################################
  2680972.6 |
  2682529.7 |
  2684086.9 |
  2685644.1 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_per_w (n=6, range 2655556.7-2672601.2 ns)
  2655556.7 |#############
  2656408.9 |
  2657261.2 |
  2658113.4 |
  2658965.6 |
  2659817.8 |
  2660670.1 |
  2661522.3 |
  2662374.5 |
  2663226.7 |
  2664079.0 |
  2664931.2 |
  2665783.4 |########################################
  2666635.7 |
  2667487.9 |
  2668340.1 |
  2669192.3 |#############
  2670044.6 |
  2670896.8 |
  2671749.0 |
  (0 below, 1 above range)

abi_boundary_w_madd_scalar_runtime_w (n=6, range 2678451.2-2709080.8 ns)
  2678451.2 |########################################
  2679982.7 |
  2681514.2 |
  2683045.6 |
  2684577.1 |
  2686108.6 |
  2687640.1 |
  2689171.6 |
  2690703.0 |########################################
  2692234.5 |
  2693766.0 |
  2695297.5 |
  2696829.0 |
  2698360.4 |
  2699891.9 |
  2701423.4 |########################################
  2702954.9 |
  2704486.4 |########################################
  2706017.8 |
  2707549.3 |########################################
  (0 below, 1 above range)

abi_boundary_w_madd_soa_dispatch (n=6, range 2689625.4-2693119.4 ns)
  2689625.4 |########################################
  2689800.1 |
  2689974.8 |
  2690149.5 |
  2690324.2 |
  2690498.9 |
  2690673.6 |
  2690848.3 |
  2691023.0 |
  2691197.7 |########################################
  2691372.4 |########################################
  2691547.1 |
  2691721.8 |
  2691896.5 |
  2692071.2 |
  2692245.9 |
  2692420.6 |########################################
  2692595.3 |########################################
  2692770.0 |
  2692944.7 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_per_w (n=6, range 2658177.9-2675283.8 ns)
  2658177.9 |########################################
  2659033.2 |
  2659888.5 |########################################
  2660743.8 |
  2661599.1 |
  2662454.4 |
  2663309.7 |
  2664164.9 |
  2665020.2 |
  2665875.5 |########################################
  2666730.8 |########################################
  2667586.1 |########################################
  2668441.4 |
  2669296.7 |
  2670152.0 |
  2671007.3 |
  2671862.6 |
  2672717.9 |
  2673573.2 |
  2674428.5 |
  (0 below, 1 above range)

abi_boundary_w_madd_soa_runtime_w (n=6, range 2670662.1-2699556.8 ns)
  2670662.1 |########################################
  2672106.8 |
  2673551.6 |
  2674996.3 |
  2676441.0 |
  2677885.8 |
  2679330.5 |########################################
  2680775.3 |
  2682220.0 |########################################
  2683664.7 |
  2685109.5 |
  2686554.2 |
  2687998.9 |
  2689443.7 |
  2690888.4 |
  2692333.2 |
  2693777.9 |########################################
  2695222.6 |########################################
  2696667.4 |
  2698112.1 |
  (0 below, 1 above range)

abi_boundary_w_madd_zig_runtime_w (n=6, range 2679104.6-2693646.2 ns)
  2679104.6 |####################
  2679831.7 |
  2680558.8 |########################################
  2681285.8 |
  2682012.9 |
  2682740.0 |####################
  2683467.1 |
  2684194.2 |
  2684921.3 |
  2685648.3 |
  2686375.4 |
  2687102.5 |
  2687829.6 |
  2688556.7 |
  2689283.8 |
  2690010.8 |
  2690737.9 |
  2691465.0 |####################
  2692192.1 |
  2692919.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_madd_null_entry**: bridge=3099.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_anchor**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_dispatch**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_per_w**: bridge=301.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_scalar_runtime_w**: bridge=301.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_dispatch**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_per_w**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_soa_runtime_w**: bridge=301.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_madd_zig_runtime_w**: bridge=309.7% of algo (FFI overhead may distort results)

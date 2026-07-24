# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_wideselect_null_entry dominates: 64725% faster than the next best (abi_entry_form_wideselect_per_w_set)

abi_entry_form_wideselect_null_entry (3.19 us) leads abi_entry_form_wideselect_per_w_set (2.07 ms) by 64725%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.07 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_dispatch_table is an outlier: 650.0x slower than the field

abi_entry_form_wideselect_dispatch_table (2.07 ms) is 650.0x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.57)

abi_entry_form_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_dispatch_table} (64725% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_dispatch_table} with a 64725% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 650.0x the fastest

Fastest abi_entry_form_wideselect_null_entry (3.19 us) to slowest abi_entry_form_wideselect_dispatch_table (2.07 ms): 650.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 3185.7 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 650.05x (fastest 3185.7 ns, slowest 2070821.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2071961ns | 2073812ns | 2065725ns | 2071432ns | 2075872ns | +0.05% |
| abi_entry_form_wideselect_null_entry | 5500ns | 5489ns | 5294ns | 5452ns | 5677ns | -99.73% |
| abi_entry_form_wideselect_per_w_set | 2067138ns | 2067958ns | 2059119ns | 2066432ns | 2072209ns | -0.18% |
| abi_entry_form_wideselect_runtime_w | 2070965ns | 2071760ns | 2065221ns | 2070589ns | 2074402ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2071642ns | 2071033ns | 2065078ns | 2070691ns | 2076350ns | +0.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2068938ns | 2062806ns | 2072592ns | +0.04% | 0.000 |
| abi_entry_form_wideselect_null_entry | 3196ns | 3102ns | 3295ns | -99.85% | 0.080 |
| abi_entry_form_wideselect_per_w_set | 2064280ns | 2056494ns | 2069200ns | -0.18% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2068044ns | 2062245ns | 2071393ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2068570ns | 2062225ns | 2073094ns | +0.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 56549.7 | 2068612.1 | 2068937.6 | 0 |
| abi_entry_form_wideselect_null_entry | 29118.0 | 3213.3 | 3195.8 | n/a |
| abi_entry_form_wideselect_per_w_set | 54666.5 | 2067466.4 | 2064279.9 | n/a |
| abi_entry_form_wideselect_runtime_w | 56924.4 | 2067753.0 | 2068044.1 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 57604.2 | 2071256.8 | 2068570.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.083 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_wideselect_null_entry | 0.080 | 97.4% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.2% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.1% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2071961ns | 2071961ns | +0.05% |
| abi_entry_form_wideselect_null_entry | 5500ns | 5500ns | -99.73% |
| abi_entry_form_wideselect_per_w_set | 2067138ns | 2067138ns | -0.18% |
| abi_entry_form_wideselect_runtime_w | 2070965ns | 2070965ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2071642ns | 2071642ns | +0.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2068870ns | base | --- | [2063869, 2071393] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2070822ns | no significant difference | [-4923, +7057]ns | [2063399, 2072592] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_wideselect_null_entry | 3186ns | -2065599.1ns (-99.8%) | [-2068197, -2060749]ns | [3106, 3295] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2065109ns | no significant difference | [-8568, +1212]ns | [2058530, 2069200] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2068114ns | no significant difference | [-2546, +4773]ns | [2064503, 2073094] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2065492ns | -0.1% | -99.8% | +0.2% | +0.4% |
| 2 | 2068825ns | +0.2% | -99.8% | -0.0% | -0.1% |
| 3 | 2068915ns | +0.1% | -99.8% | -0.4% | -0.1% |
| 4 | 2062245ns | +0.5% | -99.8% | -0.3% | -0.0% |
| 5 | 2071635ns | -0.1% | -99.8% | -0.1% | -0.1% |
| 6 | 2071151ns | -0.3% | -99.9% | -0.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.086 | ok |
| abi_entry_form_wideselect_null_entry | -0.566 | HIGH- (thermal bounce) |
| abi_entry_form_wideselect_per_w_set | -0.134 | ok |
| abi_entry_form_wideselect_runtime_w | -0.250 | moderate- |
| abi_entry_form_wideselect_scalar_anchor | 0.101 | ok |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 2/6, lost 3/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 4/6, lost 1/6
- **abi_entry_form_wideselect_scalar_anchor**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6271111.5ns | 2068937.6ns | 303.1% | HIGH |
| abi_entry_form_wideselect_null_entry | 122222.2ns | 3195.8ns | 3824.5% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6256834.6ns | 2064279.9ns | 303.1% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6262561.8ns | 2068044.1ns | 302.8% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6275168.9ns | 2068570.4ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2062805.8-2072592.2 ns)
  2062805.8 |####################
  2063295.1 |
  2063784.4 |####################
  2064273.8 |
  2064763.1 |
  2065252.4 |
  2065741.7 |
  2066231.1 |
  2066720.4 |
  2067209.7 |
  2067699.0 |
  2068188.3 |
  2068677.7 |
  2069167.0 |
  2069656.3 |####################
  2070145.6 |
  2070635.0 |
  2071124.3 |
  2071613.6 |########################################
  2072102.9 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 3101.7-3295.2 ns)
   3101.7 |########################################
   3111.4 |
   3121.0 |####################
   3130.7 |
   3140.4 |
   3150.1 |
   3159.8 |
   3169.4 |
   3179.1 |
   3188.8 |
   3198.4 |
   3208.1 |
   3217.8 |
   3227.5 |
   3237.1 |####################
   3246.8 |
   3256.5 |
   3266.2 |
   3275.8 |
   3285.5 |####################
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2056494.2-2069200.2 ns)
  2056494.2 |########################################
  2057129.5 |
  2057764.8 |
  2058400.1 |
  2059035.4 |
  2059670.7 |
  2060306.0 |########################################
  2060941.3 |
  2061576.6 |
  2062211.9 |########################################
  2062847.2 |
  2063482.5 |
  2064117.8 |
  2064753.1 |
  2065388.4 |
  2066023.7 |
  2066659.0 |
  2067294.3 |########################################
  2067929.6 |
  2068564.9 |########################################
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2062245.4-2071393.3 ns)
  2062245.4 |####################
  2062702.8 |
  2063160.2 |
  2063617.6 |
  2064075.0 |
  2064532.4 |
  2064989.8 |
  2065447.2 |####################
  2065904.6 |
  2066362.0 |
  2066819.3 |
  2067276.7 |
  2067734.1 |
  2068191.5 |
  2068648.9 |########################################
  2069106.3 |
  2069563.7 |
  2070021.1 |
  2070478.5 |
  2070935.9 |####################
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2062225.0-2073094.2 ns)
  2062225.0 |########################################
  2062768.5 |
  2063311.9 |
  2063855.4 |
  2064398.8 |
  2064942.3 |
  2065485.8 |
  2066029.2 |
  2066572.7 |########################################
  2067116.1 |########################################
  2067659.6 |
  2068203.1 |########################################
  2068746.5 |
  2069290.0 |
  2069833.4 |
  2070376.9 |
  2070920.4 |
  2071463.8 |
  2072007.3 |
  2072550.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=3841.8% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=303.5% of algo (FFI overhead may distort results)

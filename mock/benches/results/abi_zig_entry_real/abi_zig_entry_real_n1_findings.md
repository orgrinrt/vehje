# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 40594% faster than the next best (abi_zig_entry_real_zig_dispatch)

abi_zig_entry_real_zig_null (5.09 us) leads abi_zig_entry_real_zig_dispatch (2.07 ms) by 40594%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.08 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 764.2x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.89 ms) is 764.2x the fastest (5.09 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_null shows alternating (throttle bounce) (autocorr -0.77)

abi_zig_entry_real_zig_null's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (40594% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 40594% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 764.2x the fastest

Fastest abi_zig_entry_real_zig_null (5.09 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.89 ms): 764.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 5088.4 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 764.22x (fastest 5088.4 ns, slowest 3888618.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2078298ns | 2077252ns | 2073041ns | 2076558ns | 2083535ns | -0.55% |
| abi_zig_entry_real_zig_dispatch | 2074377ns | 2073113ns | 2071865ns | 2072929ns | 2077805ns | -0.74% |
| abi_zig_entry_real_zig_null | 7406ns | 7398ns | 7298ns | 7389ns | 7486ns | -99.65% |
| abi_zig_entry_real_zig_per_w_set | 2094268ns | 2075595ns | 2072609ns | 2074867ns | 2134200ns | +0.22% |
| abi_zig_entry_real_zig_runtime_w | 2089770ns | 2082797ns | 2076140ns | 2081066ns | 2109642ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3110646ns | 3109821ns | 3105016ns | 3108712ns | 3116361ns | +48.85% |
| abi_zig_entry_real_zig_tail_runtime_w | 3891087ns | 3891473ns | 3882392ns | 3889577ns | 3897700ns | +86.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2075622ns | 2070350ns | 2080729ns | -0.55% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2071830ns | 2069201ns | 2075188ns | -0.73% | 0.000 |
| abi_zig_entry_real_zig_null | 5095ns | 5046ns | 5147ns | -99.76% | 0.000 |
| abi_zig_entry_real_zig_per_w_set | 2091561ns | 2070095ns | 2131203ns | +0.22% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2087017ns | 2073651ns | 2106608ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3107983ns | 3102433ns | 3113645ns | +48.92% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3888210ns | 3879388ns | 3894800ns | +86.30% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 184635.0 | 2073916.9 | 2075621.7 | n/a |
| abi_zig_entry_real_zig_dispatch | 179158.9 | 2071295.0 | 2071830.2 | n/a |
| abi_zig_entry_real_zig_null | 154772.3 | 5164.3 | 5095.4 | n/a |
| abi_zig_entry_real_zig_per_w_set | 188770.5 | 2103970.1 | 2091561.0 | n/a |
| abi_zig_entry_real_zig_runtime_w | 188437.7 | 2083705.2 | 2087017.0 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 186632.1 | 3107001.2 | 3107982.8 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 196554.3 | 3887222.8 | 3888210.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_real_zig_null | 0.000 | 99.2% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.2% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2078298ns | 2078298ns | -0.55% |
| abi_zig_entry_real_zig_dispatch | 2074377ns | 2074377ns | -0.74% |
| abi_zig_entry_real_zig_null | 7406ns | 7406ns | -99.65% |
| abi_zig_entry_real_zig_per_w_set | 2094268ns | 2094268ns | +0.22% |
| abi_zig_entry_real_zig_runtime_w | 2089770ns | 2089770ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3110646ns | 3110646ns | +48.85% |
| abi_zig_entry_real_zig_tail_runtime_w | 3891087ns | 3891087ns | +86.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2080095ns | base | --- | [2074348, 2106608] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2074685ns | no significant difference | [-30308, +2215]ns | [2071451, 2080729] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_real_zig_dispatch | 2070628ns | -8657.9ns (-0.4%) | [-33156, -3747]ns | [2069675, 2075188] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_null | 5088ns | -2075007.1ns (-99.8%) | [-2101494, -2069264]ns | [5051, 5147] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2073013ns | no significant difference | [-12825, +30664]ns | [2070467, 2131203] | no | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3107160ns | +1032508.1ns (+49.6%) | [+996535, +1033855]ns | [3103143, 3113645] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3888618ns | +1801790.8ns (+86.6%) | [+1781335, +1820453]ns | [3881211, 3894800] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2073651ns | +0.4% | -0.1% | -99.8% | -0.1% | +49.8% | +87.9% |
| 2 | 2126034ns | -2.2% | -2.3% | -99.8% | +2.9% | +46.0% | +82.9% |
| 3 | 2079401ns | -0.2% | -0.5% | -99.8% | -0.4% | +49.7% | +86.6% |
| 4 | 2075045ns | -0.2% | -0.2% | -99.8% | -0.0% | +49.8% | +87.6% |
| 5 | 2087182ns | -0.7% | -0.8% | -99.8% | -0.8% | +48.6% | +86.3% |
| 6 | 2080790ns | -0.4% | -0.3% | -99.8% | -0.3% | +49.6% | +86.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.506 | HIGH+ (drift/warm-up) |
| abi_zig_entry_real_zig_dispatch | -0.310 | moderate- |
| abi_zig_entry_real_zig_null | -0.767 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_per_w_set | -0.254 | moderate- |
| abi_zig_entry_real_zig_runtime_w | -0.376 | moderate- |
| abi_zig_entry_real_zig_tail_dispatch | -0.400 | moderate- |
| abi_zig_entry_real_zig_tail_runtime_w | -0.185 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 5/6, lost 1/6
- **abi_zig_entry_real_zig_dispatch**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 3/6, lost 1/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6476438.2ns | 2075621.7ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6461201.8ns | 2071830.2ns | 311.9% | HIGH |
| abi_zig_entry_real_zig_null | 311996.8ns | 5095.4ns | 6123.1% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6560167.9ns | 2091561.0ns | 313.6% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6510088.4ns | 2087017.0ns | 311.9% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 9583787.7ns | 3107982.8ns | 308.4% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11919712.7ns | 3888210.0ns | 306.6% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2070349.6-2080729.1 ns)
  2070349.6 |########################################
  2070868.6 |
  2071387.6 |
  2071906.5 |
  2072425.5 |########################################
  2072944.5 |########################################
  2073463.5 |
  2073982.4 |
  2074501.4 |
  2075020.4 |
  2075539.4 |
  2076058.4 |########################################
  2076577.3 |
  2077096.3 |
  2077615.3 |
  2078134.3 |
  2078653.2 |
  2079172.2 |
  2079691.2 |########################################
  2080210.2 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2069200.8-2075187.7 ns)
  2069200.8 |####################
  2069500.1 |
  2069799.5 |
  2070098.8 |########################################
  2070398.2 |
  2070697.5 |
  2070996.9 |####################
  2071296.2 |
  2071595.6 |
  2071894.9 |
  2072194.2 |
  2072493.6 |
  2072792.9 |
  2073092.3 |
  2073391.6 |####################
  2073691.0 |
  2073990.3 |
  2074289.7 |
  2074589.0 |
  2074888.4 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 5046.2-5147.1 ns)
   5046.2 |########################################
   5051.2 |########################################
   5056.3 |########################################
   5061.3 |
   5066.4 |
   5071.4 |
   5076.5 |
   5081.5 |
   5086.6 |
   5091.6 |
   5096.6 |
   5101.7 |
   5106.7 |
   5111.8 |########################################
   5116.8 |
   5121.9 |########################################
   5126.9 |
   5132.0 |
   5137.0 |
   5142.1 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2070094.6-2131203.1 ns)
  2070094.6 |########################################
  2073150.0 |##########################
  2076205.5 |
  2079260.9 |
  2082316.3 |
  2085371.7 |
  2088427.2 |
  2091482.6 |
  2094538.0 |
  2097593.4 |
  2100648.9 |
  2103704.3 |
  2106759.7 |
  2109815.1 |
  2112870.6 |
  2115926.0 |
  2118981.4 |
  2122036.8 |
  2125092.2 |
  2128147.7 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2073650.8-2106608.0 ns)
  2073650.8 |########################################
  2075298.7 |
  2076946.5 |
  2078594.4 |####################
  2080242.2 |####################
  2081890.1 |
  2083537.9 |
  2085185.8 |
  2086833.7 |####################
  2088481.5 |
  2090129.4 |
  2091777.2 |
  2093425.1 |
  2095072.9 |
  2096720.8 |
  2098368.7 |
  2100016.5 |
  2101664.4 |
  2103312.2 |
  2104960.1 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3102433.3-3113645.4 ns)
  3102433.3 |########################################
  3102993.9 |
  3103554.5 |########################################
  3104115.1 |
  3104675.7 |
  3105236.3 |
  3105796.9 |########################################
  3106357.5 |
  3106918.1 |
  3107478.7 |
  3108039.4 |########################################
  3108600.0 |
  3109160.6 |
  3109721.2 |
  3110281.8 |
  3110842.4 |
  3111403.0 |
  3111963.6 |
  3112524.2 |
  3113084.8 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3879387.9-3894800.4 ns)
  3879387.9 |########################################
  3880158.5 |
  3880929.1 |
  3881699.8 |
  3882470.4 |########################################
  3883241.0 |
  3884011.6 |
  3884782.3 |
  3885552.9 |
  3886323.5 |
  3887094.1 |
  3887864.8 |########################################
  3888635.4 |########################################
  3889406.0 |
  3890176.6 |
  3890947.3 |
  3891717.9 |########################################
  3892488.5 |
  3893259.1 |
  3894029.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **abi_zig_entry_real_zig_anchor**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=311.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=6114.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=311.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=308.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=306.4% of algo (FFI overhead may distort results)

# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 41617% faster than the next best (abi_zig_entry_scatter_zig_per_w_set)

abi_zig_entry_scatter_zig_null (5.03 us) leads abi_zig_entry_scatter_zig_per_w_set (2.10 ms) by 41617%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.11 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_runtime_w is an outlier: 773.0x slower than the field

abi_zig_entry_scatter_zig_tail_runtime_w (3.89 ms) is 773.0x the fastest (5.03 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_scatter_zig_null shows alternating (throttle bounce) (autocorr -0.70)

abi_zig_entry_scatter_zig_null's per-pass series has lag-1 autocorrelation -0.70, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} (41617% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} with a 41617% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 773.0x the fastest

Fastest abi_zig_entry_scatter_zig_null (5.03 us) to slowest abi_zig_entry_scatter_zig_tail_runtime_w (3.89 ms): 773.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 5034.4 ns median (-99.8% vs baseline)
- 4 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 772.95x (fastest 5034.4 ns, slowest 3891317.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2104933ns | 2108543ns | 2089323ns | 2106636ns | 2110181ns | -1.37% |
| abi_zig_entry_scatter_zig_dispatch | 2102267ns | 2103053ns | 2095180ns | 2100718ns | 2108132ns | -1.49% |
| abi_zig_entry_scatter_zig_null | 7288ns | 7299ns | 7198ns | 7270ns | 7359ns | -99.66% |
| abi_zig_entry_scatter_zig_per_w_set | 2102969ns | 2102849ns | 2098255ns | 2101498ns | 2107534ns | -1.46% |
| abi_zig_entry_scatter_zig_runtime_w | 2134112ns | 2115558ns | 2101778ns | 2113188ns | 2181665ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3880081ns | 3877317ns | 3864652ns | 3876183ns | 3893642ns | +81.81% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3938109ns | 3894244ns | 3871411ns | 3886805ns | 4048415ns | +84.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2102325ns | 2086750ns | 2107450ns | -1.36% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2099650ns | 2092601ns | 2105391ns | -1.49% | 0.000 |
| abi_zig_entry_scatter_zig_null | 5020ns | 4935ns | 5064ns | -99.76% | 0.000 |
| abi_zig_entry_scatter_zig_per_w_set | 2100350ns | 2095577ns | 2104940ns | -1.45% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2131345ns | 2099125ns | 2178760ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3877322ns | 3862092ns | 3890880ns | +81.92% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3935194ns | 3868796ns | 4045231ns | +84.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 178057.8 | 2104097.6 | 2102325.1 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 176574.5 | 2100194.6 | 2099649.9 | n/a |
| abi_zig_entry_scatter_zig_null | 152812.2 | 5152.1 | 5019.8 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 173235.2 | 2100413.3 | 2100350.2 | n/a |
| abi_zig_entry_scatter_zig_runtime_w | 187691.4 | 2126024.5 | 2131344.9 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 190629.9 | 3885218.2 | 3877321.6 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 201750.5 | 3941912.7 | 3935194.3 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_null | 0.000 | 98.0% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2104933ns | 2104933ns | -1.37% |
| abi_zig_entry_scatter_zig_dispatch | 2102267ns | 2102267ns | -1.49% |
| abi_zig_entry_scatter_zig_null | 7288ns | 7288ns | -99.66% |
| abi_zig_entry_scatter_zig_per_w_set | 2102969ns | 2102969ns | -1.46% |
| abi_zig_entry_scatter_zig_runtime_w | 2134112ns | 2134112ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3880081ns | 3880081ns | +81.81% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3938109ns | 3938109ns | +84.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2112864ns | base | --- | [2102411, 2178760] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2105956ns | -9799.0ns (-0.5%) | [-74701, -2560]ns | [2093569, 2107450] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2100565ns | -16716.5ns (-0.8%) | [-73369, -5000]ns | [2092994, 2105391] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_null | 5034ns | -2107884.2ns (-99.8%) | [-2173744, -2097347]ns | [4961, 5064] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2100194ns | -13310.4ns (-0.6%) | [-75273, -4401]ns | [2095917, 2104940] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3874444ns | +1757413.6ns (+83.2%) | [+1701542, +1778974]ns | [3866641, 3890880] | YES | 0.0375 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3891317ns | +1779988.8ns (+84.2%) | [+1706958, +1924601]ns | [3869034, 4045231] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2139065ns | -1.5% | -1.7% | -99.8% | -1.6% | +81.2% | +96.2% |
| 2 | 2112756ns | -0.3% | -1.0% | -99.8% | -0.8% | +83.2% | +84.2% |
| 3 | 2099125ns | -0.6% | -0.3% | -99.8% | -0.0% | +84.5% | +85.3% |
| 4 | 2218455ns | -5.3% | -5.0% | -99.8% | -5.2% | +75.1% | +74.4% |
| 5 | 2112971ns | -0.3% | -0.6% | -99.8% | -0.4% | +84.5% | +84.2% |
| 6 | 2105698ns | +0.1% | -0.2% | -99.8% | -0.4% | +83.4% | +83.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.029 | ok |
| abi_zig_entry_scatter_zig_dispatch | -0.173 | ok |
| abi_zig_entry_scatter_zig_null | -0.699 | HIGH- (thermal bounce) |
| abi_zig_entry_scatter_zig_per_w_set | -0.303 | moderate- |
| abi_zig_entry_scatter_zig_runtime_w | -0.347 | moderate- |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.226 | moderate- |
| abi_zig_entry_scatter_zig_tail_runtime_w | -0.008 | ok |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 5/6, lost 0/6
- **abi_zig_entry_scatter_zig_dispatch**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 5/6, lost 0/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6553750.8ns | 2102325.1ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6544514.5ns | 2099649.9ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_null | 308012.4ns | 5019.8ns | 6135.9% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6538875.8ns | 2100350.2ns | 311.3% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6714275.6ns | 2131344.9ns | 315.0% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11915411.8ns | 3877321.6ns | 307.3% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 12094667.6ns | 3935194.3ns | 307.3% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2086749.6-2107449.6 ns)
  2086749.6 |####################
  2087784.6 |
  2088819.6 |
  2089854.6 |
  2090889.6 |
  2091924.6 |
  2092959.6 |
  2093994.6 |
  2095029.6 |
  2096064.6 |
  2097099.6 |
  2098134.6 |
  2099169.6 |
  2100204.6 |####################
  2101239.6 |
  2102274.6 |
  2103309.6 |
  2104344.6 |
  2105379.6 |########################################
  2106414.6 |####################
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2092601.2-2105391.2 ns)
  2092601.2 |########################################
  2093240.7 |########################################
  2093880.2 |
  2094519.7 |
  2095159.2 |
  2095798.7 |
  2096438.2 |
  2097077.7 |
  2097717.2 |
  2098356.7 |
  2098996.2 |
  2099635.7 |########################################
  2100275.2 |
  2100914.7 |########################################
  2101554.2 |
  2102193.7 |########################################
  2102833.2 |
  2103472.7 |
  2104112.2 |
  2104751.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 4935.4-5064.1 ns)
   4935.4 |####################
   4941.8 |
   4948.3 |
   4954.7 |
   4961.1 |
   4967.6 |
   4974.0 |
   4980.5 |####################
   4986.9 |
   4993.3 |
   4999.8 |
   5006.2 |
   5012.6 |
   5019.1 |####################
   5025.5 |
   5032.0 |
   5038.4 |
   5044.8 |########################################
   5051.3 |
   5057.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2095577.1-2104940.5 ns)
  2095577.1 |########################################
  2096045.3 |########################################
  2096513.4 |
  2096981.6 |
  2097449.8 |
  2097917.9 |########################################
  2098386.1 |
  2098854.3 |
  2099322.4 |
  2099790.6 |
  2100258.8 |
  2100726.9 |
  2101195.1 |
  2101663.3 |########################################
  2102131.4 |
  2102599.6 |
  2103067.8 |
  2103535.9 |
  2104004.1 |
  2104472.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2099124.6-2178760.0 ns)
  2099124.6 |####################
  2103106.4 |####################
  2107088.1 |
  2111069.9 |########################################
  2115051.7 |
  2119033.5 |
  2123015.2 |
  2126997.0 |
  2130978.8 |
  2134960.5 |
  2138942.3 |####################
  2142924.1 |
  2146905.8 |
  2150887.6 |
  2154869.4 |
  2158851.1 |
  2162832.9 |
  2166814.7 |
  2170796.5 |
  2174778.2 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3862092.1-3890880.4 ns)
  3862092.1 |########################################
  3863531.5 |
  3864970.9 |
  3866410.3 |
  3867849.8 |
  3869289.2 |
  3870728.6 |########################################
  3872168.0 |########################################
  3873607.4 |
  3875046.8 |
  3876486.2 |########################################
  3877925.7 |
  3879365.1 |
  3880804.5 |
  3882243.9 |
  3883683.3 |########################################
  3885122.7 |
  3886562.2 |
  3888001.6 |
  3889441.0 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3868795.8-4045231.2 ns)
  3868795.8 |##########################
  3877617.6 |
  3886439.3 |########################################
  3895261.1 |
  3904082.9 |
  3912904.7 |
  3921726.4 |
  3930548.2 |
  3939370.0 |
  3948191.8 |
  3957013.5 |
  3965835.3 |
  3974657.1 |
  3983478.8 |
  3992300.6 |
  4001122.4 |
  4009944.2 |
  4018765.9 |
  4027587.7 |
  4036409.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=6110.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=311.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=307.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=306.4% of algo (FFI overhead may distort results)

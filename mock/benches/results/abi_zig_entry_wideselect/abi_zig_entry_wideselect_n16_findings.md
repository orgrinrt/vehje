# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 78496% faster than the next best (abi_zig_entry_wideselect_zig_runtime_w)

abi_zig_entry_wideselect_zig_null (2.51 us) leads abi_zig_entry_wideselect_zig_runtime_w (1.97 ms) by 78496%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_runtime_w is an outlier: 1316.8x slower than the field

abi_zig_entry_wideselect_zig_tail_runtime_w (3.31 ms) is 1316.8x the fastest (2.51 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} (78496% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} with a 78496% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1316.8x the fastest

Fastest abi_zig_entry_wideselect_zig_null (2.51 us) to slowest abi_zig_entry_wideselect_zig_tail_runtime_w (3.31 ms): 1316.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 2512.5 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1316.78x (fastest 2512.5 ns, slowest 3308402.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1989939ns | 1983769ns | 1982047ns | 1983315ns | 2003822ns | -0.87% |
| abi_zig_entry_wideselect_zig_dispatch | 1977984ns | 1977765ns | 1972951ns | 1976920ns | 1982097ns | -1.47% |
| abi_zig_entry_wideselect_zig_null | 4813ns | 4815ns | 4697ns | 4801ns | 4890ns | -99.76% |
| abi_zig_entry_wideselect_zig_per_w_set | 1983055ns | 1978473ns | 1973842ns | 1977716ns | 1995669ns | -1.22% |
| abi_zig_entry_wideselect_zig_runtime_w | 2007493ns | 1977493ns | 1975394ns | 1976928ns | 2069391ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3309774ns | 3310440ns | 3304978ns | 3309777ns | 3312166ns | +64.87% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3312201ns | 3311170ns | 3306653ns | 3309850ns | 3318501ns | +64.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1987225ns | 1979536ns | 2001043ns | -0.87% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1975292ns | 1970257ns | 1979325ns | -1.47% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 2513ns | 2465ns | 2551ns | -99.87% | 0.006 |
| abi_zig_entry_wideselect_zig_per_w_set | 1980361ns | 1971235ns | 1992792ns | -1.21% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 2004712ns | 1972805ns | 2066379ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3307002ns | 3302220ns | 3309394ns | +64.96% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3309419ns | 3304025ns | 3315643ns | +65.08% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 202116.2 | 1985604.6 | 1987224.6 | 1 |
| abi_zig_entry_wideselect_zig_dispatch | 178748.4 | 1973463.1 | 1975292.2 | n/a |
| abi_zig_entry_wideselect_zig_null | 156671.8 | 2730.1 | 2512.6 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 183652.2 | 1980595.8 | 1980360.7 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 189782.8 | 1994578.8 | 2004712.4 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 189333.1 | 3308950.5 | 3307001.9 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 191756.4 | 3307483.6 | 3309419.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_null | 0.006 | 98.1% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1989939ns | 1989939ns | -0.87% |
| abi_zig_entry_wideselect_zig_dispatch | 1977984ns | 1977984ns | -1.47% |
| abi_zig_entry_wideselect_zig_null | 4813ns | 4813ns | -99.76% |
| abi_zig_entry_wideselect_zig_per_w_set | 1983055ns | 1983055ns | -1.22% |
| abi_zig_entry_wideselect_zig_runtime_w | 2007493ns | 2007493ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3309774ns | 3309774ns | +64.87% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3312201ns | 3312201ns | +64.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1974733ns | base | --- | [1973025, 2066379] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1980975ns | no significant difference | [-69673, +10469]ns | [1979656, 2001043] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1975113ns | no significant difference | [-89682, +2372]ns | [1971439, 1979325] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_null | 2512ns | -1972233.4ns (-99.9%) | [-2063878, -1970488]ns | [2474, 2551] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1975850ns | no significant difference | [-77035, +4669]ns | [1972439, 1992792] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3307702ns | +1332957.7ns (+67.5%) | [+1239250, +1334661]ns | [3303910, 3309394] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3308402ns | +1332297.9ns (+67.5%) | [+1245012, +1336811]ns | [3304212, 3315643] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1975218ns | +0.7% | -0.3% | -99.9% | -0.0% | +67.6% | +67.3% |
| 2 | 1973245ns | +0.3% | -0.0% | -99.9% | +0.2% | +67.6% | +67.6% |
| 3 | 2156435ns | -6.6% | -8.1% | -99.9% | -6.9% | +53.4% | +53.8% |
| 4 | 1972805ns | +0.4% | +0.2% | -99.9% | +0.3% | +67.6% | +67.5% |
| 5 | 1976322ns | +0.2% | -0.1% | -99.9% | -0.3% | +67.1% | +67.5% |
| 6 | 1974249ns | +0.4% | +0.0% | -99.9% | -0.0% | +67.6% | +67.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.333 | moderate- |
| abi_zig_entry_wideselect_zig_dispatch | 0.051 | ok |
| abi_zig_entry_wideselect_zig_null | -0.009 | ok |
| abi_zig_entry_wideselect_zig_per_w_set | -0.049 | ok |
| abi_zig_entry_wideselect_zig_runtime_w | -0.250 | moderate- |
| abi_zig_entry_wideselect_zig_tail_dispatch | -0.222 | moderate- |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.350 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 1/6, lost 5/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 2/6, lost 1/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 2/6, lost 2/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6226386.3ns | 1987224.6ns | 313.3% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6169397.1ns | 1975292.2ns | 312.3% | HIGH |
| abi_zig_entry_wideselect_zig_null | 302590.1ns | 2512.6ns | 12043.1% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6191212.1ns | 1980360.7ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6273083.3ns | 2004712.4ns | 312.9% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10186662.9ns | 3307001.9ns | 308.0% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10189557.8ns | 3309419.2ns | 307.9% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1979535.8-2001042.9 ns)
  1979535.8 |########################################
  1980611.2 |#############
  1981686.5 |
  1982761.9 |
  1983837.2 |
  1984912.6 |
  1985987.9 |
  1987063.3 |
  1988138.6 |#############
  1989214.0 |
  1990289.4 |
  1991364.7 |
  1992440.1 |
  1993515.4 |
  1994590.8 |
  1995666.1 |
  1996741.5 |
  1997816.8 |
  1998892.2 |
  1999967.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1970256.7-1979324.8 ns)
  1970256.7 |####################
  1970710.1 |
  1971163.5 |
  1971616.9 |
  1972070.3 |
  1972523.7 |####################
  1972977.1 |
  1973430.5 |
  1973883.9 |
  1974337.3 |
  1974790.8 |########################################
  1975244.2 |
  1975697.6 |
  1976151.0 |
  1976604.4 |####################
  1977057.8 |
  1977511.2 |
  1977964.6 |
  1978418.0 |
  1978871.4 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 2465.0-2551.4 ns)
   2465.0 |########################################
   2469.3 |
   2473.6 |
   2478.0 |
   2482.3 |########################################
   2486.6 |
   2490.9 |
   2495.3 |
   2499.6 |
   2503.9 |########################################
   2508.2 |
   2512.5 |
   2516.9 |########################################
   2521.2 |
   2525.5 |
   2529.8 |
   2534.2 |########################################
   2538.5 |
   2542.8 |
   2547.1 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1971234.6-1992792.5 ns)
  1971234.6 |####################
  1972312.5 |
  1973390.4 |########################################
  1974468.3 |
  1975546.2 |
  1976624.1 |####################
  1977702.0 |####################
  1978779.9 |
  1979857.8 |
  1980935.7 |
  1982013.6 |
  1983091.4 |
  1984169.3 |
  1985247.2 |
  1986325.1 |
  1987403.0 |
  1988480.9 |
  1989558.8 |
  1990636.7 |
  1991714.6 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1972805.4-2066378.8 ns)
  1972805.4 |########################################
  1977484.1 |
  1982162.7 |
  1986841.4 |
  1991520.1 |
  1996198.7 |
  2000877.4 |
  2005556.1 |
  2010234.7 |
  2014913.4 |
  2019592.1 |
  2024270.7 |
  2028949.4 |
  2033628.1 |
  2038306.7 |
  2042985.4 |
  2047664.1 |
  2052342.7 |
  2057021.4 |
  2061700.1 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3302220.4-3309394.0 ns)
  3302220.4 |####################
  3302579.1 |
  3302937.8 |
  3303296.4 |
  3303655.1 |
  3304013.8 |
  3304372.5 |
  3304731.1 |
  3305089.8 |
  3305448.5 |####################
  3305807.2 |
  3306165.9 |####################
  3306524.5 |
  3306883.2 |
  3307241.9 |
  3307600.6 |
  3307959.2 |
  3308317.9 |
  3308676.6 |
  3309035.3 |########################################
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3304025.4-3315643.4 ns)
  3304025.4 |########################################
  3304606.3 |
  3305187.2 |
  3305768.1 |
  3306349.0 |####################
  3306929.9 |
  3307510.8 |
  3308091.7 |
  3308672.6 |
  3309253.5 |
  3309834.4 |####################
  3310415.3 |
  3310996.2 |
  3311577.1 |
  3312158.0 |
  3312738.9 |
  3313319.8 |
  3313900.7 |####################
  3314481.6 |
  3315062.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=12006.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=307.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=307.9% of algo (FFI overhead may distort results)

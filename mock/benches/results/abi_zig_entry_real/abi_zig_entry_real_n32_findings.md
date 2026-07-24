# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 90457% faster than the next best (abi_zig_entry_real_zig_dispatch)

abi_zig_entry_real_zig_null (2.28 us) leads abi_zig_entry_real_zig_dispatch (2.07 ms) by 90457%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.07 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 1687.3x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.85 ms) is 1687.3x the fastest (2.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_null shows alternating (throttle bounce) (autocorr -0.73)

abi_zig_entry_real_zig_null's per-pass series has lag-1 autocorrelation -0.73, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (90457% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 90457% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1687.3x the fastest

Fastest abi_zig_entry_real_zig_null (2.28 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.85 ms): 1687.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 2284.6 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1687.26x (fastest 2284.6 ns, slowest 3854714.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2084393ns | 2074576ns | 2066577ns | 2074007ns | 2108879ns | -0.10% |
| abi_zig_entry_real_zig_dispatch | 2072197ns | 2071537ns | 2068851ns | 2071066ns | 2075568ns | -0.68% |
| abi_zig_entry_real_zig_null | 4594ns | 4587ns | 4538ns | 4574ns | 4652ns | -99.78% |
| abi_zig_entry_real_zig_per_w_set | 2071756ns | 2071596ns | 2066754ns | 2070923ns | 2075506ns | -0.71% |
| abi_zig_entry_real_zig_runtime_w | 2086478ns | 2072181ns | 2066396ns | 2070865ns | 2119938ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3630423ns | 3853202ns | 3106413ns | 3650542ns | 3862251ns | +74.00% |
| abi_zig_entry_real_zig_tail_runtime_w | 3820754ns | 3857604ns | 3113074ns | 3851202ns | 4128924ns | +83.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2081542ns | 2063805ns | 2105817ns | -0.11% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2069534ns | 2066149ns | 2072883ns | -0.68% | 0.000 |
| abi_zig_entry_real_zig_null | 2285ns | 2258ns | 2308ns | -99.89% | 0.014 |
| abi_zig_entry_real_zig_per_w_set | 2069061ns | 2064242ns | 2072732ns | -0.70% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2083732ns | 2063730ns | 2117019ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3627410ns | 3103614ns | 3859157ns | +74.08% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3817705ns | 3110212ns | 4125452ns | +83.21% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 193513.1 | 2100212.1 | 2081542.0 | 4 |
| abi_zig_entry_real_zig_dispatch | 181075.7 | 2069938.7 | 2069533.8 | n/a |
| abi_zig_entry_real_zig_null | 157217.7 | 2565.6 | 2284.9 | n/a |
| abi_zig_entry_real_zig_per_w_set | 184046.9 | 2070041.7 | 2069061.2 | n/a |
| abi_zig_entry_real_zig_runtime_w | 185577.8 | 2078308.5 | 2083731.6 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 204886.6 | 3624972.6 | 3627410.4 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 211372.1 | 3847632.4 | 3817705.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_null | 0.014 | 98.8% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2084393ns | 2084393ns | -0.10% |
| abi_zig_entry_real_zig_dispatch | 2072197ns | 2072197ns | -0.68% |
| abi_zig_entry_real_zig_null | 4594ns | 4594ns | -99.78% |
| abi_zig_entry_real_zig_per_w_set | 2071756ns | 2071756ns | -0.71% |
| abi_zig_entry_real_zig_runtime_w | 2086478ns | 2086478ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3630423ns | 3630423ns | +74.00% |
| abi_zig_entry_real_zig_tail_runtime_w | 3820754ns | 3820754ns | +83.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2069486ns | base | --- | [2064689, 2117019] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2071859ns | no significant difference | [-14291, +8351]ns | [2066950, 2105817] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_dispatch | 2068867ns | no significant difference | [-45116, +3401]ns | [2066852, 2072883] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_null | 2285ns | -2067185.9ns (-99.9%) | [-2114735, -2062419]ns | [2262, 2308] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2068893ns | no significant difference | [-46834, +4862]ns | [2065559, 2072732] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3850316ns | +1740298.6ns (+84.1%) | [+1106589, +1784148]ns | [3172758, 3859157] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3854714ns | +1767665.0ns (+85.4%) | [+1375851, +2058405]ns | [3472949, 4125452] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2068608ns | +0.4% | +0.1% | -99.9% | -0.2% | +50.0% | +85.4% |
| 2 | 2063730ns | +0.4% | +0.2% | -99.9% | +0.2% | +57.1% | +112.7% |
| 3 | 2086223ns | -0.8% | -0.8% | -99.9% | -0.9% | +84.9% | +84.8% |
| 4 | 2065649ns | +0.3% | +0.1% | -99.9% | +0.3% | +86.1% | +50.6% |
| 5 | 2070365ns | -0.3% | -0.2% | -99.9% | +0.0% | +86.5% | +86.5% |
| 6 | 2147816ns | -0.6% | -3.4% | -99.9% | -3.5% | +79.6% | +79.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.140 | ok |
| abi_zig_entry_real_zig_dispatch | -0.357 | moderate- |
| abi_zig_entry_real_zig_null | -0.729 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_per_w_set | 0.302 | moderate+ |
| abi_zig_entry_real_zig_runtime_w | -0.078 | ok |
| abi_zig_entry_real_zig_tail_dispatch | 0.424 | moderate+ |
| abi_zig_entry_real_zig_tail_runtime_w | -0.029 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 3/6, lost 3/6
- **abi_zig_entry_real_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 3/6, lost 2/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6533550.1ns | 2081542.0ns | 313.9% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6457252.8ns | 2069533.8ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_null | 299030.7ns | 2284.9ns | 13087.1% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6462105.0ns | 2069061.2ns | 312.3% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6555829.9ns | 2083731.6ns | 314.6% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 11268656.4ns | 3627410.4ns | 310.7% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11838136.5ns | 3817705.2ns | 310.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2063804.6-2105816.7 ns)
  2063804.6 |########################################
  2065905.2 |
  2068005.8 |########################################
  2070106.4 |########################################
  2072207.0 |########################################
  2074307.6 |########################################
  2076408.2 |
  2078508.8 |
  2080609.4 |
  2082710.0 |
  2084810.7 |
  2086911.3 |
  2089011.9 |
  2091112.5 |
  2093213.1 |
  2095313.7 |
  2097414.3 |
  2099514.9 |
  2101615.5 |
  2103716.1 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2066149.2-2072882.9 ns)
  2066149.2 |########################################
  2066485.9 |
  2066822.6 |
  2067159.3 |
  2067495.9 |########################################
  2067832.6 |
  2068169.3 |
  2068506.0 |########################################
  2068842.7 |########################################
  2069179.4 |
  2069516.0 |
  2069852.7 |
  2070189.4 |
  2070526.1 |
  2070862.8 |########################################
  2071199.5 |
  2071536.2 |
  2071872.8 |
  2072209.5 |
  2072546.2 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 2258.3-2307.7 ns)
   2258.3 |########################################
   2260.8 |
   2263.2 |
   2265.7 |########################################
   2268.2 |
   2270.7 |
   2273.1 |########################################
   2275.6 |
   2278.1 |
   2280.5 |
   2283.0 |
   2285.5 |
   2287.9 |
   2290.4 |
   2292.9 |########################################
   2295.3 |
   2297.8 |
   2300.3 |
   2302.8 |
   2305.2 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2064241.7-2072731.9 ns)
  2064241.7 |####################
  2064666.2 |
  2065090.7 |
  2065515.2 |
  2065939.7 |
  2066364.2 |
  2066788.7 |########################################
  2067213.3 |
  2067637.8 |
  2068062.3 |
  2068486.8 |
  2068911.3 |
  2069335.8 |
  2069760.3 |
  2070184.8 |
  2070609.3 |####################
  2071033.8 |
  2071458.3 |
  2071882.8 |####################
  2072307.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2063730.0-2117019.3 ns)
  2063730.0 |########################################
  2066394.5 |####################
  2069058.9 |####################
  2071723.4 |
  2074387.9 |
  2077052.3 |
  2079716.8 |
  2082381.3 |
  2085045.7 |####################
  2087710.2 |
  2090374.7 |
  2093039.1 |
  2095703.6 |
  2098368.1 |
  2101032.5 |
  2103697.0 |
  2106361.5 |
  2109025.9 |
  2111690.4 |
  2114354.9 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3103614.2-3859157.1 ns)
  3103614.2 |#############
  3141391.3 |
  3179168.5 |
  3216945.6 |#############
  3254722.8 |
  3292499.9 |
  3330277.1 |
  3368054.2 |
  3405831.4 |
  3443608.5 |
  3481385.7 |
  3519162.8 |
  3556939.9 |
  3594717.1 |
  3632494.2 |
  3670271.4 |
  3708048.5 |
  3745825.7 |
  3783602.8 |
  3821380.0 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3110212.5-4125452.2 ns)
  3110212.5 |##########
  3160974.5 |
  3211736.5 |
  3262498.5 |
  3313260.5 |
  3364022.4 |
  3414784.4 |
  3465546.4 |
  3516308.4 |
  3567070.4 |
  3617832.4 |
  3668594.4 |
  3719356.4 |
  3770118.3 |
  3820880.3 |########################################
  3871642.3 |
  3922404.3 |
  3973166.3 |
  4023928.3 |
  4074690.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=311.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=13116.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=307.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.3% of algo (FFI overhead may distort results)

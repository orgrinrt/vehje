# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 55058% faster than the next best (abi_zig_entry_tight_zig_dispatch)

abi_zig_entry_tight_zig_null (3.60 us) leads abi_zig_entry_tight_zig_dispatch (1.98 ms) by 55058%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -1.99 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_dispatch is an outlier: 860.1x slower than the field

abi_zig_entry_tight_zig_tail_dispatch (3.09 ms) is 860.1x the fastest (3.60 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_tight_zig_tail_dispatch shows alternating (throttle bounce) (autocorr -0.51)

abi_zig_entry_tight_zig_tail_dispatch's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} (55058% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} with a 55058% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 860.1x the fastest

Fastest abi_zig_entry_tight_zig_null (3.60 us) to slowest abi_zig_entry_tight_zig_tail_dispatch (3.09 ms): 860.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 3597.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 860.07x (fastest 3597.9 ns, slowest 3094456.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2038994ns | 1999091ns | 1987930ns | 1997572ns | 2126658ns | -0.79% |
| abi_zig_entry_tight_zig_dispatch | 1988827ns | 1987161ns | 1982693ns | 1985924ns | 1996249ns | -3.23% |
| abi_zig_entry_tight_zig_null | 5993ns | 5961ns | 5793ns | 5939ns | 6173ns | -99.71% |
| abi_zig_entry_tight_zig_per_w_set | 2033268ns | 1987296ns | 1977961ns | 1986378ns | 2131256ns | -1.07% |
| abi_zig_entry_tight_zig_runtime_w | 2055291ns | 1997655ns | 1984256ns | 1994555ns | 2181912ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3098603ns | 3097396ns | 3081586ns | 3094070ns | 3113911ns | +50.76% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3097909ns | 3089739ns | 3085799ns | 3089146ns | 3117108ns | +50.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2036078ns | 1985241ns | 2123154ns | -0.80% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1986145ns | 1980085ns | 1993372ns | -3.23% | 0.000 |
| abi_zig_entry_tight_zig_null | 3679ns | 3547ns | 3888ns | -99.82% | 0.002 |
| abi_zig_entry_tight_zig_per_w_set | 2030488ns | 1975455ns | 2128074ns | -1.07% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 2052416ns | 1981673ns | 2178663ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3095662ns | 3078844ns | 3110872ns | +50.83% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3095066ns | 3082953ns | 3114036ns | +50.80% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 200876.8 | 2024850.7 | 2036078.2 | 0 |
| abi_zig_entry_tight_zig_dispatch | 184207.8 | 1985742.7 | 1986144.7 | n/a |
| abi_zig_entry_tight_zig_null | 154736.8 | 3909.4 | 3679.4 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 198827.8 | 2010472.6 | 2030488.1 | 0 |
| abi_zig_entry_tight_zig_runtime_w | 193818.8 | 2047680.9 | 2052415.7 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 197062.2 | 3094774.4 | 3095661.7 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 193237.4 | 3091564.2 | 3095065.6 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_null | 0.002 | 98.6% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 2038994ns | 2038994ns | -0.79% |
| abi_zig_entry_tight_zig_dispatch | 1988827ns | 1988827ns | -3.23% |
| abi_zig_entry_tight_zig_null | 5993ns | 5993ns | -99.71% |
| abi_zig_entry_tight_zig_per_w_set | 2033268ns | 2033268ns | -1.07% |
| abi_zig_entry_tight_zig_runtime_w | 2055291ns | 2055291ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3098603ns | 3098603ns | +50.76% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3097909ns | 3097909ns | +50.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 1994879ns | base | --- | [1983705, 2178663] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1996500ns | no significant difference | [-59474, +9784]ns | [1988581, 2123154] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1984547ns | no significant difference | [-186856, +842]ns | [1980515, 1993372] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_tight_zig_null | 3598ns | -1991180.0ns (-99.8%) | [-2175082, -1979946]ns | [3552, 3888] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1984682ns | no significant difference | [-188255, +128681]ns | [1978708, 2128074] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3094457ns | +1095122.7ns (+54.9%) | [+914456, +1120159]ns | [3081656, 3110872] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3087033ns | +1095410.0ns (+54.9%) | [+919551, +1112989]ns | [3084128, 3114036] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1993715ns | +0.2% | -0.7% | -99.8% | -0.9% | +54.4% | +54.6% |
| 2 | 1981673ns | +0.7% | +0.1% | -99.8% | +0.0% | +57.2% | +55.8% |
| 3 | 1985737ns | +0.3% | +0.0% | -99.8% | -0.1% | +55.7% | +55.4% |
| 4 | 1996043ns | -0.5% | -0.8% | -99.8% | -0.5% | +54.5% | +54.7% |
| 5 | 2004039ns | -0.1% | -0.6% | -99.8% | +12.8% | +55.0% | +55.9% |
| 6 | 2353288ns | -4.6% | -15.2% | -99.8% | -15.2% | +31.6% | +31.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.002 | ok |
| abi_zig_entry_tight_zig_dispatch | 0.215 | moderate+ |
| abi_zig_entry_tight_zig_null | 0.322 | moderate+ |
| abi_zig_entry_tight_zig_per_w_set | -0.178 | ok |
| abi_zig_entry_tight_zig_runtime_w | 0.007 | ok |
| abi_zig_entry_tight_zig_tail_dispatch | -0.506 | HIGH- (thermal bounce) |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.201 | moderate+ |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 3/6, lost 3/6
- **abi_zig_entry_tight_zig_dispatch**: won 4/6, lost 0/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 4/6, lost 1/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6403419.6ns | 2036078.2ns | 314.5% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6217306.8ns | 1986144.7ns | 313.0% | HIGH |
| abi_zig_entry_tight_zig_null | 304325.2ns | 3679.4ns | 8271.1% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6289497.6ns | 2030488.1ns | 309.8% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6316894.3ns | 2052415.7ns | 307.8% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9552132.6ns | 3095661.7ns | 308.6% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9560819.2ns | 3095065.6ns | 308.9% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1985240.8-2123153.5 ns)
  1985240.8 |########################################
  1992136.4 |########################################
  1999032.1 |####################
  2005927.7 |
  2012823.3 |
  2019719.0 |
  2026614.6 |
  2033510.2 |
  2040405.9 |
  2047301.5 |
  2054197.1 |
  2061092.8 |
  2067988.4 |
  2074884.1 |
  2081779.7 |
  2088675.3 |
  2095571.0 |
  2102466.6 |
  2109362.2 |
  2116257.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1980084.6-1993372.5 ns)
  1980084.6 |########################################
  1980749.0 |########################################
  1981413.4 |
  1982077.8 |
  1982742.2 |########################################
  1983406.6 |
  1984071.0 |
  1984735.4 |
  1985399.8 |
  1986064.2 |########################################
  1986728.6 |
  1987392.9 |
  1988057.3 |
  1988721.7 |
  1989386.1 |
  1990050.5 |
  1990714.9 |
  1991379.3 |
  1992043.7 |########################################
  1992708.1 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 3546.7-3888.1 ns)
   3546.7 |########################################
   3563.8 |
   3580.8 |####################
   3597.9 |####################
   3615.0 |
   3632.0 |
   3649.1 |
   3666.2 |
   3683.3 |
   3700.3 |
   3717.4 |
   3734.5 |
   3751.5 |
   3768.6 |
   3785.7 |
   3802.8 |
   3819.8 |
   3836.9 |####################
   3854.0 |
   3871.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1975455.4-2128074.4 ns)
  1975455.4 |########################################
  1983086.3 |########################################
  1990717.3 |####################
  1998348.2 |
  2005979.2 |
  2013610.1 |
  2021241.1 |
  2028872.0 |
  2036503.0 |
  2044133.9 |
  2051764.9 |
  2059395.8 |
  2067026.8 |
  2074657.7 |
  2082288.7 |
  2089919.6 |
  2097550.6 |
  2105181.5 |
  2112812.5 |
  2120443.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1981673.3-2178663.1 ns)
  1981673.3 |########################################
  1991522.8 |########################################
  2001372.3 |####################
  2011221.8 |
  2021071.3 |
  2030920.8 |
  2040770.3 |
  2050619.7 |
  2060469.2 |
  2070318.7 |
  2080168.2 |
  2090017.7 |
  2099867.2 |
  2109716.7 |
  2119566.2 |
  2129415.7 |
  2139265.2 |
  2149114.7 |
  2158964.2 |
  2168813.7 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3078844.2-3110872.3 ns)
  3078844.2 |########################################
  3080445.6 |
  3082047.0 |
  3083648.4 |########################################
  3085249.8 |
  3086851.2 |
  3088452.6 |
  3090054.0 |
  3091655.4 |########################################
  3093256.8 |
  3094858.2 |
  3096459.7 |########################################
  3098061.1 |
  3099662.5 |
  3101263.9 |
  3102865.3 |
  3104466.7 |########################################
  3106068.1 |
  3107669.5 |
  3109270.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3082952.9-3114036.0 ns)
  3082952.9 |####################
  3084507.1 |####################
  3086061.2 |########################################
  3087615.4 |
  3089169.5 |
  3090723.7 |
  3092277.8 |
  3093832.0 |
  3095386.1 |
  3096940.3 |
  3098494.5 |
  3100048.6 |
  3101602.8 |####################
  3103156.9 |
  3104711.1 |
  3106265.2 |
  3107819.4 |
  3109373.5 |
  3110927.7 |
  3112481.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=311.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=8444.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=308.6% of algo (FFI overhead may distort results)

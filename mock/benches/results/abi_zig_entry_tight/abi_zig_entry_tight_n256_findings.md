# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 62999% faster than the next best (abi_zig_entry_tight_zig_runtime_w)

abi_zig_entry_tight_zig_null (3.14 us) leads abi_zig_entry_tight_zig_runtime_w (1.98 ms) by 62999%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -1.98 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_dispatch is an outlier: 984.6x slower than the field

abi_zig_entry_tight_zig_tail_dispatch (3.10 ms) is 984.6x the fastest (3.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_tight_zig_dispatch shows alternating (throttle bounce) (autocorr -0.80)

abi_zig_entry_tight_zig_dispatch's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} (62999% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} with a 62999% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 984.6x the fastest

Fastest abi_zig_entry_tight_zig_null (3.14 us) to slowest abi_zig_entry_tight_zig_tail_dispatch (3.10 ms): 984.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 3143.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 984.59x (fastest 3143.6 ns, slowest 3095092.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1997809ns | 1998696ns | 1994834ns | 1997799ns | 1999312ns | +0.56% |
| abi_zig_entry_tight_zig_dispatch | 1986005ns | 1986460ns | 1981268ns | 1985198ns | 1989584ns | -0.04% |
| abi_zig_entry_tight_zig_null | 5478ns | 5450ns | 5370ns | 5435ns | 5595ns | -99.72% |
| abi_zig_entry_tight_zig_per_w_set | 1987337ns | 1987368ns | 1982566ns | 1986775ns | 1990567ns | +0.03% |
| abi_zig_entry_tight_zig_runtime_w | 1986727ns | 1986197ns | 1982249ns | 1984916ns | 1991684ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3097985ns | 3097882ns | 3090805ns | 3096436ns | 3103898ns | +55.93% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3093386ns | 3093252ns | 3087547ns | 3092537ns | 3097578ns | +55.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1995062ns | 1992055ns | 1996621ns | +0.56% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1983326ns | 1978663ns | 1986796ns | -0.03% | 0.000 |
| abi_zig_entry_tight_zig_null | 3147ns | 3105ns | 3189ns | -99.84% | 0.081 |
| abi_zig_entry_tight_zig_per_w_set | 1984598ns | 1980043ns | 1987781ns | +0.03% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 1984018ns | 1979718ns | 1988758ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3095210ns | 3088162ns | 3101093ns | +56.01% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3090574ns | 3084832ns | 3094758ns | +55.77% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 181407.6 | 1993276.9 | 1995062.1 | 0 |
| abi_zig_entry_tight_zig_dispatch | 182979.2 | 1983956.2 | 1983325.6 | n/a |
| abi_zig_entry_tight_zig_null | 156466.0 | 3199.9 | 3147.4 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 184100.7 | 1986522.3 | 1984598.0 | 0 |
| abi_zig_entry_tight_zig_runtime_w | 184351.1 | 1983749.6 | 1984017.6 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 192060.6 | 3092948.0 | 3095209.6 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 187487.6 | 3085737.4 | 3090574.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_null | 0.081 | 98.8% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1997809ns | 1997809ns | +0.56% |
| abi_zig_entry_tight_zig_dispatch | 1986005ns | 1986005ns | -0.04% |
| abi_zig_entry_tight_zig_null | 5478ns | 5478ns | -99.72% |
| abi_zig_entry_tight_zig_per_w_set | 1987337ns | 1987337ns | +0.03% |
| abi_zig_entry_tight_zig_runtime_w | 1986727ns | 1986727ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3097985ns | 3097985ns | +55.93% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3093386ns | 3093386ns | +55.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 1983554ns | base | --- | [1979740, 1988758] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1995948ns | +10012.3ns (+0.5%) | [+6240, +16881]ns | [1992617, 1996621] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1983771ns | no significant difference | [-6049, +4595]ns | [1979410, 1986796] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_null | 3144ns | -1980381.9ns (-99.8%) | [-1985598, -1976630]ns | [3110, 3189] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1984630ns | no significant difference | [-4768, +6813]ns | [1981383, 1987781] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3095093ns | +1111422.3ns (+56.0%) | [+1105096, +1117058]ns | [3089442, 3101093] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3090490ns | +1106954.8ns (+55.8%) | [+1101382, +1111334]ns | [3086476, 3094758] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1989639ns | +0.3% | -0.2% | -99.8% | -0.3% | +56.0% | +55.3% |
| 2 | 1979718ns | +0.9% | +0.2% | -99.8% | +0.4% | +56.4% | +56.2% |
| 3 | 1987877ns | +0.4% | -0.1% | -99.8% | -0.0% | +55.5% | +55.8% |
| 4 | 1985809ns | +0.3% | -0.4% | -99.8% | -0.2% | +55.8% | +55.5% |
| 5 | 1981300ns | +0.6% | +0.3% | -99.8% | +0.2% | +56.4% | +56.0% |
| 6 | 1979763ns | +0.8% | +0.0% | -99.8% | +0.0% | +56.0% | +55.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.154 | ok |
| abi_zig_entry_tight_zig_dispatch | -0.805 | HIGH- (thermal bounce) |
| abi_zig_entry_tight_zig_null | 0.103 | ok |
| abi_zig_entry_tight_zig_per_w_set | -0.201 | moderate- |
| abi_zig_entry_tight_zig_runtime_w | -0.290 | moderate- |
| abi_zig_entry_tight_zig_tail_dispatch | -0.055 | ok |
| abi_zig_entry_tight_zig_tail_runtime_w | -0.062 | ok |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_dispatch**: won 2/6, lost 2/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 2/6, lost 2/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6234671.0ns | 1995062.1ns | 312.5% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6208745.1ns | 1983325.6ns | 313.0% | HIGH |
| abi_zig_entry_tight_zig_null | 306942.3ns | 3147.4ns | 9752.1% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6209097.6ns | 1984598.0ns | 312.9% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6208107.6ns | 1984017.6ns | 312.9% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9547580.6ns | 3095209.6ns | 308.5% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9521706.1ns | 3090574.4ns | 308.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1992055.0-1996621.4 ns)
  1992055.0 |########################################
  1992283.3 |
  1992511.6 |
  1992740.0 |
  1992968.3 |########################################
  1993196.6 |
  1993424.9 |
  1993653.3 |
  1993881.6 |
  1994109.9 |
  1994338.2 |
  1994566.5 |
  1994794.9 |
  1995023.2 |
  1995251.5 |
  1995479.8 |
  1995708.2 |########################################
  1995936.5 |########################################
  1996164.8 |
  1996393.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1978662.9-1986795.6 ns)
  1978662.9 |########################################
  1979069.5 |
  1979476.2 |
  1979882.8 |########################################
  1980289.4 |
  1980696.1 |
  1981102.7 |
  1981509.4 |
  1981916.0 |
  1982322.6 |
  1982729.3 |########################################
  1983135.9 |
  1983542.5 |
  1983949.2 |
  1984355.8 |########################################
  1984762.5 |
  1985169.1 |
  1985575.7 |
  1985982.4 |########################################
  1986389.0 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 3105.0-3188.6 ns)
   3105.0 |########################################
   3109.2 |
   3113.4 |########################################
   3117.5 |
   3121.7 |########################################
   3125.9 |
   3130.1 |
   3134.2 |
   3138.4 |
   3142.6 |
   3146.8 |
   3151.0 |
   3155.1 |
   3159.3 |########################################
   3163.5 |
   3167.7 |
   3171.8 |
   3176.0 |
   3180.2 |########################################
   3184.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1980042.9-1987781.0 ns)
  1980042.9 |########################################
  1980429.8 |
  1980816.7 |
  1981203.6 |
  1981590.5 |
  1981977.4 |
  1982364.3 |########################################
  1982751.2 |
  1983138.1 |########################################
  1983525.0 |
  1983911.9 |
  1984298.9 |
  1984685.8 |
  1985072.7 |
  1985459.6 |
  1985846.5 |########################################
  1986233.4 |
  1986620.3 |########################################
  1987007.2 |
  1987394.1 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1979717.5-1988758.0 ns)
  1979717.5 |########################################
  1980169.5 |
  1980621.5 |
  1981073.6 |####################
  1981525.6 |
  1981977.6 |
  1982429.6 |
  1982881.7 |
  1983333.7 |
  1983785.7 |
  1984237.7 |
  1984689.7 |
  1985141.8 |
  1985593.8 |####################
  1986045.8 |
  1986497.8 |
  1986949.9 |
  1987401.9 |
  1987853.9 |####################
  1988305.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3088162.1-3101093.4 ns)
  3088162.1 |########################################
  3088808.7 |
  3089455.2 |
  3090101.8 |########################################
  3090748.4 |
  3091394.9 |
  3092041.5 |
  3092688.0 |########################################
  3093334.6 |
  3093981.2 |
  3094627.7 |
  3095274.3 |
  3095920.9 |
  3096567.4 |########################################
  3097214.0 |
  3097860.5 |########################################
  3098507.1 |
  3099153.7 |
  3099800.2 |
  3100446.8 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3084832.5-3094757.7 ns)
  3084832.5 |########################################
  3085328.8 |
  3085825.0 |
  3086321.3 |
  3086817.5 |
  3087313.8 |
  3087810.1 |########################################
  3088306.3 |
  3088802.6 |
  3089298.8 |
  3089795.1 |########################################
  3090291.4 |
  3090787.6 |########################################
  3091283.9 |
  3091780.1 |
  3092276.4 |
  3092772.7 |########################################
  3093268.9 |
  3093765.2 |
  3094261.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=313.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=9758.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=308.2% of algo (FFI overhead may distort results)

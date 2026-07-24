# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 86786% faster than the next best (abi_zig_entry_tight_zig_dispatch)

abi_zig_entry_tight_zig_null (2.28 us) leads abi_zig_entry_tight_zig_dispatch (1.98 ms) by 86786%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -1.98 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_dispatch is an outlier: 1355.3x slower than the field

abi_zig_entry_tight_zig_tail_dispatch (3.09 ms) is 1355.3x the fastest (2.28 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_tight_zig_anchor shows alternating (throttle bounce) (autocorr -0.55)

abi_zig_entry_tight_zig_anchor's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} (86786% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} with a 86786% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1355.3x the fastest

Fastest abi_zig_entry_tight_zig_null (2.28 us) to slowest abi_zig_entry_tight_zig_tail_dispatch (3.09 ms): 1355.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 2282.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1355.31x (fastest 2282.7 ns, slowest 3093771.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1995963ns | 1995988ns | 1993680ns | 1995342ns | 1998037ns | +0.40% |
| abi_zig_entry_tight_zig_dispatch | 1988734ns | 1985988ns | 1981370ns | 1985147ns | 1997796ns | +0.04% |
| abi_zig_entry_tight_zig_null | 4622ns | 4581ns | 4520ns | 4572ns | 4746ns | -99.77% |
| abi_zig_entry_tight_zig_per_w_set | 1988741ns | 1988925ns | 1985182ns | 1988291ns | 1991196ns | +0.04% |
| abi_zig_entry_tight_zig_runtime_w | 1987981ns | 1987837ns | 1983288ns | 1987520ns | 1991020ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3097602ns | 3096430ns | 3088794ns | 3094867ns | 3106111ns | +55.82% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3088422ns | 3096552ns | 3033923ns | 3095868ns | 3104503ns | +55.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1993384ns | 1991206ns | 1995422ns | +0.40% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1986035ns | 1978833ns | 1994985ns | +0.03% | 0.000 |
| abi_zig_entry_tight_zig_null | 2290ns | 2252ns | 2335ns | -99.88% | 0.014 |
| abi_zig_entry_tight_zig_per_w_set | 1986017ns | 1982553ns | 1988489ns | +0.03% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 1985378ns | 1980796ns | 1988421ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3094823ns | 3086102ns | 3103133ns | +55.88% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3085623ns | 3031242ns | 3101649ns | +55.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 175549.5 | 1989989.1 | 1993383.9 | n/a |
| abi_zig_entry_tight_zig_dispatch | 178098.1 | 1983915.5 | 1986034.6 | n/a |
| abi_zig_entry_tight_zig_null | 154466.8 | 2552.4 | 2290.3 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 181877.1 | 1989025.8 | 1986016.5 | n/a |
| abi_zig_entry_tight_zig_runtime_w | 174518.1 | 1984880.4 | 1985377.8 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 187317.5 | 3091182.8 | 3094823.5 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 189661.4 | 3085916.0 | 3085622.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.014 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_null | 0.014 | 98.7% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1995963ns | 1995963ns | +0.40% |
| abi_zig_entry_tight_zig_dispatch | 1988734ns | 1988734ns | +0.04% |
| abi_zig_entry_tight_zig_null | 4622ns | 4622ns | -99.77% |
| abi_zig_entry_tight_zig_per_w_set | 1988741ns | 1988741ns | +0.04% |
| abi_zig_entry_tight_zig_runtime_w | 1987981ns | 1987981ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3097602ns | 3097602ns | +55.82% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3088422ns | 3088422ns | +55.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 1985184ns | base | --- | [1982528, 1988421] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1993385ns | +8215.6ns (+0.4%) | [+4597, +11206]ns | [1991345, 1995422] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1983342ns | no significant difference | [-6048, +9031]ns | [1979777, 1994985] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_null | 2283ns | -1982915.4ns (-99.9%) | [-1986138, -1980209]ns | [2254, 2335] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1986150ns | no significant difference | [-1355, +3478]ns | [1983411, 1988489] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3093771ns | +1108602.1ns (+55.8%) | [+1101596, +1118139]ns | [3087566, 3103133] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3093755ns | +1108428.8ns (+55.8%) | [+1077242, +1115064]ns | [3061464, 3101649] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1989193ns | +0.3% | -0.2% | -99.9% | -0.0% | +56.2% | +55.7% |
| 2 | 1987649ns | +0.2% | +0.1% | -99.9% | -0.1% | +55.4% | +55.5% |
| 3 | 1984291ns | +0.6% | -0.2% | -99.9% | -0.0% | +55.5% | +55.9% |
| 4 | 1986077ns | +0.3% | -0.4% | -99.9% | +0.1% | +55.8% | +56.4% |
| 5 | 1984261ns | +0.5% | +0.8% | -99.9% | -0.1% | +55.9% | +56.0% |
| 6 | 1980796ns | +0.5% | +0.1% | -99.9% | +0.2% | +56.5% | +53.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.545 | HIGH- (thermal bounce) |
| abi_zig_entry_tight_zig_dispatch | -0.470 | moderate- |
| abi_zig_entry_tight_zig_null | -0.243 | moderate- |
| abi_zig_entry_tight_zig_per_w_set | -0.333 | moderate- |
| abi_zig_entry_tight_zig_runtime_w | 0.224 | moderate+ |
| abi_zig_entry_tight_zig_tail_dispatch | -0.087 | ok |
| abi_zig_entry_tight_zig_tail_runtime_w | -0.019 | ok |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_dispatch**: won 3/6, lost 2/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 0/6, lost 2/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6217580.1ns | 1993383.9ns | 311.9% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6199754.4ns | 1986034.6ns | 312.2% | HIGH |
| abi_zig_entry_tight_zig_null | 295572.0ns | 2290.3ns | 12905.5% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6215776.0ns | 1986016.5ns | 313.0% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6200101.7ns | 1985377.8ns | 312.3% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9537674.8ns | 3094823.5ns | 308.2% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9516227.5ns | 3085622.7ns | 308.4% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1991206.2-1995422.3 ns)
  1991206.2 |########################################
  1991417.0 |########################################
  1991627.8 |
  1991838.6 |
  1992049.4 |
  1992260.2 |
  1992471.0 |
  1992681.8 |
  1992892.6 |########################################
  1993103.4 |
  1993314.2 |
  1993525.1 |
  1993735.9 |########################################
  1993946.7 |
  1994157.5 |
  1994368.3 |########################################
  1994579.1 |
  1994789.9 |
  1995000.7 |
  1995211.5 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1978833.3-1994985.4 ns)
  1978833.3 |########################################
  1979640.9 |
  1980448.5 |########################################
  1981256.1 |
  1982063.7 |########################################
  1982871.3 |
  1983678.9 |########################################
  1984486.6 |
  1985294.2 |
  1986101.8 |
  1986909.4 |
  1987717.0 |
  1988524.6 |
  1989332.2 |
  1990139.8 |########################################
  1990947.4 |
  1991755.0 |
  1992562.6 |
  1993370.2 |
  1994177.8 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 2252.1-2334.6 ns)
   2252.1 |########################################
   2256.2 |
   2260.3 |
   2264.5 |####################
   2268.6 |
   2272.7 |
   2276.8 |
   2281.0 |
   2285.1 |
   2289.2 |
   2293.3 |
   2297.5 |####################
   2301.6 |
   2305.7 |
   2309.8 |
   2314.0 |
   2318.1 |
   2322.2 |####################
   2326.3 |
   2330.5 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1982552.9-1988489.0 ns)
  1982552.9 |########################################
  1982849.7 |
  1983146.5 |
  1983443.3 |
  1983740.1 |
  1984036.9 |########################################
  1984333.7 |
  1984630.5 |
  1984927.3 |
  1985224.1 |
  1985520.9 |########################################
  1985817.7 |
  1986114.5 |
  1986411.3 |########################################
  1986708.1 |
  1987004.9 |
  1987301.7 |
  1987598.5 |
  1987895.3 |########################################
  1988192.1 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1980795.8-1988421.0 ns)
  1980795.8 |####################
  1981177.1 |
  1981558.3 |
  1981939.6 |
  1982320.8 |
  1982702.1 |
  1983083.4 |
  1983464.6 |
  1983845.9 |
  1984227.1 |########################################
  1984608.4 |
  1984989.7 |
  1985370.9 |
  1985752.2 |####################
  1986133.4 |
  1986514.7 |
  1986896.0 |
  1987277.2 |####################
  1987658.5 |
  1988039.7 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3086102.1-3103133.2 ns)
  3086102.1 |########################################
  3086953.7 |
  3087805.2 |
  3088656.8 |########################################
  3089508.3 |
  3090359.9 |
  3091211.4 |
  3092063.0 |
  3092914.5 |########################################
  3093766.1 |########################################
  3094617.6 |
  3095469.2 |
  3096320.7 |
  3097172.3 |
  3098023.8 |
  3098875.4 |########################################
  3099726.9 |
  3100578.5 |
  3101430.0 |
  3102281.6 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3031242.5-3101648.8 ns)
  3031242.5 |####################
  3034762.8 |
  3038283.1 |
  3041803.4 |
  3045323.8 |
  3048844.1 |
  3052364.4 |
  3055884.7 |
  3059405.0 |
  3062925.3 |
  3066445.6 |
  3069965.9 |
  3073486.2 |
  3077006.6 |
  3080526.9 |
  3084047.2 |
  3087567.5 |
  3091087.8 |########################################
  3094608.1 |########################################
  3098128.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=12917.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=307.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=308.3% of algo (FFI overhead may distort results)

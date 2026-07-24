# abi_zig_entry (tight)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_tight_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_tight_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_tight_zig_null dominates: 73894% faster than the next best (abi_zig_entry_tight_zig_per_w_set)

abi_zig_entry_tight_zig_null (2.68 us) leads abi_zig_entry_tight_zig_per_w_set (1.98 ms) by 73894%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_tight_zig_null beats baseline by 100% (significant)

abi_zig_entry_tight_zig_null is -1.98 ms (100%) faster than baseline abi_zig_entry_tight_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_tight_zig_tail_dispatch is an outlier: 1154.7x slower than the field

abi_zig_entry_tight_zig_tail_dispatch (3.09 ms) is 1154.7x the fastest (2.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_tight_zig_dispatch shows alternating (throttle bounce) (autocorr -0.62)

abi_zig_entry_tight_zig_dispatch's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_tight_zig_null} vs {abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} (73894% apart)

The field splits into a fast tier {abi_zig_entry_tight_zig_null} and a slow tier {abi_zig_entry_tight_zig_per_w_set, abi_zig_entry_tight_zig_dispatch, abi_zig_entry_tight_zig_runtime_w, abi_zig_entry_tight_zig_anchor, abi_zig_entry_tight_zig_tail_runtime_w, abi_zig_entry_tight_zig_tail_dispatch} with a 73894% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1154.7x the fastest

Fastest abi_zig_entry_tight_zig_null (2.68 us) to slowest abi_zig_entry_tight_zig_tail_dispatch (3.09 ms): 1154.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_tight_zig_null** at 2679.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1154.67x (fastest 2679.1 ns, slowest 3093536.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1997708ns | 1997617ns | 1991104ns | 1997380ns | 2001501ns | +0.55% |
| abi_zig_entry_tight_zig_dispatch | 1985038ns | 1985912ns | 1981089ns | 1984411ns | 1987953ns | -0.09% |
| abi_zig_entry_tight_zig_null | 5016ns | 5031ns | 4910ns | 5013ns | 5074ns | -99.75% |
| abi_zig_entry_tight_zig_per_w_set | 1985846ns | 1985266ns | 1983433ns | 1984775ns | 1988659ns | -0.05% |
| abi_zig_entry_tight_zig_runtime_w | 1986764ns | 1986719ns | 1982500ns | 1986231ns | 1989695ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3099066ns | 3096365ns | 3093721ns | 3095778ns | 3106671ns | +55.99% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3127399ns | 3093244ns | 3087817ns | 3092051ns | 3200211ns | +57.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1995007ns | 1988521ns | 1998737ns | +0.55% | 0.000 |
| abi_zig_entry_tight_zig_dispatch | 1982291ns | 1978341ns | 1985167ns | -0.09% | 0.000 |
| abi_zig_entry_tight_zig_null | 2678ns | 2632ns | 2714ns | -99.87% | 0.048 |
| abi_zig_entry_tight_zig_per_w_set | 1983104ns | 1980927ns | 1985890ns | -0.05% | 0.000 |
| abi_zig_entry_tight_zig_runtime_w | 1984095ns | 1979905ns | 1986920ns | base | 0.000 |
| abi_zig_entry_tight_zig_tail_dispatch | 3096189ns | 3090932ns | 3103696ns | +56.05% | 0.000 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3124453ns | 3085132ns | 3197068ns | +57.47% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 181076.0 | 1989185.3 | 1995006.7 | n/a |
| abi_zig_entry_tight_zig_dispatch | 181584.4 | 1981285.1 | 1982291.3 | 2 |
| abi_zig_entry_tight_zig_null | 155907.8 | 2766.7 | 2678.0 | n/a |
| abi_zig_entry_tight_zig_per_w_set | 180616.6 | 1982239.0 | 1983103.9 | 3 |
| abi_zig_entry_tight_zig_runtime_w | 177540.3 | 1984143.4 | 1984094.9 | n/a |
| abi_zig_entry_tight_zig_tail_dispatch | 190302.5 | 3095544.3 | 3096189.3 | n/a |
| abi_zig_entry_tight_zig_tail_runtime_w | 205023.4 | 3100064.9 | 3124453.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_zig_entry_tight_zig_null; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_null | 0.048 | 98.3% |
| abi_zig_entry_tight_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_tight_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 1997708ns | 1997708ns | +0.55% |
| abi_zig_entry_tight_zig_dispatch | 1985038ns | 1985038ns | -0.09% |
| abi_zig_entry_tight_zig_null | 5016ns | 5016ns | -99.75% |
| abi_zig_entry_tight_zig_per_w_set | 1985846ns | 1985846ns | -0.05% |
| abi_zig_entry_tight_zig_runtime_w | 1986764ns | 1986764ns | base |
| abi_zig_entry_tight_zig_tail_dispatch | 3099066ns | 3099066ns | +55.99% |
| abi_zig_entry_tight_zig_tail_runtime_w | 3127399ns | 3127399ns | +57.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_tight_zig_runtime_w | 1984066ns | base | --- | [1981299, 1986920] | --- | --- | --- | --- |
| abi_zig_entry_tight_zig_anchor | 1994942ns | +10365.6ns (+0.5%) | [+6160, +16209]ns | [1991341, 1998737] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_dispatch | 1983168ns | no significant difference | [-7858, +1870]ns | [1978539, 1985167] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_tight_zig_null | 2679ns | -1981375.1ns (-99.9%) | [-1984260, -1978616]ns | [2641, 2714] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_per_w_set | 1982409ns | no significant difference | [-4414, +3143]ns | [1981013, 1985890] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_tight_zig_tail_dispatch | 3093537ns | +1108511.6ns (+55.9%) | [+1106603, +1121168]ns | [3091335, 3103696] | YES | 0.0469 | 0.0313 | 0 |
| abi_zig_entry_tight_zig_tail_runtime_w | 3090339ns | +1105026.0ns (+55.7%) | [+1102031, +1214017]ns | [3085952, 3197068] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_tight_zig_runtime_w | abi_zig_entry_tight_zig_anchor | abi_zig_entry_tight_zig_dispatch | abi_zig_entry_tight_zig_null | abi_zig_entry_tight_zig_per_w_set | abi_zig_entry_tight_zig_tail_dispatch | abi_zig_entry_tight_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1987643ns | +0.3% | -0.5% | -99.9% | -0.2% | +55.7% | +55.6% |
| 2 | 1982692ns | +0.6% | +0.1% | -99.9% | -0.0% | +55.9% | +55.6% |
| 3 | 1979905ns | +1.0% | +0.1% | -99.9% | +0.4% | +56.3% | +66.6% |
| 4 | 1986197ns | +0.5% | -0.0% | -99.9% | -0.1% | +55.7% | +55.9% |
| 5 | 1985150ns | +0.6% | -0.3% | -99.9% | -0.2% | +56.8% | +55.5% |
| 6 | 1982982ns | +0.3% | +0.1% | -99.9% | -0.1% | +55.9% | +55.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_tight_zig_anchor | -0.179 | ok |
| abi_zig_entry_tight_zig_dispatch | -0.620 | HIGH- (thermal bounce) |
| abi_zig_entry_tight_zig_null | 0.109 | ok |
| abi_zig_entry_tight_zig_per_w_set | 0.070 | ok |
| abi_zig_entry_tight_zig_runtime_w | -0.176 | ok |
| abi_zig_entry_tight_zig_tail_dispatch | -0.446 | moderate- |
| abi_zig_entry_tight_zig_tail_runtime_w | -0.223 | moderate- |

**Consistency summary:**

- **abi_zig_entry_tight_zig_anchor**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_dispatch**: won 2/6, lost 0/6
- **abi_zig_entry_tight_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_tight_zig_per_w_set**: won 2/6, lost 1/6
- **abi_zig_entry_tight_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_tight_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_tight_zig_anchor | 6223533.9ns | 1995006.7ns | 312.0% | HIGH |
| abi_zig_entry_tight_zig_dispatch | 6197667.9ns | 1982291.3ns | 312.7% | HIGH |
| abi_zig_entry_tight_zig_null | 304802.8ns | 2678.0ns | 11381.8% | HIGH |
| abi_zig_entry_tight_zig_per_w_set | 6196379.3ns | 1983103.9ns | 312.5% | HIGH |
| abi_zig_entry_tight_zig_runtime_w | 6203307.8ns | 1984094.9ns | 312.7% | HIGH |
| abi_zig_entry_tight_zig_tail_dispatch | 9548190.0ns | 3096189.3ns | 308.4% | HIGH |
| abi_zig_entry_tight_zig_tail_runtime_w | 9593817.2ns | 3124453.1ns | 307.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_tight_zig_anchor (n=6, range 1988520.8-1998737.3 ns)
  1988520.8 |####################
  1989031.6 |
  1989542.4 |
  1990053.3 |
  1990564.1 |
  1991074.9 |
  1991585.8 |
  1992096.6 |
  1992607.4 |
  1993118.2 |
  1993629.1 |
  1994139.9 |########################################
  1994650.7 |
  1995161.5 |####################
  1995672.4 |
  1996183.2 |
  1996694.0 |####################
  1997204.8 |
  1997715.7 |
  1998226.5 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_dispatch (n=6, range 1978341.2-1985166.7 ns)
  1978341.2 |####################
  1978682.5 |####################
  1979023.8 |
  1979365.0 |
  1979706.3 |
  1980047.6 |
  1980388.8 |
  1980730.1 |
  1981071.4 |
  1981412.7 |
  1981753.9 |####################
  1982095.2 |
  1982436.5 |
  1982777.8 |
  1983119.1 |
  1983460.3 |
  1983801.6 |
  1984142.9 |########################################
  1984484.1 |
  1984825.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_null (n=6, range 2632.5-2714.1 ns)
   2632.5 |########################################
   2636.6 |
   2640.7 |
   2644.7 |########################################
   2648.8 |
   2652.9 |
   2657.0 |
   2661.1 |
   2665.2 |
   2669.2 |########################################
   2673.3 |
   2677.4 |
   2681.5 |
   2685.6 |########################################
   2689.7 |
   2693.7 |
   2697.8 |
   2701.9 |
   2706.0 |
   2710.1 |########################################
  (0 below, 1 above range)

abi_zig_entry_tight_zig_per_w_set (n=6, range 1980926.7-1985889.5 ns)
  1980926.7 |########################################
  1981174.8 |
  1981423.0 |
  1981671.1 |####################
  1981919.3 |
  1982167.4 |
  1982415.6 |
  1982663.7 |
  1982911.8 |####################
  1983160.0 |
  1983408.1 |
  1983656.3 |
  1983904.4 |
  1984152.6 |
  1984400.7 |
  1984648.8 |####################
  1984897.0 |
  1985145.1 |
  1985393.3 |
  1985641.4 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_runtime_w (n=6, range 1979905.4-1986920.0 ns)
  1979905.4 |########################################
  1980256.1 |
  1980606.9 |
  1980957.6 |
  1981308.3 |
  1981659.0 |
  1982009.8 |
  1982360.5 |########################################
  1982711.2 |########################################
  1983062.0 |
  1983412.7 |
  1983763.4 |
  1984114.2 |
  1984464.9 |
  1984815.6 |########################################
  1985166.4 |
  1985517.1 |
  1985867.8 |########################################
  1986218.5 |
  1986569.3 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_dispatch (n=6, range 3090932.1-3103696.0 ns)
  3090932.1 |####################
  3091570.3 |########################################
  3092208.5 |
  3092846.7 |
  3093484.9 |
  3094123.1 |
  3094761.3 |########################################
  3095399.5 |
  3096037.7 |
  3096675.9 |
  3097314.1 |
  3097952.3 |
  3098590.5 |
  3099228.7 |
  3099866.9 |
  3100505.1 |
  3101143.3 |
  3101781.5 |
  3102419.7 |
  3103057.9 |
  (0 below, 1 above range)

abi_zig_entry_tight_zig_tail_runtime_w (n=6, range 3085131.7-3197068.2 ns)
  3085131.7 |########################################
  3090728.5 |##########################
  3096325.3 |
  3101922.2 |
  3107519.0 |
  3113115.8 |
  3118712.6 |
  3124309.5 |
  3129906.3 |
  3135503.1 |
  3141099.9 |
  3146696.7 |
  3152293.6 |
  3157890.4 |
  3163487.2 |
  3169084.0 |
  3174680.9 |
  3180277.7 |
  3185874.5 |
  3191471.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_tight_zig_anchor**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_dispatch**: bridge=312.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_null**: bridge=11363.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_per_w_set**: bridge=312.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_runtime_w**: bridge=312.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_dispatch**: bridge=308.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_tight_zig_tail_runtime_w**: bridge=308.5% of algo (FFI overhead may distort results)

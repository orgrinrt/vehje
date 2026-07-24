# abi_entry_form (madd)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_madd_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_madd_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_madd_null_entry dominates: 70368% faster than the next best (abi_entry_form_madd_per_w_set)

abi_entry_form_madd_null_entry (4.25 us) leads abi_entry_form_madd_per_w_set (3.00 ms) by 70368%, a clear separation rather than a photo finish. CV 19.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_madd_null_entry beats baseline by 100% (significant)

abi_entry_form_madd_null_entry is -3.00 ms (100%) faster than baseline abi_entry_form_madd_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_madd_dispatch_table is an outlier: 730.4x slower than the field

abi_entry_form_madd_dispatch_table (3.11 ms) is 730.4x the fastest (4.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_madd_null_entry is fastest but the noisiest (CV 19.5%)

abi_entry_form_madd_null_entry wins on median (4.25 us) yet has the highest variance (CV 19.5%), while abi_entry_form_madd_runtime_w is the steadiest (CV 3.2%, 3.00 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_entry_form_madd_runtime_w shows warm-up / thermal drift (autocorr +0.50)

abi_entry_form_madd_runtime_w's per-pass series has lag-1 autocorrelation +0.50, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_madd_null_entry} vs {abi_entry_form_madd_per_w_set, abi_entry_form_madd_runtime_w, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_dispatch_table} (70368% apart)

The field splits into a fast tier {abi_entry_form_madd_null_entry} and a slow tier {abi_entry_form_madd_per_w_set, abi_entry_form_madd_runtime_w, abi_entry_form_madd_scalar_anchor, abi_entry_form_madd_dispatch_table} with a 70368% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 730.4x the fastest

Fastest abi_entry_form_madd_null_entry (4.25 us) to slowest abi_entry_form_madd_dispatch_table (3.11 ms): 730.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_madd_null_entry** at 4253.1 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 730.44x (fastest 4253.1 ns, slowest 3106630.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3086159ns | 3112074ns | 2861581ns | 3029901ns | 3282836ns | +1.63% |
| abi_entry_form_madd_null_entry | 7283ns | 6669ns | 6439ns | 6602ns | 8726ns | -99.76% |
| abi_entry_form_madd_per_w_set | 3041995ns | 3002503ns | 2901809ns | 2981603ns | 3202677ns | +0.18% |
| abi_entry_form_madd_runtime_w | 3036571ns | 3008992ns | 2918300ns | 2990393ns | 3164974ns | base |
| abi_entry_form_madd_scalar_anchor | 3185997ns | 3017374ns | 2913611ns | 2984606ns | 3624278ns | +4.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3080729ns | 2857040ns | 3276532ns | +1.62% | 0.000 |
| abi_entry_form_madd_null_entry | 4611ns | 4110ns | 5466ns | -99.85% | 0.001 |
| abi_entry_form_madd_per_w_set | 3036824ns | 2896747ns | 3197416ns | +0.18% | 0.000 |
| abi_entry_form_madd_runtime_w | 3031494ns | 2913551ns | 3159652ns | base | 0.000 |
| abi_entry_form_madd_scalar_anchor | 3180836ns | 2909048ns | 3618128ns | +4.93% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 107965.7 | 3044873.4 | 3080729.1 | n/a |
| abi_entry_form_madd_null_entry | 35524.3 | 4859.2 | 4610.8 | n/a |
| abi_entry_form_madd_per_w_set | 111137.5 | 3042357.0 | 3036823.8 | n/a |
| abi_entry_form_madd_runtime_w | 105919.2 | 3046189.0 | 3031493.8 | n/a |
| abi_entry_form_madd_scalar_anchor | 109923.1 | 3185749.1 | 3180836.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_madd_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_madd_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_madd_null_entry | 0.001 | 96.6% |
| abi_entry_form_madd_per_w_set | 0.000 | 0.1% |
| abi_entry_form_madd_runtime_w | 0.000 | 0.1% |
| abi_entry_form_madd_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 3086159ns | 3086159ns | +1.63% |
| abi_entry_form_madd_null_entry | 7283ns | 7283ns | -99.76% |
| abi_entry_form_madd_per_w_set | 3041995ns | 3041995ns | +0.18% |
| abi_entry_form_madd_runtime_w | 3036571ns | 3036571ns | base |
| abi_entry_form_madd_scalar_anchor | 3185997ns | 3185997ns | +4.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_madd_runtime_w | 3004063ns | base | --- | [2930766, 3159652] | --- | --- | --- | --- |
| abi_entry_form_madd_dispatch_table | 3106631ns | no significant difference | [-111346, +270777]ns | [2859024, 3276532] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_madd_null_entry | 4253ns | -2999809.8ns (-99.9%) | [-3154377, -2926462]ns | [4113, 5466] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_madd_per_w_set | 2997093ns | no significant difference | [-87258, +138772]ns | [2915963, 3197416] | no | 1.0000 | 0.6875 | 0 |
| abi_entry_form_madd_scalar_anchor | 3012476ns | no significant difference | [-77180, +529473]ns | [2911905, 3618128] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_madd_runtime_w | abi_entry_form_madd_dispatch_table | abi_entry_form_madd_null_entry | abi_entry_form_madd_per_w_set | abi_entry_form_madd_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 3144769ns | +1.1% | -99.8% | -3.4% | -1.8% |
| 2 | 3174535ns | -4.3% | -99.9% | -0.6% | +27.4% |
| 3 | 3005350ns | +5.6% | -99.9% | +7.8% | -3.2% |
| 4 | 2947982ns | -3.0% | -99.9% | -1.7% | -1.1% |
| 5 | 2913551ns | -1.9% | -99.8% | +1.5% | +0.8% |
| 6 | 3002775ns | +12.4% | -99.9% | -2.3% | +6.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_madd_dispatch_table | -0.221 | moderate- |
| abi_entry_form_madd_null_entry | -0.125 | ok |
| abi_entry_form_madd_per_w_set | 0.164 | ok |
| abi_entry_form_madd_runtime_w | 0.501 | HIGH+ (drift/warm-up) |
| abi_entry_form_madd_scalar_anchor | -0.190 | ok |

**Consistency summary:**

- **abi_entry_form_madd_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_madd_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_madd_per_w_set**: won 4/6, lost 2/6
- **abi_entry_form_madd_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_madd_dispatch_table | 9231091.8ns | 3080729.1ns | 299.6% | HIGH |
| abi_entry_form_madd_null_entry | 137344.9ns | 4610.8ns | 2978.8% | HIGH |
| abi_entry_form_madd_per_w_set | 9251613.0ns | 3036823.8ns | 304.6% | HIGH |
| abi_entry_form_madd_runtime_w | 9236079.0ns | 3031493.8ns | 304.7% | HIGH |
| abi_entry_form_madd_scalar_anchor | 9666962.0ns | 3180836.4ns | 303.9% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_madd_dispatch_table (n=6, range 2857039.6-3276532.2 ns)
  2857039.6 |########################################
  2878014.2 |
  2898988.9 |
  2919963.5 |
  2940938.1 |
  2961912.8 |
  2982887.4 |
  3003862.0 |
  3024836.7 |####################
  3045811.3 |
  3066785.9 |
  3087760.6 |
  3108735.2 |
  3129709.8 |
  3150684.5 |
  3171659.1 |########################################
  3192633.7 |
  3213608.4 |
  3234583.0 |
  3255557.6 |
  (0 below, 1 above range)

abi_entry_form_madd_null_entry (n=6, range 4109.6-5466.4 ns)
   4109.6 |########################################
   4177.4 |####################
   4245.3 |
   4313.1 |####################
   4381.0 |
   4448.8 |####################
   4516.7 |
   4584.5 |
   4652.3 |
   4720.2 |
   4788.0 |
   4855.9 |
   4923.7 |
   4991.6 |
   5059.4 |
   5127.2 |
   5195.1 |
   5262.9 |
   5330.8 |
   5398.6 |
  (0 below, 1 above range)

abi_entry_form_madd_per_w_set (n=6, range 2896746.7-3197416.0 ns)
  2896746.7 |########################################
  2911780.2 |
  2926813.6 |########################################
  2941847.1 |########################################
  2956880.6 |
  2971914.0 |
  2986947.5 |
  3001981.0 |
  3017014.4 |
  3032047.9 |########################################
  3047081.4 |
  3062114.8 |
  3077148.3 |
  3092181.7 |
  3107215.2 |
  3122248.7 |
  3137282.1 |
  3152315.6 |########################################
  3167349.1 |
  3182382.5 |
  (0 below, 1 above range)

abi_entry_form_madd_runtime_w (n=6, range 2913551.2-3159652.0 ns)
  2913551.2 |####################
  2925856.2 |
  2938161.3 |####################
  2950466.3 |
  2962771.4 |
  2975076.4 |
  2987381.5 |
  2999686.5 |########################################
  3011991.5 |
  3024296.6 |
  3036601.6 |
  3048906.7 |
  3061211.7 |
  3073516.8 |
  3085821.8 |
  3098126.8 |
  3110431.9 |
  3122736.9 |
  3135042.0 |####################
  3147347.0 |
  (0 below, 1 above range)

abi_entry_form_madd_scalar_anchor (n=6, range 2909047.5-3618128.2 ns)
  2909047.5 |########################################
  2944501.5 |
  2979955.6 |
  3015409.6 |
  3050863.6 |
  3086317.7 |#############
  3121771.7 |
  3157225.7 |
  3192679.8 |#############
  3228133.8 |
  3263587.8 |
  3299041.9 |
  3334495.9 |
  3369949.9 |
  3405404.0 |
  3440858.0 |
  3476312.0 |
  3511766.1 |
  3547220.1 |
  3582674.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_madd_dispatch_table**: bridge=300.4% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_null_entry**: bridge=2949.3% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_per_w_set**: bridge=306.2% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_runtime_w**: autocorrelation=0.50 (measurement drift or warm-up artifact)
- **abi_entry_form_madd_runtime_w**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_madd_scalar_anchor**: bridge=304.6% of algo (FFI overhead may distort results)

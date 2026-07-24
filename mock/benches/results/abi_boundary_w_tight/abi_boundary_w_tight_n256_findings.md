# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 30128% faster than the next best (abi_boundary_w_tight_soa_runtime_w)

abi_boundary_w_tight_null_entry (3.21 us) leads abi_boundary_w_tight_soa_runtime_w (971.08 us) by 30128%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.03 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 641.4x slower than the field

abi_boundary_w_tight_scalar_anchor (2.06 ms) is 641.4x the fastest (3.21 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_scalar_dispatch shows alternating (throttle bounce) (autocorr -0.52)

abi_boundary_w_tight_scalar_dispatch's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} (30128% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} with a 30128% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 641.4x the fastest

Fastest abi_boundary_w_tight_null_entry (3.21 us) to slowest abi_boundary_w_tight_scalar_anchor (2.06 ms): 641.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_boundary_w_tight_scalar_per_w's edge over baseline is significant but tiny (-23 ns, 0.00%)

abi_boundary_w_tight_scalar_per_w differs from baseline abi_boundary_w_tight_scalar_runtime_w by -23 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 3212.5 ns median (-99.8% vs baseline)
- 5 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 641.39x (fastest 3212.5 ns, slowest 2060461.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 5485ns | 5478ns | 5332ns | 5441ns | 5628ns | -99.73% |
| abi_boundary_w_tight_scalar_anchor | 2063618ns | 2063754ns | 2040814ns | 2057267ns | 2084545ns | +1.18% |
| abi_boundary_w_tight_scalar_dispatch | 2041163ns | 2041327ns | 2032425ns | 2039997ns | 2047280ns | +0.08% |
| abi_boundary_w_tight_scalar_per_w | 2040495ns | 2040622ns | 2032604ns | 2038713ns | 2047115ns | +0.05% |
| abi_boundary_w_tight_scalar_runtime_w | 2039532ns | 2037354ns | 2025680ns | 2035060ns | 2053166ns | base |
| abi_boundary_w_tight_soa_dispatch | 974609ns | 976325ns | 964336ns | 973940ns | 980749ns | -52.21% |
| abi_boundary_w_tight_soa_per_w | 979124ns | 978626ns | 968989ns | 976293ns | 988439ns | -51.99% |
| abi_boundary_w_tight_soa_runtime_w | 972884ns | 974018ns | 962510ns | 973094ns | 977754ns | -52.30% |
| abi_boundary_w_tight_zig_runtime_w | 2018798ns | 2018709ns | 2005373ns | 2016343ns | 2029192ns | -1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 3215ns | 3114ns | 3288ns | -99.84% | 0.080 |
| abi_boundary_w_tight_scalar_anchor | 2060292ns | 2038002ns | 2080737ns | +1.17% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2037920ns | 2029101ns | 2043912ns | +0.07% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2037286ns | 2029527ns | 2043630ns | +0.04% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2036419ns | 2022842ns | 2049701ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 971641ns | 961508ns | 977920ns | -52.29% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 976215ns | 966382ns | 985205ns | -52.06% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 970028ns | 959789ns | 974962ns | -52.37% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2015474ns | 2002402ns | 2025821ns | -1.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 28040.5 | 3205.9 | 3214.6 | n/a |
| abi_boundary_w_tight_scalar_anchor | 63153.2 | 2054696.2 | 2060292.4 | n/a |
| abi_boundary_w_tight_scalar_dispatch | 58353.8 | 2035874.4 | 2037919.9 | n/a |
| abi_boundary_w_tight_scalar_per_w | 63220.5 | 2034390.3 | 2037285.6 | n/a |
| abi_boundary_w_tight_scalar_runtime_w | 55385.0 | 2034750.9 | 2036419.4 | n/a |
| abi_boundary_w_tight_soa_dispatch | 48305.8 | 972631.5 | 971640.5 | n/a |
| abi_boundary_w_tight_soa_per_w | 50912.8 | 975929.9 | 976214.9 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 45587.8 | 969335.0 | 970027.7 | n/a |
| abi_boundary_w_tight_zig_runtime_w | 227103.1 | 2016843.0 | 2015473.8 | 8 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.080 | 96.9% |
| abi_boundary_w_tight_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_tight_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_tight_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_tight_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_tight_null_entry | 5485ns | 5485ns | -99.73% |
| abi_boundary_w_tight_scalar_anchor | 2063618ns | 2063618ns | +1.18% |
| abi_boundary_w_tight_scalar_dispatch | 2041163ns | 2041163ns | +0.08% |
| abi_boundary_w_tight_scalar_per_w | 2040495ns | 2040495ns | +0.05% |
| abi_boundary_w_tight_scalar_runtime_w | 2039532ns | 2039532ns | base |
| abi_boundary_w_tight_soa_dispatch | 974609ns | 974609ns | -52.21% |
| abi_boundary_w_tight_soa_per_w | 979124ns | 979124ns | -51.99% |
| abi_boundary_w_tight_soa_runtime_w | 972884ns | 972884ns | -52.30% |
| abi_boundary_w_tight_zig_runtime_w | 2018798ns | 2018798ns | -1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2034443ns | base | --- | [2025114, 2049701] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 3212ns | -2031207.7ns (-99.8%) | [-2046541, -2021866]ns | [3144, 3288] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2060462ns | +26018.3ns (+1.3%) | [+240, +45360]ns | [2039679, 2080737] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2038334ns | no significant difference | [-6711, +11491]ns | [2031514, 2043912] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2037550ns | no significant difference | [-9694, +12315]ns | [2030676, 2043630] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_soa_dispatch | 973086ns | -1065199.2ns (-52.4%) | [-1076615, -1052522]ns | [963916, 977920] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_per_w | 975716ns | -1061801.6ns (-52.2%) | [-1077043, -1041769]ns | [967723, 985205] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 971084ns | -1064877.5ns (-52.3%) | [-1080144, -1054154]ns | [964038, 974962] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2015292ns | -21045.6ns (-1.0%) | [-32533, -9258]ns | [2005309, 2025821] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2047911ns | -99.8% | +2.3% | -0.1% | -0.2% | -52.5% | -52.1% | -52.7% | -0.7% |
| 2 | 2038042ns | -99.8% | +1.1% | -0.2% | +0.1% | -52.0% | -52.5% | -52.3% | -1.5% |
| 3 | 2022842ns | -99.8% | +2.1% | +0.6% | +1.0% | -51.6% | -51.5% | -52.6% | -0.3% |
| 4 | 2030845ns | -99.8% | +1.4% | +0.5% | -0.1% | -52.7% | -51.3% | -51.9% | -1.4% |
| 5 | 2027386ns | -99.8% | +0.7% | +0.1% | +0.2% | -52.3% | -52.1% | -52.0% | -0.6% |
| 6 | 2051490ns | -99.8% | -0.7% | -0.5% | -0.8% | -52.6% | -52.9% | -52.7% | -1.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.060 | ok |
| abi_boundary_w_tight_scalar_anchor | 0.219 | moderate+ |
| abi_boundary_w_tight_scalar_dispatch | -0.516 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_scalar_per_w | 0.179 | ok |
| abi_boundary_w_tight_scalar_runtime_w | -0.020 | ok |
| abi_boundary_w_tight_soa_dispatch | 0.124 | ok |
| abi_boundary_w_tight_soa_per_w | -0.066 | ok |
| abi_boundary_w_tight_soa_runtime_w | -0.411 | moderate- |
| abi_boundary_w_tight_zig_runtime_w | -0.277 | moderate- |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 1/6, lost 5/6
- **abi_boundary_w_tight_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_tight_scalar_per_w**: won 2/6, lost 2/6
- **abi_boundary_w_tight_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_tight_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 122311.9ns | 3214.6ns | 3804.9% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6232432.8ns | 2060292.4ns | 302.5% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6169966.3ns | 2037919.9ns | 302.8% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6170208.2ns | 2037285.6ns | 302.9% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6158583.4ns | 2036419.4ns | 302.4% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 2965783.1ns | 971640.5ns | 305.2% | HIGH |
| abi_boundary_w_tight_soa_per_w | 2979349.3ns | 976214.9ns | 305.2% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 2956041.9ns | 970027.7ns | 304.7% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6361992.6ns | 2015473.8ns | 315.7% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 3113.8-3287.5 ns)
   3113.8 |########################################
   3122.5 |
   3131.2 |
   3139.9 |
   3148.5 |
   3157.2 |
   3165.9 |########################################
   3174.6 |
   3183.3 |
   3192.0 |
   3200.7 |########################################
   3209.3 |
   3218.0 |########################################
   3226.7 |
   3235.4 |
   3244.1 |
   3252.8 |
   3261.4 |
   3270.1 |########################################
   3278.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2038002.1-2080737.1 ns)
  2038002.1 |####################
  2040138.9 |####################
  2042275.6 |
  2044412.4 |
  2046549.1 |
  2048685.9 |
  2050822.6 |
  2052959.4 |
  2055096.1 |
  2057232.9 |
  2059369.6 |########################################
  2061506.4 |
  2063643.1 |
  2065779.9 |####################
  2067916.6 |
  2070053.4 |
  2072190.1 |
  2074326.9 |
  2076463.6 |
  2078600.4 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2029101.2-2043911.7 ns)
  2029101.2 |########################################
  2029841.7 |
  2030582.2 |
  2031322.8 |
  2032063.3 |
  2032803.8 |
  2033544.3 |########################################
  2034284.9 |
  2035025.4 |########################################
  2035765.9 |
  2036506.4 |
  2037247.0 |
  2037987.5 |
  2038728.0 |
  2039468.6 |
  2040209.1 |
  2040949.6 |########################################
  2041690.1 |########################################
  2042430.6 |
  2043171.2 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2029526.7-2043630.4 ns)
  2029526.7 |########################################
  2030231.9 |
  2030937.1 |
  2031642.3 |########################################
  2032347.4 |
  2033052.6 |
  2033757.8 |
  2034463.0 |
  2035168.2 |########################################
  2035873.4 |
  2036578.5 |
  2037283.7 |
  2037988.9 |
  2038694.1 |########################################
  2039399.3 |
  2040104.5 |
  2040809.7 |
  2041514.8 |
  2042220.0 |
  2042925.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2022842.1-2049700.8 ns)
  2022842.1 |########################################
  2024185.0 |
  2025528.0 |
  2026870.9 |########################################
  2028213.8 |
  2029556.8 |########################################
  2030899.7 |
  2032242.6 |
  2033585.6 |
  2034928.5 |
  2036271.4 |
  2037614.4 |########################################
  2038957.3 |
  2040300.3 |
  2041643.2 |
  2042986.1 |
  2044329.1 |
  2045672.0 |
  2047014.9 |########################################
  2048357.9 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 961507.9-977919.6 ns)
  961507.9 |########################################
  962328.5 |
  963149.1 |
  963969.6 |
  964790.2 |
  965610.8 |########################################
  966431.4 |
  967252.0 |
  968072.6 |
  968893.1 |
  969713.7 |
  970534.3 |
  971354.9 |
  972175.5 |########################################
  972996.1 |########################################
  973816.6 |
  974637.2 |
  975457.8 |
  976278.4 |
  977099.0 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 966381.7-985205.4 ns)
  966381.7 |####################
  967322.9 |
  968264.1 |####################
  969205.3 |
  970146.4 |####################
  971087.6 |
  972028.8 |
  972970.0 |
  973911.2 |
  974852.4 |
  975793.6 |
  976734.7 |
  977675.9 |
  978617.1 |
  979558.3 |
  980499.5 |########################################
  981440.7 |
  982381.8 |
  983323.0 |
  984264.2 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 959788.8-974961.7 ns)
  959788.8 |########################################
  960547.4 |
  961306.1 |
  962064.7 |
  962823.4 |
  963582.0 |
  964340.7 |
  965099.3 |
  965858.0 |
  966616.6 |
  967375.2 |
  968133.9 |########################################
  968892.5 |
  969651.2 |
  970409.8 |########################################
  971168.5 |########################################
  971927.1 |########################################
  972685.8 |
  973444.4 |
  974203.1 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2002402.1-2025820.6 ns)
  2002402.1 |########################################
  2003573.0 |
  2004744.0 |
  2005914.9 |
  2007085.8 |########################################
  2008256.7 |
  2009427.7 |
  2010598.6 |
  2011769.5 |
  2012940.4 |
  2014111.4 |########################################
  2015282.3 |########################################
  2016453.2 |########################################
  2017624.1 |
  2018795.1 |
  2019966.0 |
  2021136.9 |
  2022307.8 |
  2023478.8 |
  2024649.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=3808.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=305.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=304.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=304.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=315.4% of algo (FFI overhead may distort results)

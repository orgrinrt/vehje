# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_scatter_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_scatter_runtime_w has the worst median (2.19 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_scatter_null_entry at 3.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_scatter_null_entry dominates: 68122% faster than the next best (abi_entry_form_scatter_dispatch_table)

abi_entry_form_scatter_null_entry (3.18 us) leads abi_entry_form_scatter_dispatch_table (2.17 ms) by 68122%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.19 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_runtime_w is an outlier: 687.6x slower than the field

abi_entry_form_scatter_runtime_w (2.19 ms) is 687.6x the fastest (3.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_scatter_scalar_anchor shows alternating (throttle bounce) (autocorr -0.71)

abi_entry_form_scatter_scalar_anchor's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w} (68122% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_dispatch_table, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w} with a 68122% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 687.6x the fastest

Fastest abi_entry_form_scatter_null_entry (3.18 us) to slowest abi_entry_form_scatter_runtime_w (2.19 ms): 687.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 3183.8 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 687.61x (fastest 3183.8 ns, slowest 2189170.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2179944ns | 2175657ns | 2170119ns | 2173914ns | 2193901ns | -0.63% |
| abi_entry_form_scatter_null_entry | 5523ns | 5535ns | 5395ns | 5494ns | 5631ns | -99.75% |
| abi_entry_form_scatter_per_w_set | 2177065ns | 2178237ns | 2162213ns | 2174868ns | 2187785ns | -0.76% |
| abi_entry_form_scatter_runtime_w | 2193680ns | 2193055ns | 2163682ns | 2188926ns | 2215809ns | base |
| abi_entry_form_scatter_scalar_anchor | 2193201ns | 2181698ns | 2168760ns | 2177418ns | 2229095ns | -0.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2176363ns | 2166754ns | 2190082ns | -0.62% | 0.000 |
| abi_entry_form_scatter_null_entry | 3200ns | 3150ns | 3265ns | -99.85% | 0.080 |
| abi_entry_form_scatter_per_w_set | 2173754ns | 2159278ns | 2184311ns | -0.74% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2190048ns | 2160679ns | 2212075ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2189610ns | 2165420ns | 2225150ns | -0.02% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 68162.9 | 2179105.6 | 2176362.5 | n/a |
| abi_entry_form_scatter_null_entry | 28390.8 | 3233.6 | 3199.5 | n/a |
| abi_entry_form_scatter_per_w_set | 63079.7 | 2172770.6 | 2173754.3 | n/a |
| abi_entry_form_scatter_runtime_w | 72223.2 | 2211250.7 | 2190047.6 | n/a |
| abi_entry_form_scatter_scalar_anchor | 70408.8 | 2186786.1 | 2189610.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.081 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_scatter_null_entry | 0.080 | 98.9% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.1% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.1% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2179944ns | 2179944ns | -0.63% |
| abi_entry_form_scatter_null_entry | 5523ns | 5523ns | -99.75% |
| abi_entry_form_scatter_per_w_set | 2177065ns | 2177065ns | -0.76% |
| abi_entry_form_scatter_runtime_w | 2193680ns | 2193680ns | base |
| abi_entry_form_scatter_scalar_anchor | 2193201ns | 2193201ns | -0.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2189171ns | base | --- | [2168897, 2212075] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2172006ns | no significant difference | [-40069, +6774]ns | [2166999, 2190082] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_scatter_null_entry | 3184ns | -2185956.7ns (-99.9%) | [-2208874, -2165714]ns | [3150, 3265] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2174866ns | -10516.9ns (-0.5%) | [-37351, -1012]ns | [2162086, 2184311] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2178209ns | no significant difference | [-17989, +16744]ns | [2165472, 2225150] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2184690ns | +0.3% | -99.8% | -0.0% | -0.9% |
| 2 | 2220293ns | -2.2% | -99.9% | -1.6% | +0.7% |
| 3 | 2177115ns | -0.5% | -99.9% | -0.4% | -0.5% |
| 4 | 2203858ns | -1.4% | -99.9% | -1.8% | +0.5% |
| 5 | 2160679ns | +0.3% | -99.9% | -0.1% | +0.9% |
| 6 | 2193651ns | -0.2% | -99.9% | -0.5% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.109 | ok |
| abi_entry_form_scatter_null_entry | 0.138 | ok |
| abi_entry_form_scatter_per_w_set | 0.184 | ok |
| abi_entry_form_scatter_runtime_w | -0.571 | HIGH- (thermal bounce) |
| abi_entry_form_scatter_scalar_anchor | -0.710 | HIGH- (thermal bounce) |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 4/6, lost 2/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 4/6, lost 0/6
- **abi_entry_form_scatter_scalar_anchor**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6608562.6ns | 2176362.5ns | 303.7% | HIGH |
| abi_entry_form_scatter_null_entry | 121697.8ns | 3199.5ns | 3803.6% | HIGH |
| abi_entry_form_scatter_per_w_set | 6581992.4ns | 2173754.3ns | 302.8% | HIGH |
| abi_entry_form_scatter_runtime_w | 6703054.5ns | 2190047.6ns | 306.1% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6644445.6ns | 2189610.4ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2166754.2-2190081.7 ns)
  2166754.2 |########################################
  2167920.6 |
  2169086.9 |
  2170253.3 |
  2171419.7 |########################################
  2172586.1 |
  2173752.4 |
  2174918.8 |
  2176085.2 |
  2177251.6 |
  2178417.9 |
  2179584.3 |
  2180750.7 |
  2181917.0 |
  2183083.4 |
  2184249.8 |
  2185416.2 |
  2186582.5 |
  2187748.9 |####################
  2188915.3 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 3149.6-3265.2 ns)
   3149.6 |########################################
   3155.4 |
   3161.2 |
   3166.9 |
   3172.7 |
   3178.5 |
   3184.3 |
   3190.1 |
   3195.8 |
   3201.6 |
   3207.4 |
   3213.2 |#############
   3219.0 |
   3224.7 |
   3230.5 |
   3236.3 |
   3242.1 |
   3247.9 |#############
   3253.6 |
   3259.4 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2159277.5-2184310.7 ns)
  2159277.5 |########################################
  2160529.2 |
  2161780.8 |
  2163032.5 |
  2164284.1 |########################################
  2165535.8 |
  2166787.4 |########################################
  2168039.1 |
  2169290.8 |
  2170542.4 |
  2171794.1 |
  2173045.7 |
  2174297.4 |
  2175549.0 |
  2176800.7 |
  2178052.4 |
  2179304.0 |
  2180555.7 |########################################
  2181807.3 |
  2183059.0 |########################################
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2160678.8-2212075.4 ns)
  2160678.8 |########################################
  2163248.6 |
  2165818.5 |
  2168388.3 |
  2170958.1 |
  2173527.9 |
  2176097.8 |########################################
  2178667.6 |
  2181237.4 |
  2183807.3 |########################################
  2186377.1 |
  2188946.9 |
  2191516.8 |########################################
  2194086.6 |
  2196656.4 |
  2199226.2 |
  2201796.1 |########################################
  2204365.9 |
  2206935.7 |
  2209505.6 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2165420.4-2225149.6 ns)
  2165420.4 |########################################
  2168406.9 |
  2171393.3 |
  2174379.8 |####################
  2177366.2 |####################
  2180352.7 |
  2183339.2 |
  2186325.6 |
  2189312.1 |
  2192298.5 |
  2195285.0 |
  2198271.5 |
  2201257.9 |
  2204244.4 |
  2207230.8 |
  2210217.3 |
  2213203.8 |####################
  2216190.2 |
  2219176.7 |
  2222163.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=3797.8% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=302.6% of algo (FFI overhead may distort results)

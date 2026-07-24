# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_real_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_real_runtime_w has the worst median (2.20 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_real_null_entry at 3.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_real_null_entry dominates: 68461% faster than the next best (abi_entry_form_real_per_w_set)

abi_entry_form_real_null_entry (3.19 us) leads abi_entry_form_real_per_w_set (2.18 ms) by 68461%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.19 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_runtime_w is an outlier: 688.9x slower than the field

abi_entry_form_real_runtime_w (2.20 ms) is 688.9x the fastest (3.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_runtime_w} (68461% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_runtime_w} with a 68461% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 688.9x the fastest

Fastest abi_entry_form_real_null_entry (3.19 us) to slowest abi_entry_form_real_runtime_w (2.20 ms): 688.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 3186.8 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 688.90x (fastest 3186.8 ns, slowest 2195430.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2236701ns | 2192817ns | 2175587ns | 2188720ns | 2339228ns | +0.34% |
| abi_entry_form_real_null_entry | 5529ns | 5516ns | 5389ns | 5493ns | 5652ns | -99.75% |
| abi_entry_form_real_per_w_set | 2191666ns | 2188350ns | 2174728ns | 2185576ns | 2209271ns | -1.68% |
| abi_entry_form_real_runtime_w | 2229061ns | 2198943ns | 2176869ns | 2194040ns | 2307688ns | base |
| abi_entry_form_real_scalar_anchor | 2189254ns | 2192875ns | 2174683ns | 2188906ns | 2197063ns | -1.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2232969ns | 2172170ns | 2335217ns | +0.37% | 0.000 |
| abi_entry_form_real_null_entry | 3195ns | 3129ns | 3247ns | -99.86% | 0.080 |
| abi_entry_form_real_per_w_set | 2188177ns | 2171378ns | 2205585ns | -1.64% | 0.000 |
| abi_entry_form_real_runtime_w | 2224666ns | 2173238ns | 2301590ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2185853ns | 2171381ns | 2193512ns | -1.74% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 90014.5 | 2266528.5 | 2232968.7 | n/a |
| abi_entry_form_real_null_entry | 29321.3 | 3233.1 | 3195.2 | n/a |
| abi_entry_form_real_per_w_set | 72786.2 | 2187244.5 | 2188176.7 | n/a |
| abi_entry_form_real_runtime_w | 75192.9 | 2238755.7 | 2224665.8 | n/a |
| abi_entry_form_real_scalar_anchor | 69103.5 | 2183630.8 | 2185853.1 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.082 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.080 | 98.2% |
| abi_entry_form_real_per_w_set | 0.000 | 0.1% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2236701ns | 2236701ns | +0.34% |
| abi_entry_form_real_null_entry | 5529ns | 5529ns | -99.75% |
| abi_entry_form_real_per_w_set | 2191666ns | 2191666ns | -1.68% |
| abi_entry_form_real_runtime_w | 2229061ns | 2229061ns | base |
| abi_entry_form_real_scalar_anchor | 2189254ns | 2189254ns | -1.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2195431ns | base | --- | [2176976, 2301590] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2189212ns | no significant difference | [-34863, +58865]ns | [2174476, 2335217] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_real_null_entry | 3187ns | -2192255.0ns (-99.9%) | [-2298343, -2173814]ns | [3152, 3247] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2184944ns | no significant difference | [-121710, +15479]ns | [2174002, 2205585] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_real_scalar_anchor | 2189470ns | no significant difference | [-112120, +4025]ns | [2174577, 2193512] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2253542ns | -1.7% | -99.9% | -3.4% | -2.8% |
| 2 | 2203540ns | -1.4% | -99.9% | +0.8% | -0.3% |
| 3 | 2349639ns | +4.5% | -99.9% | -7.1% | -6.8% |
| 4 | 2180715ns | -0.2% | -99.9% | +0.4% | +0.5% |
| 5 | 2173238ns | +0.6% | -99.9% | +0.6% | -0.1% |
| 6 | 2187321ns | +0.3% | -99.9% | -0.7% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.337 | moderate- |
| abi_entry_form_real_null_entry | -0.261 | moderate- |
| abi_entry_form_real_per_w_set | -0.350 | moderate- |
| abi_entry_form_real_runtime_w | -0.199 | ok |
| abi_entry_form_real_scalar_anchor | 0.325 | moderate+ |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 3/6, lost 3/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 3/6, lost 3/6
- **abi_entry_form_real_scalar_anchor**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6771240.5ns | 2232968.7ns | 303.2% | HIGH |
| abi_entry_form_real_null_entry | 122530.7ns | 3195.2ns | 3834.9% | HIGH |
| abi_entry_form_real_per_w_set | 6644894.4ns | 2188176.7ns | 303.7% | HIGH |
| abi_entry_form_real_runtime_w | 6812619.5ns | 2224665.8ns | 306.2% | HIGH |
| abi_entry_form_real_scalar_anchor | 6630183.1ns | 2185853.1ns | 303.3% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2172170.4-2335217.3 ns)
  2172170.4 |########################################
  2180322.7 |####################
  2188475.1 |####################
  2196627.4 |
  2204779.8 |
  2212932.1 |####################
  2221084.5 |
  2229236.8 |
  2237389.2 |
  2245541.5 |
  2253693.8 |
  2261846.2 |
  2269998.5 |
  2278150.9 |
  2286303.2 |
  2294455.6 |
  2302607.9 |
  2310760.3 |
  2318912.6 |
  2327065.0 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 3129.2-3247.1 ns)
   3129.2 |########################################
   3135.1 |
   3141.0 |
   3146.9 |
   3152.8 |
   3158.7 |
   3164.6 |
   3170.5 |########################################
   3176.4 |########################################
   3182.3 |
   3188.1 |
   3194.0 |########################################
   3199.9 |
   3205.8 |########################################
   3211.7 |
   3217.6 |
   3223.5 |
   3229.4 |
   3235.3 |
   3241.2 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2171378.3-2205584.6 ns)
  2171378.3 |########################################
  2173088.6 |
  2174798.9 |
  2176509.2 |########################################
  2178219.6 |
  2179929.9 |
  2181640.2 |########################################
  2183350.5 |
  2185060.8 |########################################
  2186771.1 |
  2188481.5 |########################################
  2190191.8 |
  2191902.1 |
  2193612.4 |
  2195322.7 |
  2197033.0 |
  2198743.3 |
  2200453.7 |
  2202164.0 |
  2203874.3 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2173237.9-2301590.5 ns)
  2173237.9 |########################################
  2179655.5 |########################################
  2186073.2 |########################################
  2192490.8 |
  2198908.4 |########################################
  2205326.0 |
  2211743.7 |
  2218161.3 |
  2224578.9 |
  2230996.5 |
  2237414.2 |
  2243831.8 |
  2250249.4 |########################################
  2256667.1 |
  2263084.7 |
  2269502.3 |
  2275919.9 |
  2282337.6 |
  2288755.2 |
  2295172.8 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2171380.8-2193512.1 ns)
  2171380.8 |####################
  2172487.4 |
  2173593.9 |
  2174700.5 |
  2175807.1 |
  2176913.6 |####################
  2178020.2 |
  2179126.8 |
  2180233.3 |
  2181339.9 |
  2182446.5 |
  2183553.0 |
  2184659.6 |
  2185766.1 |
  2186872.7 |
  2187979.3 |
  2189085.8 |########################################
  2190192.4 |####################
  2191299.0 |
  2192405.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=3846.1% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=303.3% of algo (FFI overhead may distort results)

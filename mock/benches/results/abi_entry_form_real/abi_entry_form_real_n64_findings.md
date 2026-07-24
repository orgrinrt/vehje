# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_real_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_real_runtime_w has the worst median (2.20 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_real_null_entry at 2.50 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_real_null_entry dominates: 87239% faster than the next best (abi_entry_form_real_dispatch_table)

abi_entry_form_real_null_entry (2.50 us) leads abi_entry_form_real_dispatch_table (2.18 ms) by 87239%, a clear separation rather than a photo finish. CV 7.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.20 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_runtime_w is an outlier: 880.0x slower than the field

abi_entry_form_real_runtime_w (2.20 ms) is 880.0x the fastest (2.50 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_real_null_entry is fastest but the noisiest (CV 7.4%)

abi_entry_form_real_null_entry wins on median (2.50 us) yet has the highest variance (CV 7.4%), while abi_entry_form_real_dispatch_table is the steadiest (CV 0.7%, 2.18 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w} (87239% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_dispatch_table, abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_runtime_w} with a 87239% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 880.0x the fastest

Fastest abi_entry_form_real_null_entry (2.50 us) to slowest abi_entry_form_real_runtime_w (2.20 ms): 880.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 2500.4 ns median (-99.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 880.03x (fastest 2500.4 ns, slowest 2200469.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2193073ns | 2187481ns | 2180145ns | 2186130ns | 2209952ns | -1.46% |
| abi_entry_form_real_null_entry | 4876ns | 4739ns | 4605ns | 4731ns | 5229ns | -99.78% |
| abi_entry_form_real_per_w_set | 2209667ns | 2193638ns | 2178124ns | 2190093ns | 2254799ns | -0.72% |
| abi_entry_form_real_runtime_w | 2225619ns | 2204280ns | 2187731ns | 2200672ns | 2281984ns | base |
| abi_entry_form_real_scalar_anchor | 2199965ns | 2192400ns | 2181118ns | 2191576ns | 2221971ns | -1.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2189319ns | 2176593ns | 2205868ns | -1.45% | 0.000 |
| abi_entry_form_real_null_entry | 2546ns | 2385ns | 2742ns | -99.89% | 0.025 |
| abi_entry_form_real_per_w_set | 2206017ns | 2174672ns | 2250949ns | -0.70% | 0.000 |
| abi_entry_form_real_runtime_w | 2221606ns | 2184302ns | 2277288ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2196261ns | 2177732ns | 2218034ns | -1.14% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 78074.1 | 2188526.1 | 2189318.7 | n/a |
| abi_entry_form_real_null_entry | 30927.7 | 2766.0 | 2545.6 | n/a |
| abi_entry_form_real_per_w_set | 78506.6 | 2209718.1 | 2206016.5 | n/a |
| abi_entry_form_real_runtime_w | 86248.1 | 2226544.2 | 2221605.9 | n/a |
| abi_entry_form_real_scalar_anchor | 81027.7 | 2204477.7 | 2196260.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_real_null_entry | 0.026 | 95.4% |
| abi_entry_form_real_per_w_set | 0.000 | 0.1% |
| abi_entry_form_real_runtime_w | 0.000 | 0.1% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2193073ns | 2193073ns | -1.46% |
| abi_entry_form_real_null_entry | 4876ns | 4876ns | -99.78% |
| abi_entry_form_real_per_w_set | 2209667ns | 2209667ns | -0.72% |
| abi_entry_form_real_runtime_w | 2225619ns | 2225619ns | base |
| abi_entry_form_real_scalar_anchor | 2199965ns | 2199965ns | -1.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2200469ns | base | --- | [2187061, 2277288] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2183876ns | -20289.1ns (-0.9%) | [-71419, -5153]ns | [2178212, 2205868] | YES | 0.0417 | 0.0313 | 0 |
| abi_entry_form_real_null_entry | 2500ns | -2198004.5ns (-99.9%) | [-2274564, -2184612]ns | [2394, 2742] | YES | 0.0417 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2189841ns | no significant difference | [-61298, +27972]ns | [2177259, 2250949] | no | 0.2188 | 0.2188 | 0 |
| abi_entry_form_real_scalar_anchor | 2188635ns | -12481.6ns (-0.6%) | [-59254, -4300]ns | [2182113, 2218034] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2189820ns | -0.1% | -99.9% | -0.7% | -0.1% |
| 2 | 2204888ns | -1.1% | -99.9% | +2.7% | -0.8% |
| 3 | 2279405ns | -3.9% | -99.9% | -3.7% | -2.6% |
| 4 | 2275170ns | -2.3% | -99.9% | -1.7% | -2.6% |
| 5 | 2184302ns | -0.4% | -99.9% | -0.2% | -0.3% |
| 6 | 2196050ns | -0.7% | -99.9% | -0.5% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.193 | ok |
| abi_entry_form_real_null_entry | -0.276 | moderate- |
| abi_entry_form_real_per_w_set | -0.455 | moderate- |
| abi_entry_form_real_runtime_w | 0.169 | ok |
| abi_entry_form_real_scalar_anchor | 0.048 | ok |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 6/6, lost 0/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 5/6, lost 1/6
- **abi_entry_form_real_scalar_anchor**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6645586.3ns | 2189318.7ns | 303.5% | HIGH |
| abi_entry_form_real_null_entry | 117575.6ns | 2545.6ns | 4618.8% | HIGH |
| abi_entry_form_real_per_w_set | 6703985.8ns | 2206016.5ns | 303.9% | HIGH |
| abi_entry_form_real_runtime_w | 6769127.2ns | 2221605.9ns | 304.7% | HIGH |
| abi_entry_form_real_scalar_anchor | 6720611.4ns | 2196260.8ns | 306.0% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2176593.3-2205868.3 ns)
  2176593.3 |####################
  2178057.0 |
  2179520.8 |########################################
  2180984.5 |
  2182448.3 |
  2183912.0 |
  2185375.8 |
  2186839.5 |####################
  2188303.3 |
  2189767.0 |####################
  2191230.8 |
  2192694.5 |
  2194158.3 |
  2195622.0 |
  2197085.8 |
  2198549.5 |
  2200013.3 |
  2201477.0 |
  2202940.8 |
  2204404.5 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 2385.4-2741.9 ns)
   2385.4 |####################
   2403.2 |####################
   2421.1 |
   2438.9 |
   2456.7 |
   2474.5 |
   2492.3 |########################################
   2510.2 |
   2528.0 |####################
   2545.8 |
   2563.6 |
   2581.5 |
   2599.3 |
   2617.1 |
   2634.9 |
   2652.8 |
   2670.6 |
   2688.4 |
   2706.2 |
   2724.1 |
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2174672.5-2250949.1 ns)
  2174672.5 |########################################
  2178486.3 |########################################
  2182300.2 |########################################
  2186114.0 |
  2189927.8 |
  2193741.7 |########################################
  2197555.5 |
  2201369.3 |
  2205183.2 |
  2208997.0 |
  2212810.8 |
  2216624.7 |
  2220438.5 |
  2224252.3 |
  2228066.2 |
  2231880.0 |
  2235693.8 |########################################
  2239507.7 |
  2243321.5 |
  2247135.3 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2184301.7-2277287.7 ns)
  2184301.7 |########################################
  2188951.0 |########################################
  2193600.3 |########################################
  2198249.6 |
  2202898.9 |########################################
  2207548.2 |
  2212197.5 |
  2216846.8 |
  2221496.1 |
  2226145.4 |
  2230794.7 |
  2235444.0 |
  2240093.3 |
  2244742.6 |
  2249391.9 |
  2254041.2 |
  2258690.5 |
  2263339.8 |
  2267989.1 |
  2272638.4 |########################################
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2177731.7-2218033.5 ns)
  2177731.7 |####################
  2179746.8 |
  2181761.9 |
  2183777.0 |
  2185792.1 |########################################
  2187807.2 |
  2189822.3 |####################
  2191837.3 |
  2193852.4 |
  2195867.5 |
  2197882.6 |
  2199897.7 |
  2201912.8 |
  2203927.9 |
  2205943.0 |
  2207958.1 |
  2209973.2 |
  2211988.3 |
  2214003.4 |####################
  2216018.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=4553.8% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=304.4% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=304.2% of algo (FFI overhead may distort results)

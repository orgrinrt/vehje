# abi_entry_form (real)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_real_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_real_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_real_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_real_runtime_w has the worst median (2.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_real_null_entry at 4.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_real_null_entry dominates: 53808% faster than the next best (abi_entry_form_real_scalar_anchor)

abi_entry_form_real_null_entry (4.06 us) leads abi_entry_form_real_scalar_anchor (2.19 ms) by 53808%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_real_null_entry beats baseline by 100% (significant)

abi_entry_form_real_null_entry is -2.20 ms (100%) faster than baseline abi_entry_form_real_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_real_runtime_w is an outlier: 543.3x slower than the field

abi_entry_form_real_runtime_w (2.21 ms) is 543.3x the fastest (4.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_real_null_entry} vs {abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_runtime_w} (53808% apart)

The field splits into a fast tier {abi_entry_form_real_null_entry} and a slow tier {abi_entry_form_real_scalar_anchor, abi_entry_form_real_per_w_set, abi_entry_form_real_dispatch_table, abi_entry_form_real_runtime_w} with a 53808% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 543.3x the fastest

Fastest abi_entry_form_real_null_entry (4.06 us) to slowest abi_entry_form_real_runtime_w (2.21 ms): 543.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_real_null_entry** at 4063.1 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 543.26x (fastest 4063.1 ns, slowest 2207315.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2212842ns | 2196803ns | 2189047ns | 2195331ns | 2251005ns | +0.10% |
| abi_entry_form_real_null_entry | 6392ns | 6410ns | 6228ns | 6392ns | 6473ns | -99.71% |
| abi_entry_form_real_per_w_set | 2239698ns | 2194371ns | 2177430ns | 2190067ns | 2345278ns | +1.31% |
| abi_entry_form_real_runtime_w | 2210637ns | 2211423ns | 2195436ns | 2207631ns | 2222747ns | base |
| abi_entry_form_real_scalar_anchor | 2213003ns | 2194033ns | 2178827ns | 2192287ns | 2261165ns | +0.11% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2208998ns | 2185379ns | 2246984ns | +0.10% | 0.000 |
| abi_entry_form_real_null_entry | 4050ns | 3955ns | 4124ns | -99.82% | 0.001 |
| abi_entry_form_real_per_w_set | 2235578ns | 2174256ns | 2339764ns | +1.31% | 0.000 |
| abi_entry_form_real_runtime_w | 2206754ns | 2191839ns | 2218769ns | base | 0.000 |
| abi_entry_form_real_scalar_anchor | 2209353ns | 2175518ns | 2257188ns | +0.12% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 82358.2 | 2203048.5 | 2208998.1 | 0 |
| abi_entry_form_real_null_entry | 29290.9 | 4190.2 | 4050.0 | n/a |
| abi_entry_form_real_per_w_set | 80345.6 | 2228978.7 | 2235578.0 | 0 |
| abi_entry_form_real_runtime_w | 82367.4 | 2205720.9 | 2206754.2 | n/a |
| abi_entry_form_real_scalar_anchor | 80058.9 | 2203900.2 | 2209353.0 | 1 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_real_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_real_null_entry | 0.001 | 97.3% |
| abi_entry_form_real_per_w_set | 0.000 | 0.2% |
| abi_entry_form_real_runtime_w | 0.000 | 0.2% |
| abi_entry_form_real_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_real_dispatch_table | 2212842ns | 2212842ns | +0.10% |
| abi_entry_form_real_null_entry | 6392ns | 6392ns | -99.71% |
| abi_entry_form_real_per_w_set | 2239698ns | 2239698ns | +1.31% |
| abi_entry_form_real_runtime_w | 2210637ns | 2210637ns | base |
| abi_entry_form_real_scalar_anchor | 2213003ns | 2213003ns | +0.11% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_real_runtime_w | 2207316ns | base | --- | [2194178, 2218769] | --- | --- | --- | --- |
| abi_entry_form_real_dispatch_table | 2192997ns | no significant difference | [-27170, +39668]ns | [2187014, 2246984] | no | 0.2188 | 0.2188 | 0 |
| abi_entry_form_real_null_entry | 4063ns | -2203268.0ns (-99.8%) | [-2214693, -2190152]ns | [3963, 4124] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_real_per_w_set | 2190810ns | no significant difference | [-37042, +133280]ns | [2176161, 2339764] | no | 0.2188 | 0.2188 | 0 |
| abi_entry_form_real_scalar_anchor | 2190336ns | no significant difference | [-27606, +48215]ns | [2180536, 2257188] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_real_runtime_w | abi_entry_form_real_dispatch_table | abi_entry_form_real_null_entry | abi_entry_form_real_per_w_set | abi_entry_form_real_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2215955ns | -1.0% | -99.8% | -1.4% | -1.4% |
| 2 | 2200328ns | -0.0% | -99.8% | -0.1% | -1.1% |
| 3 | 2196518ns | -0.2% | -99.8% | -0.0% | -0.2% |
| 4 | 2214304ns | +3.6% | -99.8% | +12.1% | +4.5% |
| 5 | 2221582ns | -1.5% | -99.8% | -2.0% | -0.9% |
| 6 | 2191839ns | -0.3% | -99.8% | -0.8% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_real_dispatch_table | -0.276 | moderate- |
| abi_entry_form_real_null_entry | 0.004 | ok |
| abi_entry_form_real_per_w_set | -0.232 | moderate- |
| abi_entry_form_real_runtime_w | -0.246 | moderate- |
| abi_entry_form_real_scalar_anchor | -0.086 | ok |

**Consistency summary:**

- **abi_entry_form_real_dispatch_table**: won 4/6, lost 1/6
- **abi_entry_form_real_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_real_per_w_set**: won 3/6, lost 1/6
- **abi_entry_form_real_scalar_anchor**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_real_dispatch_table | 6697105.2ns | 2208998.1ns | 303.2% | HIGH |
| abi_entry_form_real_null_entry | 123774.0ns | 4050.0ns | 3056.2% | HIGH |
| abi_entry_form_real_per_w_set | 6811381.2ns | 2235578.0ns | 304.7% | HIGH |
| abi_entry_form_real_runtime_w | 6703977.4ns | 2206754.2ns | 303.8% | HIGH |
| abi_entry_form_real_scalar_anchor | 6686815.6ns | 2209353.0ns | 302.7% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_real_dispatch_table (n=6, range 2185379.2-2246983.8 ns)
  2185379.2 |####################
  2188459.4 |########################################
  2191539.7 |####################
  2194619.9 |
  2197700.1 |####################
  2200780.3 |
  2203860.6 |
  2206940.8 |
  2210021.0 |
  2213101.2 |
  2216181.5 |
  2219261.7 |
  2222341.9 |
  2225422.2 |
  2228502.4 |
  2231582.6 |
  2234662.8 |
  2237743.1 |
  2240823.3 |
  2243903.5 |
  (0 below, 1 above range)

abi_entry_form_real_null_entry (n=6, range 3955.4-4123.9 ns)
   3955.4 |########################################
   3963.8 |########################################
   3972.3 |
   3980.7 |
   3989.1 |
   3997.5 |
   4006.0 |
   4014.4 |
   4022.8 |########################################
   4031.2 |
   4039.7 |
   4048.1 |
   4056.5 |
   4065.0 |
   4073.4 |
   4081.8 |
   4090.2 |########################################
   4098.7 |
   4107.1 |
   4115.5 |########################################
  (0 below, 1 above range)

abi_entry_form_real_per_w_set (n=6, range 2174255.8-2339764.0 ns)
  2174255.8 |########################################
  2182531.2 |####################
  2190806.6 |########################################
  2199082.0 |
  2207357.4 |
  2215632.8 |
  2223908.2 |
  2232183.7 |
  2240459.1 |
  2248734.5 |
  2257009.9 |
  2265285.3 |
  2273560.7 |
  2281836.1 |
  2290111.5 |
  2298386.9 |
  2306662.3 |
  2314937.7 |
  2323213.1 |
  2331488.5 |
  (0 below, 1 above range)

abi_entry_form_real_runtime_w (n=6, range 2191838.7-2218768.8 ns)
  2191838.7 |########################################
  2193185.2 |
  2194531.7 |
  2195878.2 |########################################
  2197224.7 |
  2198571.2 |
  2199917.7 |########################################
  2201264.2 |
  2202610.7 |
  2203957.2 |
  2205303.7 |
  2206650.2 |
  2207996.7 |
  2209343.2 |
  2210689.7 |
  2212036.2 |
  2213382.7 |########################################
  2214729.2 |########################################
  2216075.7 |
  2217422.2 |
  (0 below, 1 above range)

abi_entry_form_real_scalar_anchor (n=6, range 2175517.5-2257187.7 ns)
  2175517.5 |####################
  2179601.0 |
  2183684.5 |####################
  2187768.0 |########################################
  2191851.5 |
  2195935.0 |
  2200018.6 |####################
  2204102.1 |
  2208185.6 |
  2212269.1 |
  2216352.6 |
  2220436.1 |
  2224519.6 |
  2228603.1 |
  2232686.6 |
  2236770.2 |
  2240853.7 |
  2244937.2 |
  2249020.7 |
  2253104.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_real_dispatch_table**: bridge=303.8% of algo (FFI overhead may distort results)
- **abi_entry_form_real_null_entry**: bridge=3054.0% of algo (FFI overhead may distort results)
- **abi_entry_form_real_per_w_set**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_entry_form_real_runtime_w**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_entry_form_real_scalar_anchor**: bridge=304.0% of algo (FFI overhead may distort results)

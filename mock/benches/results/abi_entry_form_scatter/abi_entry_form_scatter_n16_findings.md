# abi_entry_form (scatter)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_scatter_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_scatter_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_scatter_null_entry dominates: 86196% faster than the next best (abi_entry_form_scatter_scalar_anchor)

abi_entry_form_scatter_null_entry (2.52 us) leads abi_entry_form_scatter_scalar_anchor (2.17 ms) by 86196%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_scatter_null_entry beats baseline by 100% (significant)

abi_entry_form_scatter_null_entry is -2.17 ms (100%) faster than baseline abi_entry_form_scatter_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_scatter_dispatch_table is an outlier: 868.2x slower than the field

abi_entry_form_scatter_dispatch_table (2.19 ms) is 868.2x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_scatter_null_entry} vs {abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_dispatch_table} (86196% apart)

The field splits into a fast tier {abi_entry_form_scatter_null_entry} and a slow tier {abi_entry_form_scatter_scalar_anchor, abi_entry_form_scatter_runtime_w, abi_entry_form_scatter_per_w_set, abi_entry_form_scatter_dispatch_table} with a 86196% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 868.2x the fastest

Fastest abi_entry_form_scatter_null_entry (2.52 us) to slowest abi_entry_form_scatter_dispatch_table (2.19 ms): 868.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_scatter_null_entry** at 2517.7 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 868.16x (fastest 2517.7 ns, slowest 2185761.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2214391ns | 2189628ns | 2179462ns | 2188322ns | 2270960ns | +1.07% |
| abi_entry_form_scatter_null_entry | 4829ns | 4777ns | 4726ns | 4762ns | 4983ns | -99.78% |
| abi_entry_form_scatter_per_w_set | 2180227ns | 2181485ns | 2166189ns | 2177329ns | 2191592ns | -0.49% |
| abi_entry_form_scatter_runtime_w | 2190894ns | 2180694ns | 2170616ns | 2179868ns | 2217570ns | base |
| abi_entry_form_scatter_scalar_anchor | 2184931ns | 2176157ns | 2172793ns | 2175467ns | 2205195ns | -0.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2210598ns | 2175874ns | 2267001ns | +1.07% | 0.000 |
| abi_entry_form_scatter_null_entry | 2549ns | 2502ns | 2626ns | -99.88% | 0.006 |
| abi_entry_form_scatter_per_w_set | 2176752ns | 2162940ns | 2187925ns | -0.48% | 0.000 |
| abi_entry_form_scatter_runtime_w | 2187267ns | 2167337ns | 2213678ns | base | 0.000 |
| abi_entry_form_scatter_scalar_anchor | 2181476ns | 2169588ns | 2201628ns | -0.26% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 77953.3 | 2207665.2 | 2210598.3 | n/a |
| abi_entry_form_scatter_null_entry | 27544.8 | 2636.0 | 2549.2 | n/a |
| abi_entry_form_scatter_per_w_set | 70332.3 | 2181688.0 | 2176751.8 | 0 |
| abi_entry_form_scatter_runtime_w | 68313.1 | 2184458.5 | 2187267.4 | n/a |
| abi_entry_form_scatter_scalar_anchor | 68346.2 | 2181417.1 | 2181476.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_entry_form_scatter_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_scatter_null_entry | 0.006 | 99.4% |
| abi_entry_form_scatter_per_w_set | 0.000 | 0.1% |
| abi_entry_form_scatter_runtime_w | 0.000 | 0.1% |
| abi_entry_form_scatter_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 2214391ns | 2214391ns | +1.07% |
| abi_entry_form_scatter_null_entry | 4829ns | 4829ns | -99.78% |
| abi_entry_form_scatter_per_w_set | 2180227ns | 2180227ns | -0.49% |
| abi_entry_form_scatter_runtime_w | 2190894ns | 2190894ns | base |
| abi_entry_form_scatter_scalar_anchor | 2184931ns | 2184931ns | -0.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_scatter_runtime_w | 2177080ns | base | --- | [2171045, 2213678] | --- | --- | --- | --- |
| abi_entry_form_scatter_dispatch_table | 2185762ns | +9097.8ns (+0.4%) | [+2004, +58890]ns | [2179032, 2267001] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| abi_entry_form_scatter_null_entry | 2518ns | -2174470.0ns (-99.9%) | [-2211174, -2168511]ns | [2504, 2626] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_scatter_per_w_set | 2178019ns | no significant difference | [-33324, +6007]ns | [2164311, 2187925] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_scatter_scalar_anchor | 2172669ns | no significant difference | [-39890, +29241]ns | [2170132, 2201628] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_scatter_runtime_w | abi_entry_form_scatter_dispatch_table | abi_entry_form_scatter_null_entry | abi_entry_form_scatter_per_w_set | abi_entry_form_scatter_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2186332ns | -0.2% | -99.9% | +0.0% | -0.6% |
| 2 | 2167337ns | +0.4% | -99.9% | +0.1% | +0.2% |
| 3 | 2241025ns | +4.2% | -99.9% | -2.4% | -3.0% |
| 4 | 2174752ns | +0.3% | -99.9% | -0.5% | -0.2% |
| 5 | 2175196ns | +1.1% | -99.9% | -0.4% | +2.5% |
| 6 | 2178963ns | +0.4% | -99.9% | +0.4% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_scatter_dispatch_table | -0.333 | moderate- |
| abi_entry_form_scatter_null_entry | -0.263 | moderate- |
| abi_entry_form_scatter_per_w_set | -0.351 | moderate- |
| abi_entry_form_scatter_runtime_w | -0.403 | moderate- |
| abi_entry_form_scatter_scalar_anchor | -0.312 | moderate- |

**Consistency summary:**

- **abi_entry_form_scatter_dispatch_table**: won 1/6, lost 5/6
- **abi_entry_form_scatter_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_scatter_per_w_set**: won 3/6, lost 2/6
- **abi_entry_form_scatter_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_scatter_dispatch_table | 6715674.2ns | 2210598.3ns | 303.8% | HIGH |
| abi_entry_form_scatter_null_entry | 118583.6ns | 2549.2ns | 4651.9% | HIGH |
| abi_entry_form_scatter_per_w_set | 6611106.2ns | 2176751.8ns | 303.7% | HIGH |
| abi_entry_form_scatter_runtime_w | 6627681.7ns | 2187267.4ns | 303.0% | HIGH |
| abi_entry_form_scatter_scalar_anchor | 6619325.0ns | 2181476.1ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_scatter_dispatch_table (n=6, range 2175873.8-2267000.6 ns)
  2175873.8 |####################
  2180430.1 |########################################
  2184986.5 |####################
  2189542.8 |
  2194099.2 |####################
  2198655.5 |
  2203211.8 |
  2207768.2 |
  2212324.5 |
  2216880.9 |
  2221437.2 |
  2225993.5 |
  2230549.9 |
  2235106.2 |
  2239662.6 |
  2244218.9 |
  2248775.2 |
  2253331.6 |
  2257887.9 |
  2262444.3 |
  (0 below, 1 above range)

abi_entry_form_scatter_null_entry (n=6, range 2501.7-2625.9 ns)
   2501.7 |########################################
   2507.9 |####################
   2514.1 |
   2520.3 |####################
   2526.5 |
   2532.7 |
   2538.9 |####################
   2545.2 |
   2551.4 |
   2557.6 |
   2563.8 |
   2570.0 |
   2576.2 |
   2582.4 |
   2588.6 |
   2594.8 |
   2601.0 |
   2607.2 |
   2613.4 |
   2619.6 |
  (0 below, 1 above range)

abi_entry_form_scatter_per_w_set (n=6, range 2162940.4-2187924.8 ns)
  2162940.4 |########################################
  2164189.6 |
  2165438.8 |########################################
  2166688.1 |
  2167937.3 |
  2169186.5 |########################################
  2170435.7 |
  2171684.9 |
  2172934.1 |
  2174183.4 |
  2175432.6 |
  2176681.8 |
  2177931.0 |
  2179180.2 |
  2180429.4 |
  2181678.7 |
  2182927.9 |
  2184177.1 |
  2185426.3 |########################################
  2186675.5 |########################################
  (0 below, 1 above range)

abi_entry_form_scatter_runtime_w (n=6, range 2167336.7-2213678.2 ns)
  2167336.7 |####################
  2169653.8 |
  2171970.8 |
  2174287.9 |########################################
  2176605.0 |
  2178922.1 |####################
  2181239.1 |
  2183556.2 |
  2185873.3 |####################
  2188190.4 |
  2190507.4 |
  2192824.5 |
  2195141.6 |
  2197458.6 |
  2199775.7 |
  2202092.8 |
  2204409.9 |
  2206726.9 |
  2209044.0 |
  2211361.1 |
  (0 below, 1 above range)

abi_entry_form_scatter_scalar_anchor (n=6, range 2169587.5-2201627.5 ns)
  2169587.5 |########################################
  2171189.5 |####################
  2172791.5 |########################################
  2174393.5 |
  2175995.5 |
  2177597.5 |
  2179199.5 |
  2180801.5 |
  2182403.5 |
  2184005.5 |
  2185607.5 |
  2187209.5 |
  2188811.5 |
  2190413.5 |
  2192015.5 |
  2193617.5 |
  2195219.5 |
  2196821.5 |
  2198423.5 |
  2200025.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_scatter_dispatch_table**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_null_entry**: bridge=4695.2% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_per_w_set**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_runtime_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_entry_form_scatter_scalar_anchor**: bridge=303.6% of algo (FFI overhead may distort results)

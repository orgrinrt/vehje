# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_entry_form_wideselect_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_entry_form_wideselect_runtime_w has the worst median (2.08 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_entry_form_wideselect_null_entry at 4.02 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_entry_form_wideselect_null_entry dominates: 51402% faster than the next best (abi_entry_form_wideselect_dispatch_table)

abi_entry_form_wideselect_null_entry (4.02 us) leads abi_entry_form_wideselect_dispatch_table (2.07 ms) by 51402%, a clear separation rather than a photo finish. CV 2.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.07 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_runtime_w is an outlier: 516.5x slower than the field

abi_entry_form_wideselect_runtime_w (2.08 ms) is 516.5x the fastest (4.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_runtime_w} (51402% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_scalar_anchor, abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_runtime_w} with a 51402% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 516.5x the fastest

Fastest abi_entry_form_wideselect_null_entry (4.02 us) to slowest abi_entry_form_wideselect_runtime_w (2.08 ms): 516.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 4022.3 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 516.46x (fastest 4022.3 ns, slowest 2077355.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2084182ns | 2074627ns | 2067212ns | 2074192ns | 2107650ns | -0.08% |
| abi_entry_form_wideselect_null_entry | 6371ns | 6352ns | 6199ns | 6319ns | 6534ns | -99.69% |
| abi_entry_form_wideselect_per_w_set | 2082039ns | 2077931ns | 2067102ns | 2074479ns | 2100846ns | -0.18% |
| abi_entry_form_wideselect_runtime_w | 2085880ns | 2080566ns | 2076884ns | 2079526ns | 2099908ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2076927ns | 2075021ns | 2065498ns | 2073671ns | 2087526ns | -0.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2080919ns | 2064208ns | 2103960ns | -0.08% | 0.000 |
| abi_entry_form_wideselect_null_entry | 4050ns | 3965ns | 4160ns | -99.81% | 0.001 |
| abi_entry_form_wideselect_per_w_set | 2078810ns | 2064191ns | 2097362ns | -0.18% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2082615ns | 2073774ns | 2096419ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2073900ns | 2062603ns | 2084230ns | -0.42% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 65720.9 | 2082650.2 | 2080919.4 | 1 |
| abi_entry_form_wideselect_null_entry | 29370.5 | 4240.7 | 4050.5 | n/a |
| abi_entry_form_wideselect_per_w_set | 67118.5 | 2081268.1 | 2078810.0 | 1 |
| abi_entry_form_wideselect_runtime_w | 63235.9 | 2078207.6 | 2082615.3 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 58556.4 | 2074035.3 | 2073899.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.2% |
| abi_entry_form_wideselect_null_entry | 0.001 | 98.6% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.2% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.2% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2084182ns | 2084182ns | -0.08% |
| abi_entry_form_wideselect_null_entry | 6371ns | 6371ns | -99.69% |
| abi_entry_form_wideselect_per_w_set | 2082039ns | 2082039ns | -0.18% |
| abi_entry_form_wideselect_runtime_w | 2085880ns | 2085880ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2076927ns | 2076927ns | -0.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2077355ns | base | --- | [2074072, 2096419] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2071555ns | no significant difference | [-21315, +20577]ns | [2067243, 2103960] | no | 0.4375 | 0.2188 | 0 |
| abi_entry_form_wideselect_null_entry | 4022ns | -2073332.7ns (-99.8%) | [-2092259, -2070103]ns | [3970, 4160] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2074759ns | no significant difference | [-20706, +11886]ns | [2064309, 2097362] | no | 1.0000 | 1.0000 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2072112ns | no significant difference | [-24307, +8352]ns | [2065358, 2084230] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2073774ns | -0.1% | -99.8% | -0.5% | +0.4% |
| 2 | 2077983ns | +2.1% | -99.8% | +0.0% | +0.4% |
| 3 | 2096257ns | -1.5% | -99.8% | -1.5% | -1.2% |
| 4 | 2076727ns | -0.2% | -99.8% | -0.3% | -0.4% |
| 5 | 2074371ns | -0.2% | -99.8% | +0.7% | -0.6% |
| 6 | 2096581ns | -0.5% | -99.8% | +0.5% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.398 | moderate- |
| abi_entry_form_wideselect_null_entry | -0.202 | moderate- |
| abi_entry_form_wideselect_per_w_set | 0.229 | moderate+ |
| abi_entry_form_wideselect_runtime_w | -0.290 | moderate- |
| abi_entry_form_wideselect_scalar_anchor | 0.429 | moderate+ |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 5/6, lost 1/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 3/6, lost 2/6
- **abi_entry_form_wideselect_scalar_anchor**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6335408.5ns | 2080919.4ns | 304.5% | HIGH |
| abi_entry_form_wideselect_null_entry | 124347.7ns | 4050.5ns | 3069.9% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6330311.6ns | 2078810.0ns | 304.5% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6303899.6ns | 2082615.3ns | 302.7% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6283407.0ns | 2073899.9ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2064208.3-2103960.2 ns)
  2064208.3 |#############
  2066195.9 |
  2068183.5 |
  2070171.1 |########################################
  2072158.7 |
  2074146.3 |
  2076133.9 |
  2078121.5 |
  2080109.1 |
  2082096.7 |
  2084084.2 |#############
  2086071.8 |
  2088059.4 |
  2090047.0 |
  2092034.6 |
  2094022.2 |
  2096009.8 |
  2097997.4 |
  2099985.0 |
  2101972.6 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 3965.4-4159.5 ns)
   3965.4 |########################################
   3975.1 |
   3984.8 |####################
   3994.5 |
   4004.2 |
   4013.9 |
   4023.6 |
   4033.4 |
   4043.1 |
   4052.8 |####################
   4062.5 |
   4072.2 |
   4081.9 |
   4091.6 |
   4101.3 |
   4111.0 |
   4120.7 |####################
   4130.4 |
   4140.1 |
   4149.8 |
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2064190.8-2097362.1 ns)
  2064190.8 |########################################
  2065849.4 |
  2067507.9 |
  2069166.5 |
  2070825.1 |####################
  2072483.6 |
  2074142.2 |
  2075800.8 |
  2077459.3 |####################
  2079117.9 |
  2080776.5 |
  2082435.0 |
  2084093.6 |
  2085752.1 |
  2087410.7 |####################
  2089069.3 |
  2090727.8 |
  2092386.4 |
  2094045.0 |
  2095703.5 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2073773.7-2096418.8 ns)
  2073773.7 |########################################
  2074906.0 |
  2076038.2 |####################
  2077170.5 |####################
  2078302.7 |
  2079435.0 |
  2080567.2 |
  2081699.5 |
  2082831.7 |
  2083964.0 |
  2085096.2 |
  2086228.5 |
  2087360.7 |
  2088493.0 |
  2089625.2 |
  2090757.5 |
  2091889.7 |
  2093022.0 |
  2094154.2 |
  2095286.5 |####################
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2062602.9-2084230.0 ns)
  2062602.9 |########################################
  2063684.3 |
  2064765.6 |
  2065847.0 |
  2066928.3 |
  2068009.7 |########################################
  2069091.0 |
  2070172.4 |
  2071253.7 |########################################
  2072335.1 |########################################
  2073416.4 |
  2074497.8 |
  2075579.2 |
  2076660.5 |
  2077741.9 |
  2078823.2 |
  2079904.6 |
  2080985.9 |
  2082067.3 |########################################
  2083148.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=3101.6% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=302.7% of algo (FFI overhead may distort results)

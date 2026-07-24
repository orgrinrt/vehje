# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 50754% faster than the next best (abi_zig_entry_scatter_zig_per_w_set)

abi_zig_entry_scatter_zig_null (4.12 us) leads abi_zig_entry_scatter_zig_per_w_set (2.09 ms) by 50754%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.10 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_runtime_w is an outlier: 942.6x slower than the field

abi_zig_entry_scatter_zig_tail_runtime_w (3.88 ms) is 942.6x the fastest (4.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} (50754% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_tail_dispatch, abi_zig_entry_scatter_zig_tail_runtime_w} with a 50754% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 942.6x the fastest

Fastest abi_zig_entry_scatter_zig_null (4.12 us) to slowest abi_zig_entry_scatter_zig_tail_runtime_w (3.88 ms): 942.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### abi_zig_entry_scatter_zig_dispatch's edge over baseline is significant but tiny (-42 ns, 0.00%)

abi_zig_entry_scatter_zig_dispatch differs from baseline abi_zig_entry_scatter_zig_runtime_w by -42 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 4117.5 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 942.61x (fastest 4117.5 ns, slowest 3881214.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2110757ns | 2108586ns | 2095851ns | 2104658ns | 2127358ns | +0.25% |
| abi_zig_entry_scatter_zig_dispatch | 2121709ns | 2108339ns | 2097428ns | 2105347ns | 2158393ns | +0.77% |
| abi_zig_entry_scatter_zig_null | 6434ns | 6393ns | 6345ns | 6386ns | 6552ns | -99.69% |
| abi_zig_entry_scatter_zig_per_w_set | 2098855ns | 2096469ns | 2093839ns | 2095622ns | 2106214ns | -0.32% |
| abi_zig_entry_scatter_zig_runtime_w | 2105540ns | 2105986ns | 2099860ns | 2105587ns | 2108308ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3852627ns | 3855835ns | 3833672ns | 3852268ns | 3862643ns | +82.98% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3882755ns | 3884051ns | 3870100ns | 3883726ns | 3887626ns | +84.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2108099ns | 2093336ns | 2124519ns | +0.25% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2118910ns | 2094765ns | 2155360ns | +0.77% | 0.000 |
| abi_zig_entry_scatter_zig_null | 4118ns | 4072ns | 4160ns | -99.80% | 0.001 |
| abi_zig_entry_scatter_zig_per_w_set | 2096273ns | 2091198ns | 2103600ns | -0.31% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2102815ns | 2097030ns | 2105690ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3849751ns | 3830590ns | 3859823ns | +83.08% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3879943ns | 3867432ns | 3884868ns | +84.51% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 181531.4 | 2107367.6 | 2108099.2 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 184483.1 | 2108715.1 | 2118910.4 | n/a |
| abi_zig_entry_scatter_zig_null | 154121.5 | 4182.6 | 4117.5 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 173313.9 | 2097156.8 | 2096273.4 | n/a |
| abi_zig_entry_scatter_zig_runtime_w | 178415.1 | 2105340.8 | 2102815.1 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 190041.5 | 3856688.4 | 3849751.3 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 190893.8 | 3884301.0 | 3879943.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_null | 0.001 | 98.9% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2110757ns | 2110757ns | +0.25% |
| abi_zig_entry_scatter_zig_dispatch | 2121709ns | 2121709ns | +0.77% |
| abi_zig_entry_scatter_zig_null | 6434ns | 6434ns | -99.69% |
| abi_zig_entry_scatter_zig_per_w_set | 2098855ns | 2098855ns | -0.32% |
| abi_zig_entry_scatter_zig_runtime_w | 2105540ns | 2105540ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3852627ns | 3852627ns | +82.98% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3882755ns | 3882755ns | +84.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2103204ns | base | --- | [2099551, 2105690] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2105968ns | no significant difference | [-6679, +21314]ns | [2093811, 2124519] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2105648ns | no significant difference | [-6904, +55232]ns | [2095723, 2155360] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_null | 4118ns | -2099110.0ns (-99.8%) | [-2101560, -2095423]ns | [4075, 4160] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2093914ns | -5859.2ns (-0.3%) | [-12059, -1708]ns | [2091306, 2103600] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3852973ns | +1747851.9ns (+83.1%) | [+1733447, +1759509]ns | [3836458, 3859823] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3881215ns | +1778505.8ns (+84.6%) | [+1770543, +1782336]ns | [3873747, 3884868] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2102072ns | +0.1% | -0.3% | -99.8% | -0.3% | +82.8% | +84.6% |
| 2 | 2107431ns | -0.0% | -0.0% | -99.8% | +0.1% | +82.9% | +84.4% |
| 3 | 2103949ns | -0.5% | +0.0% | -99.8% | -0.6% | +82.1% | +84.6% |
| 4 | 2097030ns | -0.1% | +0.5% | -99.8% | -0.3% | +83.8% | +85.1% |
| 5 | 2103226ns | +1.6% | +4.7% | -99.8% | -0.5% | +83.8% | +83.9% |
| 6 | 2103183ns | +0.4% | -0.3% | -99.8% | -0.3% | +83.2% | +84.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | -0.067 | ok |
| abi_zig_entry_scatter_zig_dispatch | -0.251 | moderate- |
| abi_zig_entry_scatter_zig_null | -0.271 | moderate- |
| abi_zig_entry_scatter_zig_per_w_set | -0.128 | ok |
| abi_zig_entry_scatter_zig_runtime_w | -0.123 | ok |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.099 | ok |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.018 | ok |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 2/6, lost 3/6
- **abi_zig_entry_scatter_zig_dispatch**: won 2/6, lost 2/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 5/6, lost 0/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6571293.7ns | 2108099.2ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6610323.5ns | 2118910.4ns | 312.0% | HIGH |
| abi_zig_entry_scatter_zig_null | 306735.4ns | 4117.5ns | 7449.6% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6528707.8ns | 2096273.4ns | 311.4% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6554149.4ns | 2102815.1ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11822007.7ns | 3849751.3ns | 307.1% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 11911361.0ns | 3879943.4ns | 307.0% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2093335.8-2124518.5 ns)
  2093335.8 |########################################
  2094894.9 |
  2096454.1 |
  2098013.2 |
  2099572.4 |
  2101131.5 |
  2102690.6 |
  2104249.8 |####################
  2105808.9 |####################
  2107368.0 |
  2108927.2 |
  2110486.3 |####################
  2112045.4 |
  2113604.6 |
  2115163.7 |
  2116722.9 |
  2118282.0 |
  2119841.1 |
  2121400.3 |
  2122959.4 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2094765.0-2155359.5 ns)
  2094765.0 |########################################
  2097794.7 |
  2100824.5 |
  2103854.2 |####################
  2106883.9 |########################################
  2109913.6 |
  2112943.4 |
  2115973.1 |
  2119002.8 |
  2122032.5 |
  2125062.3 |
  2128092.0 |
  2131121.7 |
  2134151.5 |
  2137181.2 |
  2140210.9 |
  2143240.6 |
  2146270.4 |
  2149300.1 |
  2152329.8 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 4071.7-4160.2 ns)
   4071.7 |########################################
   4076.1 |########################################
   4080.5 |
   4085.0 |
   4089.4 |
   4093.8 |
   4098.2 |
   4102.7 |
   4107.1 |########################################
   4111.5 |
   4116.0 |
   4120.4 |########################################
   4124.8 |
   4129.2 |########################################
   4133.7 |
   4138.1 |
   4142.5 |
   4146.9 |
   4151.4 |
   4155.8 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2091198.3-2103599.5 ns)
  2091198.3 |########################################
  2091818.4 |####################
  2092438.4 |
  2093058.5 |
  2093678.6 |
  2094298.6 |
  2094918.7 |
  2095538.7 |####################
  2096158.8 |
  2096778.9 |
  2097398.9 |####################
  2098019.0 |
  2098639.0 |
  2099259.1 |
  2099879.2 |
  2100499.2 |
  2101119.3 |
  2101739.4 |
  2102359.4 |
  2102979.5 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2097030.0-2105690.2 ns)
  2097030.0 |####################
  2097463.0 |
  2097896.0 |
  2098329.0 |
  2098762.0 |
  2099195.0 |
  2099628.1 |
  2100061.1 |
  2100494.1 |
  2100927.1 |
  2101360.1 |
  2101793.1 |####################
  2102226.1 |
  2102659.1 |
  2103092.1 |########################################
  2103525.2 |####################
  2103958.2 |
  2104391.2 |
  2104824.2 |
  2105257.2 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3830590.0-3859823.0 ns)
  3830590.0 |#############
  3832051.6 |
  3833513.3 |
  3834974.9 |
  3836436.6 |
  3837898.2 |
  3839359.9 |
  3840821.5 |
  3842283.2 |#############
  3843744.8 |
  3845206.5 |
  3846668.1 |
  3848129.8 |
  3849591.4 |
  3851053.1 |
  3852514.7 |########################################
  3853976.4 |
  3855438.0 |
  3856899.7 |
  3858361.3 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3867432.5-3884868.1 ns)
  3867432.5 |####################
  3868304.3 |
  3869176.1 |
  3870047.8 |
  3870919.6 |
  3871791.4 |
  3872663.2 |
  3873535.0 |
  3874406.7 |
  3875278.5 |
  3876150.3 |
  3877022.1 |
  3877893.9 |
  3878765.6 |
  3879637.4 |########################################
  3880509.2 |
  3881381.0 |####################
  3882252.8 |
  3883124.5 |####################
  3883996.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=7446.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=306.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=307.0% of algo (FFI overhead may distort results)

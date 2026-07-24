# abi_zig_entry (scatter)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_scatter_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_scatter_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_scatter_zig_null dominates: 86723% faster than the next best (abi_zig_entry_scatter_zig_anchor)

abi_zig_entry_scatter_zig_null (2.42 us) leads abi_zig_entry_scatter_zig_anchor (2.10 ms) by 86723%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_scatter_zig_null beats baseline by 100% (significant)

abi_zig_entry_scatter_zig_null is -2.10 ms (100%) faster than baseline abi_zig_entry_scatter_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_scatter_zig_tail_dispatch is an outlier: 1601.8x slower than the field

abi_zig_entry_scatter_zig_tail_dispatch (3.87 ms) is 1601.8x the fastest (2.42 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_scatter_zig_runtime_w shows alternating (throttle bounce) (autocorr -0.52)

abi_zig_entry_scatter_zig_runtime_w's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_scatter_zig_null} vs {abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} (86723% apart)

The field splits into a fast tier {abi_zig_entry_scatter_zig_null} and a slow tier {abi_zig_entry_scatter_zig_anchor, abi_zig_entry_scatter_zig_dispatch, abi_zig_entry_scatter_zig_per_w_set, abi_zig_entry_scatter_zig_runtime_w, abi_zig_entry_scatter_zig_tail_runtime_w, abi_zig_entry_scatter_zig_tail_dispatch} with a 86723% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 1601.8x the fastest

Fastest abi_zig_entry_scatter_zig_null (2.42 us) to slowest abi_zig_entry_scatter_zig_tail_dispatch (3.87 ms): 1601.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_scatter_zig_null** at 2417.9 ns median (-99.9% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1601.84x (fastest 2417.9 ns, slowest 3873091.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2103369ns | 2101905ns | 2099495ns | 2101512ns | 2108091ns | +0.02% |
| abi_zig_entry_scatter_zig_dispatch | 2099085ns | 2103078ns | 2084814ns | 2099292ns | 2105910ns | -0.18% |
| abi_zig_entry_scatter_zig_null | 4725ns | 4725ns | 4634ns | 4705ns | 4801ns | -99.78% |
| abi_zig_entry_scatter_zig_per_w_set | 2102918ns | 2103593ns | 2097715ns | 2102673ns | 2105888ns | +0.00% |
| abi_zig_entry_scatter_zig_runtime_w | 2102871ns | 2103749ns | 2094379ns | 2103215ns | 2106600ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3872415ns | 3875868ns | 3850603ns | 3874336ns | 3880440ns | +84.15% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3866672ns | 3869435ns | 3850204ns | 3865689ns | 3876382ns | +83.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2100742ns | 2096813ns | 2105432ns | +0.02% | 0.000 |
| abi_zig_entry_scatter_zig_dispatch | 2096473ns | 2082348ns | 2103187ns | -0.18% | 0.000 |
| abi_zig_entry_scatter_zig_null | 2438ns | 2402ns | 2492ns | -99.88% | 0.026 |
| abi_zig_entry_scatter_zig_per_w_set | 2100226ns | 2095065ns | 2103135ns | -0.00% | 0.000 |
| abi_zig_entry_scatter_zig_runtime_w | 2100246ns | 2091843ns | 2103930ns | base | 0.000 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3869656ns | 3847943ns | 3877583ns | +84.25% | 0.000 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3863866ns | 3847666ns | 3873590ns | +83.97% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 178887.2 | 2098915.0 | 2100741.7 | n/a |
| abi_zig_entry_scatter_zig_dispatch | 173547.4 | 2097978.5 | 2096473.5 | n/a |
| abi_zig_entry_scatter_zig_null | 155064.0 | 2702.2 | 2437.9 | n/a |
| abi_zig_entry_scatter_zig_per_w_set | 177605.4 | 2099683.3 | 2100226.1 | 10 |
| abi_zig_entry_scatter_zig_runtime_w | 177405.3 | 2101223.5 | 2100245.6 | n/a |
| abi_zig_entry_scatter_zig_tail_dispatch | 189551.9 | 3875707.1 | 3869655.8 | n/a |
| abi_zig_entry_scatter_zig_tail_runtime_w | 191866.1 | 3870212.6 | 3863865.6 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_zig_entry_scatter_zig_null; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_null | 0.026 | 99.3% |
| abi_zig_entry_scatter_zig_per_w_set | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_runtime_w | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 2103369ns | 2103369ns | +0.02% |
| abi_zig_entry_scatter_zig_dispatch | 2099085ns | 2099085ns | -0.18% |
| abi_zig_entry_scatter_zig_null | 4725ns | 4725ns | -99.78% |
| abi_zig_entry_scatter_zig_per_w_set | 2102918ns | 2102918ns | +0.00% |
| abi_zig_entry_scatter_zig_runtime_w | 2102871ns | 2102871ns | base |
| abi_zig_entry_scatter_zig_tail_dispatch | 3872415ns | 3872415ns | +84.15% |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3866672ns | 3866672ns | +83.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_scatter_zig_runtime_w | 2101162ns | base | --- | [2095645, 2103930] | --- | --- | --- | --- |
| abi_zig_entry_scatter_zig_anchor | 2099282ns | no significant difference | [-4648, +5106]ns | [2097511, 2105432] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_dispatch | 2100452ns | no significant difference | [-17715, +7542]ns | [2085781, 2103187] | no | 1.0000 | 0.6875 | 0 |
| abi_zig_entry_scatter_zig_null | 2418ns | -2098757.5ns (-99.9%) | [-2101472, -2093194]ns | [2404, 2492] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_per_w_set | 2100954ns | no significant difference | [-5006, +5788]ns | [2096589, 2103135] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_scatter_zig_tail_dispatch | 3873091ns | +1772821.6ns (+84.4%) | [+1754363, +1781046]ns | [3858294, 3877583] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_scatter_zig_tail_runtime_w | 3866440ns | +1766053.8ns (+84.1%) | [+1748071, +1776736]ns | [3851567, 3873590] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_scatter_zig_runtime_w | abi_zig_entry_scatter_zig_anchor | abi_zig_entry_scatter_zig_dispatch | abi_zig_entry_scatter_zig_null | abi_zig_entry_scatter_zig_per_w_set | abi_zig_entry_scatter_zig_tail_dispatch | abi_zig_entry_scatter_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2099447ns | -0.1% | +0.1% | -99.9% | +0.0% | +84.5% | +84.6% |
| 2 | 2101092ns | -0.1% | -0.0% | -99.9% | -0.3% | +84.3% | +83.9% |
| 3 | 2102098ns | +0.2% | -0.1% | -99.9% | -0.2% | +83.1% | +84.2% |
| 4 | 2091843ns | +0.3% | +0.6% | -99.9% | +0.5% | +85.3% | +84.9% |
| 5 | 2105762ns | -0.3% | -1.1% | -99.9% | -0.1% | +83.7% | +83.1% |
| 6 | 2101231ns | +0.2% | -0.6% | -99.9% | +0.0% | +84.6% | +83.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_scatter_zig_anchor | -0.248 | moderate- |
| abi_zig_entry_scatter_zig_dispatch | 0.125 | ok |
| abi_zig_entry_scatter_zig_null | 0.075 | ok |
| abi_zig_entry_scatter_zig_per_w_set | 0.454 | moderate+ |
| abi_zig_entry_scatter_zig_runtime_w | -0.521 | HIGH- (thermal bounce) |
| abi_zig_entry_scatter_zig_tail_dispatch | -0.335 | moderate- |
| abi_zig_entry_scatter_zig_tail_runtime_w | 0.246 | moderate+ |

**Consistency summary:**

- **abi_zig_entry_scatter_zig_anchor**: won 2/6, lost 3/6
- **abi_zig_entry_scatter_zig_dispatch**: won 2/6, lost 1/6
- **abi_zig_entry_scatter_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_scatter_zig_per_w_set**: won 2/6, lost 1/6
- **abi_zig_entry_scatter_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_scatter_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_scatter_zig_anchor | 6545860.6ns | 2100741.7ns | 311.6% | HIGH |
| abi_zig_entry_scatter_zig_dispatch | 6530673.9ns | 2096473.5ns | 311.5% | HIGH |
| abi_zig_entry_scatter_zig_null | 296604.6ns | 2437.9ns | 12166.3% | HIGH |
| abi_zig_entry_scatter_zig_per_w_set | 6544372.6ns | 2100226.1ns | 311.6% | HIGH |
| abi_zig_entry_scatter_zig_runtime_w | 6547509.1ns | 2100245.6ns | 311.7% | HIGH |
| abi_zig_entry_scatter_zig_tail_dispatch | 11877718.5ns | 3869655.8ns | 306.9% | HIGH |
| abi_zig_entry_scatter_zig_tail_runtime_w | 11867345.5ns | 3863865.6ns | 307.1% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_scatter_zig_anchor (n=6, range 2096813.3-2105431.9 ns)
  2096813.3 |########################################
  2097244.2 |
  2097675.2 |
  2098106.1 |########################################
  2098537.0 |
  2098968.0 |########################################
  2099398.9 |########################################
  2099829.8 |
  2100260.7 |
  2100691.7 |
  2101122.6 |
  2101553.5 |
  2101984.5 |
  2102415.4 |
  2102846.3 |
  2103277.2 |
  2103708.2 |
  2104139.1 |
  2104570.0 |########################################
  2105001.0 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_dispatch (n=6, range 2082347.9-2103187.0 ns)
  2082347.9 |####################
  2083389.9 |
  2084431.8 |
  2085473.8 |
  2086515.7 |
  2087557.7 |
  2088599.6 |####################
  2089641.6 |
  2090683.6 |
  2091725.5 |
  2092767.5 |
  2093809.4 |
  2094851.4 |
  2095893.3 |
  2096935.3 |
  2097977.3 |
  2099019.2 |
  2100061.2 |########################################
  2101103.1 |####################
  2102145.1 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_null (n=6, range 2402.1-2491.7 ns)
   2402.1 |########################################
   2406.6 |
   2411.1 |####################
   2415.5 |
   2420.0 |####################
   2424.5 |
   2429.0 |
   2433.5 |
   2437.9 |
   2442.4 |
   2446.9 |
   2451.4 |
   2455.9 |
   2460.3 |
   2464.8 |
   2469.3 |
   2473.8 |
   2478.3 |####################
   2482.7 |
   2487.2 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_per_w_set (n=6, range 2095064.6-2103135.4 ns)
  2095064.6 |########################################
  2095468.1 |
  2095871.7 |
  2096275.2 |
  2096678.8 |
  2097082.3 |
  2097485.8 |
  2097889.4 |########################################
  2098292.9 |
  2098696.5 |
  2099100.0 |
  2099503.5 |########################################
  2099907.1 |
  2100310.6 |
  2100714.2 |
  2101117.7 |
  2101521.2 |
  2101924.8 |########################################
  2102328.3 |########################################
  2102731.9 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_runtime_w (n=6, range 2091842.9-2103930.2 ns)
  2091842.9 |####################
  2092447.3 |
  2093051.6 |
  2093656.0 |
  2094260.4 |
  2094864.7 |
  2095469.1 |
  2096073.5 |
  2096677.8 |
  2097282.2 |
  2097886.5 |
  2098490.9 |
  2099095.3 |####################
  2099699.6 |
  2100304.0 |
  2100908.4 |########################################
  2101512.7 |####################
  2102117.1 |
  2102721.5 |
  2103325.8 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_dispatch (n=6, range 3847943.3-3877582.7 ns)
  3847943.3 |########################################
  3849425.3 |
  3850907.2 |
  3852389.2 |
  3853871.2 |
  3855353.1 |
  3856835.1 |
  3858317.1 |
  3859799.1 |
  3861281.0 |
  3862763.0 |
  3864245.0 |
  3865726.9 |
  3867208.9 |########################################
  3868690.9 |
  3870172.9 |
  3871654.8 |########################################
  3873136.8 |########################################
  3874618.8 |########################################
  3876100.7 |
  (0 below, 1 above range)

abi_zig_entry_scatter_zig_tail_runtime_w (n=6, range 3847665.8-3873589.8 ns)
  3847665.8 |########################################
  3848962.0 |
  3850258.2 |
  3851554.4 |
  3852850.6 |
  3854146.8 |
  3855443.0 |########################################
  3856739.2 |
  3858035.4 |
  3859331.6 |
  3860627.8 |
  3861924.0 |
  3863220.2 |########################################
  3864516.4 |
  3865812.6 |
  3867108.8 |
  3868405.0 |########################################
  3869701.2 |
  3870997.4 |########################################
  3872293.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_scatter_zig_anchor**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_dispatch**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_null**: bridge=12360.3% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_per_w_set**: bridge=311.7% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_runtime_w**: bridge=311.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_dispatch**: bridge=306.9% of algo (FFI overhead may distort results)
- **abi_zig_entry_scatter_zig_tail_runtime_w**: bridge=307.2% of algo (FFI overhead may distort results)

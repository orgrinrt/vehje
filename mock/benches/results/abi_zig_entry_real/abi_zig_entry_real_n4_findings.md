# abi_zig_entry (real)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_real_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_real_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_real_zig_null dominates: 52524% faster than the next best (abi_zig_entry_real_zig_per_w_set)

abi_zig_entry_real_zig_null (3.93 us) leads abi_zig_entry_real_zig_per_w_set (2.07 ms) by 52524%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_real_zig_null beats baseline by 100% (significant)

abi_zig_entry_real_zig_null is -2.07 ms (100%) faster than baseline abi_zig_entry_real_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_real_zig_tail_runtime_w is an outlier: 983.4x slower than the field

abi_zig_entry_real_zig_tail_runtime_w (3.87 ms) is 983.4x the fastest (3.93 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_zig_entry_real_zig_null shows alternating (throttle bounce) (autocorr -0.77)

abi_zig_entry_real_zig_null's per-pass series has lag-1 autocorrelation -0.77, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_zig_entry_real_zig_null} vs {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} (52524% apart)

The field splits into a fast tier {abi_zig_entry_real_zig_null} and a slow tier {abi_zig_entry_real_zig_per_w_set, abi_zig_entry_real_zig_dispatch, abi_zig_entry_real_zig_runtime_w, abi_zig_entry_real_zig_anchor, abi_zig_entry_real_zig_tail_dispatch, abi_zig_entry_real_zig_tail_runtime_w} with a 52524% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 983.4x the fastest

Fastest abi_zig_entry_real_zig_null (3.93 us) to slowest abi_zig_entry_real_zig_tail_runtime_w (3.87 ms): 983.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_real_zig_null** at 3932.9 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 983.36x (fastest 3932.9 ns, slowest 3867489.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2076109ns | 2077338ns | 2070172ns | 2075063ns | 2080647ns | +0.10% |
| abi_zig_entry_real_zig_dispatch | 2072874ns | 2072661ns | 2069909ns | 2071760ns | 2076029ns | -0.05% |
| abi_zig_entry_real_zig_null | 6248ns | 6224ns | 6181ns | 6210ns | 6338ns | -99.70% |
| abi_zig_entry_real_zig_per_w_set | 2071431ns | 2072324ns | 2067408ns | 2071057ns | 2074004ns | -0.12% |
| abi_zig_entry_real_zig_runtime_w | 2073933ns | 2073396ns | 2071288ns | 2072804ns | 2076950ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3105626ns | 3105382ns | 3103922ns | 3105027ns | 3107375ns | +49.75% |
| abi_zig_entry_real_zig_tail_runtime_w | 3870141ns | 3870314ns | 3865823ns | 3868852ns | 3874233ns | +86.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2073455ns | 2067544ns | 2077938ns | +0.10% | 0.000 |
| abi_zig_entry_real_zig_dispatch | 2070223ns | 2067085ns | 2073404ns | -0.06% | 0.000 |
| abi_zig_entry_real_zig_null | 3954ns | 3908ns | 4017ns | -99.81% | 0.001 |
| abi_zig_entry_real_zig_per_w_set | 2068813ns | 2064937ns | 2071367ns | -0.12% | 0.000 |
| abi_zig_entry_real_zig_runtime_w | 2071389ns | 2068927ns | 2074361ns | base | 0.000 |
| abi_zig_entry_real_zig_tail_dispatch | 3102945ns | 3101253ns | 3104685ns | +49.80% | 0.000 |
| abi_zig_entry_real_zig_tail_runtime_w | 3867345ns | 3863014ns | 3871465ns | +86.70% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 178738.6 | 2073378.7 | 2073455.1 | n/a |
| abi_zig_entry_real_zig_dispatch | 179782.5 | 2071101.5 | 2070222.8 | 4 |
| abi_zig_entry_real_zig_null | 155280.3 | 4163.5 | 3954.4 | n/a |
| abi_zig_entry_real_zig_per_w_set | 180534.3 | 2068618.2 | 2068813.1 | 2 |
| abi_zig_entry_real_zig_runtime_w | 174619.5 | 2073702.8 | 2071388.5 | n/a |
| abi_zig_entry_real_zig_tail_dispatch | 184527.2 | 3101757.4 | 3102944.7 | n/a |
| abi_zig_entry_real_zig_tail_runtime_w | 195563.6 | 3875102.1 | 3867344.7 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_zig_entry_real_zig_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_real_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_real_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_real_zig_null | 0.001 | 99.4% |
| abi_zig_entry_real_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_real_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_real_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_real_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 2076109ns | 2076109ns | +0.10% |
| abi_zig_entry_real_zig_dispatch | 2072874ns | 2072874ns | -0.05% |
| abi_zig_entry_real_zig_null | 6248ns | 6248ns | -99.70% |
| abi_zig_entry_real_zig_per_w_set | 2071431ns | 2071431ns | -0.12% |
| abi_zig_entry_real_zig_runtime_w | 2073933ns | 2073933ns | base |
| abi_zig_entry_real_zig_tail_dispatch | 3105626ns | 3105626ns | +49.75% |
| abi_zig_entry_real_zig_tail_runtime_w | 3870141ns | 3870141ns | +86.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_real_zig_runtime_w | 2070797ns | base | --- | [2069007, 2074361] | --- | --- | --- | --- |
| abi_zig_entry_real_zig_anchor | 2074753ns | no significant difference | [-6656, +8241]ns | [2067674, 2077938] | no | 0.8250 | 0.6875 | 0 |
| abi_zig_entry_real_zig_dispatch | 2070021ns | no significant difference | [-6578, +3018]ns | [2067243, 2073404] | no | 1.0000 | 1.0000 | 0 |
| abi_zig_entry_real_zig_null | 3933ns | -2066859.5ns (-99.8%) | [-2070361, -2065082]ns | [3913, 4017] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_per_w_set | 2069681ns | no significant difference | [-4840, +380]ns | [2065391, 2071367] | no | 0.3281 | 0.2188 | 0 |
| abi_zig_entry_real_zig_tail_dispatch | 3102731ns | +1032580.8ns (+49.9%) | [+1028464, +1033624]ns | [3101418, 3104685] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_real_zig_tail_runtime_w | 3867490ns | +1795074.1ns (+86.7%) | [+1793891, +1798903]ns | [3863079, 3871465] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_real_zig_runtime_w | abi_zig_entry_real_zig_anchor | abi_zig_entry_real_zig_dispatch | abi_zig_entry_real_zig_null | abi_zig_entry_real_zig_per_w_set | abi_zig_entry_real_zig_tail_dispatch | abi_zig_entry_real_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 2069288ns | +0.4% | -0.1% | -99.8% | +0.1% | +49.9% | +86.7% |
| 2 | 2076354ns | -0.4% | -0.4% | -99.8% | -0.3% | +49.4% | +86.5% |
| 3 | 2068927ns | +0.2% | +0.1% | -99.8% | -0.1% | +50.0% | +87.0% |
| 4 | 2072368ns | +0.2% | +0.1% | -99.8% | -0.2% | +49.8% | +86.8% |
| 5 | 2072307ns | -0.2% | -0.2% | -99.8% | -0.1% | +49.8% | +86.6% |
| 6 | 2069088ns | +0.3% | +0.1% | -99.8% | -0.2% | +49.9% | +86.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_real_zig_anchor | -0.586 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_dispatch | -0.037 | ok |
| abi_zig_entry_real_zig_null | -0.765 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_per_w_set | -0.155 | ok |
| abi_zig_entry_real_zig_runtime_w | -0.622 | HIGH- (thermal bounce) |
| abi_zig_entry_real_zig_tail_dispatch | 0.126 | ok |
| abi_zig_entry_real_zig_tail_runtime_w | -0.118 | ok |

**Consistency summary:**

- **abi_zig_entry_real_zig_anchor**: won 2/6, lost 4/6
- **abi_zig_entry_real_zig_dispatch**: won 3/6, lost 3/6
- **abi_zig_entry_real_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_real_zig_per_w_set**: won 4/6, lost 1/6
- **abi_zig_entry_real_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_real_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_real_zig_anchor | 6469317.6ns | 2073455.1ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_dispatch | 6459347.1ns | 2070222.8ns | 312.0% | HIGH |
| abi_zig_entry_real_zig_null | 305317.9ns | 3954.4ns | 7720.9% | HIGH |
| abi_zig_entry_real_zig_per_w_set | 6461195.1ns | 2068813.1ns | 312.3% | HIGH |
| abi_zig_entry_real_zig_runtime_w | 6460301.3ns | 2071388.5ns | 311.9% | HIGH |
| abi_zig_entry_real_zig_tail_dispatch | 9562470.6ns | 3102944.7ns | 308.2% | HIGH |
| abi_zig_entry_real_zig_tail_runtime_w | 11880855.5ns | 3867344.7ns | 307.2% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_real_zig_anchor (n=6, range 2067543.8-2077937.7 ns)
  2067543.8 |########################################
  2068063.5 |
  2068583.2 |
  2069102.9 |
  2069622.6 |
  2070142.3 |
  2070662.0 |
  2071181.7 |
  2071701.4 |
  2072221.1 |
  2072740.8 |####################
  2073260.4 |
  2073780.1 |
  2074299.8 |
  2074819.5 |
  2075339.2 |
  2075858.9 |####################
  2076378.6 |
  2076898.3 |####################
  2077418.0 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_dispatch (n=6, range 2067084.6-2073404.4 ns)
  2067084.6 |####################
  2067400.6 |####################
  2067716.6 |
  2068032.6 |####################
  2068348.6 |
  2068664.5 |
  2068980.5 |
  2069296.5 |
  2069612.5 |
  2069928.5 |
  2070244.5 |
  2070560.5 |
  2070876.5 |
  2071192.4 |
  2071508.4 |
  2071824.4 |########################################
  2072140.4 |
  2072456.4 |
  2072772.4 |
  2073088.4 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_null (n=6, range 3908.3-4017.1 ns)
   3908.3 |########################################
   3913.7 |########################################
   3919.2 |########################################
   3924.6 |
   3930.1 |
   3935.5 |
   3940.9 |########################################
   3946.4 |
   3951.8 |
   3957.2 |
   3962.7 |
   3968.1 |
   3973.6 |
   3979.0 |
   3984.4 |
   3989.9 |
   3995.3 |
   4000.7 |
   4006.2 |
   4011.6 |########################################
  (0 below, 1 above range)

abi_zig_entry_real_zig_per_w_set (n=6, range 2064936.7-2071367.2 ns)
  2064936.7 |########################################
  2065258.2 |
  2065579.8 |########################################
  2065901.3 |
  2066222.8 |
  2066544.3 |
  2066865.9 |
  2067187.4 |
  2067508.9 |
  2067830.4 |
  2068152.0 |
  2068473.5 |
  2068795.0 |########################################
  2069116.6 |
  2069438.1 |
  2069759.6 |
  2070081.1 |
  2070402.7 |########################################
  2070724.2 |########################################
  2071045.7 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_runtime_w (n=6, range 2068926.7-2074361.0 ns)
  2068926.7 |########################################
  2069198.4 |####################
  2069470.1 |
  2069741.9 |
  2070013.6 |
  2070285.3 |
  2070557.0 |
  2070828.7 |
  2071100.4 |
  2071372.2 |
  2071643.9 |
  2071915.6 |
  2072187.3 |########################################
  2072459.0 |
  2072730.7 |
  2073002.5 |
  2073274.2 |
  2073545.9 |
  2073817.6 |
  2074089.3 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_dispatch (n=6, range 3101252.9-3104685.4 ns)
  3101252.9 |########################################
  3101424.5 |########################################
  3101596.1 |
  3101767.8 |
  3101939.4 |
  3102111.0 |########################################
  3102282.6 |
  3102454.3 |
  3102625.9 |
  3102797.5 |
  3102969.2 |
  3103140.8 |
  3103312.4 |########################################
  3103484.0 |
  3103655.7 |
  3103827.3 |
  3103998.9 |
  3104170.5 |
  3104342.2 |########################################
  3104513.8 |
  (0 below, 1 above range)

abi_zig_entry_real_zig_tail_runtime_w (n=6, range 3863013.8-3871465.4 ns)
  3863013.8 |########################################
  3863436.4 |
  3863859.0 |
  3864281.5 |
  3864704.1 |
  3865126.7 |
  3865549.3 |
  3865971.9 |####################
  3866394.4 |
  3866817.0 |
  3867239.6 |
  3867662.2 |
  3868084.8 |
  3868507.3 |####################
  3868929.9 |
  3869352.5 |
  3869775.1 |
  3870197.7 |####################
  3870620.2 |
  3871042.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_real_zig_anchor**: bridge=311.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_dispatch**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_null**: bridge=7731.5% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_per_w_set**: bridge=312.4% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_runtime_w**: bridge=312.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_dispatch**: bridge=308.2% of algo (FFI overhead may distort results)
- **abi_zig_entry_real_zig_tail_runtime_w**: bridge=307.2% of algo (FFI overhead may distort results)

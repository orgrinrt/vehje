# abi_zig_entry (wideselect)

7 variants, 6 samples per variant.
Baseline: **abi_zig_entry_wideselect_zig_runtime_w**

## Highlights

Baseline for all deltas below: **abi_zig_entry_wideselect_zig_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_zig_entry_wideselect_zig_null dominates: 55339% faster than the next best (abi_zig_entry_wideselect_zig_per_w_set)

abi_zig_entry_wideselect_zig_null (3.56 us) leads abi_zig_entry_wideselect_zig_per_w_set (1.97 ms) by 55339%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_zig_entry_wideselect_zig_null beats baseline by 100% (significant)

abi_zig_entry_wideselect_zig_null is -1.97 ms (100%) faster than baseline abi_zig_entry_wideselect_zig_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_zig_entry_wideselect_zig_tail_runtime_w is an outlier: 929.4x slower than the field

abi_zig_entry_wideselect_zig_tail_runtime_w (3.31 ms) is 929.4x the fastest (3.56 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_zig_entry_wideselect_zig_null} vs {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} (55339% apart)

The field splits into a fast tier {abi_zig_entry_wideselect_zig_null} and a slow tier {abi_zig_entry_wideselect_zig_per_w_set, abi_zig_entry_wideselect_zig_dispatch, abi_zig_entry_wideselect_zig_runtime_w, abi_zig_entry_wideselect_zig_anchor, abi_zig_entry_wideselect_zig_tail_dispatch, abi_zig_entry_wideselect_zig_tail_runtime_w} with a 55339% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 929.4x the fastest

Fastest abi_zig_entry_wideselect_zig_null (3.56 us) to slowest abi_zig_entry_wideselect_zig_tail_runtime_w (3.31 ms): 929.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_zig_entry_wideselect_zig_null** at 3557.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 929.43x (fastest 3557.5 ns, slowest 3306451.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1981452ns | 1979557ns | 1977148ns | 1979134ns | 1987080ns | +0.13% |
| abi_zig_entry_wideselect_zig_dispatch | 1976806ns | 1976290ns | 1974714ns | 1975865ns | 1979265ns | -0.11% |
| abi_zig_entry_wideselect_zig_null | 5861ns | 5833ns | 5760ns | 5821ns | 5971ns | -99.70% |
| abi_zig_entry_wideselect_zig_per_w_set | 1974870ns | 1974810ns | 1970300ns | 1974392ns | 1977873ns | -0.21% |
| abi_zig_entry_wideselect_zig_runtime_w | 1978975ns | 1977236ns | 1975214ns | 1977058ns | 1983732ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3310650ns | 3309257ns | 3304521ns | 3307697ns | 3318145ns | +67.29% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3309970ns | 3309231ns | 3306317ns | 3308744ns | 3313636ns | +67.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1978825ns | 1974580ns | 1984282ns | +0.13% | 0.000 |
| abi_zig_entry_wideselect_zig_dispatch | 1974175ns | 1972154ns | 1976684ns | -0.11% | 0.000 |
| abi_zig_entry_wideselect_zig_null | 3574ns | 3521ns | 3639ns | -99.82% | 0.002 |
| abi_zig_entry_wideselect_zig_per_w_set | 1972225ns | 1967557ns | 1975155ns | -0.21% | 0.000 |
| abi_zig_entry_wideselect_zig_runtime_w | 1976314ns | 1972711ns | 1981008ns | base | 0.000 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3307816ns | 3301654ns | 3315355ns | +67.37% | 0.000 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3307128ns | 3303473ns | 3310779ns | +67.34% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 176928.1 | 1975956.7 | 1978824.6 | 1 |
| abi_zig_entry_wideselect_zig_dispatch | 178700.2 | 1974062.2 | 1974174.5 | n/a |
| abi_zig_entry_wideselect_zig_null | 154881.4 | 3827.8 | 3574.2 | n/a |
| abi_zig_entry_wideselect_zig_per_w_set | 180073.7 | 1972530.8 | 1972225.4 | n/a |
| abi_zig_entry_wideselect_zig_runtime_w | 180183.5 | 1976591.8 | 1976314.4 | n/a |
| abi_zig_entry_wideselect_zig_tail_dispatch | 192918.9 | 3308114.4 | 3307815.7 | n/a |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 192836.6 | 3306728.7 | 3307127.5 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.002 Gops/s** (abi_zig_entry_wideselect_zig_null; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_dispatch | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_null | 0.002 | 99.0% |
| abi_zig_entry_wideselect_zig_per_w_set | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_runtime_w | 0.000 | 0.2% |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.000 | 0.1% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 1981452ns | 1981452ns | +0.13% |
| abi_zig_entry_wideselect_zig_dispatch | 1976806ns | 1976806ns | -0.11% |
| abi_zig_entry_wideselect_zig_null | 5861ns | 5861ns | -99.70% |
| abi_zig_entry_wideselect_zig_per_w_set | 1974870ns | 1974870ns | -0.21% |
| abi_zig_entry_wideselect_zig_runtime_w | 1978975ns | 1978975ns | base |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3310650ns | 3310650ns | +67.29% |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3309970ns | 3309970ns | +67.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_runtime_w | 1974538ns | base | --- | [1973398, 1981008] | --- | --- | --- | --- |
| abi_zig_entry_wideselect_zig_anchor | 1977037ns | no significant difference | [-3790, +8798]ns | [1975155, 1984282] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_dispatch | 1973587ns | no significant difference | [-7635, +2146]ns | [1972253, 1976684] | no | 0.6875 | 0.6875 | 0 |
| abi_zig_entry_wideselect_zig_null | 3558ns | -1970947.7ns (-99.8%) | [-1977428, -1969845]ns | [3526, 3639] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_per_w_set | 1972237ns | no significant difference | [-9661, +1200]ns | [1969284, 1975155] | no | 0.2625 | 0.2188 | 0 |
| abi_zig_entry_wideselect_zig_tail_dispatch | 3306373ns | +1330033.7ns (+67.4%) | [+1322775, +1341695]ns | [3301720, 3315355] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 3306451ns | +1330755.2ns (+67.4%) | [+1327506, +1334178]ns | [3304153, 3310779] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_zig_entry_wideselect_zig_runtime_w | abi_zig_entry_wideselect_zig_anchor | abi_zig_entry_wideselect_zig_dispatch | abi_zig_entry_wideselect_zig_null | abi_zig_entry_wideselect_zig_per_w_set | abi_zig_entry_wideselect_zig_tail_dispatch | abi_zig_entry_wideselect_zig_tail_runtime_w |
|---|---|---|---|---|---|---|---|
| 1 | 1972711ns | +0.2% | -0.0% | -99.8% | -0.0% | +67.9% | +67.5% |
| 2 | 1978594ns | +0.1% | -0.3% | -99.8% | -0.3% | +67.1% | +67.4% |
| 3 | 1983422ns | -0.4% | -0.4% | -99.8% | -0.6% | +66.5% | +66.8% |
| 4 | 1974468ns | +0.0% | +0.2% | -99.8% | -0.4% | +67.2% | +67.4% |
| 5 | 1974084ns | +0.7% | -0.1% | -99.8% | -0.1% | +67.5% | +67.4% |
| 6 | 1974608ns | +0.2% | +0.0% | -99.8% | +0.2% | +68.0% | +67.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | -0.387 | moderate- |
| abi_zig_entry_wideselect_zig_dispatch | -0.118 | ok |
| abi_zig_entry_wideselect_zig_null | 0.284 | moderate+ |
| abi_zig_entry_wideselect_zig_per_w_set | 0.100 | ok |
| abi_zig_entry_wideselect_zig_runtime_w | 0.035 | ok |
| abi_zig_entry_wideselect_zig_tail_dispatch | 0.157 | ok |
| abi_zig_entry_wideselect_zig_tail_runtime_w | -0.335 | moderate- |

**Consistency summary:**

- **abi_zig_entry_wideselect_zig_anchor**: won 1/6, lost 4/6
- **abi_zig_entry_wideselect_zig_dispatch**: won 2/6, lost 1/6
- **abi_zig_entry_wideselect_zig_null**: won 6/6, lost 0/6
- **abi_zig_entry_wideselect_zig_per_w_set**: won 3/6, lost 1/6
- **abi_zig_entry_wideselect_zig_tail_dispatch**: won 0/6, lost 6/6
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_zig_entry_wideselect_zig_anchor | 6175147.5ns | 1978824.6ns | 312.1% | HIGH |
| abi_zig_entry_wideselect_zig_dispatch | 6168356.4ns | 1974174.5ns | 312.5% | HIGH |
| abi_zig_entry_wideselect_zig_null | 304504.9ns | 3574.2ns | 8519.6% | HIGH |
| abi_zig_entry_wideselect_zig_per_w_set | 6169693.2ns | 1972225.4ns | 312.8% | HIGH |
| abi_zig_entry_wideselect_zig_runtime_w | 6178157.2ns | 1976314.4ns | 312.6% | HIGH |
| abi_zig_entry_wideselect_zig_tail_dispatch | 10188857.8ns | 3307815.7ns | 308.0% | HIGH |
| abi_zig_entry_wideselect_zig_tail_runtime_w | 10186428.4ns | 3307127.5ns | 308.0% | HIGH |

## Distribution (algo ns)

```
abi_zig_entry_wideselect_zig_anchor (n=6, range 1974580.0-1984282.0 ns)
  1974580.0 |########################################
  1975065.1 |
  1975550.2 |########################################
  1976035.3 |########################################
  1976520.4 |
  1977005.5 |
  1977490.6 |########################################
  1977975.7 |
  1978460.8 |
  1978945.9 |
  1979431.0 |
  1979916.1 |
  1980401.2 |########################################
  1980886.3 |
  1981371.4 |
  1981856.5 |
  1982341.6 |
  1982826.7 |
  1983311.8 |
  1983796.9 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_dispatch (n=6, range 1972154.2-1976683.8 ns)
  1972154.2 |########################################
  1972380.7 |####################
  1972607.2 |
  1972833.6 |
  1973060.1 |
  1973286.6 |
  1973513.1 |
  1973739.5 |
  1973966.0 |
  1974192.5 |
  1974419.0 |####################
  1974645.5 |
  1974871.9 |####################
  1975098.4 |
  1975324.9 |
  1975551.4 |
  1975777.8 |
  1976004.3 |
  1976230.8 |
  1976457.3 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_null (n=6, range 3521.2-3639.2 ns)
   3521.2 |####################
   3527.1 |########################################
   3533.0 |
   3538.9 |
   3544.8 |
   3550.7 |
   3556.6 |
   3562.5 |
   3568.4 |
   3574.3 |
   3580.2 |####################
   3586.1 |
   3592.0 |
   3597.9 |
   3603.8 |
   3609.7 |
   3615.6 |
   3621.5 |
   3627.4 |####################
   3633.3 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_per_w_set (n=6, range 1967556.7-1975155.4 ns)
  1967556.7 |####################
  1967936.6 |
  1968316.6 |
  1968696.5 |
  1969076.4 |
  1969456.4 |
  1969836.3 |
  1970216.2 |
  1970596.2 |
  1970976.1 |####################
  1971356.0 |
  1971736.0 |####################
  1972115.9 |
  1972495.9 |########################################
  1972875.8 |
  1973255.7 |
  1973635.7 |
  1974015.6 |
  1974395.5 |
  1974775.5 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_runtime_w (n=6, range 1972710.8-1981008.0 ns)
  1972710.8 |####################
  1973125.7 |
  1973540.5 |
  1973955.4 |####################
  1974370.2 |########################################
  1974785.1 |
  1975199.9 |
  1975614.8 |
  1976029.7 |
  1976444.5 |
  1976859.4 |
  1977274.2 |
  1977689.1 |
  1978103.9 |
  1978518.8 |####################
  1978933.7 |
  1979348.5 |
  1979763.4 |
  1980178.2 |
  1980593.1 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_dispatch (n=6, range 3301654.2-3315354.6 ns)
  3301654.2 |########################################
  3302339.2 |
  3303024.2 |
  3303709.3 |
  3304394.3 |
  3305079.3 |
  3305764.3 |####################
  3306449.3 |####################
  3307134.4 |
  3307819.4 |
  3308504.4 |
  3309189.4 |
  3309874.4 |
  3310559.5 |
  3311244.5 |
  3311929.5 |
  3312614.5 |####################
  3313299.5 |
  3313984.6 |
  3314669.6 |
  (0 below, 1 above range)

abi_zig_entry_wideselect_zig_tail_runtime_w (n=6, range 3303472.9-3310778.5 ns)
  3303472.9 |########################################
  3303838.2 |
  3304203.5 |
  3304568.7 |########################################
  3304934.0 |########################################
  3305299.3 |
  3305664.6 |
  3306029.9 |
  3306395.2 |
  3306760.4 |
  3307125.7 |
  3307491.0 |########################################
  3307856.3 |
  3308221.6 |
  3308586.9 |
  3308952.1 |
  3309317.4 |
  3309682.7 |
  3310048.0 |########################################
  3310413.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_zig_entry_wideselect_zig_anchor**: bridge=312.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_dispatch**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_null**: bridge=8556.0% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_per_w_set**: bridge=312.6% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_runtime_w**: bridge=312.8% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_dispatch**: bridge=308.1% of algo (FFI overhead may distort results)
- **abi_zig_entry_wideselect_zig_tail_runtime_w**: bridge=308.1% of algo (FFI overhead may distort results)

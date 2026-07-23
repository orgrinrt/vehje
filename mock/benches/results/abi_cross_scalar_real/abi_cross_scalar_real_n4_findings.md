# abi_cross_scalar (real)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_real_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_real_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_real_null_entry dominates: 53457% faster than the next best (abi_cross_scalar_real_inproc_direct)

abi_cross_scalar_real_null_entry (4.04 us) leads abi_cross_scalar_real_inproc_direct (2.17 ms) by 53457%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_real_null_entry beats baseline by 100% (significant)

abi_cross_scalar_real_null_entry is -2.16 ms (100%) faster than baseline abi_cross_scalar_real_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_real_inproc_fnptr is an outlier: 538.8x slower than the field

abi_cross_scalar_real_inproc_fnptr (2.18 ms) is 538.8x the fastest (4.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_cross_scalar_real_null_entry} vs {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} (53457% apart)

The field splits into a fast tier {abi_cross_scalar_real_null_entry} and a slow tier {abi_cross_scalar_real_inproc_direct, abi_cross_scalar_real_ffi_batched_scalar, abi_cross_scalar_real_inproc_fnptr} with a 53457% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 538.8x the fastest

Fastest abi_cross_scalar_real_null_entry (4.04 us) to slowest abi_cross_scalar_real_inproc_fnptr (2.18 ms): 538.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_real_null_entry** at 4042.5 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 538.81x (fastest 4042.5 ns, slowest 2178120.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2181337ns | 2179602ns | 2172097ns | 2179017ns | 2189438ns | +0.48% |
| abi_cross_scalar_real_inproc_direct | 2170926ns | 2168226ns | 2165092ns | 2167697ns | 2178685ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2181476ns | 2181161ns | 2175825ns | 2179630ns | 2187071ns | +0.49% |
| abi_cross_scalar_real_null_entry | 6427ns | 6429ns | 6265ns | 6400ns | 6548ns | -99.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2178063ns | 2169102ns | 2186134ns | +0.48% | 0.000 |
| abi_cross_scalar_real_inproc_direct | 2167737ns | 2162073ns | 2175344ns | base | 0.000 |
| abi_cross_scalar_real_inproc_fnptr | 2178420ns | 2172927ns | 2183820ns | +0.49% | 0.000 |
| abi_cross_scalar_real_null_entry | 4068ns | 4010ns | 4149ns | -99.81% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 65834.7 | 2180031.6 | 2178062.8 | n/a |
| abi_cross_scalar_real_inproc_direct | 10119.2 | 2170589.3 | 2167737.0 | n/a |
| abi_cross_scalar_real_inproc_fnptr | 10024.2 | 2179509.1 | 2178420.5 | 0 |
| abi_cross_scalar_real_null_entry | 28468.2 | 4253.5 | 4068.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_real_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_real_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_real_null_entry | 0.001 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 2181337ns | 2181337ns | +0.48% |
| abi_cross_scalar_real_inproc_direct | 2170926ns | 2170926ns | base |
| abi_cross_scalar_real_inproc_fnptr | 2181476ns | 2181476ns | +0.49% |
| abi_cross_scalar_real_null_entry | 6427ns | 6427ns | -99.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_real_inproc_direct | 2165033ns | base | --- | [2162835, 2175344] | --- | --- | --- | --- |
| abi_cross_scalar_real_ffi_batched_scalar | 2176235ns | +12763.6ns (+0.6%) | [+283, +17930]ns | [2171819, 2186134] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| abi_cross_scalar_real_inproc_fnptr | 2178120ns | +9704.2ns (+0.4%) | [+6401, +15945]ns | [2173321, 2183820] | YES | 0.0469 | 0.0313 | 0 |
| abi_cross_scalar_real_null_entry | 4042ns | -2160984.4ns (-99.8%) | [-2171210, -2158812]ns | [4013, 4149] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_real_inproc_direct | abi_cross_scalar_real_ffi_batched_scalar | abi_cross_scalar_real_inproc_fnptr | abi_cross_scalar_real_null_entry |
|---|---|---|---|---|
| 1 | 2171502ns | +1.0% | +0.3% | -99.8% |
| 2 | 2164906ns | +0.6% | +0.7% | -99.8% |
| 3 | 2179186ns | -0.2% | +0.3% | -99.8% |
| 4 | 2163596ns | +0.6% | +0.8% | -99.8% |
| 5 | 2162073ns | +0.6% | +0.5% | -99.8% |
| 6 | 2165160ns | +0.2% | +0.4% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 0.155 | ok |
| abi_cross_scalar_real_inproc_direct | -0.251 | moderate- |
| abi_cross_scalar_real_inproc_fnptr | 0.301 | moderate+ |
| abi_cross_scalar_real_null_entry | -0.415 | moderate- |

**Consistency summary:**

- **abi_cross_scalar_real_ffi_batched_scalar**: won 1/6, lost 5/6
- **abi_cross_scalar_real_inproc_fnptr**: won 0/6, lost 6/6
- **abi_cross_scalar_real_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_real_ffi_batched_scalar | 6604029.0ns | 2178062.8ns | 303.2% | HIGH |
| abi_cross_scalar_real_inproc_direct | 6523009.6ns | 2167737.0ns | 300.9% | HIGH |
| abi_cross_scalar_real_inproc_fnptr | 6550038.7ns | 2178420.5ns | 300.7% | HIGH |
| abi_cross_scalar_real_null_entry | 121964.0ns | 4068.1ns | 2998.0% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_real_ffi_batched_scalar (n=6, range 2169101.7-2186134.2 ns)
  2169101.7 |########################################
  2169953.3 |
  2170805.0 |
  2171656.6 |
  2172508.2 |
  2173359.8 |
  2174211.5 |########################################
  2175063.1 |########################################
  2175914.7 |########################################
  2176766.3 |
  2177618.0 |
  2178469.6 |########################################
  2179321.2 |
  2180172.8 |
  2181024.5 |
  2181876.1 |
  2182727.7 |
  2183579.3 |
  2184431.0 |
  2185282.6 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_direct (n=6, range 2162073.3-2175343.8 ns)
  2162073.3 |####################
  2162736.8 |
  2163400.3 |####################
  2164063.9 |
  2164727.4 |########################################
  2165390.9 |
  2166054.4 |
  2166718.0 |
  2167381.5 |
  2168045.0 |
  2168708.5 |
  2169372.0 |
  2170035.6 |
  2170699.1 |
  2171362.6 |####################
  2172026.1 |
  2172689.7 |
  2173353.2 |
  2174016.7 |
  2174680.2 |
  (0 below, 1 above range)

abi_cross_scalar_real_inproc_fnptr (n=6, range 2172926.7-2183820.4 ns)
  2172926.7 |########################################
  2173471.4 |########################################
  2174016.1 |
  2174560.8 |
  2175105.4 |
  2175650.1 |
  2176194.8 |
  2176739.5 |########################################
  2177284.2 |
  2177828.9 |
  2178373.6 |
  2178918.2 |########################################
  2179462.9 |
  2180007.6 |
  2180552.3 |
  2181097.0 |########################################
  2181641.7 |
  2182186.3 |
  2182731.0 |
  2183275.7 |
  (0 below, 1 above range)

abi_cross_scalar_real_null_entry (n=6, range 4010.4-4149.1 ns)
   4010.4 |########################################
   4017.3 |
   4024.3 |####################
   4031.2 |
   4038.2 |
   4045.1 |
   4052.0 |####################
   4059.0 |
   4065.9 |
   4072.8 |
   4079.8 |####################
   4086.7 |
   4093.6 |
   4100.6 |
   4107.5 |
   4114.5 |
   4121.4 |
   4128.3 |
   4135.3 |
   4142.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_real_ffi_batched_scalar**: bridge=303.4% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_direct**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_inproc_fnptr**: bridge=300.6% of algo (FFI overhead may distort results)
- **abi_cross_scalar_real_null_entry**: bridge=3011.9% of algo (FFI overhead may distort results)

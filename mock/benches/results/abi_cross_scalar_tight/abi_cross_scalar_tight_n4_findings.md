# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_tight_null_entry dominates: 53741% faster than the next best (abi_cross_scalar_tight_inproc_direct)

abi_cross_scalar_tight_null_entry (4.12 us) leads abi_cross_scalar_tight_inproc_direct (2.22 ms) by 53741%, a clear separation rather than a photo finish. CV 21.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.21 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_ffi_batched_scalar is an outlier: 567.1x slower than the field

abi_cross_scalar_tight_ffi_batched_scalar (2.34 ms) is 567.1x the fastest (4.12 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_tight_null_entry is fastest but the noisiest (CV 21.0%)

abi_cross_scalar_tight_null_entry wins on median (4.12 us) yet has the highest variance (CV 21.0%), while abi_cross_scalar_tight_ffi_batched_scalar is the steadiest (CV 5.1%, 2.34 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_ffi_batched_scalar} (53741% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_ffi_batched_scalar} with a 53741% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 567.1x the fastest

Fastest abi_cross_scalar_tight_null_entry (4.12 us) to slowest abi_cross_scalar_tight_ffi_batched_scalar (2.34 ms): 567.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 4119.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 567.15x (fastest 4119.4 ns, slowest 2336271.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2358386ns | 2341386ns | 2197881ns | 2325261ns | 2488326ns | +3.03% |
| abi_cross_scalar_tight_inproc_direct | 2289073ns | 2222509ns | 2154338ns | 2207294ns | 2479111ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2288587ns | 2242013ns | 2166922ns | 2222566ns | 2448450ns | -0.02% |
| abi_cross_scalar_tight_null_entry | 7870ns | 6515ns | 6375ns | 6489ns | 10688ns | -99.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2353285ns | 2193446ns | 2482752ns | +3.03% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2284186ns | 2150117ns | 2473318ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2282964ns | 2162479ns | 2442828ns | -0.05% | 0.000 |
| abi_cross_scalar_tight_null_entry | 4538ns | 4075ns | 5406ns | -99.80% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 114555.3 | 2331689.7 | 2353285.1 | n/a |
| abi_cross_scalar_tight_inproc_direct | 10625.9 | 2271504.0 | 2284186.5 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 10708.5 | 2284544.5 | 2282963.8 | 0 |
| abi_cross_scalar_tight_null_entry | 41891.0 | 5030.4 | 4538.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_tight_null_entry | 0.001 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2358386ns | 2358386ns | +3.03% |
| abi_cross_scalar_tight_inproc_direct | 2289073ns | 2289073ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2288587ns | 2288587ns | -0.02% |
| abi_cross_scalar_tight_null_entry | 7870ns | 7870ns | -99.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2217886ns | base | --- | [2161355, 2473318] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2336271ns | no significant difference | [-131642, +259461]ns | [2240831, 2482752] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2235092ns | no significant difference | [-135818, +130263]ns | [2170972, 2442828] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_null_entry | 4119ns | -2212480.2ns (-99.8%) | [-2469209, -2157256]ns | [4089, 5406] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2323777ns | -1.1% | +6.4% | -99.8% |
| 2 | 2622859ns | -9.1% | -8.0% | -99.8% |
| 3 | 2194742ns | +8.2% | -0.4% | -99.8% |
| 4 | 2241031ns | +15.1% | -2.7% | -99.7% |
| 5 | 2150117ns | +2.0% | +0.6% | -99.8% |
| 6 | 2172593ns | +5.3% | +5.1% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | -0.258 | moderate- |
| abi_cross_scalar_tight_inproc_direct | 0.049 | ok |
| abi_cross_scalar_tight_inproc_fnptr | 0.394 | moderate+ |
| abi_cross_scalar_tight_null_entry | -0.177 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 2/6, lost 4/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 7124993.0ns | 2353285.1ns | 302.8% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6870828.3ns | 2284186.5ns | 300.8% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 6828153.4ns | 2282963.8ns | 299.1% | HIGH |
| abi_cross_scalar_tight_null_entry | 140085.8ns | 4538.1ns | 3086.9% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2193446.2-2482752.3 ns)
  2193446.2 |########################################
  2207911.5 |
  2222376.8 |
  2236842.1 |
  2251307.4 |
  2265772.7 |
  2280238.0 |########################################
  2294703.3 |########################################
  2309168.6 |
  2323633.9 |
  2338099.2 |
  2352564.6 |
  2367029.9 |########################################
  2381495.2 |########################################
  2395960.5 |
  2410425.8 |
  2424891.1 |
  2439356.4 |
  2453821.7 |
  2468287.0 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2150116.7-2473318.0 ns)
  2150116.7 |########################################
  2166276.8 |########################################
  2182436.8 |########################################
  2198596.9 |
  2214757.0 |
  2230917.0 |########################################
  2247077.1 |
  2263237.1 |
  2279397.2 |
  2295557.3 |
  2311717.3 |########################################
  2327877.4 |
  2344037.5 |
  2360197.5 |
  2376357.6 |
  2392517.6 |
  2408677.7 |
  2424837.8 |
  2440997.8 |
  2457157.9 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2162478.8-2442827.7 ns)
  2162478.8 |####################
  2176496.2 |########################################
  2190513.7 |
  2204531.1 |
  2218548.6 |
  2232566.0 |
  2246583.5 |
  2260600.9 |
  2274618.4 |####################
  2288635.8 |
  2302653.2 |
  2316670.7 |
  2330688.1 |
  2344705.6 |
  2358723.0 |
  2372740.5 |
  2386757.9 |
  2400775.4 |####################
  2414792.8 |
  2428810.3 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 4075.0-5406.2 ns)
   4075.0 |########################################
   4141.6 |
   4208.1 |
   4274.7 |
   4341.2 |##########
   4407.8 |
   4474.4 |
   4540.9 |
   4607.5 |
   4674.1 |
   4740.6 |
   4807.2 |
   4873.8 |
   4940.3 |
   5006.9 |
   5073.4 |
   5140.0 |
   5206.6 |
   5273.1 |
   5339.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=300.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=298.9% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=3064.7% of algo (FFI overhead may distort results)

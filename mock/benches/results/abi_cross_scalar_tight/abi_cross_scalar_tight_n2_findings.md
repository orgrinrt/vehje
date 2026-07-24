# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_cross_scalar_tight_inproc_direct) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_cross_scalar_tight_inproc_direct has the worst median (2.28 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_cross_scalar_tight_null_entry at 3.65 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_cross_scalar_tight_null_entry dominates: 60970% faster than the next best (abi_cross_scalar_tight_inproc_fnptr)

abi_cross_scalar_tight_null_entry (3.65 us) leads abi_cross_scalar_tight_inproc_fnptr (2.23 ms) by 60970%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.28 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_direct is an outlier: 625.2x slower than the field

abi_cross_scalar_tight_inproc_direct (2.28 ms) is 625.2x the fastest (3.65 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_tight_inproc_direct shows alternating (throttle bounce) (autocorr -0.62)

abi_cross_scalar_tight_inproc_direct's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_direct} (60970% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_inproc_fnptr, abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_direct} with a 60970% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 625.2x the fastest

Fastest abi_cross_scalar_tight_null_entry (3.65 us) to slowest abi_cross_scalar_tight_inproc_direct (2.28 ms): 625.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 3649.6 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 625.23x (fastest 3649.6 ns, slowest 2281855.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2297338ns | 2252839ns | 2197472ns | 2241033ns | 2431727ns | +0.07% |
| abi_cross_scalar_tight_inproc_direct | 2295822ns | 2286011ns | 2216161ns | 2266048ns | 2380315ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2240387ns | 2233504ns | 2155926ns | 2220834ns | 2311949ns | -2.41% |
| abi_cross_scalar_tight_null_entry | 6060ns | 6047ns | 5678ns | 5982ns | 6368ns | -99.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2292368ns | 2192653ns | 2426666ns | +0.05% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2291244ns | 2211438ns | 2375259ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2235797ns | 2152140ns | 2307089ns | -2.42% | 0.000 |
| abi_cross_scalar_tight_null_entry | 3612ns | 3408ns | 3741ns | -99.84% | 0.001 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 104078.1 | 2293562.4 | 2292368.1 | n/a |
| abi_cross_scalar_tight_inproc_direct | 10190.1 | 2290537.2 | 2291244.3 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 10452.2 | 2235128.5 | 2235797.3 | 0 |
| abi_cross_scalar_tight_null_entry | 32148.3 | 4409.2 | 3612.4 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.1% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_tight_null_entry | 0.001 | 93.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2297338ns | 2297338ns | +0.07% |
| abi_cross_scalar_tight_inproc_direct | 2295822ns | 2295822ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2240387ns | 2240387ns | -2.41% |
| abi_cross_scalar_tight_null_entry | 6060ns | 6060ns | -99.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2281855ns | base | --- | [2216618, 2375259] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2247788ns | no significant difference | [-125995, +105998]ns | [2202651, 2426666] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2228828ns | no significant difference | [-148637, +19135]ns | [2171474, 2307089] | no | 1.0000 | 0.6875 | 0 |
| abi_cross_scalar_tight_null_entry | 3650ns | -2278205.8ns (-99.8%) | [-2371634, -2213056]ns | [3447, 3741] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2319604ns | +3.5% | +1.1% | -99.8% |
| 2 | 2244107ns | -2.3% | +0.5% | -99.8% |
| 3 | 2221799ns | -0.4% | -0.9% | -99.8% |
| 4 | 2428786ns | -8.3% | -9.8% | -99.8% |
| 5 | 2211438ns | +2.5% | -2.7% | -99.8% |
| 6 | 2321732ns | +5.7% | -2.3% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | -0.002 | ok |
| abi_cross_scalar_tight_inproc_direct | -0.617 | HIGH- (thermal bounce) |
| abi_cross_scalar_tight_inproc_fnptr | 0.173 | ok |
| abi_cross_scalar_tight_null_entry | 0.182 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 4/6, lost 2/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 6977110.3ns | 2292368.1ns | 304.4% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6855621.2ns | 2291244.3ns | 299.2% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 6701383.2ns | 2235797.3ns | 299.7% | HIGH |
| abi_cross_scalar_tight_null_entry | 126747.8ns | 3612.4ns | 3508.7% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2192652.9-2426665.6 ns)
  2192652.9 |########################################
  2204353.5 |########################################
  2216054.2 |
  2227754.8 |########################################
  2239455.4 |
  2251156.1 |
  2262856.7 |########################################
  2274557.4 |
  2286258.0 |
  2297958.6 |
  2309659.3 |
  2321359.9 |
  2333060.5 |
  2344761.2 |
  2356461.8 |
  2368162.5 |
  2379863.1 |
  2391563.7 |########################################
  2403264.4 |
  2414965.0 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2211437.5-2375259.2 ns)
  2211437.5 |####################
  2219628.6 |####################
  2227819.7 |
  2236010.7 |####################
  2244201.8 |
  2252392.9 |
  2260584.0 |
  2268775.1 |
  2276966.2 |
  2285157.2 |
  2293348.3 |
  2301539.4 |
  2309730.5 |
  2317921.6 |########################################
  2326112.7 |
  2334303.7 |
  2342494.8 |
  2350685.9 |
  2358877.0 |
  2367068.1 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2152140.0-2307089.1 ns)
  2152140.0 |########################################
  2159887.5 |
  2167634.9 |
  2175382.4 |
  2183129.8 |########################################
  2190877.3 |
  2198624.7 |########################################
  2206372.2 |
  2214119.7 |
  2221867.1 |
  2229614.6 |
  2237362.0 |
  2245109.5 |
  2252856.9 |########################################
  2260604.4 |########################################
  2268351.9 |
  2276099.3 |
  2283846.8 |
  2291594.2 |
  2299341.7 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 3407.9-3740.8 ns)
   3407.9 |########################################
   3424.5 |
   3441.2 |
   3457.8 |
   3474.5 |########################################
   3491.1 |
   3507.8 |
   3524.4 |
   3541.1 |
   3557.7 |
   3574.4 |
   3591.0 |
   3607.7 |
   3624.3 |########################################
   3641.0 |
   3657.6 |########################################
   3674.3 |
   3690.9 |
   3707.6 |########################################
   3724.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=306.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=301.8% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=297.3% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=3455.3% of algo (FFI overhead may distort results)

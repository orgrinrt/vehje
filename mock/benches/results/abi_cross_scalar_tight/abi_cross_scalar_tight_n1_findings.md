# abi_cross_scalar (tight)

4 variants, 6 samples per variant.
Baseline: **abi_cross_scalar_tight_inproc_direct**

## Highlights

Baseline for all deltas below: **abi_cross_scalar_tight_inproc_direct**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_cross_scalar_tight_null_entry dominates: 42177% faster than the next best (abi_cross_scalar_tight_inproc_direct)

abi_cross_scalar_tight_null_entry (5.06 us) leads abi_cross_scalar_tight_inproc_direct (2.14 ms) by 42177%, a clear separation rather than a photo finish. CV 18.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_cross_scalar_tight_null_entry beats baseline by 100% (significant)

abi_cross_scalar_tight_null_entry is -2.13 ms (100%) faster than baseline abi_cross_scalar_tight_inproc_direct, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_cross_scalar_tight_inproc_fnptr is an outlier: 427.2x slower than the field

abi_cross_scalar_tight_inproc_fnptr (2.16 ms) is 427.2x the fastest (5.06 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_cross_scalar_tight_null_entry is fastest but the noisiest (CV 18.2%)

abi_cross_scalar_tight_null_entry wins on median (5.06 us) yet has the highest variance (CV 18.2%), while abi_cross_scalar_tight_ffi_batched_scalar is the steadiest (CV 3.1%, 2.15 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_cross_scalar_tight_inproc_fnptr shows warm-up / thermal drift (autocorr +0.52)

abi_cross_scalar_tight_inproc_fnptr's per-pass series has lag-1 autocorrelation +0.52, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_cross_scalar_tight_null_entry} vs {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr} (42177% apart)

The field splits into a fast tier {abi_cross_scalar_tight_null_entry} and a slow tier {abi_cross_scalar_tight_inproc_direct, abi_cross_scalar_tight_ffi_batched_scalar, abi_cross_scalar_tight_inproc_fnptr} with a 42177% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 427.2x the fastest

Fastest abi_cross_scalar_tight_null_entry (5.06 us) to slowest abi_cross_scalar_tight_inproc_fnptr (2.16 ms): 427.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_cross_scalar_tight_null_entry** at 5061.4 ns median (-99.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 427.15x (fastest 5061.4 ns, slowest 2162006.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2159112ns | 2157623ns | 2052980ns | 2148606ns | 2227935ns | -4.21% |
| abi_cross_scalar_tight_inproc_direct | 2253940ns | 2143838ns | 2056592ns | 2128278ns | 2541108ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2167718ns | 2166279ns | 2049204ns | 2127922ns | 2286670ns | -3.83% |
| abi_cross_scalar_tight_null_entry | 7983ns | 7423ns | 7152ns | 7343ns | 9357ns | -99.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2154679ns | 2049755ns | 2222981ns | -4.21% | 0.000 |
| abi_cross_scalar_tight_inproc_direct | 2249433ns | 2053041ns | 2535496ns | base | 0.000 |
| abi_cross_scalar_tight_inproc_fnptr | 2163540ns | 2045865ns | 2281624ns | -3.82% | 0.000 |
| abi_cross_scalar_tight_null_entry | 5532ns | 4851ns | 6653ns | -99.75% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 96000.3 | 2148498.1 | 2154679.1 | 1 |
| abi_cross_scalar_tight_inproc_direct | 9879.9 | 2253450.2 | 2249432.9 | n/a |
| abi_cross_scalar_tight_inproc_fnptr | 9518.5 | 2168517.4 | 2163540.1 | n/a |
| abi_cross_scalar_tight_null_entry | 34114.7 | 5228.1 | 5532.5 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_cross_scalar_tight_null_entry; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.000 | 0.2% |
| abi_cross_scalar_tight_inproc_direct | 0.000 | 0.2% |
| abi_cross_scalar_tight_inproc_fnptr | 0.000 | 0.2% |
| abi_cross_scalar_tight_null_entry | 0.000 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 2159112ns | 2159112ns | -4.21% |
| abi_cross_scalar_tight_inproc_direct | 2253940ns | 2253940ns | base |
| abi_cross_scalar_tight_inproc_fnptr | 2167718ns | 2167718ns | -3.83% |
| abi_cross_scalar_tight_null_entry | 7983ns | 7983ns | -99.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_cross_scalar_tight_inproc_direct | 2139810ns | base | --- | [2072993, 2535496] | --- | --- | --- | --- |
| abi_cross_scalar_tight_ffi_batched_scalar | 2153091ns | no significant difference | [-328198, +32839]ns | [2087966, 2222981] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_inproc_fnptr | 2162006ns | no significant difference | [-417104, +141814]ns | [2046990, 2281624] | no | 1.0000 | 1.0000 | 0 |
| abi_cross_scalar_tight_null_entry | 5061ns | -2133551.6ns (-99.7%) | [-2530040, -2068110]ns | [4883, 6653] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_cross_scalar_tight_inproc_direct | abi_cross_scalar_tight_ffi_batched_scalar | abi_cross_scalar_tight_inproc_fnptr | abi_cross_scalar_tight_null_entry |
|---|---|---|---|---|
| 1 | 2053041ns | -0.2% | -0.3% | -99.8% |
| 2 | 2382005ns | -10.2% | -14.0% | -99.8% |
| 3 | 2092945ns | +1.6% | +2.0% | -99.8% |
| 4 | 2142836ns | +1.2% | +4.6% | -99.8% |
| 5 | 2136785ns | +1.5% | +8.6% | -99.7% |
| 6 | 2688987ns | -15.3% | -18.6% | -99.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 0.140 | ok |
| abi_cross_scalar_tight_inproc_direct | -0.227 | moderate- |
| abi_cross_scalar_tight_inproc_fnptr | 0.520 | HIGH+ (drift/warm-up) |
| abi_cross_scalar_tight_null_entry | 0.113 | ok |

**Consistency summary:**

- **abi_cross_scalar_tight_ffi_batched_scalar**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_inproc_fnptr**: won 3/6, lost 3/6
- **abi_cross_scalar_tight_null_entry**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_cross_scalar_tight_ffi_batched_scalar | 6568093.5ns | 2154679.1ns | 304.8% | HIGH |
| abi_cross_scalar_tight_inproc_direct | 6804816.1ns | 2249432.9ns | 302.5% | HIGH |
| abi_cross_scalar_tight_inproc_fnptr | 6566953.1ns | 2163540.1ns | 303.5% | HIGH |
| abi_cross_scalar_tight_null_entry | 138483.9ns | 5532.5ns | 2503.1% | HIGH |

## Distribution (algo ns)

```
abi_cross_scalar_tight_ffi_batched_scalar (n=6, range 2049754.6-2222980.6 ns)
  2049754.6 |####################
  2058415.9 |
  2067077.2 |
  2075738.5 |
  2084399.8 |
  2093061.1 |
  2101722.4 |
  2110383.7 |
  2119045.0 |####################
  2127706.3 |
  2136367.6 |####################
  2145028.9 |
  2153690.2 |
  2162351.5 |########################################
  2171012.8 |
  2179674.1 |
  2188335.4 |
  2196996.7 |
  2205658.0 |
  2214319.3 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_direct (n=6, range 2053040.8-2535495.9 ns)
  2053040.8 |####################
  2077163.6 |####################
  2101286.3 |
  2125409.1 |########################################
  2149531.8 |
  2173654.6 |
  2197777.3 |
  2221900.1 |
  2246022.8 |
  2270145.6 |
  2294268.3 |
  2318391.1 |
  2342513.8 |
  2366636.6 |####################
  2390759.3 |
  2414882.1 |
  2439004.8 |
  2463127.6 |
  2487250.3 |
  2511373.1 |
  (0 below, 1 above range)

abi_cross_scalar_tight_inproc_fnptr (n=6, range 2045864.6-2281623.8 ns)
  2045864.6 |########################################
  2057652.6 |
  2069440.5 |
  2081228.5 |
  2093016.4 |
  2104804.4 |
  2116592.3 |
  2128380.3 |####################
  2140168.3 |
  2151956.2 |
  2163744.2 |
  2175532.1 |
  2187320.1 |####################
  2199108.0 |
  2210896.0 |
  2222684.0 |
  2234471.9 |####################
  2246259.9 |
  2258047.8 |
  2269835.8 |
  (0 below, 1 above range)

abi_cross_scalar_tight_null_entry (n=6, range 4850.8-6653.4 ns)
   4850.8 |########################################
   4940.9 |
   5031.1 |########################################
   5121.2 |
   5211.3 |
   5301.4 |
   5391.6 |
   5481.7 |
   5571.8 |
   5661.9 |
   5752.1 |
   5842.2 |####################
   5932.3 |
   6022.5 |
   6112.6 |
   6202.7 |
   6292.8 |
   6383.0 |
   6473.1 |
   6563.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_cross_scalar_tight_ffi_batched_scalar**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_direct**: bridge=300.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_inproc_fnptr**: autocorrelation=0.52 (measurement drift or warm-up artifact)
- **abi_cross_scalar_tight_inproc_fnptr**: bridge=301.2% of algo (FFI overhead may distort results)
- **abi_cross_scalar_tight_null_entry**: bridge=2546.6% of algo (FFI overhead may distort results)

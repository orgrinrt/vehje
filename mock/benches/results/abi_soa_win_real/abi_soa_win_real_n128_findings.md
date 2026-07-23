# abi_soa_win (real)

3 variants, 6 samples per variant.
Baseline: **abi_soa_win_real_scalar_payload**

## Highlights

Baseline for all deltas below: **abi_soa_win_real_scalar_payload**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_soa_win_real_scalar_payload) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_soa_win_real_scalar_payload has the worst median (2.17 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_soa_win_real_null_entry at 2.76 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_soa_win_real_null_entry dominates: 32456% faster than the next best (abi_soa_win_real_soa_payload)

abi_soa_win_real_null_entry (2.76 us) leads abi_soa_win_real_soa_payload (897.13 us) by 32456%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_soa_win_real_null_entry beats baseline by 100% (significant)

abi_soa_win_real_null_entry is -2.17 ms (100%) faster than baseline abi_soa_win_real_scalar_payload, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_soa_win_real_scalar_payload is an outlier: 788.3x slower than the field

abi_soa_win_real_scalar_payload (2.17 ms) is 788.3x the fastest (2.76 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_soa_win_real_null_entry shows alternating (throttle bounce) (autocorr -0.75)

abi_soa_win_real_null_entry's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 788.3x the fastest

Fastest abi_soa_win_real_null_entry (2.76 us) to slowest abi_soa_win_real_scalar_payload (2.17 ms): 788.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_soa_win_real_null_entry** at 2755.7 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 788.30x (fastest 2755.7 ns, slowest 2172285.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 5033ns | 5024ns | 4847ns | 4993ns | 5187ns | -99.77% |
| abi_soa_win_real_scalar_payload | 2178698ns | 2175716ns | 2168407ns | 2174958ns | 2189453ns | base |
| abi_soa_win_real_soa_payload | 900632ns | 900010ns | 890101ns | 899182ns | 908073ns | -58.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_soa_win_real_null_entry | 2750ns | 2639ns | 2821ns | -99.87% | 0.047 |
| abi_soa_win_real_scalar_payload | 2175262ns | 2165166ns | 2185904ns | base | 0.000 |
| abi_soa_win_real_soa_payload | 897723ns | 887395ns | 905001ns | -58.73% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 29456.0 | 2871.8 | 2749.5 | n/a |
| abi_soa_win_real_scalar_payload | 68228.7 | 2177716.5 | 2175262.0 | n/a |
| abi_soa_win_real_soa_payload | 49562.6 | 897063.0 | 897723.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.049 Gops/s** (abi_soa_win_real_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_soa_win_real_null_entry | 0.046 | 95.8% |
| abi_soa_win_real_scalar_payload | 0.000 | 0.1% |
| abi_soa_win_real_soa_payload | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_soa_win_real_null_entry | 5033ns | 5033ns | -99.77% |
| abi_soa_win_real_scalar_payload | 2178698ns | 2178698ns | base |
| abi_soa_win_real_soa_payload | 900632ns | 900632ns | -58.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_soa_win_real_scalar_payload | 2172285ns | base | --- | [2167597, 2185904] | --- | --- | --- | --- |
| abi_soa_win_real_null_entry | 2756ns | -2169467.5ns (-99.9%) | [-2183232, -2164838]ns | [2672, 2821] | YES | 0.0313 | 0.0313 | 0 |
| abi_soa_win_real_soa_payload | 897127ns | -1280176.4ns (-58.9%) | [-1288896, -1263543]ns | [891042, 905001] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_soa_win_real_scalar_payload | abi_soa_win_real_null_entry | abi_soa_win_real_soa_payload |
|---|---|---|---|
| 1 | 2171922ns | -99.9% | -58.3% |
| 2 | 2179409ns | -99.9% | -58.8% |
| 3 | 2172648ns | -99.9% | -58.8% |
| 4 | 2170028ns | -99.9% | -59.1% |
| 5 | 2165166ns | -99.9% | -58.3% |
| 6 | 2192399ns | -99.9% | -59.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_soa_win_real_null_entry | -0.749 | HIGH- (thermal bounce) |
| abi_soa_win_real_scalar_payload | -0.286 | moderate- |
| abi_soa_win_real_soa_payload | -0.162 | ok |

**Consistency summary:**

- **abi_soa_win_real_null_entry**: won 6/6, lost 0/6
- **abi_soa_win_real_soa_payload**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_soa_win_real_null_entry | 122154.7ns | 2749.5ns | 4442.7% | HIGH |
| abi_soa_win_real_scalar_payload | 6601352.7ns | 2175262.0ns | 303.5% | HIGH |
| abi_soa_win_real_soa_payload | 2742228.5ns | 897723.4ns | 305.5% | HIGH |

## Distribution (algo ns)

```
abi_soa_win_real_null_entry (n=6, range 2638.8-2821.1 ns)
   2638.8 |########################################
   2647.9 |
   2657.0 |
   2666.1 |
   2675.2 |
   2684.4 |
   2693.5 |
   2702.6 |########################################
   2711.7 |
   2720.8 |########################################
   2729.9 |
   2739.0 |
   2748.2 |
   2757.3 |
   2766.4 |
   2775.5 |
   2784.6 |########################################
   2793.7 |########################################
   2802.8 |
   2811.9 |
  (0 below, 1 above range)

abi_soa_win_real_scalar_payload (n=6, range 2165166.2-2185903.8 ns)
  2165166.2 |########################################
  2166203.1 |
  2167240.0 |
  2168276.8 |
  2169313.7 |########################################
  2170350.6 |
  2171387.5 |########################################
  2172424.4 |########################################
  2173461.2 |
  2174498.1 |
  2175535.0 |
  2176571.9 |
  2177608.8 |
  2178645.6 |########################################
  2179682.5 |
  2180719.4 |
  2181756.3 |
  2182793.2 |
  2183830.0 |
  2184866.9 |
  (0 below, 1 above range)

abi_soa_win_real_soa_payload (n=6, range 887395.0-905001.1 ns)
  887395.0 |########################################
  888275.3 |
  889155.6 |
  890035.9 |
  890916.2 |
  891796.5 |
  892676.8 |
  893557.1 |
  894437.4 |########################################
  895317.7 |
  896198.0 |########################################
  897078.3 |########################################
  897958.6 |
  898838.9 |
  899719.2 |
  900599.5 |
  901479.8 |
  902360.1 |
  903240.4 |########################################
  904120.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_soa_win_real_null_entry**: bridge=4405.2% of algo (FFI overhead may distort results)
- **abi_soa_win_real_scalar_payload**: bridge=303.6% of algo (FFI overhead may distort results)
- **abi_soa_win_real_soa_payload**: bridge=305.7% of algo (FFI overhead may distort results)

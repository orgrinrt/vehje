# abi_entry_form (wideselect)

5 variants, 6 samples per variant.
Baseline: **abi_entry_form_wideselect_runtime_w**

## Highlights

Baseline for all deltas below: **abi_entry_form_wideselect_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_entry_form_wideselect_null_entry dominates: 83774% faster than the next best (abi_entry_form_wideselect_per_w_set)

abi_entry_form_wideselect_null_entry (2.46 us) leads abi_entry_form_wideselect_per_w_set (2.06 ms) by 83774%, a clear separation rather than a photo finish. CV 1.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_entry_form_wideselect_null_entry beats baseline by 100% (significant)

abi_entry_form_wideselect_null_entry is -2.06 ms (100%) faster than baseline abi_entry_form_wideselect_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_entry_form_wideselect_scalar_anchor is an outlier: 841.4x slower than the field

abi_entry_form_wideselect_scalar_anchor (2.07 ms) is 841.4x the fastest (2.46 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_entry_form_wideselect_dispatch_table shows alternating (throttle bounce) (autocorr -0.53)

abi_entry_form_wideselect_dispatch_table's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_entry_form_wideselect_null_entry} vs {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_scalar_anchor} (83774% apart)

The field splits into a fast tier {abi_entry_form_wideselect_null_entry} and a slow tier {abi_entry_form_wideselect_per_w_set, abi_entry_form_wideselect_dispatch_table, abi_entry_form_wideselect_runtime_w, abi_entry_form_wideselect_scalar_anchor} with a 83774% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 841.4x the fastest

Fastest abi_entry_form_wideselect_null_entry (2.46 us) to slowest abi_entry_form_wideselect_scalar_anchor (2.07 ms): 841.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_entry_form_wideselect_null_entry** at 2458.1 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 841.36x (fastest 2458.1 ns, slowest 2068149.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2069295ns | 2069120ns | 2066193ns | 2068989ns | 2071306ns | -0.08% |
| abi_entry_form_wideselect_null_entry | 4694ns | 4682ns | 4657ns | 4674ns | 4742ns | -99.77% |
| abi_entry_form_wideselect_per_w_set | 2064005ns | 2064384ns | 2059190ns | 2063451ns | 2067243ns | -0.33% |
| abi_entry_form_wideselect_runtime_w | 2070875ns | 2069859ns | 2066754ns | 2069333ns | 2075248ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2071626ns | 2071236ns | 2063765ns | 2069293ns | 2079058ns | +0.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2066437ns | 2063494ns | 2068372ns | -0.08% | 0.000 |
| abi_entry_form_wideselect_null_entry | 2459ns | 2393ns | 2505ns | -99.88% | 0.026 |
| abi_entry_form_wideselect_per_w_set | 2061237ns | 2056387ns | 2064440ns | -0.33% | 0.000 |
| abi_entry_form_wideselect_runtime_w | 2068053ns | 2064086ns | 2072338ns | base | 0.000 |
| abi_entry_form_wideselect_scalar_anchor | 2068649ns | 2060881ns | 2076104ns | +0.03% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 52316.7 | 2064124.0 | 2066436.6 | 0 |
| abi_entry_form_wideselect_null_entry | 28116.3 | 2676.7 | 2458.9 | n/a |
| abi_entry_form_wideselect_per_w_set | 50424.6 | 2062252.8 | 2061237.0 | n/a |
| abi_entry_form_wideselect_runtime_w | 51725.7 | 2067032.0 | 2068052.9 | n/a |
| abi_entry_form_wideselect_scalar_anchor | 54490.3 | 2072122.4 | 2068649.2 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.027 Gops/s** (abi_entry_form_wideselect_null_entry; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 0.000 | 0.1% |
| abi_entry_form_wideselect_null_entry | 0.026 | 97.4% |
| abi_entry_form_wideselect_per_w_set | 0.000 | 0.1% |
| abi_entry_form_wideselect_runtime_w | 0.000 | 0.1% |
| abi_entry_form_wideselect_scalar_anchor | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 2069295ns | 2069295ns | -0.08% |
| abi_entry_form_wideselect_null_entry | 4694ns | 4694ns | -99.77% |
| abi_entry_form_wideselect_per_w_set | 2064005ns | 2064005ns | -0.33% |
| abi_entry_form_wideselect_runtime_w | 2070875ns | 2070875ns | base |
| abi_entry_form_wideselect_scalar_anchor | 2071626ns | 2071626ns | +0.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_entry_form_wideselect_runtime_w | 2067072ns | base | --- | [2064748, 2072338] | --- | --- | --- | --- |
| abi_entry_form_wideselect_dispatch_table | 2066280ns | no significant difference | [-4221, +1786]ns | [2064658, 2068372] | no | 0.9167 | 0.6875 | 0 |
| abi_entry_form_wideselect_null_entry | 2458ns | -2064644.9ns (-99.9%) | [-2069834, -2062304]ns | [2414, 2505] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_wideselect_per_w_set | 2061716ns | -5356.5ns (-0.3%) | [-14783, -308]ns | [2057555, 2064440] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_entry_form_wideselect_scalar_anchor | 2068150ns | no significant difference | [-10644, +11309]ns | [2061694, 2076104] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_entry_form_wideselect_runtime_w | abi_entry_form_wideselect_dispatch_table | abi_entry_form_wideselect_null_entry | abi_entry_form_wideselect_per_w_set | abi_entry_form_wideselect_scalar_anchor |
|---|---|---|---|---|---|
| 1 | 2065411ns | +0.1% | -99.9% | -0.0% | -0.1% |
| 2 | 2068639ns | -0.1% | -99.9% | -0.3% | +0.2% |
| 3 | 2072818ns | -0.2% | -99.9% | -0.7% | -0.6% |
| 4 | 2065505ns | -0.1% | -99.9% | -0.3% | +0.5% |
| 5 | 2064086ns | +0.1% | -99.9% | -0.0% | +0.6% |
| 6 | 2071859ns | -0.3% | -99.9% | -0.7% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_entry_form_wideselect_dispatch_table | -0.525 | HIGH- (thermal bounce) |
| abi_entry_form_wideselect_null_entry | -0.435 | moderate- |
| abi_entry_form_wideselect_per_w_set | -0.173 | ok |
| abi_entry_form_wideselect_runtime_w | -0.238 | moderate- |
| abi_entry_form_wideselect_scalar_anchor | -0.371 | moderate- |

**Consistency summary:**

- **abi_entry_form_wideselect_dispatch_table**: won 3/6, lost 0/6
- **abi_entry_form_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_entry_form_wideselect_per_w_set**: won 4/6, lost 0/6
- **abi_entry_form_wideselect_scalar_anchor**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_entry_form_wideselect_dispatch_table | 6251302.4ns | 2066436.6ns | 302.5% | HIGH |
| abi_entry_form_wideselect_null_entry | 113448.3ns | 2458.9ns | 4613.8% | HIGH |
| abi_entry_form_wideselect_per_w_set | 6240608.5ns | 2061237.0ns | 302.8% | HIGH |
| abi_entry_form_wideselect_runtime_w | 6256164.0ns | 2068052.9ns | 302.5% | HIGH |
| abi_entry_form_wideselect_scalar_anchor | 6267932.9ns | 2068649.2ns | 303.0% | HIGH |

## Distribution (algo ns)

```
abi_entry_form_wideselect_dispatch_table (n=6, range 2063493.7-2068372.1 ns)
  2063493.7 |########################################
  2063737.6 |
  2063981.5 |
  2064225.5 |
  2064469.4 |
  2064713.3 |
  2064957.2 |
  2065201.1 |
  2065445.0 |
  2065689.0 |########################################
  2065932.9 |########################################
  2066176.8 |
  2066420.7 |########################################
  2066664.6 |
  2066908.5 |########################################
  2067152.5 |
  2067396.4 |
  2067640.3 |
  2067884.2 |
  2068128.1 |
  (0 below, 1 above range)

abi_entry_form_wideselect_null_entry (n=6, range 2393.3-2504.8 ns)
   2393.3 |########################################
   2398.9 |
   2404.5 |
   2410.0 |
   2415.6 |
   2421.2 |
   2426.8 |
   2432.3 |########################################
   2437.9 |
   2443.5 |
   2449.1 |
   2454.6 |########################################
   2460.2 |########################################
   2465.8 |
   2471.4 |
   2476.9 |
   2482.5 |
   2488.1 |
   2493.7 |
   2499.2 |########################################
  (0 below, 1 above range)

abi_entry_form_wideselect_per_w_set (n=6, range 2056387.1-2064440.4 ns)
  2056387.1 |########################################
  2056789.8 |
  2057192.4 |
  2057595.1 |
  2057997.8 |
  2058400.4 |########################################
  2058803.1 |
  2059205.8 |
  2059608.4 |
  2060011.1 |########################################
  2060413.8 |
  2060816.4 |
  2061219.1 |
  2061621.7 |
  2062024.4 |
  2062427.1 |
  2062829.7 |
  2063232.4 |########################################
  2063635.1 |########################################
  2064037.7 |
  (0 below, 1 above range)

abi_entry_form_wideselect_runtime_w (n=6, range 2064085.8-2072338.4 ns)
  2064085.8 |####################
  2064498.4 |
  2064911.1 |
  2065323.7 |########################################
  2065736.3 |
  2066148.9 |
  2066561.6 |
  2066974.2 |
  2067386.8 |
  2067799.4 |
  2068212.1 |
  2068624.7 |####################
  2069037.3 |
  2069450.0 |
  2069862.6 |
  2070275.2 |
  2070687.8 |
  2071100.5 |
  2071513.1 |####################
  2071925.7 |
  (0 below, 1 above range)

abi_entry_form_wideselect_scalar_anchor (n=6, range 2060880.8-2076104.0 ns)
  2060880.8 |########################################
  2061642.0 |
  2062403.1 |########################################
  2063164.3 |
  2063925.4 |########################################
  2064686.6 |
  2065447.8 |
  2066208.9 |
  2066970.1 |
  2067731.2 |
  2068492.4 |
  2069253.6 |
  2070014.7 |
  2070775.9 |
  2071537.0 |########################################
  2072298.2 |
  2073059.4 |
  2073820.5 |
  2074581.7 |
  2075342.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **abi_entry_form_wideselect_dispatch_table**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_null_entry**: bridge=4616.5% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_per_w_set**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_runtime_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_entry_form_wideselect_scalar_anchor**: bridge=302.9% of algo (FFI overhead may distort results)

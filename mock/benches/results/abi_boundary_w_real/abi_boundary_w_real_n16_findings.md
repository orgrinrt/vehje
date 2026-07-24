# abi_boundary_w (real)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_real_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_real_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_real_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_real_scalar_runtime_w has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_real_null_entry at 2.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_real_null_entry dominates: 34835% faster than the next best (abi_boundary_w_real_soa_per_w)

abi_boundary_w_real_null_entry (2.53 us) leads abi_boundary_w_real_soa_per_w (882.27 us) by 34835%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_real_null_entry beats baseline by 100% (significant)

abi_boundary_w_real_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_real_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_real_scalar_runtime_w is an outlier: 854.7x slower than the field

abi_boundary_w_real_scalar_runtime_w (2.16 ms) is 854.7x the fastest (2.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_real_null_entry} vs {abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_runtime_w} (34835% apart)

The field splits into a fast tier {abi_boundary_w_real_null_entry} and a slow tier {abi_boundary_w_real_soa_per_w, abi_boundary_w_real_soa_runtime_w, abi_boundary_w_real_soa_dispatch, abi_boundary_w_real_zig_runtime_w, abi_boundary_w_real_scalar_per_w, abi_boundary_w_real_scalar_anchor, abi_boundary_w_real_scalar_dispatch, abi_boundary_w_real_scalar_runtime_w} with a 34835% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 854.7x the fastest

Fastest abi_boundary_w_real_null_entry (2.53 us) to slowest abi_boundary_w_real_scalar_runtime_w (2.16 ms): 854.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_real_null_entry** at 2525.4 ns median (-99.9% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 854.73x (fastest 2525.4 ns, slowest 2158577.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 4794ns | 4788ns | 4641ns | 4751ns | 4936ns | -99.78% |
| abi_boundary_w_real_scalar_anchor | 2154735ns | 2152786ns | 2146398ns | 2150743ns | 2164894ns | -0.42% |
| abi_boundary_w_real_scalar_dispatch | 2159207ns | 2155973ns | 2148486ns | 2154396ns | 2171785ns | -0.22% |
| abi_boundary_w_real_scalar_per_w | 2151884ns | 2150926ns | 2145588ns | 2150380ns | 2157288ns | -0.55% |
| abi_boundary_w_real_scalar_runtime_w | 2163892ns | 2161461ns | 2146171ns | 2160922ns | 2177208ns | base |
| abi_boundary_w_real_soa_dispatch | 898018ns | 890992ns | 883461ns | 889616ns | 917899ns | -58.50% |
| abi_boundary_w_real_soa_per_w | 898559ns | 884813ns | 882751ns | 884350ns | 927775ns | -58.47% |
| abi_boundary_w_real_soa_runtime_w | 885713ns | 885875ns | 882388ns | 885626ns | 887506ns | -59.07% |
| abi_boundary_w_real_zig_runtime_w | 2092995ns | 2079487ns | 2075379ns | 2079112ns | 2122627ns | -3.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 2537ns | 2474ns | 2604ns | -99.88% | 0.006 |
| abi_boundary_w_real_scalar_anchor | 2151963ns | 2143681ns | 2161949ns | -0.42% | 0.000 |
| abi_boundary_w_real_scalar_dispatch | 2156316ns | 2145834ns | 2168731ns | -0.22% | 0.000 |
| abi_boundary_w_real_scalar_per_w | 2149054ns | 2142827ns | 2154389ns | -0.55% | 0.000 |
| abi_boundary_w_real_scalar_runtime_w | 2160987ns | 2143465ns | 2174105ns | base | 0.000 |
| abi_boundary_w_real_soa_dispatch | 895343ns | 881042ns | 914909ns | -58.57% | 0.000 |
| abi_boundary_w_real_soa_per_w | 895834ns | 880156ns | 924687ns | -58.55% | 0.000 |
| abi_boundary_w_real_soa_runtime_w | 883194ns | 879922ns | 884940ns | -59.13% | 0.000 |
| abi_boundary_w_real_zig_runtime_w | 2089908ns | 2072616ns | 2119124ns | -3.29% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 28059.8 | 2598.4 | 2537.2 | n/a |
| abi_boundary_w_real_scalar_anchor | 49122.6 | 2151818.2 | 2151963.1 | n/a |
| abi_boundary_w_real_scalar_dispatch | 51111.9 | 2155123.7 | 2156315.9 | 0 |
| abi_boundary_w_real_scalar_per_w | 54210.7 | 2150192.7 | 2149054.0 | 0 |
| abi_boundary_w_real_scalar_runtime_w | 50666.5 | 2161500.5 | 2160987.4 | n/a |
| abi_boundary_w_real_soa_dispatch | 41903.8 | 895426.1 | 895342.6 | n/a |
| abi_boundary_w_real_soa_per_w | 42771.8 | 891182.1 | 895834.0 | n/a |
| abi_boundary_w_real_soa_runtime_w | 38235.5 | 882890.4 | 883194.0 | n/a |
| abi_boundary_w_real_zig_runtime_w | 212707.4 | 2088518.3 | 2089908.0 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.006 Gops/s** (abi_boundary_w_real_null_entry; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_real_null_entry | 0.006 | 98.0% |
| abi_boundary_w_real_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_real_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_real_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_real_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_real_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_real_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_real_null_entry | 4794ns | 4794ns | -99.78% |
| abi_boundary_w_real_scalar_anchor | 2154735ns | 2154735ns | -0.42% |
| abi_boundary_w_real_scalar_dispatch | 2159207ns | 2159207ns | -0.22% |
| abi_boundary_w_real_scalar_per_w | 2151884ns | 2151884ns | -0.55% |
| abi_boundary_w_real_scalar_runtime_w | 2163892ns | 2163892ns | base |
| abi_boundary_w_real_soa_dispatch | 898018ns | 898018ns | -58.50% |
| abi_boundary_w_real_soa_per_w | 898559ns | 898559ns | -58.47% |
| abi_boundary_w_real_soa_runtime_w | 885713ns | 885713ns | -59.07% |
| abi_boundary_w_real_zig_runtime_w | 2092995ns | 2092995ns | -3.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_real_scalar_runtime_w | 2158578ns | base | --- | [2150280, 2174105] | --- | --- | --- | --- |
| abi_boundary_w_real_null_entry | 2525ns | -2156052.0ns (-99.9%) | [-2171581, -2147718]ns | [2482, 2604] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_scalar_anchor | 2150078ns | no significant difference | [-28195, +5274]ns | [2143862, 2161949] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_real_scalar_dispatch | 2153059ns | no significant difference | [-13962, +4160]ns | [2147157, 2168731] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_real_scalar_per_w | 2148260ns | -7609.6ns (-0.4%) | [-26423, -1768]ns | [2144513, 2154389] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_soa_dispatch | 888395ns | -1267181.2ns (-58.7%) | [-1287396, -1242357]ns | [882723, 914909] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_soa_per_w | 882274ns | -1272563.0ns (-59.0%) | [-1279271, -1243626]ns | [880541, 924687] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_soa_runtime_w | 883404ns | -1275558.4ns (-59.1%) | [-1290950, -1266871]ns | [881238, 884940] | YES | 0.0417 | 0.0313 | 0 |
| abi_boundary_w_real_zig_runtime_w | 2076544ns | -80856.4ns (-3.7%) | [-98522, -33860]ns | [2074056, 2119124] | YES | 0.0417 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_real_scalar_runtime_w | abi_boundary_w_real_null_entry | abi_boundary_w_real_scalar_anchor | abi_boundary_w_real_scalar_dispatch | abi_boundary_w_real_scalar_per_w | abi_boundary_w_real_soa_dispatch | abi_boundary_w_real_soa_per_w | abi_boundary_w_real_soa_runtime_w | abi_boundary_w_real_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2160503ns | -99.9% | -0.3% | -0.7% | -0.7% | -58.9% | -59.2% | -59.2% | -3.9% |
| 2 | 2157095ns | -99.9% | +0.5% | +0.0% | -0.2% | -56.7% | -58.9% | -59.0% | -0.0% |
| 3 | 2159705ns | -99.9% | -0.1% | +0.2% | -0.1% | -58.6% | -59.2% | -59.0% | -3.6% |
| 4 | 2157450ns | -99.9% | -0.6% | -0.4% | -0.5% | -58.9% | -59.1% | -59.2% | -3.9% |
| 5 | 2143465ns | -99.9% | +0.0% | +0.2% | -0.0% | -58.9% | -58.9% | -58.8% | -3.1% |
| 6 | 2187706ns | -99.9% | -1.9% | -0.6% | -1.8% | -59.6% | -56.0% | -59.6% | -5.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_real_null_entry | -0.026 | ok |
| abi_boundary_w_real_scalar_anchor | 0.455 | moderate+ |
| abi_boundary_w_real_scalar_dispatch | -0.227 | moderate- |
| abi_boundary_w_real_scalar_per_w | 0.093 | ok |
| abi_boundary_w_real_scalar_runtime_w | -0.376 | moderate- |
| abi_boundary_w_real_soa_dispatch | 0.002 | ok |
| abi_boundary_w_real_soa_per_w | -0.060 | ok |
| abi_boundary_w_real_soa_runtime_w | -0.456 | moderate- |
| abi_boundary_w_real_zig_runtime_w | -0.162 | ok |

**Consistency summary:**

- **abi_boundary_w_real_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_real_scalar_anchor**: won 4/6, lost 1/6
- **abi_boundary_w_real_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_real_scalar_per_w**: won 5/6, lost 0/6
- **abi_boundary_w_real_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_real_zig_runtime_w**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_real_null_entry | 119197.9ns | 2537.2ns | 4698.1% | HIGH |
| abi_boundary_w_real_scalar_anchor | 6511829.0ns | 2151963.1ns | 302.6% | HIGH |
| abi_boundary_w_real_scalar_dispatch | 6518702.9ns | 2156315.9ns | 302.3% | HIGH |
| abi_boundary_w_real_scalar_per_w | 6507705.9ns | 2149054.0ns | 302.8% | HIGH |
| abi_boundary_w_real_scalar_runtime_w | 6538049.1ns | 2160987.4ns | 302.5% | HIGH |
| abi_boundary_w_real_soa_dispatch | 2725949.9ns | 895342.6ns | 304.5% | HIGH |
| abi_boundary_w_real_soa_per_w | 2727355.1ns | 895834.0ns | 304.4% | HIGH |
| abi_boundary_w_real_soa_runtime_w | 2688911.6ns | 883194.0ns | 304.5% | HIGH |
| abi_boundary_w_real_zig_runtime_w | 6559655.3ns | 2089908.0ns | 313.9% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_real_null_entry (n=6, range 2474.2-2604.3 ns)
   2474.2 |########################################
   2480.7 |
   2487.2 |########################################
   2493.7 |########################################
   2500.2 |
   2506.7 |
   2513.2 |
   2519.8 |
   2526.3 |
   2532.8 |
   2539.3 |
   2545.8 |########################################
   2552.3 |########################################
   2558.8 |
   2565.3 |
   2571.8 |
   2578.3 |
   2584.8 |
   2591.3 |
   2597.8 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_anchor (n=6, range 2143680.8-2161949.2 ns)
  2143680.8 |########################################
  2144594.2 |####################
  2145507.6 |
  2146421.1 |
  2147334.5 |
  2148247.9 |
  2149161.3 |
  2150074.7 |
  2150988.2 |
  2151901.6 |
  2152815.0 |
  2153728.4 |
  2154641.8 |####################
  2155555.3 |
  2156468.7 |####################
  2157382.1 |
  2158295.5 |
  2159208.9 |
  2160122.4 |
  2161035.8 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_dispatch (n=6, range 2145834.2-2168731.5 ns)
  2145834.2 |####################
  2146979.1 |
  2148123.9 |########################################
  2149268.8 |
  2150413.7 |
  2151558.5 |
  2152703.4 |
  2153848.2 |
  2154993.1 |
  2156138.0 |
  2157282.8 |####################
  2158427.7 |
  2159572.6 |
  2160717.4 |
  2161862.3 |
  2163007.1 |####################
  2164152.0 |
  2165296.9 |
  2166441.7 |
  2167586.6 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_per_w (n=6, range 2142826.7-2154388.8 ns)
  2142826.7 |########################################
  2143404.8 |
  2143982.9 |
  2144561.0 |
  2145139.1 |
  2145717.2 |########################################
  2146295.3 |
  2146873.4 |########################################
  2147451.5 |
  2148029.6 |
  2148607.7 |########################################
  2149185.8 |
  2149763.9 |
  2150342.0 |
  2150920.1 |
  2151498.2 |########################################
  2152076.3 |
  2152654.4 |
  2153232.5 |
  2153810.6 |
  (0 below, 1 above range)

abi_boundary_w_real_scalar_runtime_w (n=6, range 2143465.4-2174104.5 ns)
  2143465.4 |########################################
  2144997.4 |
  2146529.3 |
  2148061.3 |
  2149593.2 |
  2151125.2 |
  2152657.1 |
  2154189.1 |
  2155721.1 |########################################
  2157253.0 |########################################
  2158785.0 |########################################
  2160316.9 |########################################
  2161848.9 |
  2163380.8 |
  2164912.8 |
  2166444.8 |
  2167976.7 |
  2169508.7 |
  2171040.6 |
  2172572.6 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_dispatch (n=6, range 881042.5-914909.4 ns)
  881042.5 |########################################
  882735.8 |########################################
  884429.2 |
  886122.5 |########################################
  887815.9 |########################################
  889509.2 |
  891202.6 |
  892895.9 |
  894589.3 |########################################
  896282.6 |
  897975.9 |
  899669.3 |
  901362.6 |
  903056.0 |
  904749.3 |
  906442.7 |
  908136.0 |
  909829.4 |
  911522.7 |
  913216.1 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_per_w (n=6, range 880156.2-924686.6 ns)
  880156.2 |########################################
  882382.7 |#############
  884609.2 |#############
  886835.8 |
  889062.3 |
  891288.8 |
  893515.3 |
  895741.9 |
  897968.4 |
  900194.9 |
  902421.4 |
  904647.9 |
  906874.5 |
  909101.0 |
  911327.5 |
  913554.0 |
  915780.6 |
  918007.1 |
  920233.6 |
  922460.1 |
  (0 below, 1 above range)

abi_boundary_w_real_soa_runtime_w (n=6, range 879922.5-884940.4 ns)
  879922.5 |########################################
  880173.4 |
  880424.3 |
  880675.2 |
  880926.1 |
  881177.0 |
  881427.9 |
  881678.8 |
  881929.7 |
  882180.6 |
  882431.4 |########################################
  882682.3 |
  882933.2 |
  883184.1 |########################################
  883435.0 |########################################
  883685.9 |########################################
  883936.8 |
  884187.7 |
  884438.6 |
  884689.5 |
  (0 below, 1 above range)

abi_boundary_w_real_zig_runtime_w (n=6, range 2072615.8-2119124.4 ns)
  2072615.8 |#############
  2074941.2 |########################################
  2077266.7 |
  2079592.1 |
  2081917.5 |#############
  2084242.9 |
  2086568.4 |
  2088893.8 |
  2091219.2 |
  2093544.6 |
  2095870.1 |
  2098195.5 |
  2100520.9 |
  2102846.4 |
  2105171.8 |
  2107497.2 |
  2109822.6 |
  2112148.1 |
  2114473.5 |
  2116798.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_real_null_entry**: bridge=4710.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_anchor**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_dispatch**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_per_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_scalar_runtime_w**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_dispatch**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_per_w**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_soa_runtime_w**: bridge=304.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_real_zig_runtime_w**: bridge=313.4% of algo (FFI overhead may distort results)

# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 58161% faster than the next best (abi_boundary_w_wideselect_zig_runtime_w)

abi_boundary_w_wideselect_null_entry (3.43 us) leads abi_boundary_w_wideselect_zig_runtime_w (2.00 ms) by 58161%, a clear separation rather than a photo finish. CV 8.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.09 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_soa_runtime_w is an outlier: 610.4x slower than the field

abi_boundary_w_wideselect_soa_runtime_w (2.10 ms) is 610.4x the fastest (3.43 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_null_entry is fastest but the noisiest (CV 8.7%)

abi_boundary_w_wideselect_null_entry wins on median (3.43 us) yet has the highest variance (CV 8.7%), while abi_boundary_w_wideselect_scalar_per_w is the steadiest (CV 0.1%, 2.08 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_boundary_w_wideselect_scalar_runtime_w shows alternating (throttle bounce) (autocorr -0.80)

abi_boundary_w_wideselect_scalar_runtime_w's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_soa_runtime_w} (58161% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_anchor, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_soa_runtime_w} with a 58161% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 610.4x the fastest

Fastest abi_boundary_w_wideselect_null_entry (3.43 us) to slowest abi_boundary_w_wideselect_soa_runtime_w (2.10 ms): 610.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 3434.1 ns median (-99.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 610.37x (fastest 3434.1 ns, slowest 2096101.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5921ns | 5686ns | 5599ns | 5668ns | 6460ns | -99.72% |
| abi_boundary_w_wideselect_scalar_anchor | 2090053ns | 2085082ns | 2071802ns | 2080931ns | 2112862ns | -0.55% |
| abi_boundary_w_wideselect_scalar_dispatch | 2092322ns | 2090532ns | 2080572ns | 2089796ns | 2101986ns | -0.44% |
| abi_boundary_w_wideselect_scalar_per_w | 2080903ns | 2081588ns | 2076199ns | 2080812ns | 2083392ns | -0.99% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2101657ns | 2097343ns | 2088374ns | 2094403ns | 2119179ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 2090513ns | 2088950ns | 2080644ns | 2088658ns | 2098231ns | -0.53% |
| abi_boundary_w_wideselect_soa_per_w | 2087934ns | 2085564ns | 2074969ns | 2083210ns | 2101501ns | -0.65% |
| abi_boundary_w_wideselect_soa_runtime_w | 2101414ns | 2099050ns | 2094362ns | 2098287ns | 2109632ns | -0.01% |
| abi_boundary_w_wideselect_zig_runtime_w | 2004487ns | 2003725ns | 1995541ns | 2002234ns | 2012340ns | -4.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 3572ns | 3384ns | 3891ns | -99.83% | 0.001 |
| abi_boundary_w_wideselect_scalar_anchor | 2087052ns | 2069211ns | 2109500ns | -0.54% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2089342ns | 2077988ns | 2098778ns | -0.43% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2078189ns | 2073520ns | 2080680ns | -0.96% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2098396ns | 2085706ns | 2115398ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 2087466ns | 2077478ns | 2095088ns | -0.52% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 2085123ns | 2072412ns | 2098478ns | -0.63% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 2098212ns | 2091153ns | 2106089ns | -0.01% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2001470ns | 1992799ns | 2009100ns | -4.62% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 32561.4 | 3717.7 | 3571.9 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 86926.7 | 2087389.6 | 2087052.2 | 2 |
| abi_boundary_w_wideselect_scalar_dispatch | 55440.6 | 2088175.7 | 2089341.9 | n/a |
| abi_boundary_w_wideselect_scalar_per_w | 46413.9 | 2077594.7 | 2078189.1 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 64020.4 | 2100343.1 | 2098396.4 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 72960.8 | 2086013.3 | 2087465.8 | 1 |
| abi_boundary_w_wideselect_soa_per_w | 50460.8 | 2083489.4 | 2085122.7 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 66135.6 | 2098984.5 | 2098212.3 | 11 |
| abi_boundary_w_wideselect_zig_runtime_w | 209548.7 | 2003183.0 | 2001470.4 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.001 | 98.5% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 5921ns | 5921ns | -99.72% |
| abi_boundary_w_wideselect_scalar_anchor | 2090053ns | 2090053ns | -0.55% |
| abi_boundary_w_wideselect_scalar_dispatch | 2092322ns | 2092322ns | -0.44% |
| abi_boundary_w_wideselect_scalar_per_w | 2080903ns | 2080903ns | -0.99% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2101657ns | 2101657ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 2090513ns | 2090513ns | -0.53% |
| abi_boundary_w_wideselect_soa_per_w | 2087934ns | 2087934ns | -0.65% |
| abi_boundary_w_wideselect_soa_runtime_w | 2101414ns | 2101414ns | -0.01% |
| abi_boundary_w_wideselect_zig_runtime_w | 2004487ns | 2004487ns | -4.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2094031ns | base | --- | [2085761, 2115398] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 3434ns | -2090184.8ns (-99.8%) | [-2112007, -2082282]ns | [3391, 3891] | YES (adj: no) | 0.0833 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2082085ns | no significant difference | [-34823, +21251]ns | [2069571, 2109500] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2087492ns | no significant difference | [-23100, +5078]ns | [2081756, 2098778] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2078790ns | -15583.1ns (-0.7%) | [-35655, -9384]ns | [2075097, 2080680] | YES (adj: no) | 0.0833 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 2086048ns | no significant difference | [-29349, +1702]ns | [2081261, 2095088] | no | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 2082700ns | -11131.2ns (-0.5%) | [-27029, -1661]ns | [2074190, 2098478] | YES (adj: no) | 0.3500 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 2096101ns | no significant difference | [-13517, +10468]ns | [2092446, 2106089] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2000774ns | -93514.8ns (-4.5%) | [-117167, -80096]ns | [1994538, 2009100] | YES (adj: no) | 0.0833 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2097379ns | -99.8% | -1.2% | -0.9% | -0.8% | -0.1% | -0.4% | -0.2% | -4.5% |
| 2 | 2090683ns | -99.8% | +0.8% | +0.4% | -0.7% | +0.2% | -0.7% | +0.3% | -4.3% |
| 3 | 2116247ns | -99.8% | -1.2% | -0.9% | -1.7% | -1.4% | -1.9% | -0.0% | -5.5% |
| 4 | 2085706ns | -99.8% | -0.8% | -0.0% | -0.6% | -0.0% | +0.1% | +0.5% | -4.5% |
| 5 | 2114548ns | -99.8% | -2.1% | -1.3% | -1.7% | -1.4% | -0.3% | -1.1% | -5.6% |
| 6 | 2085815ns | -99.8% | +1.2% | +0.1% | -0.3% | -0.4% | -0.6% | +0.5% | -3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.030 | ok |
| abi_boundary_w_wideselect_scalar_anchor | -0.213 | moderate- |
| abi_boundary_w_wideselect_scalar_dispatch | -0.151 | ok |
| abi_boundary_w_wideselect_scalar_per_w | -0.575 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_scalar_runtime_w | -0.797 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_soa_dispatch | 0.359 | moderate+ |
| abi_boundary_w_wideselect_soa_per_w | -0.229 | moderate- |
| abi_boundary_w_wideselect_soa_runtime_w | -0.070 | ok |
| abi_boundary_w_wideselect_zig_runtime_w | -0.048 | ok |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 4/6, lost 2/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 3/6, lost 1/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 3/6, lost 1/6
- **abi_boundary_w_wideselect_soa_per_w**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 2/6, lost 3/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 126229.7ns | 3571.9ns | 3533.9% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6356514.7ns | 2087052.2ns | 304.6% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6321686.3ns | 2089341.9ns | 302.6% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6289444.5ns | 2078189.1ns | 302.6% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6369687.6ns | 2098396.4ns | 303.6% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 6343893.1ns | 2087465.8ns | 303.9% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 6304666.0ns | 2085122.7ns | 302.4% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 6366034.7ns | 2098212.3ns | 303.4% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6308052.8ns | 2001470.4ns | 315.2% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 3384.2-3891.0 ns)
   3384.2 |########################################
   3409.5 |
   3434.9 |
   3460.2 |#############
   3485.6 |
   3510.9 |
   3536.3 |#############
   3561.6 |
   3586.9 |
   3612.3 |
   3637.6 |
   3663.0 |
   3688.3 |
   3713.7 |
   3739.0 |
   3764.3 |
   3789.7 |
   3815.0 |
   3840.4 |
   3865.7 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2069211.2-2109500.5 ns)
  2069211.2 |########################################
  2071225.7 |####################
  2073240.1 |
  2075254.6 |
  2077269.1 |
  2079283.5 |
  2081298.0 |
  2083312.4 |
  2085326.9 |
  2087341.4 |
  2089355.8 |
  2091370.3 |####################
  2093384.8 |
  2095399.2 |
  2097413.7 |
  2099428.1 |
  2101442.6 |
  2103457.1 |
  2105471.5 |
  2107486.0 |####################
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2077987.9-2098778.0 ns)
  2077987.9 |########################################
  2079027.4 |
  2080066.9 |
  2081106.4 |
  2082145.9 |
  2083185.4 |
  2084224.9 |
  2085264.4 |########################################
  2086303.9 |########################################
  2087343.4 |########################################
  2088382.9 |
  2089422.4 |
  2090461.9 |
  2091501.4 |
  2092540.9 |
  2093580.4 |
  2094619.9 |
  2095659.4 |
  2096698.9 |
  2097738.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2073520.4-2080680.2 ns)
  2073520.4 |########################################
  2073878.4 |
  2074236.4 |
  2074594.4 |
  2074952.4 |
  2075310.4 |
  2075668.4 |
  2076026.3 |
  2076384.3 |########################################
  2076742.3 |
  2077100.3 |
  2077458.3 |
  2077816.3 |
  2078174.3 |########################################
  2078532.3 |
  2078890.3 |########################################
  2079248.3 |
  2079606.3 |
  2079964.3 |########################################
  2080322.3 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2085705.8-2115397.5 ns)
  2085705.8 |########################################
  2087190.4 |
  2088675.0 |
  2090159.6 |####################
  2091644.1 |
  2093128.7 |
  2094613.3 |
  2096097.9 |####################
  2097582.5 |
  2099067.1 |
  2100551.6 |
  2102036.2 |
  2103520.8 |
  2105005.4 |
  2106490.0 |
  2107974.6 |
  2109459.2 |
  2110943.7 |
  2112428.3 |
  2113912.9 |####################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 2077478.3-2095087.9 ns)
  2077478.3 |####################
  2078358.8 |
  2079239.3 |
  2080119.7 |
  2081000.2 |
  2081880.7 |
  2082761.2 |
  2083641.7 |
  2084522.2 |########################################
  2085402.6 |
  2086283.1 |####################
  2087163.6 |
  2088044.1 |
  2088924.6 |
  2089805.1 |
  2090685.5 |
  2091566.0 |
  2092446.5 |
  2093327.0 |
  2094207.5 |####################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 2072412.1-2098478.2 ns)
  2072412.1 |####################
  2073715.4 |
  2075018.7 |####################
  2076322.0 |####################
  2077625.3 |
  2078928.6 |
  2080231.9 |
  2081535.2 |
  2082838.5 |
  2084141.8 |
  2085445.1 |
  2086748.4 |
  2088051.7 |########################################
  2089355.0 |
  2090658.3 |
  2091961.6 |
  2093264.9 |
  2094568.2 |
  2095871.5 |
  2097174.8 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 2091152.9-2106089.4 ns)
  2091152.9 |####################
  2091899.7 |
  2092646.5 |
  2093393.4 |####################
  2094140.2 |
  2094887.0 |
  2095633.8 |########################################
  2096380.7 |####################
  2097127.5 |
  2097874.3 |
  2098621.1 |
  2099368.0 |
  2100114.8 |
  2100861.6 |
  2101608.4 |
  2102355.3 |
  2103102.1 |
  2103848.9 |
  2104595.8 |
  2105342.6 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 1992798.7-2009099.5 ns)
  1992798.7 |########################################
  1993613.7 |
  1994428.8 |
  1995243.8 |
  1996058.9 |########################################
  1996873.9 |
  1997689.0 |
  1998504.0 |
  1999319.0 |
  2000134.1 |########################################
  2000949.1 |########################################
  2001764.2 |
  2002579.2 |########################################
  2003394.3 |
  2004209.3 |
  2005024.3 |
  2005839.4 |
  2006654.4 |
  2007469.5 |
  2008284.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=3550.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=314.4% of algo (FFI overhead may distort results)

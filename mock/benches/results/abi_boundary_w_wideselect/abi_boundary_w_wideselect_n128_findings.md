# abi_boundary_w (wideselect)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_wideselect_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_wideselect_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_wideselect_null_entry dominates: 34476% faster than the next best (abi_boundary_w_wideselect_soa_runtime_w)

abi_boundary_w_wideselect_null_entry (2.70 us) leads abi_boundary_w_wideselect_soa_runtime_w (934.48 us) by 34476%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_wideselect_null_entry beats baseline by 100% (significant)

abi_boundary_w_wideselect_null_entry is -2.11 ms (100%) faster than baseline abi_boundary_w_wideselect_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_wideselect_scalar_anchor is an outlier: 784.4x slower than the field

abi_boundary_w_wideselect_scalar_anchor (2.12 ms) is 784.4x the fastest (2.70 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_wideselect_null_entry shows alternating (throttle bounce) (autocorr -0.72)

abi_boundary_w_wideselect_null_entry's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_wideselect_null_entry} vs {abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_anchor} (34476% apart)

The field splits into a fast tier {abi_boundary_w_wideselect_null_entry} and a slow tier {abi_boundary_w_wideselect_soa_runtime_w, abi_boundary_w_wideselect_soa_dispatch, abi_boundary_w_wideselect_soa_per_w, abi_boundary_w_wideselect_zig_runtime_w, abi_boundary_w_wideselect_scalar_per_w, abi_boundary_w_wideselect_scalar_dispatch, abi_boundary_w_wideselect_scalar_runtime_w, abi_boundary_w_wideselect_scalar_anchor} with a 34476% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 784.4x the fastest

Fastest abi_boundary_w_wideselect_null_entry (2.70 us) to slowest abi_boundary_w_wideselect_scalar_anchor (2.12 ms): 784.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_wideselect_null_entry** at 2702.7 ns median (-99.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 784.37x (fastest 2702.7 ns, slowest 2119923.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4975ns | 4982ns | 4889ns | 4959ns | 5043ns | -99.77% |
| abi_boundary_w_wideselect_scalar_anchor | 2124660ns | 2124002ns | 2096798ns | 2121594ns | 2143191ns | +0.08% |
| abi_boundary_w_wideselect_scalar_dispatch | 2123846ns | 2120269ns | 2109735ns | 2118354ns | 2139139ns | +0.04% |
| abi_boundary_w_wideselect_scalar_per_w | 2110461ns | 2106302ns | 2098749ns | 2105005ns | 2124500ns | -0.59% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2122940ns | 2120372ns | 2107218ns | 2117070ns | 2139605ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 950818ns | 950954ns | 944820ns | 949323ns | 956060ns | -55.21% |
| abi_boundary_w_wideselect_soa_per_w | 954274ns | 956759ns | 943825ns | 953787ns | 960231ns | -55.05% |
| abi_boundary_w_wideselect_soa_runtime_w | 942008ns | 937654ns | 934825ns | 937460ns | 952421ns | -55.63% |
| abi_boundary_w_wideselect_zig_runtime_w | 2039435ns | 2039192ns | 2023690ns | 2037204ns | 2050653ns | -3.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 2708ns | 2654ns | 2746ns | -99.87% | 0.047 |
| abi_boundary_w_wideselect_scalar_anchor | 2120796ns | 2093365ns | 2139328ns | +0.09% | 0.000 |
| abi_boundary_w_wideselect_scalar_dispatch | 2119942ns | 2105908ns | 2135363ns | +0.05% | 0.000 |
| abi_boundary_w_wideselect_scalar_per_w | 2106708ns | 2095200ns | 2120673ns | -0.57% | 0.000 |
| abi_boundary_w_wideselect_scalar_runtime_w | 2118885ns | 2103647ns | 2135392ns | base | 0.000 |
| abi_boundary_w_wideselect_soa_dispatch | 947518ns | 941721ns | 952706ns | -55.28% | 0.000 |
| abi_boundary_w_wideselect_soa_per_w | 950957ns | 940747ns | 956757ns | -55.12% | 0.000 |
| abi_boundary_w_wideselect_soa_runtime_w | 938926ns | 932258ns | 949139ns | -55.69% | 0.000 |
| abi_boundary_w_wideselect_zig_runtime_w | 2035365ns | 2019858ns | 2046610ns | -3.94% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 28850.1 | 2754.5 | 2708.5 | n/a |
| abi_boundary_w_wideselect_scalar_anchor | 84094.4 | 2124906.2 | 2120796.5 | 4 |
| abi_boundary_w_wideselect_scalar_dispatch | 84297.5 | 2115365.0 | 2119942.0 | 8 |
| abi_boundary_w_wideselect_scalar_per_w | 87781.7 | 2107777.0 | 2106707.9 | n/a |
| abi_boundary_w_wideselect_scalar_runtime_w | 92242.7 | 2118461.5 | 2118885.4 | n/a |
| abi_boundary_w_wideselect_soa_dispatch | 60101.6 | 947843.4 | 947517.7 | n/a |
| abi_boundary_w_wideselect_soa_per_w | 66688.9 | 950170.1 | 950957.1 | n/a |
| abi_boundary_w_wideselect_soa_runtime_w | 59166.1 | 939029.2 | 938925.7 | n/a |
| abi_boundary_w_wideselect_zig_runtime_w | 285119.9 | 2037501.6 | 2035365.2 | 2 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.048 Gops/s** (abi_boundary_w_wideselect_null_entry; best 20% batches)
- Ops per call: 128

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | 0.047 | 98.2% |
| abi_boundary_w_wideselect_scalar_anchor | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_dispatch | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_per_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_scalar_runtime_w | 0.000 | 0.1% |
| abi_boundary_w_wideselect_soa_dispatch | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_per_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_soa_runtime_w | 0.000 | 0.3% |
| abi_boundary_w_wideselect_zig_runtime_w | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 4975ns | 4975ns | -99.77% |
| abi_boundary_w_wideselect_scalar_anchor | 2124660ns | 2124660ns | +0.08% |
| abi_boundary_w_wideselect_scalar_dispatch | 2123846ns | 2123846ns | +0.04% |
| abi_boundary_w_wideselect_scalar_per_w | 2110461ns | 2110461ns | -0.59% |
| abi_boundary_w_wideselect_scalar_runtime_w | 2122940ns | 2122940ns | base |
| abi_boundary_w_wideselect_soa_dispatch | 950818ns | 950818ns | -55.21% |
| abi_boundary_w_wideselect_soa_per_w | 954274ns | 954274ns | -55.05% |
| abi_boundary_w_wideselect_soa_runtime_w | 942008ns | 942008ns | -55.63% |
| abi_boundary_w_wideselect_zig_runtime_w | 2039435ns | 2039435ns | -3.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_wideselect_scalar_runtime_w | 2116290ns | base | --- | [2104974, 2135392] | --- | --- | --- | --- |
| abi_boundary_w_wideselect_null_entry | 2703ns | -2113610.0ns (-99.9%) | [-2132693, -2102228]ns | [2676, 2746] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_scalar_anchor | 2119924ns | no significant difference | [-27604, +32005]ns | [2103138, 2139328] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_scalar_dispatch | 2116120ns | no significant difference | [-24721, +27476]ns | [2108343, 2135363] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_wideselect_scalar_per_w | 2102524ns | no significant difference | [-32868, +6731]ns | [2096928, 2120673] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_wideselect_soa_dispatch | 947727ns | -1171363.6ns (-55.3%) | [-1189450, -1153290]ns | [942120, 952706] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_per_w | 953374ns | -1167474.6ns (-55.2%) | [-1187072, -1149239]ns | [942741, 956757] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_soa_runtime_w | 934479ns | -1182110.0ns (-55.9%) | [-1200912, -1156856]ns | [933159, 949139] | YES (adj: no) | 0.0500 | 0.0313 | 0 |
| abi_boundary_w_wideselect_zig_runtime_w | 2035036ns | -80515.2ns (-3.8%) | [-110942, -59103]ns | [2024449, 2046610] | YES (adj: no) | 0.0500 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_wideselect_scalar_runtime_w | abi_boundary_w_wideselect_null_entry | abi_boundary_w_wideselect_scalar_anchor | abi_boundary_w_wideselect_scalar_dispatch | abi_boundary_w_wideselect_scalar_per_w | abi_boundary_w_wideselect_soa_dispatch | abi_boundary_w_wideselect_soa_per_w | abi_boundary_w_wideselect_soa_runtime_w | abi_boundary_w_wideselect_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2127968ns | -99.9% | -1.6% | -0.6% | -1.3% | -55.7% | -55.3% | -56.1% | -5.1% |
| 2 | 2108344ns | -99.9% | +1.1% | +0.1% | -0.6% | -54.9% | -54.6% | -55.0% | -3.6% |
| 3 | 2142816ns | -99.9% | -1.0% | -1.7% | -1.7% | -55.7% | -55.9% | -56.4% | -5.3% |
| 4 | 2106302ns | -99.9% | +1.9% | +0.5% | -0.4% | -55.3% | -55.3% | -55.7% | -2.5% |
| 5 | 2124236ns | -99.9% | -0.3% | -0.1% | -0.3% | -55.5% | -55.1% | -56.0% | -4.0% |
| 6 | 2103647ns | -99.9% | +0.4% | +2.1% | +1.0% | -54.7% | -54.6% | -54.9% | -3.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_wideselect_null_entry | -0.722 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_scalar_anchor | -0.196 | ok |
| abi_boundary_w_wideselect_scalar_dispatch | 0.253 | moderate+ |
| abi_boundary_w_wideselect_scalar_per_w | 0.308 | moderate+ |
| abi_boundary_w_wideselect_scalar_runtime_w | -0.673 | HIGH- (thermal bounce) |
| abi_boundary_w_wideselect_soa_dispatch | -0.231 | moderate- |
| abi_boundary_w_wideselect_soa_per_w | 0.042 | ok |
| abi_boundary_w_wideselect_soa_runtime_w | -0.251 | moderate- |
| abi_boundary_w_wideselect_zig_runtime_w | 0.076 | ok |

**Consistency summary:**

- **abi_boundary_w_wideselect_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_scalar_anchor**: won 3/6, lost 3/6
- **abi_boundary_w_wideselect_scalar_dispatch**: won 2/6, lost 3/6
- **abi_boundary_w_wideselect_scalar_per_w**: won 5/6, lost 1/6
- **abi_boundary_w_wideselect_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_per_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_soa_runtime_w**: won 6/6, lost 0/6
- **abi_boundary_w_wideselect_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_wideselect_null_entry | 120936.8ns | 2708.5ns | 4465.2% | HIGH |
| abi_boundary_w_wideselect_scalar_anchor | 6458758.3ns | 2120796.5ns | 304.5% | HIGH |
| abi_boundary_w_wideselect_scalar_dispatch | 6443435.4ns | 2119942.0ns | 303.9% | HIGH |
| abi_boundary_w_wideselect_scalar_per_w | 6423474.1ns | 2106707.9ns | 304.9% | HIGH |
| abi_boundary_w_wideselect_scalar_runtime_w | 6448865.7ns | 2118885.4ns | 304.4% | HIGH |
| abi_boundary_w_wideselect_soa_dispatch | 2906255.0ns | 947517.7ns | 306.7% | HIGH |
| abi_boundary_w_wideselect_soa_per_w | 2918046.3ns | 950957.1ns | 306.9% | HIGH |
| abi_boundary_w_wideselect_soa_runtime_w | 2878489.5ns | 938925.7ns | 306.6% | HIGH |
| abi_boundary_w_wideselect_zig_runtime_w | 6497598.0ns | 2035365.2ns | 319.2% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_wideselect_null_entry (n=6, range 2653.7-2746.4 ns)
   2653.7 |####################
   2658.3 |
   2663.0 |
   2667.6 |
   2672.2 |
   2676.9 |
   2681.5 |
   2686.2 |
   2690.8 |
   2695.4 |########################################
   2700.1 |
   2704.7 |####################
   2709.3 |
   2714.0 |
   2718.6 |
   2723.3 |
   2727.9 |
   2732.5 |
   2737.2 |####################
   2741.8 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_anchor (n=6, range 2093365.4-2139327.9 ns)
  2093365.4 |########################################
  2095663.5 |
  2097961.6 |
  2100259.8 |
  2102557.9 |
  2104856.0 |
  2107154.1 |
  2109452.3 |
  2111750.4 |########################################
  2114048.5 |
  2116346.7 |########################################
  2118644.8 |
  2120942.9 |########################################
  2123241.0 |
  2125539.2 |
  2127837.3 |
  2130135.4 |########################################
  2132433.5 |
  2134731.7 |
  2137029.8 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_dispatch (n=6, range 2105908.3-2135363.1 ns)
  2105908.3 |########################################
  2107381.0 |
  2108853.8 |
  2110326.5 |########################################
  2111799.3 |
  2113272.0 |
  2114744.8 |########################################
  2116217.5 |########################################
  2117690.2 |
  2119163.0 |
  2120635.7 |
  2122108.5 |########################################
  2123581.2 |
  2125054.0 |
  2126526.7 |
  2127999.4 |
  2129472.2 |
  2130944.9 |
  2132417.7 |
  2133890.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_per_w (n=6, range 2095199.6-2120672.7 ns)
  2095199.6 |########################################
  2096473.3 |
  2097746.9 |########################################
  2099020.6 |########################################
  2100294.2 |
  2101567.9 |
  2102841.5 |
  2104115.2 |
  2105388.8 |########################################
  2106662.5 |
  2107936.2 |
  2109209.8 |
  2110483.5 |
  2111757.1 |
  2113030.8 |
  2114304.4 |
  2115578.1 |
  2116851.7 |########################################
  2118125.4 |
  2119399.0 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_scalar_runtime_w (n=6, range 2103646.7-2135391.8 ns)
  2103646.7 |########################################
  2105234.0 |########################################
  2106821.2 |########################################
  2108408.5 |
  2109995.7 |
  2111583.0 |
  2113170.2 |
  2114757.5 |
  2116344.8 |
  2117932.0 |
  2119519.3 |
  2121106.5 |
  2122693.8 |########################################
  2124281.0 |
  2125868.3 |
  2127455.6 |########################################
  2129042.8 |
  2130630.1 |
  2132217.3 |
  2133804.6 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_dispatch (n=6, range 941720.8-952705.6 ns)
  941720.8 |########################################
  942270.0 |########################################
  942819.3 |
  943368.5 |
  943917.8 |
  944467.0 |
  945016.2 |########################################
  945565.5 |
  946114.7 |
  946664.0 |
  947213.2 |
  947762.4 |
  948311.7 |
  948860.9 |
  949410.2 |
  949959.4 |########################################
  950508.6 |
  951057.9 |
  951607.1 |########################################
  952156.4 |
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_per_w (n=6, range 940747.1-956756.7 ns)
  940747.1 |########################################
  941547.6 |
  942348.1 |
  943148.5 |
  943949.0 |########################################
  944749.5 |
  945550.0 |
  946350.4 |
  947150.9 |
  947951.4 |
  948751.9 |
  949552.4 |
  950352.8 |
  951153.3 |########################################
  951953.8 |
  952754.3 |
  953554.7 |
  954355.2 |########################################
  955155.7 |
  955956.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_wideselect_soa_runtime_w (n=6, range 932257.9-949138.8 ns)
  932257.9 |#############
  933101.9 |
  933946.0 |########################################
  934790.0 |
  935634.1 |
  936478.1 |
  937322.2 |
  938166.2 |
  939010.2 |
  939854.3 |
  940698.3 |
  941542.4 |
  942386.4 |
  943230.5 |
  944074.5 |
  944918.5 |
  945762.6 |
  946606.6 |
  947450.7 |
  948294.7 |#############
  (0 below, 1 above range)

abi_boundary_w_wideselect_zig_runtime_w (n=6, range 2019857.5-2046610.2 ns)
  2019857.5 |########################################
  2021195.1 |
  2022532.8 |
  2023870.4 |
  2025208.0 |
  2026545.7 |
  2027883.3 |########################################
  2029220.9 |
  2030558.6 |
  2031896.2 |########################################
  2033233.9 |
  2034571.5 |
  2035909.1 |
  2037246.8 |########################################
  2038584.4 |########################################
  2039922.0 |
  2041259.7 |
  2042597.3 |
  2043934.9 |
  2045272.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_wideselect_null_entry**: bridge=4471.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_anchor**: bridge=303.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_dispatch**: bridge=304.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_per_w**: bridge=305.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_scalar_runtime_w**: bridge=304.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_dispatch**: bridge=307.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_per_w**: bridge=306.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_soa_runtime_w**: bridge=307.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_wideselect_zig_runtime_w**: bridge=319.3% of algo (FFI overhead may distort results)

# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 50107% faster than the next best (abi_boundary_w_tight_zig_runtime_w)

abi_boundary_w_tight_null_entry (4.04 us) leads abi_boundary_w_tight_zig_runtime_w (2.03 ms) by 50107%, a clear separation rather than a photo finish. CV 2.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.03 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 509.1x slower than the field

abi_boundary_w_tight_scalar_anchor (2.06 ms) is 509.1x the fastest (4.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_soa_dispatch shows alternating (throttle bounce) (autocorr -0.54)

abi_boundary_w_tight_soa_dispatch's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_scalar_anchor} (50107% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_scalar_anchor} with a 50107% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 509.1x the fastest

Fastest abi_boundary_w_tight_null_entry (4.04 us) to slowest abi_boundary_w_tight_scalar_anchor (2.06 ms): 509.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 4039.1 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 509.10x (fastest 4039.1 ns, slowest 2056333.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 6388ns | 6319ns | 6182ns | 6282ns | 6651ns | -99.69% |
| abi_boundary_w_tight_scalar_anchor | 2058607ns | 2059385ns | 2047458ns | 2058922ns | 2063708ns | +0.66% |
| abi_boundary_w_tight_scalar_dispatch | 2044321ns | 2043786ns | 2036751ns | 2043022ns | 2050054ns | -0.04% |
| abi_boundary_w_tight_scalar_per_w | 2050213ns | 2050429ns | 2040572ns | 2048044ns | 2058287ns | +0.25% |
| abi_boundary_w_tight_scalar_runtime_w | 2045134ns | 2040884ns | 2032919ns | 2038279ns | 2061524ns | base |
| abi_boundary_w_tight_soa_dispatch | 2043531ns | 2042257ns | 2035248ns | 2040351ns | 2052443ns | -0.08% |
| abi_boundary_w_tight_soa_per_w | 2052285ns | 2051075ns | 2045050ns | 2050283ns | 2058905ns | +0.35% |
| abi_boundary_w_tight_soa_runtime_w | 2050340ns | 2043456ns | 2041009ns | 2042861ns | 2066225ns | +0.25% |
| abi_boundary_w_tight_zig_runtime_w | 2032374ns | 2031611ns | 2017146ns | 2029560ns | 2044209ns | -0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 4057ns | 3968ns | 4152ns | -99.80% | 0.001 |
| abi_boundary_w_tight_scalar_anchor | 2055339ns | 2044258ns | 2060386ns | +0.67% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2041183ns | 2033891ns | 2046766ns | -0.02% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2046945ns | 2037462ns | 2054910ns | +0.26% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2041639ns | 2029824ns | 2057506ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 2040282ns | 2032121ns | 2049014ns | -0.07% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 2049011ns | 2041990ns | 2055599ns | +0.36% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 2046940ns | 2037855ns | 2062499ns | +0.26% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2028825ns | 2014153ns | 2040457ns | -0.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 28828.2 | 4219.9 | 4057.4 | n/a |
| abi_boundary_w_tight_scalar_anchor | 60503.9 | 2052144.8 | 2055339.1 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 56171.8 | 2040159.8 | 2041182.8 | n/a |
| abi_boundary_w_tight_scalar_per_w | 64818.2 | 2045354.8 | 2046945.1 | 0 |
| abi_boundary_w_tight_scalar_runtime_w | 67085.7 | 2043364.0 | 2041639.3 | n/a |
| abi_boundary_w_tight_soa_dispatch | 61935.8 | 2039771.0 | 2040282.5 | n/a |
| abi_boundary_w_tight_soa_per_w | 67396.6 | 2045518.8 | 2049010.7 | n/a |
| abi_boundary_w_tight_soa_runtime_w | 65325.6 | 2045764.4 | 2046940.4 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 239718.3 | 2030780.5 | 2028824.7 | 13 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.001 | 98.2% |
| abi_boundary_w_tight_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_tight_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_tight_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_tight_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_tight_null_entry | 6388ns | 6388ns | -99.69% |
| abi_boundary_w_tight_scalar_anchor | 2058607ns | 2058607ns | +0.66% |
| abi_boundary_w_tight_scalar_dispatch | 2044321ns | 2044321ns | -0.04% |
| abi_boundary_w_tight_scalar_per_w | 2050213ns | 2050213ns | +0.25% |
| abi_boundary_w_tight_scalar_runtime_w | 2045134ns | 2045134ns | base |
| abi_boundary_w_tight_soa_dispatch | 2043531ns | 2043531ns | -0.08% |
| abi_boundary_w_tight_soa_per_w | 2052285ns | 2052285ns | +0.35% |
| abi_boundary_w_tight_soa_runtime_w | 2050340ns | 2050340ns | +0.25% |
| abi_boundary_w_tight_zig_runtime_w | 2032374ns | 2032374ns | -0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2037557ns | base | --- | [2029855, 2057506] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 4039ns | -2033552.8ns (-99.8%) | [-2053418, -2025774]ns | [3981, 4152] | YES (adj: no) | 0.2500 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2056334ns | +15502.9ns (+0.8%) | [+1566, +24031]ns | [2049298, 2060386] | YES (adj: no) | 0.4375 | 0.2188 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2040682ns | no significant difference | [-18380, +15055]ns | [2036101, 2046766] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2047169ns | no significant difference | [-10337, +21293]ns | [2038757, 2054910] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_dispatch | 2039096ns | no significant difference | [-8492, +8370]ns | [2032738, 2049014] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_per_w | 2047765ns | no significant difference | [-13176, +21661]ns | [2043667, 2055599] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 2040188ns | no significant difference | [-9239, +21456]ns | [2038135, 2062499] | no | 0.4375 | 0.2188 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2027936ns | -8452.7ns (-0.4%) | [-27722, -2269]ns | [2018081, 2040457] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2037705ns | -99.8% | +0.8% | +0.4% | +0.1% | -0.2% | +1.2% | +0.0% | -1.2% |
| 2 | 2057380ns | -99.8% | +0.2% | -0.9% | -0.6% | -0.4% | -0.5% | -0.9% | -0.4% |
| 3 | 2037410ns | -99.8% | +0.9% | -0.2% | +0.8% | -0.2% | +0.6% | +0.2% | -0.2% |
| 4 | 2057631ns | -99.8% | -0.0% | -0.9% | -0.4% | -0.4% | -0.8% | +0.2% | -1.5% |
| 5 | 2029824ns | -99.8% | +1.5% | +0.9% | +1.3% | +0.7% | +0.9% | +0.4% | +0.0% |
| 6 | 2029886ns | -99.8% | +0.7% | +0.6% | +0.4% | +0.1% | +0.8% | +1.7% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.413 | moderate- |
| abi_boundary_w_tight_scalar_anchor | -0.250 | moderate- |
| abi_boundary_w_tight_scalar_dispatch | 0.087 | ok |
| abi_boundary_w_tight_scalar_per_w | -0.227 | moderate- |
| abi_boundary_w_tight_scalar_runtime_w | -0.302 | moderate- |
| abi_boundary_w_tight_soa_dispatch | -0.539 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_soa_per_w | -0.129 | ok |
| abi_boundary_w_tight_soa_runtime_w | -0.298 | moderate- |
| abi_boundary_w_tight_zig_runtime_w | -0.356 | moderate- |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 0/6, lost 5/6
- **abi_boundary_w_tight_scalar_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_tight_scalar_per_w**: won 2/6, lost 4/6
- **abi_boundary_w_tight_soa_dispatch**: won 4/6, lost 2/6
- **abi_boundary_w_tight_soa_per_w**: won 2/6, lost 4/6
- **abi_boundary_w_tight_soa_runtime_w**: won 1/6, lost 4/6
- **abi_boundary_w_tight_zig_runtime_w**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 124360.8ns | 4057.4ns | 3065.0% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6221022.0ns | 2055339.1ns | 302.7% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6181650.3ns | 2041182.8ns | 302.8% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6207633.0ns | 2046945.1ns | 303.3% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6203004.9ns | 2041639.3ns | 303.8% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 6182819.2ns | 2040282.5ns | 303.0% | HIGH |
| abi_boundary_w_tight_soa_per_w | 6210208.0ns | 2049010.7ns | 303.1% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 6203485.2ns | 2046940.4ns | 303.1% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6411892.1ns | 2028824.7ns | 316.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 3968.3-4152.5 ns)
   3968.3 |########################################
   3977.5 |
   3986.7 |########################################
   3995.9 |
   4005.1 |
   4014.4 |########################################
   4023.6 |
   4032.8 |
   4042.0 |
   4051.2 |
   4060.4 |########################################
   4069.6 |
   4078.8 |
   4088.0 |
   4097.2 |########################################
   4106.4 |
   4115.7 |
   4124.9 |
   4134.1 |
   4143.3 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2044257.9-2060385.6 ns)
  2044257.9 |########################################
  2045064.3 |
  2045870.7 |
  2046677.1 |
  2047483.4 |
  2048289.8 |
  2049096.2 |
  2049902.6 |
  2050709.0 |
  2051515.4 |
  2052321.8 |
  2053128.1 |
  2053934.5 |########################################
  2054740.9 |
  2055547.3 |########################################
  2056353.7 |########################################
  2057160.1 |
  2057966.4 |
  2058772.8 |
  2059579.2 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2033890.8-2046765.6 ns)
  2033890.8 |########################################
  2034534.5 |
  2035178.3 |
  2035822.0 |
  2036465.8 |
  2037109.5 |
  2037753.3 |########################################
  2038397.0 |
  2039040.7 |
  2039684.5 |########################################
  2040328.2 |
  2040972.0 |########################################
  2041615.7 |
  2042259.5 |
  2042903.2 |
  2043546.9 |
  2044190.7 |
  2044834.4 |########################################
  2045478.2 |
  2046121.9 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2037462.5-2054909.6 ns)
  2037462.5 |########################################
  2038334.9 |
  2039207.2 |########################################
  2040079.6 |
  2040951.9 |
  2041824.3 |
  2042696.6 |
  2043569.0 |
  2044441.3 |
  2045313.7 |########################################
  2046186.1 |
  2047058.4 |
  2047930.8 |########################################
  2048803.1 |
  2049675.5 |
  2050547.8 |
  2051420.2 |
  2052292.5 |
  2053164.9 |########################################
  2054037.2 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2029823.7-2057505.6 ns)
  2029823.7 |########################################
  2031207.8 |
  2032591.9 |
  2033976.0 |
  2035360.1 |
  2036744.2 |########################################
  2038128.3 |
  2039512.4 |
  2040896.5 |
  2042280.6 |
  2043664.6 |
  2045048.7 |
  2046432.8 |
  2047816.9 |
  2049201.0 |
  2050585.1 |
  2051969.2 |
  2053353.3 |
  2054737.4 |
  2056121.5 |####################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 2032121.2-2049013.8 ns)
  2032121.2 |########################################
  2032965.8 |########################################
  2033810.5 |########################################
  2034655.1 |
  2035499.7 |
  2036344.3 |
  2037189.0 |
  2038033.6 |
  2038878.2 |
  2039722.8 |
  2040567.5 |
  2041412.1 |
  2042256.7 |
  2043101.4 |
  2043946.0 |########################################
  2044790.6 |
  2045635.2 |
  2046479.9 |
  2047324.5 |
  2048169.1 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 2041989.6-2055599.4 ns)
  2041989.6 |####################
  2042670.1 |
  2043350.6 |
  2044031.1 |
  2044711.6 |####################
  2045392.1 |
  2046072.5 |####################
  2046753.0 |
  2047433.5 |
  2048114.0 |
  2048794.5 |########################################
  2049475.0 |
  2050155.5 |
  2050836.0 |
  2051516.5 |
  2052196.9 |
  2052877.4 |
  2053557.9 |
  2054238.4 |
  2054918.9 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 2037855.0-2062498.9 ns)
  2037855.0 |########################################
  2039087.2 |
  2040319.4 |
  2041551.6 |#############
  2042783.8 |
  2044016.0 |
  2045248.2 |
  2046480.4 |
  2047712.6 |
  2048944.8 |
  2050177.0 |
  2051409.2 |
  2052641.4 |
  2053873.6 |
  2055105.8 |
  2056338.0 |
  2057570.2 |
  2058802.4 |
  2060034.6 |#############
  2061266.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2014153.3-2040457.3 ns)
  2014153.3 |########################################
  2015468.5 |
  2016783.7 |
  2018098.9 |
  2019414.1 |
  2020729.3 |########################################
  2022044.5 |
  2023359.7 |
  2024674.9 |########################################
  2025990.1 |
  2027305.3 |
  2028620.5 |
  2029935.7 |########################################
  2031250.9 |########################################
  2032566.1 |
  2033881.3 |
  2035196.5 |
  2036511.7 |
  2037826.9 |
  2039142.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=3061.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=304.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=315.8% of algo (FFI overhead may distort results)

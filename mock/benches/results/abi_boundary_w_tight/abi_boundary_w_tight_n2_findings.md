# abi_boundary_w (tight)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_tight_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_tight_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_boundary_w_tight_null_entry dominates: 59868% faster than the next best (abi_boundary_w_tight_zig_runtime_w)

abi_boundary_w_tight_null_entry (3.39 us) leads abi_boundary_w_tight_zig_runtime_w (2.03 ms) by 59868%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_tight_null_entry beats baseline by 100% (significant)

abi_boundary_w_tight_null_entry is -2.04 ms (100%) faster than baseline abi_boundary_w_tight_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_tight_scalar_anchor is an outlier: 607.0x slower than the field

abi_boundary_w_tight_scalar_anchor (2.06 ms) is 607.0x the fastest (3.39 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_tight_scalar_anchor shows alternating (throttle bounce) (autocorr -0.55)

abi_boundary_w_tight_scalar_anchor's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_tight_null_entry} vs {abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} (59868% apart)

The field splits into a fast tier {abi_boundary_w_tight_null_entry} and a slow tier {abi_boundary_w_tight_zig_runtime_w, abi_boundary_w_tight_scalar_per_w, abi_boundary_w_tight_scalar_runtime_w, abi_boundary_w_tight_soa_dispatch, abi_boundary_w_tight_soa_per_w, abi_boundary_w_tight_soa_runtime_w, abi_boundary_w_tight_scalar_dispatch, abi_boundary_w_tight_scalar_anchor} with a 59868% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 607.0x the fastest

Fastest abi_boundary_w_tight_null_entry (3.39 us) to slowest abi_boundary_w_tight_scalar_anchor (2.06 ms): 607.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_tight_null_entry** at 3389.6 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 606.96x (fastest 3389.6 ns, slowest 2057364.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 5624ns | 5621ns | 5572ns | 5612ns | 5669ns | -99.73% |
| abi_boundary_w_tight_scalar_anchor | 2060469ns | 2060407ns | 2047129ns | 2056388ns | 2073261ns | +0.58% |
| abi_boundary_w_tight_scalar_dispatch | 2051356ns | 2050497ns | 2045803ns | 2049776ns | 2056502ns | +0.14% |
| abi_boundary_w_tight_scalar_per_w | 2043482ns | 2043355ns | 2032312ns | 2042086ns | 2051162ns | -0.25% |
| abi_boundary_w_tight_scalar_runtime_w | 2048518ns | 2046310ns | 2042013ns | 2045863ns | 2055753ns | base |
| abi_boundary_w_tight_soa_dispatch | 2050784ns | 2048001ns | 2033825ns | 2044986ns | 2067960ns | +0.11% |
| abi_boundary_w_tight_soa_per_w | 2048104ns | 2048809ns | 2041487ns | 2048054ns | 2051488ns | -0.02% |
| abi_boundary_w_tight_soa_runtime_w | 2049547ns | 2049392ns | 2039352ns | 2047094ns | 2058323ns | +0.05% |
| abi_boundary_w_tight_zig_runtime_w | 2035819ns | 2035827ns | 2027328ns | 2033886ns | 2042964ns | -0.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 3394ns | 3371ns | 3414ns | -99.83% | 0.001 |
| abi_boundary_w_tight_scalar_anchor | 2057220ns | 2044275ns | 2069683ns | +0.58% | 0.000 |
| abi_boundary_w_tight_scalar_dispatch | 2047957ns | 2042320ns | 2053033ns | +0.12% | 0.000 |
| abi_boundary_w_tight_scalar_per_w | 2040436ns | 2029713ns | 2047752ns | -0.24% | 0.000 |
| abi_boundary_w_tight_scalar_runtime_w | 2045426ns | 2038879ns | 2052650ns | base | 0.000 |
| abi_boundary_w_tight_soa_dispatch | 2047610ns | 2031109ns | 2064494ns | +0.11% | 0.000 |
| abi_boundary_w_tight_soa_per_w | 2044804ns | 2037930ns | 2048198ns | -0.03% | 0.000 |
| abi_boundary_w_tight_soa_runtime_w | 2046366ns | 2036224ns | 2054884ns | +0.05% | 0.000 |
| abi_boundary_w_tight_zig_runtime_w | 2032481ns | 2024300ns | 2039399ns | -0.63% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 27273.9 | 3447.2 | 3393.7 | n/a |
| abi_boundary_w_tight_scalar_anchor | 63127.6 | 2058529.1 | 2057220.4 | n/a |
| abi_boundary_w_tight_scalar_dispatch | 63760.7 | 2051267.6 | 2047957.2 | n/a |
| abi_boundary_w_tight_scalar_per_w | 56954.1 | 2041225.1 | 2040436.1 | 0 |
| abi_boundary_w_tight_scalar_runtime_w | 56551.0 | 2044704.6 | 2045426.1 | n/a |
| abi_boundary_w_tight_soa_dispatch | 56447.3 | 2043571.2 | 2047610.4 | 0 |
| abi_boundary_w_tight_soa_per_w | 64704.4 | 2047319.2 | 2044804.4 | 13 |
| abi_boundary_w_tight_soa_runtime_w | 60678.8 | 2049203.0 | 2046365.9 | n/a |
| abi_boundary_w_tight_zig_runtime_w | 226511.0 | 2034841.4 | 2032480.7 | 13 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_tight_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_tight_null_entry | 0.001 | 99.5% |
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
| abi_boundary_w_tight_null_entry | 5624ns | 5624ns | -99.73% |
| abi_boundary_w_tight_scalar_anchor | 2060469ns | 2060469ns | +0.58% |
| abi_boundary_w_tight_scalar_dispatch | 2051356ns | 2051356ns | +0.14% |
| abi_boundary_w_tight_scalar_per_w | 2043482ns | 2043482ns | -0.25% |
| abi_boundary_w_tight_scalar_runtime_w | 2048518ns | 2048518ns | base |
| abi_boundary_w_tight_soa_dispatch | 2050784ns | 2050784ns | +0.11% |
| abi_boundary_w_tight_soa_per_w | 2048104ns | 2048104ns | -0.02% |
| abi_boundary_w_tight_soa_runtime_w | 2049547ns | 2049547ns | +0.05% |
| abi_boundary_w_tight_zig_runtime_w | 2035819ns | 2035819ns | -0.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_tight_scalar_runtime_w | 2043195ns | base | --- | [2040433, 2052650] | --- | --- | --- | --- |
| abi_boundary_w_tight_null_entry | 3390ns | -2039816.5ns (-99.8%) | [-2049256, -2037025]ns | [3378, 3414] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_tight_scalar_anchor | 2057365ns | no significant difference | [-7322, +25882]ns | [2044613, 2069683] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_tight_scalar_dispatch | 2047198ns | no significant difference | [-7744, +9930]ns | [2043640, 2053033] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_tight_scalar_per_w | 2040507ns | no significant difference | [-13674, +1492]ns | [2033050, 2047752] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_tight_soa_dispatch | 2044898ns | no significant difference | [-8332, +14047]ns | [2033440, 2064494] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_soa_per_w | 2045549ns | no significant difference | [-10155, +5239]ns | [2040666, 2048198] | no | 0.2917 | 0.2188 | 0 |
| abi_boundary_w_tight_soa_runtime_w | 2046292ns | no significant difference | [-7692, +10974]ns | [2037922, 2054884] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_tight_zig_runtime_w | 2032662ns | -7771.0ns (-0.4%) | [-26555, -4511]ns | [2025381, 2039399] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_tight_scalar_runtime_w | abi_boundary_w_tight_null_entry | abi_boundary_w_tight_scalar_anchor | abi_boundary_w_tight_scalar_dispatch | abi_boundary_w_tight_scalar_per_w | abi_boundary_w_tight_soa_dispatch | abi_boundary_w_tight_soa_per_w | abi_boundary_w_tight_soa_runtime_w | abi_boundary_w_tight_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2045646ns | -99.8% | +1.1% | +0.1% | +0.1% | +0.9% | +0.1% | +0.6% | -0.1% |
| 2 | 2042173ns | -99.8% | +0.6% | +0.2% | -0.0% | +0.4% | +0.2% | +0.5% | -0.3% |
| 3 | 2041988ns | -99.8% | +1.4% | +0.3% | -0.2% | -0.5% | +0.1% | -0.3% | -0.4% |
| 4 | 2059653ns | -99.8% | -0.7% | -0.8% | -0.6% | +0.2% | -1.1% | -0.5% | -1.6% |
| 5 | 2038879ns | -99.8% | +1.1% | +0.3% | -0.1% | -0.2% | +0.3% | +0.2% | -0.4% |
| 6 | 2044218ns | -99.8% | +0.0% | +0.7% | -0.7% | -0.3% | +0.2% | -0.2% | -1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_tight_null_entry | -0.109 | ok |
| abi_boundary_w_tight_scalar_anchor | -0.549 | HIGH- (thermal bounce) |
| abi_boundary_w_tight_scalar_dispatch | -0.103 | ok |
| abi_boundary_w_tight_scalar_per_w | 0.078 | ok |
| abi_boundary_w_tight_scalar_runtime_w | -0.459 | moderate- |
| abi_boundary_w_tight_soa_dispatch | -0.339 | moderate- |
| abi_boundary_w_tight_soa_per_w | 0.158 | ok |
| abi_boundary_w_tight_soa_runtime_w | -0.053 | ok |
| abi_boundary_w_tight_zig_runtime_w | 0.176 | ok |

**Consistency summary:**

- **abi_boundary_w_tight_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_tight_scalar_anchor**: won 1/6, lost 4/6
- **abi_boundary_w_tight_scalar_dispatch**: won 1/6, lost 4/6
- **abi_boundary_w_tight_scalar_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_tight_soa_dispatch**: won 3/6, lost 3/6
- **abi_boundary_w_tight_soa_per_w**: won 1/6, lost 3/6
- **abi_boundary_w_tight_soa_runtime_w**: won 3/6, lost 3/6
- **abi_boundary_w_tight_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_tight_null_entry | 120097.8ns | 3393.7ns | 3538.9% | HIGH |
| abi_boundary_w_tight_scalar_anchor | 6241138.1ns | 2057220.4ns | 303.4% | HIGH |
| abi_boundary_w_tight_scalar_dispatch | 6220054.6ns | 2047957.2ns | 303.7% | HIGH |
| abi_boundary_w_tight_scalar_per_w | 6184953.6ns | 2040436.1ns | 303.1% | HIGH |
| abi_boundary_w_tight_scalar_runtime_w | 6192914.3ns | 2045426.1ns | 302.8% | HIGH |
| abi_boundary_w_tight_soa_dispatch | 6194594.9ns | 2047610.4ns | 302.5% | HIGH |
| abi_boundary_w_tight_soa_per_w | 6209460.3ns | 2044804.4ns | 303.7% | HIGH |
| abi_boundary_w_tight_soa_runtime_w | 6213225.9ns | 2046365.9ns | 303.6% | HIGH |
| abi_boundary_w_tight_zig_runtime_w | 6409677.4ns | 2032480.7ns | 315.4% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_tight_null_entry (n=6, range 3371.2-3413.8 ns)
   3371.2 |########################################
   3373.3 |
   3375.5 |
   3377.6 |
   3379.7 |
   3381.8 |
   3384.0 |########################################
   3386.1 |########################################
   3388.2 |
   3390.3 |
   3392.5 |########################################
   3394.6 |
   3396.7 |
   3398.9 |
   3401.0 |
   3403.1 |########################################
   3405.2 |
   3407.4 |
   3409.5 |
   3411.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_anchor (n=6, range 2044275.0-2069683.3 ns)
  2044275.0 |########################################
  2045545.4 |
  2046815.8 |
  2048086.2 |
  2049356.7 |
  2050627.1 |
  2051897.5 |
  2053167.9 |####################
  2054438.3 |
  2055708.7 |
  2056979.1 |
  2058249.6 |
  2059520.0 |
  2060790.4 |####################
  2062060.8 |
  2063331.2 |
  2064601.6 |
  2065872.1 |
  2067142.5 |####################
  2068412.9 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_dispatch (n=6, range 2042320.4-2053033.1 ns)
  2042320.4 |########################################
  2042856.0 |
  2043391.7 |
  2043927.3 |
  2044462.9 |########################################
  2044998.6 |
  2045534.2 |
  2046069.8 |
  2046605.5 |########################################
  2047141.1 |########################################
  2047676.8 |
  2048212.4 |########################################
  2048748.0 |
  2049283.7 |
  2049819.3 |
  2050354.9 |
  2050890.6 |
  2051426.2 |
  2051961.8 |
  2052497.5 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_per_w (n=6, range 2029712.9-2047751.7 ns)
  2029712.9 |########################################
  2030614.8 |
  2031516.8 |
  2032418.7 |
  2033320.7 |
  2034222.6 |
  2035124.5 |
  2036026.5 |########################################
  2036928.4 |
  2037830.4 |
  2038732.3 |########################################
  2039634.2 |
  2040536.2 |
  2041438.1 |########################################
  2042340.1 |
  2043242.0 |
  2044143.9 |
  2045045.9 |
  2045947.8 |########################################
  2046849.8 |
  (0 below, 1 above range)

abi_boundary_w_tight_scalar_runtime_w (n=6, range 2038878.7-2052649.5 ns)
  2038878.7 |####################
  2039567.2 |
  2040255.8 |
  2040944.3 |
  2041632.9 |########################################
  2042321.4 |
  2043010.0 |
  2043698.5 |####################
  2044387.0 |
  2045075.6 |####################
  2045764.1 |
  2046452.7 |
  2047141.2 |
  2047829.8 |
  2048518.3 |
  2049206.8 |
  2049895.4 |
  2050583.9 |
  2051272.5 |
  2051961.0 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_dispatch (n=6, range 2031109.2-2064493.9 ns)
  2031109.2 |########################################
  2032778.4 |
  2034447.7 |########################################
  2036116.9 |
  2037786.1 |########################################
  2039455.4 |
  2041124.6 |
  2042793.9 |
  2044463.1 |
  2046132.3 |
  2047801.6 |
  2049470.8 |
  2051140.1 |########################################
  2052809.3 |
  2054478.5 |
  2056147.8 |
  2057817.0 |
  2059486.2 |
  2061155.5 |
  2062824.7 |########################################
  (0 below, 1 above range)

abi_boundary_w_tight_soa_per_w (n=6, range 2037929.6-2048198.1 ns)
  2037929.6 |########################################
  2038443.0 |
  2038956.5 |
  2039469.9 |
  2039983.3 |
  2040496.7 |
  2041010.2 |
  2041523.6 |
  2042037.0 |
  2042550.4 |
  2043063.9 |########################################
  2043577.3 |
  2044090.7 |
  2044604.2 |########################################
  2045117.6 |
  2045631.0 |
  2046144.4 |########################################
  2046657.9 |
  2047171.3 |########################################
  2047684.7 |
  (0 below, 1 above range)

abi_boundary_w_tight_soa_runtime_w (n=6, range 2036223.8-2054883.6 ns)
  2036223.8 |########################################
  2037156.8 |
  2038089.8 |
  2039022.8 |########################################
  2039955.8 |
  2040888.7 |
  2041821.7 |########################################
  2042754.7 |
  2043687.7 |
  2044620.7 |
  2045553.7 |
  2046486.7 |
  2047419.7 |
  2048352.6 |
  2049285.6 |########################################
  2050218.6 |
  2051151.6 |########################################
  2052084.6 |
  2053017.6 |
  2053950.6 |
  (0 below, 1 above range)

abi_boundary_w_tight_zig_runtime_w (n=6, range 2024299.6-2039399.1 ns)
  2024299.6 |########################################
  2025054.6 |
  2025809.6 |########################################
  2026564.5 |
  2027319.5 |
  2028074.5 |
  2028829.5 |
  2029584.4 |
  2030339.4 |
  2031094.4 |########################################
  2031849.4 |
  2032604.4 |
  2033359.3 |########################################
  2034114.3 |
  2034869.3 |########################################
  2035624.3 |
  2036379.2 |
  2037134.2 |
  2037889.2 |
  2038644.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_tight_null_entry**: bridge=3533.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_anchor**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_dispatch**: bridge=304.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_per_w**: bridge=302.6% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_scalar_runtime_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_dispatch**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_per_w**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_soa_runtime_w**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_tight_zig_runtime_w**: bridge=315.4% of algo (FFI overhead may distort results)

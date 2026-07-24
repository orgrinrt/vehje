# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_scatter_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_scatter_scalar_runtime_w has the worst median (2.17 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_scatter_null_entry at 3.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_scatter_null_entry dominates: 61513% faster than the next best (abi_boundary_w_scatter_zig_runtime_w)

abi_boundary_w_scatter_null_entry (3.48 us) leads abi_boundary_w_scatter_zig_runtime_w (2.14 ms) by 61513%, a clear separation rather than a photo finish. CV 5.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_runtime_w is an outlier: 623.2x slower than the field

abi_boundary_w_scatter_scalar_runtime_w (2.17 ms) is 623.2x the fastest (3.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_boundary_w_scatter_null_entry is fastest but the noisiest (CV 5.8%)

abi_boundary_w_scatter_null_entry wins on median (3.48 us) yet has the highest variance (CV 5.8%), while abi_boundary_w_scatter_soa_per_w is the steadiest (CV 0.1%, 2.16 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### abi_boundary_w_scatter_soa_runtime_w shows alternating (throttle bounce) (autocorr -0.55)

abi_boundary_w_scatter_soa_runtime_w's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_scalar_runtime_w} (61513% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_scalar_runtime_w} with a 61513% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 623.2x the fastest

Fastest abi_boundary_w_scatter_null_entry (3.48 us) to slowest abi_boundary_w_scatter_scalar_runtime_w (2.17 ms): 623.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 3477.8 ns median (-99.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 623.17x (fastest 3477.8 ns, slowest 2167237.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 5889ns | 5743ns | 5555ns | 5691ns | 6355ns | -99.73% |
| abi_boundary_w_scatter_scalar_anchor | 2162865ns | 2163114ns | 2151826ns | 2161619ns | 2170254ns | -0.29% |
| abi_boundary_w_scatter_scalar_dispatch | 2170162ns | 2168167ns | 2161130ns | 2167623ns | 2178486ns | +0.05% |
| abi_boundary_w_scatter_scalar_per_w | 2171813ns | 2167243ns | 2158508ns | 2165867ns | 2187385ns | +0.12% |
| abi_boundary_w_scatter_scalar_runtime_w | 2169106ns | 2170242ns | 2163165ns | 2167992ns | 2173746ns | base |
| abi_boundary_w_scatter_soa_dispatch | 2162184ns | 2162032ns | 2155651ns | 2160040ns | 2168668ns | -0.32% |
| abi_boundary_w_scatter_soa_per_w | 2163112ns | 2163440ns | 2156989ns | 2163036ns | 2166288ns | -0.28% |
| abi_boundary_w_scatter_soa_runtime_w | 2168199ns | 2169463ns | 2159190ns | 2167664ns | 2173508ns | -0.04% |
| abi_boundary_w_scatter_zig_runtime_w | 2144026ns | 2145822ns | 2132768ns | 2143062ns | 2151101ns | -1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 3550ns | 3350ns | 3803ns | -99.84% | 0.001 |
| abi_boundary_w_scatter_scalar_anchor | 2159865ns | 2148982ns | 2167109ns | -0.29% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2167130ns | 2158361ns | 2175361ns | +0.05% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2168736ns | 2155646ns | 2184126ns | +0.12% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2166134ns | 2160270ns | 2170807ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 2159276ns | 2152968ns | 2165693ns | -0.32% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 2160159ns | 2154348ns | 2163223ns | -0.28% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 2165295ns | 2156518ns | 2170508ns | -0.04% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2141024ns | 2129888ns | 2148075ns | -1.16% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 28484.5 | 3592.6 | 3549.8 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 51101.0 | 2158786.5 | 2159865.4 | 1 |
| abi_boundary_w_scatter_scalar_dispatch | 52300.6 | 2163856.7 | 2167129.8 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 54778.2 | 2171710.6 | 2168735.6 | n/a |
| abi_boundary_w_scatter_scalar_runtime_w | 47573.4 | 2166250.1 | 2166133.9 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 49242.3 | 2158622.8 | 2159276.0 | 0 |
| abi_boundary_w_scatter_soa_per_w | 51186.1 | 2159944.2 | 2160158.8 | 1 |
| abi_boundary_w_scatter_soa_runtime_w | 49162.2 | 2165733.7 | 2165295.1 | 2 |
| abi_boundary_w_scatter_zig_runtime_w | 205321.7 | 2139733.4 | 2141024.5 | 6 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.001 | 96.3% |
| abi_boundary_w_scatter_scalar_anchor | 0.000 | 0.2% |
| abi_boundary_w_scatter_scalar_dispatch | 0.000 | 0.2% |
| abi_boundary_w_scatter_scalar_per_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_scalar_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_dispatch | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_per_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_soa_runtime_w | 0.000 | 0.2% |
| abi_boundary_w_scatter_zig_runtime_w | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 5889ns | 5889ns | -99.73% |
| abi_boundary_w_scatter_scalar_anchor | 2162865ns | 2162865ns | -0.29% |
| abi_boundary_w_scatter_scalar_dispatch | 2170162ns | 2170162ns | +0.05% |
| abi_boundary_w_scatter_scalar_per_w | 2171813ns | 2171813ns | +0.12% |
| abi_boundary_w_scatter_scalar_runtime_w | 2169106ns | 2169106ns | base |
| abi_boundary_w_scatter_soa_dispatch | 2162184ns | 2162184ns | -0.32% |
| abi_boundary_w_scatter_soa_per_w | 2163112ns | 2163112ns | -0.28% |
| abi_boundary_w_scatter_soa_runtime_w | 2168199ns | 2168199ns | -0.04% |
| abi_boundary_w_scatter_zig_runtime_w | 2144026ns | 2144026ns | -1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2167238ns | base | --- | [2160357, 2170807] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 3478ns | -2163600.8ns (-99.8%) | [-2167402, -2156749]ns | [3368, 3803] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2160089ns | no significant difference | [-18408, +2771]ns | [2152398, 2167109] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2165073ns | no significant difference | [-9851, +11111]ns | [2160955, 2175361] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2164230ns | no significant difference | [-6996, +17809]ns | [2157850, 2184126] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 2159009ns | no significant difference | [-14822, +1767]ns | [2153126, 2165693] | no | 0.4375 | 0.2188 | 0 |
| abi_boundary_w_scatter_soa_per_w | 2160454ns | no significant difference | [-14007, +1916]ns | [2156800, 2163223] | no | 0.4375 | 0.2188 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 2166536ns | no significant difference | [-6189, +7129]ns | [2158842, 2170508] | no | 0.7857 | 0.6875 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2142741ns | -23665.0ns (-1.1%) | [-34059, -17604]ns | [2132258, 2148075] | YES (adj: no) | 0.1250 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2160270ns | -99.8% | -0.1% | +0.6% | +0.9% | -0.3% | -0.0% | -0.2% | -1.4% |
| 2 | 2160445ns | -99.8% | +0.1% | +0.2% | -0.2% | -0.0% | +0.2% | +0.6% | -0.8% |
| 3 | 2168230ns | -99.8% | +0.2% | +0.4% | -0.1% | -0.5% | -0.3% | -0.1% | -1.1% |
| 4 | 2172366ns | -99.8% | -1.1% | -0.6% | +0.8% | -0.9% | -0.6% | -0.2% | -1.7% |
| 5 | 2169248ns | -99.8% | -0.6% | -0.3% | -0.4% | -0.4% | -0.7% | -0.4% | -0.8% |
| 6 | 2166246ns | -99.8% | -0.2% | -0.1% | -0.2% | +0.2% | -0.2% | +0.1% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.027 | ok |
| abi_boundary_w_scatter_scalar_anchor | -0.224 | moderate- |
| abi_boundary_w_scatter_scalar_dispatch | -0.261 | moderate- |
| abi_boundary_w_scatter_scalar_per_w | -0.317 | moderate- |
| abi_boundary_w_scatter_scalar_runtime_w | 0.453 | moderate+ |
| abi_boundary_w_scatter_soa_dispatch | 0.058 | ok |
| abi_boundary_w_scatter_soa_per_w | -0.057 | ok |
| abi_boundary_w_scatter_soa_runtime_w | -0.550 | HIGH- (thermal bounce) |
| abi_boundary_w_scatter_zig_runtime_w | -0.316 | moderate- |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 3/6, lost 1/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 2/6, lost 3/6
- **abi_boundary_w_scatter_scalar_per_w**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_soa_dispatch**: won 4/6, lost 1/6
- **abi_boundary_w_scatter_soa_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 4/6, lost 1/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 121570.2ns | 3549.8ns | 3424.7% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6536800.0ns | 2159865.4ns | 302.6% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6550758.1ns | 2167129.8ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6567502.2ns | 2168735.6ns | 302.8% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6551354.3ns | 2166133.9ns | 302.4% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 6526103.2ns | 2159276.0ns | 302.2% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 6533297.0ns | 2160158.8ns | 302.4% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 6549927.4ns | 2165295.1ns | 302.5% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6701732.8ns | 2141024.5ns | 313.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 3350.0-3803.3 ns)
   3350.0 |########################################
   3372.7 |########################################
   3395.3 |
   3418.0 |########################################
   3440.7 |
   3463.3 |
   3486.0 |
   3508.7 |
   3531.3 |########################################
   3554.0 |
   3576.7 |
   3599.3 |
   3622.0 |
   3644.7 |
   3667.3 |########################################
   3690.0 |
   3712.7 |
   3735.3 |
   3758.0 |
   3780.7 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2148982.1-2167108.5 ns)
  2148982.1 |########################################
  2149888.4 |
  2150794.7 |
  2151701.1 |
  2152607.4 |
  2153513.7 |
  2154420.0 |
  2155326.4 |########################################
  2156232.7 |
  2157139.0 |
  2158045.3 |########################################
  2158951.6 |
  2159858.0 |
  2160764.3 |
  2161670.6 |########################################
  2162576.9 |########################################
  2163483.3 |
  2164389.6 |
  2165295.9 |
  2166202.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2158361.2-2175361.0 ns)
  2158361.2 |########################################
  2159211.2 |
  2160061.2 |
  2160911.2 |
  2161761.2 |
  2162611.2 |
  2163461.2 |########################################
  2164311.1 |########################################
  2165161.1 |########################################
  2166011.1 |
  2166861.1 |
  2167711.1 |
  2168561.1 |
  2169411.1 |
  2170261.1 |
  2171111.1 |
  2171961.1 |
  2172811.1 |
  2173661.1 |########################################
  2174511.1 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2155645.8-2184126.2 ns)
  2155645.8 |########################################
  2157069.8 |
  2158493.8 |
  2159917.9 |########################################
  2161341.9 |########################################
  2162765.9 |
  2164189.9 |
  2165614.0 |########################################
  2167038.0 |
  2168462.0 |
  2169886.0 |
  2171310.0 |
  2172734.1 |
  2174158.1 |
  2175582.1 |
  2177006.1 |
  2178430.2 |########################################
  2179854.2 |
  2181278.2 |
  2182702.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2160269.6-2170806.6 ns)
  2160269.6 |########################################
  2160796.5 |
  2161323.3 |
  2161850.2 |
  2162377.0 |
  2162903.9 |
  2163430.7 |
  2163957.6 |
  2164484.4 |
  2165011.3 |
  2165538.1 |
  2166065.0 |####################
  2166591.8 |
  2167118.7 |
  2167645.5 |
  2168172.4 |####################
  2168699.2 |
  2169226.1 |####################
  2169752.9 |
  2170279.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 2152967.5-2165693.2 ns)
  2152967.5 |########################################
  2153603.8 |
  2154240.1 |
  2154876.3 |
  2155512.6 |
  2156148.9 |
  2156785.2 |
  2157421.5 |####################
  2158057.8 |
  2158694.0 |
  2159330.3 |
  2159966.6 |####################
  2160602.9 |
  2161239.2 |####################
  2161875.5 |
  2162511.7 |
  2163148.0 |
  2163784.3 |
  2164420.6 |
  2165056.9 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 2154347.9-2163223.4 ns)
  2154347.9 |########################################
  2154791.7 |
  2155235.4 |
  2155679.2 |
  2156123.0 |
  2156566.8 |
  2157010.5 |
  2157454.3 |
  2157898.1 |
  2158341.9 |
  2158785.6 |
  2159229.4 |########################################
  2159673.2 |########################################
  2160116.9 |
  2160560.7 |########################################
  2161004.5 |
  2161448.3 |
  2161892.0 |########################################
  2162335.8 |
  2162779.6 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 2156517.5-2170507.5 ns)
  2156517.5 |####################
  2157217.0 |
  2157916.5 |
  2158616.0 |
  2159315.5 |
  2160015.0 |
  2160714.5 |####################
  2161414.0 |
  2162113.5 |
  2162813.0 |
  2163512.5 |
  2164212.0 |
  2164911.5 |####################
  2165611.0 |
  2166310.5 |
  2167010.0 |
  2167709.5 |########################################
  2168409.0 |
  2169108.5 |
  2169808.0 |
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2129887.9-2148074.6 ns)
  2129887.9 |########################################
  2130797.2 |
  2131706.6 |
  2132615.9 |
  2133525.2 |
  2134434.6 |########################################
  2135343.9 |
  2136253.2 |
  2137162.6 |
  2138071.9 |
  2138981.2 |
  2139890.6 |
  2140799.9 |
  2141709.3 |########################################
  2142618.6 |########################################
  2143527.9 |
  2144437.3 |########################################
  2145346.6 |
  2146255.9 |
  2147165.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=3447.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.1% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=302.5% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=313.3% of algo (FFI overhead may distort results)

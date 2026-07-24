# abi_boundary_w (scatter)

9 variants, 6 samples per variant.
Baseline: **abi_boundary_w_scatter_scalar_runtime_w**

## Highlights

Baseline for all deltas below: **abi_boundary_w_scatter_scalar_runtime_w**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_boundary_w_scatter_scalar_runtime_w) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_boundary_w_scatter_scalar_runtime_w has the worst median (2.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_boundary_w_scatter_null_entry at 3.97 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_boundary_w_scatter_null_entry dominates: 53566% faster than the next best (abi_boundary_w_scatter_zig_runtime_w)

abi_boundary_w_scatter_null_entry (3.97 us) leads abi_boundary_w_scatter_zig_runtime_w (2.13 ms) by 53566%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_boundary_w_scatter_null_entry beats baseline by 100% (significant)

abi_boundary_w_scatter_null_entry is -2.16 ms (100%) faster than baseline abi_boundary_w_scatter_scalar_runtime_w, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_boundary_w_scatter_scalar_runtime_w is an outlier: 544.0x slower than the field

abi_boundary_w_scatter_scalar_runtime_w (2.16 ms) is 544.0x the fastest (3.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_boundary_w_scatter_null_entry} vs {abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_scalar_runtime_w} (53566% apart)

The field splits into a fast tier {abi_boundary_w_scatter_null_entry} and a slow tier {abi_boundary_w_scatter_zig_runtime_w, abi_boundary_w_scatter_soa_dispatch, abi_boundary_w_scatter_scalar_per_w, abi_boundary_w_scatter_soa_per_w, abi_boundary_w_scatter_scalar_anchor, abi_boundary_w_scatter_scalar_dispatch, abi_boundary_w_scatter_soa_runtime_w, abi_boundary_w_scatter_scalar_runtime_w} with a 53566% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 544.0x the fastest

Fastest abi_boundary_w_scatter_null_entry (3.97 us) to slowest abi_boundary_w_scatter_scalar_runtime_w (2.16 ms): 544.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_boundary_w_scatter_null_entry** at 3973.5 ns median (-99.8% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 544.04x (fastest 3973.5 ns, slowest 2161748.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 6220ns | 6181ns | 6115ns | 6175ns | 6339ns | -99.71% |
| abi_boundary_w_scatter_scalar_anchor | 2161980ns | 2160669ns | 2158837ns | 2160609ns | 2165608ns | -0.17% |
| abi_boundary_w_scatter_scalar_dispatch | 2166065ns | 2161917ns | 2156136ns | 2160532ns | 2179330ns | +0.02% |
| abi_boundary_w_scatter_scalar_per_w | 2159548ns | 2159287ns | 2152680ns | 2158415ns | 2164680ns | -0.28% |
| abi_boundary_w_scatter_scalar_runtime_w | 2165668ns | 2164697ns | 2157742ns | 2163971ns | 2172177ns | base |
| abi_boundary_w_scatter_soa_dispatch | 2159847ns | 2158808ns | 2154935ns | 2158398ns | 2164476ns | -0.27% |
| abi_boundary_w_scatter_soa_per_w | 2160365ns | 2160048ns | 2155559ns | 2159349ns | 2164293ns | -0.24% |
| abi_boundary_w_scatter_soa_runtime_w | 2162575ns | 2163076ns | 2159455ns | 2162540ns | 2164187ns | -0.14% |
| abi_boundary_w_scatter_zig_runtime_w | 2135875ns | 2135434ns | 2130707ns | 2134434ns | 2140620ns | -1.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 3989ns | 3931ns | 4046ns | -99.82% | 0.001 |
| abi_boundary_w_scatter_scalar_anchor | 2159098ns | 2155954ns | 2162607ns | -0.17% | 0.000 |
| abi_boundary_w_scatter_scalar_dispatch | 2163042ns | 2153238ns | 2176070ns | +0.01% | 0.000 |
| abi_boundary_w_scatter_scalar_per_w | 2156658ns | 2149966ns | 2161556ns | -0.28% | 0.000 |
| abi_boundary_w_scatter_scalar_runtime_w | 2162720ns | 2154992ns | 2169145ns | base | 0.000 |
| abi_boundary_w_scatter_soa_dispatch | 2156911ns | 2152155ns | 2161464ns | -0.27% | 0.000 |
| abi_boundary_w_scatter_soa_per_w | 2157479ns | 2152858ns | 2161252ns | -0.24% | 0.000 |
| abi_boundary_w_scatter_soa_runtime_w | 2159704ns | 2156655ns | 2161280ns | -0.14% | 0.000 |
| abi_boundary_w_scatter_zig_runtime_w | 2132846ns | 2127738ns | 2137540ns | -1.38% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 26648.3 | 4101.0 | 3988.5 | n/a |
| abi_boundary_w_scatter_scalar_anchor | 49491.4 | 2159249.3 | 2159098.4 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 51161.7 | 2165254.2 | 2163042.3 | n/a |
| abi_boundary_w_scatter_scalar_per_w | 49158.6 | 2156060.6 | 2156657.8 | 0 |
| abi_boundary_w_scatter_scalar_runtime_w | 48585.8 | 2163747.7 | 2162720.3 | n/a |
| abi_boundary_w_scatter_soa_dispatch | 49369.6 | 2155507.5 | 2156911.4 | 0 |
| abi_boundary_w_scatter_soa_per_w | 50123.3 | 2158851.1 | 2157478.5 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 47623.1 | 2157886.7 | 2159703.9 | n/a |
| abi_boundary_w_scatter_zig_runtime_w | 203691.2 | 2133460.2 | 2132846.2 | 5 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.001 Gops/s** (abi_boundary_w_scatter_null_entry; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.001 | 98.9% |
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
| abi_boundary_w_scatter_null_entry | 6220ns | 6220ns | -99.71% |
| abi_boundary_w_scatter_scalar_anchor | 2161980ns | 2161980ns | -0.17% |
| abi_boundary_w_scatter_scalar_dispatch | 2166065ns | 2166065ns | +0.02% |
| abi_boundary_w_scatter_scalar_per_w | 2159548ns | 2159548ns | -0.28% |
| abi_boundary_w_scatter_scalar_runtime_w | 2165668ns | 2165668ns | base |
| abi_boundary_w_scatter_soa_dispatch | 2159847ns | 2159847ns | -0.27% |
| abi_boundary_w_scatter_soa_per_w | 2160365ns | 2160365ns | -0.24% |
| abi_boundary_w_scatter_soa_runtime_w | 2162575ns | 2162575ns | -0.14% |
| abi_boundary_w_scatter_zig_runtime_w | 2135875ns | 2135875ns | -1.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_boundary_w_scatter_scalar_runtime_w | 2161749ns | base | --- | [2157267, 2169145] | --- | --- | --- | --- |
| abi_boundary_w_scatter_null_entry | 3974ns | -2157801.5ns (-99.8%) | [-2165106, -2153288]ns | [3946, 4046] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_scatter_scalar_anchor | 2157920ns | no significant difference | [-11417, +5340]ns | [2156768, 2162607] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_scatter_scalar_dispatch | 2159101ns | no significant difference | [-12848, +14687]ns | [2153955, 2176070] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_scalar_per_w | 2156427ns | no significant difference | [-14813, +1353]ns | [2151991, 2161556] | no | 0.9167 | 0.6875 | 0 |
| abi_boundary_w_scatter_soa_dispatch | 2155967ns | -6079.3ns (-0.3%) | [-8301, -3046]ns | [2153302, 2161464] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_per_w | 2157120ns | -3125.0ns (-0.1%) | [-11451, -1149]ns | [2154063, 2161252] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| abi_boundary_w_scatter_soa_runtime_w | 2160224ns | no significant difference | [-10136, +3379]ns | [2157608, 2161280] | no | 1.0000 | 1.0000 | 0 |
| abi_boundary_w_scatter_zig_runtime_w | 2132413ns | -29256.5ns (-1.4%) | [-36681, -23685]ns | [2128586, 2137540] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_boundary_w_scatter_scalar_runtime_w | abi_boundary_w_scatter_null_entry | abi_boundary_w_scatter_scalar_anchor | abi_boundary_w_scatter_scalar_dispatch | abi_boundary_w_scatter_scalar_per_w | abi_boundary_w_scatter_soa_dispatch | abi_boundary_w_scatter_soa_per_w | abi_boundary_w_scatter_soa_runtime_w | abi_boundary_w_scatter_zig_runtime_w |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 2163225ns | -99.8% | -0.3% | +1.0% | +0.0% | -0.4% | -0.0% | -0.2% | -1.1% |
| 2 | 2159541ns | -99.8% | +0.1% | +0.4% | -0.2% | -0.2% | -0.2% | +0.1% | -1.2% |
| 3 | 2154992ns | -99.8% | +0.4% | +0.0% | +0.1% | -0.1% | -0.1% | +0.2% | -1.1% |
| 4 | 2160272ns | -99.8% | -0.1% | -0.3% | -0.5% | -0.2% | -0.1% | +0.0% | -1.5% |
| 5 | 2173335ns | -99.8% | -0.7% | -0.9% | -0.9% | -0.4% | -0.8% | -0.6% | -1.7% |
| 6 | 2164956ns | -99.8% | -0.3% | -0.1% | -0.3% | -0.3% | -0.2% | -0.4% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_boundary_w_scatter_null_entry | 0.046 | ok |
| abi_boundary_w_scatter_scalar_anchor | -0.007 | ok |
| abi_boundary_w_scatter_scalar_dispatch | 0.329 | moderate+ |
| abi_boundary_w_scatter_scalar_per_w | -0.006 | ok |
| abi_boundary_w_scatter_scalar_runtime_w | 0.205 | moderate+ |
| abi_boundary_w_scatter_soa_dispatch | 0.180 | ok |
| abi_boundary_w_scatter_soa_per_w | -0.246 | moderate- |
| abi_boundary_w_scatter_soa_runtime_w | -0.300 | moderate- |
| abi_boundary_w_scatter_zig_runtime_w | -0.116 | ok |

**Consistency summary:**

- **abi_boundary_w_scatter_null_entry**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_scalar_anchor**: won 4/6, lost 1/6
- **abi_boundary_w_scatter_scalar_dispatch**: won 3/6, lost 2/6
- **abi_boundary_w_scatter_scalar_per_w**: won 4/6, lost 1/6
- **abi_boundary_w_scatter_soa_dispatch**: won 6/6, lost 0/6
- **abi_boundary_w_scatter_soa_per_w**: won 3/6, lost 0/6
- **abi_boundary_w_scatter_soa_runtime_w**: won 3/6, lost 1/6
- **abi_boundary_w_scatter_zig_runtime_w**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_boundary_w_scatter_null_entry | 121526.5ns | 3988.5ns | 3046.9% | HIGH |
| abi_boundary_w_scatter_scalar_anchor | 6527193.6ns | 2159098.4ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_dispatch | 6546779.9ns | 2163042.3ns | 302.7% | HIGH |
| abi_boundary_w_scatter_scalar_per_w | 6519799.6ns | 2156657.8ns | 302.3% | HIGH |
| abi_boundary_w_scatter_scalar_runtime_w | 6540675.6ns | 2162720.3ns | 302.4% | HIGH |
| abi_boundary_w_scatter_soa_dispatch | 6516001.7ns | 2156911.4ns | 302.1% | HIGH |
| abi_boundary_w_scatter_soa_per_w | 6529484.8ns | 2157478.5ns | 302.6% | HIGH |
| abi_boundary_w_scatter_soa_runtime_w | 6525496.4ns | 2159703.9ns | 302.1% | HIGH |
| abi_boundary_w_scatter_zig_runtime_w | 6675106.6ns | 2132846.2ns | 313.0% | HIGH |

## Distribution (algo ns)

```
abi_boundary_w_scatter_null_entry (n=6, range 3931.2-4046.1 ns)
   3931.2 |####################
   3936.9 |
   3942.7 |
   3948.4 |
   3954.2 |
   3959.9 |########################################
   3965.7 |
   3971.4 |
   3977.1 |
   3982.9 |####################
   3988.6 |
   3994.4 |####################
   4000.1 |
   4005.9 |
   4011.6 |
   4017.3 |
   4023.1 |
   4028.8 |
   4034.6 |
   4040.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_anchor (n=6, range 2155954.2-2162606.9 ns)
  2155954.2 |########################################
  2156286.8 |
  2156619.5 |
  2156952.1 |
  2157284.7 |########################################
  2157617.4 |########################################
  2157950.0 |########################################
  2158282.6 |
  2158615.3 |
  2158947.9 |
  2159280.6 |
  2159613.2 |
  2159945.8 |
  2160278.5 |
  2160611.1 |
  2160943.7 |########################################
  2161276.4 |
  2161609.0 |
  2161941.6 |
  2162274.3 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_dispatch (n=6, range 2153237.9-2176070.4 ns)
  2153237.9 |########################################
  2154379.5 |########################################
  2155521.1 |########################################
  2156662.8 |
  2157804.4 |
  2158946.0 |
  2160087.6 |
  2161229.3 |########################################
  2162370.9 |
  2163512.5 |
  2164654.2 |
  2165795.8 |
  2166937.4 |########################################
  2168079.0 |
  2169220.7 |
  2170362.3 |
  2171503.9 |
  2172645.5 |
  2173787.2 |
  2174928.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_per_w (n=6, range 2149966.2-2161555.6 ns)
  2149966.2 |########################################
  2150545.7 |
  2151125.1 |
  2151704.6 |
  2152284.1 |
  2152863.6 |
  2153443.0 |########################################
  2154022.5 |
  2154602.0 |
  2155181.5 |########################################
  2155760.9 |
  2156340.4 |
  2156919.9 |########################################
  2157499.3 |
  2158078.8 |
  2158658.3 |
  2159237.8 |########################################
  2159817.2 |
  2160396.7 |
  2160976.2 |
  (0 below, 1 above range)

abi_boundary_w_scatter_scalar_runtime_w (n=6, range 2154992.5-2169145.4 ns)
  2154992.5 |########################################
  2155700.1 |
  2156407.8 |
  2157115.4 |
  2157823.1 |
  2158530.7 |
  2159238.4 |########################################
  2159946.0 |########################################
  2160653.7 |
  2161361.3 |
  2162069.0 |
  2162776.6 |########################################
  2163484.2 |
  2164191.9 |
  2164899.5 |########################################
  2165607.2 |
  2166314.8 |
  2167022.5 |
  2167730.1 |
  2168437.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_dispatch (n=6, range 2152154.6-2161464.4 ns)
  2152154.6 |########################################
  2152620.1 |
  2153085.6 |
  2153551.1 |
  2154016.6 |########################################
  2154482.1 |########################################
  2154947.5 |
  2155413.0 |
  2155878.5 |
  2156344.0 |
  2156809.5 |########################################
  2157275.0 |########################################
  2157740.5 |
  2158206.0 |
  2158671.5 |
  2159137.0 |
  2159602.4 |
  2160067.9 |
  2160533.4 |
  2160998.9 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_per_w (n=6, range 2152857.5-2161252.5 ns)
  2152857.5 |########################################
  2153277.2 |
  2153697.0 |
  2154116.8 |
  2154536.5 |
  2154956.2 |########################################
  2155376.0 |########################################
  2155795.8 |
  2156215.5 |
  2156635.2 |
  2157055.0 |
  2157474.8 |
  2157894.5 |
  2158314.2 |
  2158734.0 |########################################
  2159153.8 |
  2159573.5 |
  2159993.2 |########################################
  2160413.0 |
  2160832.8 |
  (0 below, 1 above range)

abi_boundary_w_scatter_soa_runtime_w (n=6, range 2156654.6-2161279.6 ns)
  2156654.6 |########################################
  2156885.9 |
  2157117.1 |
  2157348.4 |
  2157579.6 |
  2157810.9 |
  2158042.1 |
  2158273.4 |
  2158504.6 |########################################
  2158735.9 |
  2158967.1 |
  2159198.4 |
  2159429.6 |
  2159660.9 |
  2159892.1 |########################################
  2160123.4 |########################################
  2160354.6 |
  2160585.9 |
  2160817.1 |
  2161048.4 |########################################
  (0 below, 1 above range)

abi_boundary_w_scatter_zig_runtime_w (n=6, range 2127737.5-2137539.8 ns)
  2127737.5 |########################################
  2128227.6 |
  2128717.7 |
  2129207.8 |########################################
  2129698.0 |
  2130188.1 |
  2130678.2 |
  2131168.3 |########################################
  2131658.4 |
  2132148.5 |
  2132638.6 |
  2133128.8 |########################################
  2133618.9 |
  2134109.0 |
  2134599.1 |
  2135089.2 |########################################
  2135579.3 |
  2136069.5 |
  2136559.6 |
  2137049.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_boundary_w_scatter_null_entry**: bridge=3042.9% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_anchor**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_dispatch**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_per_w**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_scalar_runtime_w**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_dispatch**: bridge=302.0% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_per_w**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_soa_runtime_w**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_boundary_w_scatter_zig_runtime_w**: bridge=313.0% of algo (FFI overhead may distort results)

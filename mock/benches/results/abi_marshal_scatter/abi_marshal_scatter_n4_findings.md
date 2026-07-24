# abi_marshal (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_scatter_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_scatter_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_scatter_marshal_null dominates: 10245% faster than the next best (abi_marshal_scatter_soa_native)

abi_marshal_scatter_marshal_null (20.53 us) leads abi_marshal_scatter_soa_native (2.12 ms) by 10245%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_scatter_marshal_null beats baseline by 99% (significant)

abi_marshal_scatter_marshal_null is -2.12 ms (99%) faster than baseline abi_marshal_scatter_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_scatter_soa_transposed is an outlier: 104.2x slower than the field

abi_marshal_scatter_soa_transposed (2.14 ms) is 104.2x the fastest (20.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_scatter_marshal_null} vs {abi_marshal_scatter_soa_native, abi_marshal_scatter_aos, abi_marshal_scatter_soa_transposed} (10245% apart)

The field splits into a fast tier {abi_marshal_scatter_marshal_null} and a slow tier {abi_marshal_scatter_soa_native, abi_marshal_scatter_aos, abi_marshal_scatter_soa_transposed} with a 10245% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 104.2x the fastest

Fastest abi_marshal_scatter_marshal_null (20.53 us) to slowest abi_marshal_scatter_soa_transposed (2.14 ms): 104.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_scatter_marshal_null** at 20533.1 ns median (-99.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 104.23x (fastest 20533.1 ns, slowest 2140103.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2161895ns | 2140159ns | 2131221ns | 2139433ns | 2210925ns | base |
| abi_marshal_scatter_marshal_null | 22924ns | 22824ns | 21973ns | 22730ns | 23690ns | -98.94% |
| abi_marshal_scatter_soa_native | 2132884ns | 2126600ns | 2110158ns | 2124325ns | 2157084ns | -1.34% |
| abi_marshal_scatter_soa_transposed | 2207359ns | 2142733ns | 2132261ns | 2140040ns | 2345886ns | +2.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2159180ns | 2128561ns | 2208085ns | base | 0.000 |
| abi_marshal_scatter_marshal_null | 20623ns | 19776ns | 21311ns | -99.04% | 0.000 |
| abi_marshal_scatter_soa_native | 2130363ns | 2107692ns | 2154371ns | -1.33% | 0.000 |
| abi_marshal_scatter_soa_transposed | 2204662ns | 2129794ns | 2342847ns | +2.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 44217.4 | 2151945.1 | 2159179.6 | n/a |
| abi_marshal_scatter_marshal_null | 26497.5 | 20647.2 | 20623.3 | n/a |
| abi_marshal_scatter_soa_native | 39377.6 | 2129442.2 | 2130362.7 | n/a |
| abi_marshal_scatter_soa_transposed | 44770.8 | 2209512.6 | 2204662.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_scatter_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_scatter_aos | 0.000 | 0.9% |
| abi_marshal_scatter_marshal_null | 0.000 | 96.3% |
| abi_marshal_scatter_soa_native | 0.000 | 0.9% |
| abi_marshal_scatter_soa_transposed | 0.000 | 0.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_scatter_aos | 2161895ns | 2161895ns | base |
| abi_marshal_scatter_marshal_null | 22924ns | 22924ns | -98.94% |
| abi_marshal_scatter_soa_native | 2132884ns | 2132884ns | -1.34% |
| abi_marshal_scatter_soa_transposed | 2207359ns | 2207359ns | +2.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2137527ns | base | --- | [2131927, 2208085] | --- | --- | --- | --- |
| abi_marshal_scatter_marshal_null | 20533ns | -2117473.3ns (-99.1%) | [-2187371, -2110825]ns | [20026, 21311] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_scatter_soa_native | 2124198ns | -16704.6ns (-0.8%) | [-64485, -5261]ns | [2112520, 2154371] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_marshal_scatter_soa_transposed | 2140103ns | no significant difference | [-4155, +138799]ns | [2131037, 2342847] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_scatter_aos | abi_marshal_scatter_marshal_null | abi_marshal_scatter_soa_native | abi_marshal_scatter_soa_transposed |
|---|---|---|---|---|
| 1 | 2135541ns | -99.1% | -0.5% | -0.0% |
| 2 | 2268582ns | -99.1% | -3.9% | +11.5% |
| 3 | 2147589ns | -99.1% | -1.9% | -0.1% |
| 4 | 2139513ns | -99.0% | -1.0% | +0.8% |
| 5 | 2128561ns | -99.0% | +0.0% | +0.2% |
| 6 | 2135292ns | -99.0% | -0.5% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_scatter_aos | -0.157 | ok |
| abi_marshal_scatter_marshal_null | -0.243 | moderate- |
| abi_marshal_scatter_soa_native | -0.344 | moderate- |
| abi_marshal_scatter_soa_transposed | -0.236 | moderate- |

**Consistency summary:**

- **abi_marshal_scatter_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_scatter_soa_native**: won 5/6, lost 0/6
- **abi_marshal_scatter_soa_transposed**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 6522100.6ns | 2159179.6ns | 302.1% | HIGH |
| abi_marshal_scatter_marshal_null | 168359.7ns | 20623.3ns | 816.4% | HIGH |
| abi_marshal_scatter_soa_native | 6451899.0ns | 2130362.7ns | 302.9% | HIGH |
| abi_marshal_scatter_soa_transposed | 6657276.9ns | 2204662.4ns | 302.0% | HIGH |

## Distribution (algo ns)

```
abi_marshal_scatter_aos (n=6, range 2128561.2-2208085.2 ns)
  2128561.2 |####################
  2132537.4 |########################################
  2136513.6 |####################
  2140489.8 |
  2144466.0 |####################
  2148442.2 |
  2152418.4 |
  2156394.6 |
  2160370.8 |
  2164347.0 |
  2168323.2 |
  2172299.4 |
  2176275.6 |
  2180251.8 |
  2184228.0 |
  2188204.2 |
  2192180.4 |
  2196156.6 |
  2200132.8 |
  2204109.0 |
  (0 below, 1 above range)

abi_marshal_scatter_marshal_null (n=6, range 19776.2-21310.6 ns)
  19776.2 |########################################
  19852.9 |
  19929.6 |
  20006.4 |
  20083.1 |
  20159.8 |
  20236.5 |########################################
  20313.2 |########################################
  20390.0 |
  20466.7 |
  20543.4 |
  20620.1 |
  20696.8 |########################################
  20773.6 |
  20850.3 |
  20927.0 |
  21003.7 |
  21080.4 |########################################
  21157.2 |
  21233.9 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_native (n=6, range 2107691.7-2154370.7 ns)
  2107691.7 |####################
  2110025.6 |
  2112359.6 |
  2114693.5 |
  2117027.5 |####################
  2119361.4 |
  2121695.4 |
  2124029.3 |########################################
  2126363.3 |
  2128697.2 |####################
  2131031.2 |
  2133365.1 |
  2135699.1 |
  2138033.0 |
  2140367.0 |
  2142700.9 |
  2145034.9 |
  2147368.8 |
  2149702.8 |
  2152036.7 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_transposed (n=6, range 2129793.8-2342846.9 ns)
  2129793.8 |########################################
  2140446.5 |#############
  2151099.1 |#############
  2161751.8 |
  2172404.4 |
  2183057.1 |
  2193709.7 |
  2204362.4 |
  2215015.0 |
  2225667.7 |
  2236320.4 |
  2246973.0 |
  2257625.7 |
  2268278.3 |
  2278931.0 |
  2289583.6 |
  2300236.3 |
  2310888.9 |
  2321541.6 |
  2332194.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_scatter_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_marshal_null**: bridge=823.2% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_native**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_transposed**: bridge=302.0% of algo (FFI overhead may distort results)

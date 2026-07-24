# abi_marshal (scatter)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_scatter_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_scatter_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_scatter_marshal_null dominates: 20484% faster than the next best (abi_marshal_scatter_aos)

abi_marshal_scatter_marshal_null (10.29 us) leads abi_marshal_scatter_aos (2.12 ms) by 20484%, a clear separation rather than a photo finish. CV 2.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_scatter_marshal_null beats baseline by 100% (significant)

abi_marshal_scatter_marshal_null is -2.11 ms (100%) faster than baseline abi_marshal_scatter_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_scatter_soa_transposed is an outlier: 206.2x slower than the field

abi_marshal_scatter_soa_transposed (2.12 ms) is 206.2x the fastest (10.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_scatter_marshal_null} vs {abi_marshal_scatter_aos, abi_marshal_scatter_soa_native, abi_marshal_scatter_soa_transposed} (20484% apart)

The field splits into a fast tier {abi_marshal_scatter_marshal_null} and a slow tier {abi_marshal_scatter_aos, abi_marshal_scatter_soa_native, abi_marshal_scatter_soa_transposed} with a 20484% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 206.2x the fastest

Fastest abi_marshal_scatter_marshal_null (10.29 us) to slowest abi_marshal_scatter_soa_transposed (2.12 ms): 206.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_scatter_marshal_null** at 10287.7 ns median (-99.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 206.20x (fastest 10287.7 ns, slowest 2121322.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2129074ns | 2120010ns | 2109083ns | 2117875ns | 2155868ns | base |
| abi_marshal_scatter_marshal_null | 12584ns | 12621ns | 12106ns | 12547ns | 12877ns | -99.41% |
| abi_marshal_scatter_soa_native | 2124558ns | 2120075ns | 2112956ns | 2118141ns | 2139986ns | -0.21% |
| abi_marshal_scatter_soa_transposed | 2132908ns | 2123808ns | 2113740ns | 2121772ns | 2159196ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2126480ns | 2106572ns | 2153028ns | base | 0.000 |
| abi_marshal_scatter_marshal_null | 10273ns | 9903ns | 10531ns | -99.52% | 0.000 |
| abi_marshal_scatter_soa_native | 2122018ns | 2110520ns | 2137315ns | -0.21% | 0.000 |
| abi_marshal_scatter_soa_transposed | 2130325ns | 2111148ns | 2156482ns | +0.18% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 43284.4 | 2127573.0 | 2126479.5 | n/a |
| abi_marshal_scatter_marshal_null | 27279.1 | 10393.2 | 10273.2 | n/a |
| abi_marshal_scatter_soa_native | 40368.6 | 2226063.8 | 2122018.3 | n/a |
| abi_marshal_scatter_soa_transposed | 39107.2 | 2155744.9 | 2130324.6 | 1 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_scatter_marshal_null; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_scatter_aos | 0.000 | 0.5% |
| abi_marshal_scatter_marshal_null | 0.000 | 96.3% |
| abi_marshal_scatter_soa_native | 0.000 | 0.5% |
| abi_marshal_scatter_soa_transposed | 0.000 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_scatter_aos | 2129074ns | 2129074ns | base |
| abi_marshal_scatter_marshal_null | 12584ns | 12584ns | -99.41% |
| abi_marshal_scatter_soa_native | 2124558ns | 2124558ns | -0.21% |
| abi_marshal_scatter_soa_transposed | 2132908ns | 2132908ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_scatter_aos | 2117571ns | base | --- | [2108840, 2153028] | --- | --- | --- | --- |
| abi_marshal_scatter_marshal_null | 10288ns | -2107283.0ns (-99.5%) | [-2142774, -2098562]ns | [10000, 10531] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_scatter_soa_native | 2117619ns | no significant difference | [-18938, +7211]ns | [2111121, 2137315] | no | 0.6875 | 0.6875 | 0 |
| abi_marshal_scatter_soa_transposed | 2121322ns | no significant difference | [-8029, +13201]ns | [2113169, 2156482] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_scatter_aos | abi_marshal_scatter_marshal_null | abi_marshal_scatter_soa_native | abi_marshal_scatter_soa_transposed |
|---|---|---|---|---|
| 1 | 2111108ns | -99.5% | -0.0% | +0.4% |
| 2 | 2106572ns | -99.5% | +0.6% | +0.8% |
| 3 | 2120695ns | -99.5% | -0.2% | -0.3% |
| 4 | 2121702ns | -99.5% | +0.1% | -0.5% |
| 5 | 2114447ns | -99.5% | -0.1% | +0.4% |
| 6 | 2184354ns | -99.5% | -1.5% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_scatter_aos | -0.045 | ok |
| abi_marshal_scatter_marshal_null | -0.359 | moderate- |
| abi_marshal_scatter_soa_native | -0.243 | moderate- |
| abi_marshal_scatter_soa_transposed | 0.034 | ok |

**Consistency summary:**

- **abi_marshal_scatter_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_scatter_soa_native**: won 3/6, lost 1/6
- **abi_marshal_scatter_soa_transposed**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_scatter_aos | 6430291.2ns | 2126479.5ns | 302.4% | HIGH |
| abi_marshal_scatter_marshal_null | 140355.3ns | 10273.2ns | 1366.2% | HIGH |
| abi_marshal_scatter_soa_native | 6505147.9ns | 2122018.3ns | 306.6% | HIGH |
| abi_marshal_scatter_soa_transposed | 6523254.9ns | 2130324.6ns | 306.2% | HIGH |

## Distribution (algo ns)

```
abi_marshal_scatter_aos (n=6, range 2106572.5-2153028.0 ns)
  2106572.5 |####################
  2108895.3 |####################
  2111218.0 |
  2113540.8 |####################
  2115863.6 |
  2118186.4 |
  2120509.1 |########################################
  2122831.9 |
  2125154.7 |
  2127477.5 |
  2129800.2 |
  2132123.0 |
  2134445.8 |
  2136768.5 |
  2139091.3 |
  2141414.1 |
  2143736.9 |
  2146059.6 |
  2148382.4 |
  2150705.2 |
  (0 below, 1 above range)

abi_marshal_scatter_marshal_null (n=6, range 9902.9-10531.5 ns)
   9902.9 |########################################
   9934.3 |
   9965.8 |
   9997.2 |
  10028.6 |
  10060.0 |
  10091.5 |########################################
  10122.9 |
  10154.3 |
  10185.7 |########################################
  10217.2 |
  10248.6 |
  10280.0 |
  10311.5 |
  10342.9 |
  10374.3 |########################################
  10405.7 |########################################
  10437.2 |
  10468.6 |
  10500.0 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_native (n=6, range 2110519.6-2137315.4 ns)
  2110519.6 |########################################
  2111859.4 |
  2113199.2 |
  2114539.0 |
  2115878.8 |####################
  2117218.5 |
  2118558.3 |####################
  2119898.1 |
  2121237.9 |
  2122577.7 |####################
  2123917.5 |
  2125257.3 |
  2126597.1 |
  2127936.9 |
  2129276.7 |
  2130616.5 |
  2131956.2 |
  2133296.0 |
  2134635.8 |
  2135975.6 |
  (0 below, 1 above range)

abi_marshal_scatter_soa_transposed (n=6, range 2111147.9-2156482.5 ns)
  2111147.9 |####################
  2113414.6 |####################
  2115681.4 |
  2117948.1 |####################
  2120214.8 |
  2122481.5 |########################################
  2124748.3 |
  2127015.0 |
  2129281.7 |
  2131548.5 |
  2133815.2 |
  2136081.9 |
  2138348.7 |
  2140615.4 |
  2142882.1 |
  2145148.9 |
  2147415.6 |
  2149682.3 |
  2151949.0 |
  2154215.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_scatter_aos**: bridge=302.3% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_marshal_null**: bridge=1371.4% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_native**: bridge=301.7% of algo (FFI overhead may distort results)
- **abi_marshal_scatter_soa_transposed**: bridge=301.9% of algo (FFI overhead may distort results)

# abi_marshal (tight)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_tight_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_tight_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_marshal_tight_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_marshal_tight_aos has the worst median (2.04 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_marshal_tight_marshal_null at 20.59 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### abi_marshal_tight_marshal_null dominates: 9707% faster than the next best (abi_marshal_tight_soa_native)

abi_marshal_tight_marshal_null (20.59 us) leads abi_marshal_tight_soa_native (2.02 ms) by 9707%, a clear separation rather than a photo finish. CV 3.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_tight_marshal_null beats baseline by 99% (significant)

abi_marshal_tight_marshal_null is -2.02 ms (99%) faster than baseline abi_marshal_tight_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_tight_aos is an outlier: 98.9x slower than the field

abi_marshal_tight_aos (2.04 ms) is 98.9x the fastest (20.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {abi_marshal_tight_marshal_null} vs {abi_marshal_tight_soa_native, abi_marshal_tight_soa_transposed, abi_marshal_tight_aos} (9707% apart)

The field splits into a fast tier {abi_marshal_tight_marshal_null} and a slow tier {abi_marshal_tight_soa_native, abi_marshal_tight_soa_transposed, abi_marshal_tight_aos} with a 9707% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 98.9x the fastest

Fastest abi_marshal_tight_marshal_null (20.59 us) to slowest abi_marshal_tight_aos (2.04 ms): 98.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_tight_marshal_null** at 20588.5 ns median (-99.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 98.88x (fastest 20588.5 ns, slowest 2035754.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2066804ns | 2038248ns | 2030783ns | 2037284ns | 2129094ns | base |
| abi_marshal_tight_marshal_null | 22831ns | 22848ns | 22094ns | 22603ns | 23542ns | -98.90% |
| abi_marshal_tight_soa_native | 2025156ns | 2021750ns | 2016743ns | 2021600ns | 2034697ns | -2.02% |
| abi_marshal_tight_soa_transposed | 2036449ns | 2036943ns | 2025544ns | 2033959ns | 2045637ns | -1.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2064189ns | 2028324ns | 2126288ns | base | 0.000 |
| abi_marshal_tight_marshal_null | 20544ns | 19810ns | 21210ns | -99.00% | 0.000 |
| abi_marshal_tight_soa_native | 2022578ns | 2014269ns | 2032021ns | -2.02% | 0.000 |
| abi_marshal_tight_soa_transposed | 2033877ns | 2022897ns | 2043124ns | -1.47% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_tight_aos | 43389.3 | 2053194.2 | 2064189.2 | n/a |
| abi_marshal_tight_marshal_null | 27473.0 | 20508.9 | 20544.5 | n/a |
| abi_marshal_tight_soa_native | 41888.0 | 2025764.3 | 2022577.8 | n/a |
| abi_marshal_tight_soa_transposed | 40399.3 | 2044375.3 | 2033877.0 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_tight_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_tight_aos | 0.000 | 1.0% |
| abi_marshal_tight_marshal_null | 0.000 | 96.2% |
| abi_marshal_tight_soa_native | 0.000 | 1.0% |
| abi_marshal_tight_soa_transposed | 0.000 | 1.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_tight_aos | 2066804ns | 2066804ns | base |
| abi_marshal_tight_marshal_null | 22831ns | 22831ns | -98.90% |
| abi_marshal_tight_soa_native | 2025156ns | 2025156ns | -2.02% |
| abi_marshal_tight_soa_transposed | 2036449ns | 2036449ns | -1.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_tight_aos | 2035755ns | base | --- | [2030525, 2126288] | --- | --- | --- | --- |
| abi_marshal_tight_marshal_null | 20589ns | -2015215.9ns (-99.0%) | [-2106336, -2009382]ns | [19835, 21210] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_tight_soa_native | 2019219ns | -17990.0ns (-0.9%) | [-95105, -11739]ns | [2016494, 2032021] | YES | 0.0469 | 0.0313 | 0 |
| abi_marshal_tight_soa_transposed | 2034334ns | no significant difference | [-90215, +1804]ns | [2024173, 2043124] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_tight_aos | abi_marshal_tight_marshal_null | abi_marshal_tight_soa_native | abi_marshal_tight_soa_transposed |
|---|---|---|---|---|
| 1 | 2040854ns | -99.0% | -1.1% | -0.9% |
| 2 | 2028324ns | -99.0% | -0.5% | -0.1% |
| 3 | 2032725ns | -99.0% | -0.9% | +0.0% |
| 4 | 2038012ns | -99.0% | -0.9% | -0.1% |
| 5 | 2211723ns | -99.1% | -7.6% | -7.3% |
| 6 | 2033498ns | -99.0% | -0.7% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_tight_aos | -0.214 | moderate- |
| abi_marshal_tight_marshal_null | 0.100 | ok |
| abi_marshal_tight_soa_native | -0.076 | ok |
| abi_marshal_tight_soa_transposed | 0.401 | moderate+ |

**Consistency summary:**

- **abi_marshal_tight_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_tight_soa_native**: won 6/6, lost 0/6
- **abi_marshal_tight_soa_transposed**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_tight_aos | 6228290.3ns | 2064189.2ns | 301.7% | HIGH |
| abi_marshal_tight_marshal_null | 168301.9ns | 20544.5ns | 819.2% | HIGH |
| abi_marshal_tight_soa_native | 6120197.8ns | 2022577.8ns | 302.6% | HIGH |
| abi_marshal_tight_soa_transposed | 6171086.4ns | 2033877.0ns | 303.4% | HIGH |

## Distribution (algo ns)

```
abi_marshal_tight_aos (n=6, range 2028323.7-2126288.3 ns)
  2028323.7 |########################################
  2033221.9 |########################################
  2038120.2 |####################
  2043018.4 |
  2047916.6 |
  2052814.8 |
  2057713.1 |
  2062611.3 |
  2067509.5 |
  2072407.8 |
  2077306.0 |
  2082204.2 |
  2087102.5 |
  2092000.7 |
  2096898.9 |
  2101797.1 |
  2106695.4 |
  2111593.6 |
  2116491.8 |
  2121390.1 |
  (0 below, 1 above range)

abi_marshal_tight_marshal_null (n=6, range 19810.0-21210.2 ns)
  19810.0 |########################################
  19880.0 |
  19950.0 |
  20020.0 |
  20090.0 |####################
  20160.0 |
  20230.1 |
  20300.1 |
  20370.1 |
  20440.1 |
  20510.1 |
  20580.1 |
  20650.1 |
  20720.1 |
  20790.1 |
  20860.1 |
  20930.2 |
  21000.2 |
  21070.2 |####################
  21140.2 |####################
  (0 below, 1 above range)

abi_marshal_tight_soa_native (n=6, range 2014269.2-2032020.6 ns)
  2014269.2 |####################
  2015156.8 |
  2016044.3 |
  2016931.9 |
  2017819.5 |
  2018707.1 |########################################
  2019594.6 |####################
  2020482.2 |####################
  2021369.8 |
  2022257.3 |
  2023144.9 |
  2024032.5 |
  2024920.0 |
  2025807.6 |
  2026695.2 |
  2027582.8 |
  2028470.3 |
  2029357.9 |
  2030245.5 |
  2031133.0 |
  (0 below, 1 above range)

abi_marshal_tight_soa_transposed (n=6, range 2022896.7-2043123.5 ns)
  2022896.7 |########################################
  2023908.0 |
  2024919.4 |########################################
  2025930.7 |
  2026942.1 |
  2027953.4 |
  2028964.8 |
  2029976.1 |
  2030987.4 |
  2031998.8 |########################################
  2033010.1 |
  2034021.5 |
  2035032.8 |########################################
  2036044.2 |########################################
  2037055.5 |
  2038066.8 |
  2039078.2 |
  2040089.5 |
  2041100.9 |
  2042112.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_tight_aos**: bridge=301.9% of algo (FFI overhead may distort results)
- **abi_marshal_tight_marshal_null**: bridge=818.6% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_native**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_marshal_tight_soa_transposed**: bridge=302.1% of algo (FFI overhead may distort results)

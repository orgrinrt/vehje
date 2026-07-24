# abi_marshal (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_marshal_wideselect_aos**

## Highlights

Baseline for all deltas below: **abi_marshal_wideselect_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### abi_marshal_wideselect_marshal_null dominates: 10024% faster than the next best (abi_marshal_wideselect_soa_native)

abi_marshal_wideselect_marshal_null (20.36 us) leads abi_marshal_wideselect_soa_native (2.06 ms) by 10024%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### abi_marshal_wideselect_marshal_null beats baseline by 99% (significant)

abi_marshal_wideselect_marshal_null is -2.05 ms (99%) faster than baseline abi_marshal_wideselect_aos, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### abi_marshal_wideselect_soa_transposed is an outlier: 102.2x slower than the field

abi_marshal_wideselect_soa_transposed (2.08 ms) is 102.2x the fastest (20.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### abi_marshal_wideselect_soa_transposed shows warm-up / thermal drift (autocorr +0.53)

abi_marshal_wideselect_soa_transposed's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {abi_marshal_wideselect_marshal_null} vs {abi_marshal_wideselect_soa_native, abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_transposed} (10024% apart)

The field splits into a fast tier {abi_marshal_wideselect_marshal_null} and a slow tier {abi_marshal_wideselect_soa_native, abi_marshal_wideselect_aos, abi_marshal_wideselect_soa_transposed} with a 10024% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 102.2x the fastest

Fastest abi_marshal_wideselect_marshal_null (20.36 us) to slowest abi_marshal_wideselect_soa_transposed (2.08 ms): 102.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: abi_marshal_wideselect_marshal_null** at 20362.3 ns median (-99.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 102.23x (fastest 20362.3 ns, slowest 2081566.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2095670ns | 2074728ns | 2061253ns | 2070880ns | 2150062ns | base |
| abi_marshal_wideselect_marshal_null | 22778ns | 22650ns | 22073ns | 22458ns | 23611ns | -98.91% |
| abi_marshal_wideselect_soa_native | 2078439ns | 2063928ns | 2056655ns | 2062450ns | 2113314ns | -0.82% |
| abi_marshal_wideselect_soa_transposed | 2085268ns | 2084084ns | 2077572ns | 2082102ns | 2093864ns | -0.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2092999ns | 2058747ns | 2147038ns | base | 0.000 |
| abi_marshal_wideselect_marshal_null | 20485ns | 19857ns | 21207ns | -99.02% | 0.000 |
| abi_marshal_wideselect_soa_native | 2075718ns | 2054043ns | 2110281ns | -0.83% | 0.000 |
| abi_marshal_wideselect_soa_transposed | 2082716ns | 2075171ns | 2091207ns | -0.49% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 48424.0 | 2189302.7 | 2092999.1 | n/a |
| abi_marshal_wideselect_marshal_null | 29616.0 | 20448.5 | 20485.2 | n/a |
| abi_marshal_wideselect_soa_native | 49828.3 | 2086792.0 | 2075718.3 | 0 |
| abi_marshal_wideselect_soa_transposed | 40311.4 | 2081520.9 | 2082715.8 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_marshal_wideselect_marshal_null; best 20% batches)
- Ops per call: 4

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_marshal_wideselect_aos | 0.000 | 1.0% |
| abi_marshal_wideselect_marshal_null | 0.000 | 97.5% |
| abi_marshal_wideselect_soa_native | 0.000 | 1.0% |
| abi_marshal_wideselect_soa_transposed | 0.000 | 1.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_marshal_wideselect_aos | 2095670ns | 2095670ns | base |
| abi_marshal_wideselect_marshal_null | 22778ns | 22778ns | -98.91% |
| abi_marshal_wideselect_soa_native | 2078439ns | 2078439ns | -0.82% |
| abi_marshal_wideselect_soa_transposed | 2085268ns | 2085268ns | -0.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_marshal_wideselect_aos | 2072295ns | base | --- | [2059664, 2147038] | --- | --- | --- | --- |
| abi_marshal_wideselect_marshal_null | 20362ns | -2051437.7ns (-99.0%) | [-2126977, -2039127]ns | [19886, 21207] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| abi_marshal_wideselect_soa_native | 2061414ns | -9309.5ns (-0.4%) | [-40846, -1687]ns | [2055460, 2110281] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_marshal_wideselect_soa_transposed | 2081567ns | no significant difference | [-65471, +31543]ns | [2075373, 2091207] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_marshal_wideselect_aos | abi_marshal_wideselect_marshal_null | abi_marshal_wideselect_soa_native | abi_marshal_wideselect_soa_transposed |
|---|---|---|---|---|
| 1 | 2058747ns | -99.0% | -0.2% | +1.7% |
| 2 | 2060582ns | -99.0% | +0.1% | +1.4% |
| 3 | 2208030ns | -99.1% | -2.4% | -5.7% |
| 4 | 2086046ns | -99.0% | -1.4% | -0.3% |
| 5 | 2071669ns | -99.0% | -0.5% | +0.2% |
| 6 | 2072921ns | -99.0% | -0.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_marshal_wideselect_aos | -0.174 | ok |
| abi_marshal_wideselect_marshal_null | -0.219 | moderate- |
| abi_marshal_wideselect_soa_native | -0.242 | moderate- |
| abi_marshal_wideselect_soa_transposed | 0.529 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **abi_marshal_wideselect_marshal_null**: won 6/6, lost 0/6
- **abi_marshal_wideselect_soa_native**: won 5/6, lost 0/6
- **abi_marshal_wideselect_soa_transposed**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_marshal_wideselect_aos | 6414381.5ns | 2092999.1ns | 306.5% | HIGH |
| abi_marshal_wideselect_marshal_null | 170242.7ns | 20485.2ns | 831.1% | HIGH |
| abi_marshal_wideselect_soa_native | 6523055.3ns | 2075718.3ns | 314.3% | HIGH |
| abi_marshal_wideselect_soa_transposed | 6286145.7ns | 2082715.8ns | 301.8% | HIGH |

## Distribution (algo ns)

```
abi_marshal_wideselect_aos (n=6, range 2058746.7-2147037.9 ns)
  2058746.7 |########################################
  2063161.3 |
  2067575.8 |####################
  2071990.4 |####################
  2076404.9 |
  2080819.5 |
  2085234.1 |####################
  2089648.6 |
  2094063.2 |
  2098477.7 |
  2102892.3 |
  2107306.9 |
  2111721.4 |
  2116136.0 |
  2120550.5 |
  2124965.1 |
  2129379.7 |
  2133794.2 |
  2138208.8 |
  2142623.3 |
  (0 below, 1 above range)

abi_marshal_wideselect_marshal_null (n=6, range 19857.1-21207.1 ns)
  19857.1 |########################################
  19924.6 |
  19992.1 |
  20059.6 |
  20127.1 |
  20194.6 |
  20262.1 |####################
  20329.6 |
  20397.1 |####################
  20464.6 |
  20532.1 |
  20599.6 |
  20667.1 |
  20734.6 |
  20802.1 |
  20869.6 |
  20937.1 |
  21004.6 |
  21072.1 |
  21139.6 |####################
  (0 below, 1 above range)

abi_marshal_wideselect_soa_native (n=6, range 2054043.3-2110281.0 ns)
  2054043.3 |####################
  2056855.2 |####################
  2059667.1 |########################################
  2062479.0 |####################
  2065290.9 |
  2068102.7 |
  2070914.6 |
  2073726.5 |
  2076538.4 |
  2079350.3 |
  2082162.2 |
  2084974.1 |
  2087785.9 |
  2090597.8 |
  2093409.7 |
  2096221.6 |
  2099033.5 |
  2101845.4 |
  2104657.3 |
  2107469.2 |
  (0 below, 1 above range)

abi_marshal_wideselect_soa_transposed (n=6, range 2075171.2-2091207.2 ns)
  2075171.2 |########################################
  2075973.0 |
  2076774.8 |
  2077576.6 |
  2078378.4 |
  2079180.2 |####################
  2079982.0 |
  2080783.8 |
  2081585.6 |
  2082387.4 |
  2083189.2 |####################
  2083991.0 |
  2084792.8 |
  2085594.6 |
  2086396.4 |
  2087198.2 |
  2088000.0 |
  2088801.8 |####################
  2089603.6 |
  2090405.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_marshal_wideselect_aos**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_marshal_null**: bridge=833.5% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_native**: bridge=302.2% of algo (FFI overhead may distort results)
- **abi_marshal_wideselect_soa_transposed**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **abi_marshal_wideselect_soa_transposed**: bridge=302.0% of algo (FFI overhead may distort results)

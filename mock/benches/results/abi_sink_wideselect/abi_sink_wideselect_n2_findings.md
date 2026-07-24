# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (abi_sink_wideselect_null_sink)

The baseline abi_sink_wideselect_null_sink is the fastest (2.07 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.5% of the fastest

All 4 variants sit between 2.07 ms and 2.08 ms - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_wideselect_null_sink) is the fastest** at 2074887.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 2074887.5 ns, slowest 2084811.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2091772ns | 2081886ns | 2074797ns | 2080716ns | 2116844ns | +0.71% |
| abi_sink_wideselect_batched_sink_decode | 2082572ns | 2081671ns | 2074707ns | 2081131ns | 2088666ns | +0.26% |
| abi_sink_wideselect_null_sink | 2077082ns | 2077941ns | 2068040ns | 2075329ns | 2084232ns | base |
| abi_sink_wideselect_per_record_sink | 2109213ns | 2087985ns | 2083407ns | 2086722ns | 2155852ns | +1.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2088583ns | 2071955ns | 2113206ns | +0.70% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2079633ns | 2072050ns | 2085630ns | +0.27% | 0.000 |
| abi_sink_wideselect_null_sink | 2074071ns | 2065145ns | 2081159ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2105823ns | 2080287ns | 2152052ns | +1.53% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 64055.5 | 2089079.9 | 2088583.0 | n/a |
| abi_sink_wideselect_batched_sink_decode | 56327.8 | 2082092.8 | 2079633.1 | 1 |
| abi_sink_wideselect_null_sink | 59523.5 | 2079093.4 | 2074071.0 | n/a |
| abi_sink_wideselect_per_record_sink | 66667.7 | 2105940.5 | 2105822.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_null_sink; best 20% batches)
- Ops per call: 2

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.3% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.3% |
| abi_sink_wideselect_null_sink | 0.000 | 99.5% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2091772ns | 2091772ns | +0.71% |
| abi_sink_wideselect_batched_sink_decode | 2082572ns | 2082572ns | +0.26% |
| abi_sink_wideselect_null_sink | 2077082ns | 2077082ns | base |
| abi_sink_wideselect_per_record_sink | 2109213ns | 2109213ns | +1.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2074888ns | base | --- | [2066167, 2081159] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2078826ns | no significant difference | [-3290, +37812]ns | [2073718, 2113206] | no | 0.3281 | 0.2188 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2078711ns | no significant difference | [-3801, +13687]ns | [2074558, 2085630] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_wideselect_per_record_sink | 2084811ns | +14453.5ns (+0.7%) | [+1704, +79099]ns | [2080605, 2152052] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2073077ns | +0.1% | +0.3% | +0.7% |
| 2 | 2067188ns | +1.3% | +0.9% | +6.7% |
| 3 | 2065145ns | +0.8% | +0.3% | +0.7% |
| 4 | 2078719ns | -0.3% | -0.1% | +0.9% |
| 5 | 2076698ns | +0.0% | +0.5% | +0.3% |
| 6 | 2083598ns | +2.3% | -0.3% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | -0.115 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.358 | moderate- |
| abi_sink_wideselect_null_sink | 0.259 | moderate+ |
| abi_sink_wideselect_per_record_sink | -0.286 | moderate- |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 1/6, lost 4/6
- **abi_sink_wideselect_batched_sink_decode**: won 1/6, lost 4/6
- **abi_sink_wideselect_per_record_sink**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6331470.7ns | 2088583.0ns | 303.1% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6307244.9ns | 2079633.1ns | 303.3% | HIGH |
| abi_sink_wideselect_null_sink | 6297304.9ns | 2074071.0ns | 303.6% | HIGH |
| abi_sink_wideselect_per_record_sink | 6383437.8ns | 2105822.9ns | 303.1% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2071955.0-2113205.6 ns)
  2071955.0 |########################################
  2074017.5 |########################################
  2076080.1 |########################################
  2078142.6 |
  2080205.1 |########################################
  2082267.6 |
  2084330.2 |
  2086392.7 |
  2088455.2 |
  2090517.8 |
  2092580.3 |########################################
  2094642.8 |
  2096705.4 |
  2098767.9 |
  2100830.4 |
  2102893.0 |
  2104955.5 |
  2107018.0 |
  2109080.5 |
  2111143.1 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2072049.6-2085630.5 ns)
  2072049.6 |########################################
  2072728.6 |
  2073407.7 |
  2074086.7 |
  2074765.8 |
  2075444.8 |
  2076123.9 |
  2076802.9 |########################################
  2077481.9 |########################################
  2078161.0 |
  2078840.0 |
  2079519.1 |########################################
  2080198.1 |
  2080877.2 |
  2081556.2 |
  2082235.2 |
  2082914.3 |
  2083593.3 |
  2084272.4 |########################################
  2084951.4 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2065145.4-2081158.8 ns)
  2065145.4 |########################################
  2065946.1 |
  2066746.7 |########################################
  2067547.4 |
  2068348.1 |
  2069148.7 |
  2069949.4 |
  2070750.1 |
  2071550.7 |
  2072351.4 |########################################
  2073152.1 |
  2073952.7 |
  2074753.4 |
  2075554.1 |
  2076354.7 |########################################
  2077155.4 |
  2077956.1 |########################################
  2078756.7 |
  2079557.4 |
  2080358.1 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2080286.7-2152052.3 ns)
  2080286.7 |########################################
  2083875.0 |#############
  2087463.3 |
  2091051.5 |
  2094639.8 |#############
  2098228.1 |
  2101816.4 |
  2105404.7 |
  2108992.9 |
  2112581.2 |
  2116169.5 |
  2119757.8 |
  2123346.1 |
  2126934.3 |
  2130522.6 |
  2134110.9 |
  2137699.2 |
  2141287.5 |
  2144875.7 |
  2148464.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=303.1% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=303.6% of algo (FFI overhead may distort results)

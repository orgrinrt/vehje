# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (abi_sink_wideselect_null_sink)

The baseline abi_sink_wideselect_null_sink is the fastest (2.07 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.07 ms and 2.08 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (abi_sink_wideselect_null_sink) is the fastest** at 2072389.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 2072389.1 ns, slowest 2080276.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2091141ns | 2079001ns | 2074025ns | 2078093ns | 2119271ns | +0.79% |
| abi_sink_wideselect_batched_sink_decode | 2085062ns | 2083261ns | 2078322ns | 2082064ns | 2092928ns | +0.50% |
| abi_sink_wideselect_null_sink | 2074674ns | 2075284ns | 2066943ns | 2074146ns | 2079331ns | base |
| abi_sink_wideselect_per_record_sink | 2076829ns | 2075251ns | 2067755ns | 2073475ns | 2086398ns | +0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2088058ns | 2071110ns | 2115779ns | +0.78% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2082144ns | 2075504ns | 2090016ns | +0.50% | 0.000 |
| abi_sink_wideselect_null_sink | 2071836ns | 2064364ns | 2076401ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2074043ns | 2065224ns | 2083328ns | +0.11% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 60301.2 | 2090925.0 | 2088058.1 | n/a |
| abi_sink_wideselect_batched_sink_decode | 53851.5 | 2080130.6 | 2082143.9 | n/a |
| abi_sink_wideselect_null_sink | 50941.8 | 2072806.2 | 2071835.8 | n/a |
| abi_sink_wideselect_per_record_sink | 49894.4 | 2073369.2 | 2074043.1 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_null_sink; best 20% batches)
- Ops per call: 1

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.4% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.2% |
| abi_sink_wideselect_null_sink | 0.000 | 99.6% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2091141ns | 2091141ns | +0.79% |
| abi_sink_wideselect_batched_sink_decode | 2085062ns | 2085062ns | +0.50% |
| abi_sink_wideselect_null_sink | 2074674ns | 2074674ns | base |
| abi_sink_wideselect_per_record_sink | 2076829ns | 2076829ns | +0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2072389ns | base | --- | [2066718, 2076401] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2076134ns | no significant difference | [-3421, +45712]ns | [2072262, 2115779] | no | 0.3281 | 0.2188 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2080276ns | +6341.7ns (+0.3%) | [+1284, +23299]ns | [2076140, 2090016] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| abi_sink_wideselect_per_record_sink | 2072567ns | no significant difference | [-7297, +14458]ns | [2066235, 2083328] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2073375ns | +0.0% | +0.4% | +0.6% |
| 2 | 2064364ns | +0.6% | +1.3% | +0.8% |
| 3 | 2071403ns | +0.2% | +0.3% | -0.1% |
| 4 | 2069071ns | +3.9% | +0.9% | -0.2% |
| 5 | 2074810ns | +0.4% | +0.2% | +0.0% |
| 6 | 2077991ns | -0.3% | -0.1% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | -0.141 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.465 | moderate- |
| abi_sink_wideselect_null_sink | 0.027 | ok |
| abi_sink_wideselect_per_record_sink | 0.186 | ok |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 1/6, lost 4/6
- **abi_sink_wideselect_batched_sink_decode**: won 1/6, lost 5/6
- **abi_sink_wideselect_per_record_sink**: won 2/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6327965.1ns | 2088058.1ns | 303.1% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6296031.1ns | 2082143.9ns | 302.4% | HIGH |
| abi_sink_wideselect_null_sink | 6272337.1ns | 2071835.8ns | 302.7% | HIGH |
| abi_sink_wideselect_per_record_sink | 6273663.9ns | 2074043.1ns | 302.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2071110.0-2115778.8 ns)
  2071110.0 |####################
  2073343.4 |####################
  2075576.9 |########################################
  2077810.3 |
  2080043.8 |
  2082277.2 |####################
  2084510.6 |
  2086744.1 |
  2088977.5 |
  2091210.9 |
  2093444.4 |
  2095677.8 |
  2097911.2 |
  2100144.7 |
  2102378.1 |
  2104611.6 |
  2106845.0 |
  2109078.4 |
  2111311.9 |
  2113545.3 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2075503.7-2090016.1 ns)
  2075503.7 |########################################
  2076229.3 |########################################
  2076954.9 |
  2077680.6 |
  2078406.2 |
  2079131.8 |
  2079857.4 |########################################
  2080583.0 |########################################
  2081308.6 |
  2082034.3 |
  2082759.9 |
  2083485.5 |
  2084211.1 |
  2084936.7 |
  2085662.3 |
  2086388.0 |
  2087113.6 |
  2087839.2 |########################################
  2088564.8 |
  2089290.4 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2064363.8-2076400.6 ns)
  2064363.8 |########################################
  2064965.6 |
  2065567.5 |
  2066169.3 |
  2066771.2 |
  2067373.0 |
  2067974.8 |
  2068576.7 |########################################
  2069178.5 |
  2069780.4 |
  2070382.2 |
  2070984.0 |########################################
  2071585.9 |
  2072187.7 |
  2072789.6 |########################################
  2073391.4 |
  2073993.2 |
  2074595.1 |########################################
  2075196.9 |
  2075798.8 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2065223.8-2083327.5 ns)
  2065223.8 |########################################
  2066129.0 |
  2067034.2 |########################################
  2067939.4 |
  2068844.5 |########################################
  2069749.7 |
  2070654.9 |
  2071560.1 |
  2072465.3 |
  2073370.5 |
  2074275.6 |
  2075180.8 |########################################
  2076086.0 |
  2076991.2 |
  2077896.4 |
  2078801.6 |
  2079706.8 |########################################
  2080611.9 |
  2081517.1 |
  2082422.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=303.3% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=302.8% of algo (FFI overhead may distort results)

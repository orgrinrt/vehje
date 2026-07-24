# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_wideselect_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_wideselect_null_sink has the worst median (2.07 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_wideselect_batched_sink_decode at 2.06 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.06 ms and 2.07 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_batched_sink_decode** at 2063927.4 ns median (-0.4% vs baseline)
- Spread: 1.00x (fastest 2063927.4 ns, slowest 2073210.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2101224ns | 2067632ns | 2060289ns | 2065723ns | 2174941ns | +0.77% |
| abi_sink_wideselect_batched_sink_decode | 2069359ns | 2066831ns | 2064588ns | 2066789ns | 2075601ns | -0.75% |
| abi_sink_wideselect_null_sink | 2085076ns | 2076148ns | 2065531ns | 2074163ns | 2111217ns | base |
| abi_sink_wideselect_per_record_sink | 2099937ns | 2072999ns | 2069320ns | 2072588ns | 2156270ns | +0.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2098037ns | 2057277ns | 2171294ns | +0.78% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2066415ns | 2061807ns | 2072464ns | -0.74% | 0.000 |
| abi_sink_wideselect_null_sink | 2081827ns | 2062696ns | 2107318ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2096571ns | 2066410ns | 2152410ns | +0.71% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 59324.4 | 2092788.0 | 2098036.9 | 0 |
| abi_sink_wideselect_batched_sink_decode | 56579.8 | 2065208.7 | 2066414.8 | n/a |
| abi_sink_wideselect_null_sink | 65276.2 | 2084926.1 | 2081826.8 | n/a |
| abi_sink_wideselect_per_record_sink | 63786.0 | 2243536.5 | 2096571.3 | 0 |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.6% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.7% |
| abi_sink_wideselect_null_sink | 0.000 | 99.2% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2101224ns | 2101224ns | +0.77% |
| abi_sink_wideselect_batched_sink_decode | 2069359ns | 2069359ns | -0.75% |
| abi_sink_wideselect_null_sink | 2085076ns | 2085076ns | base |
| abi_sink_wideselect_per_record_sink | 2099937ns | 2099937ns | +0.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2073211ns | base | --- | [2064952, 2107318] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2064739ns | no significant difference | [-18423, +69710]ns | [2058078, 2171294] | no | 1.0000 | 1.0000 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2063927ns | no significant difference | [-43415, +4602]ns | [2062853, 2072464] | no | 0.6563 | 0.2188 | 0 |
| abi_sink_wideselect_per_record_sink | 2069794ns | no significant difference | [-36218, +82009]ns | [2067510, 2152410] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2072097ns | -0.3% | -0.4% | -0.1% |
| 2 | 2062696ns | +0.0% | +0.7% | +0.3% |
| 3 | 2074325ns | +2.0% | -0.3% | +7.6% |
| 4 | 2067207ns | -0.4% | -0.3% | -0.0% |
| 5 | 2085794ns | -1.4% | -1.0% | -0.8% |
| 6 | 2128842ns | +4.6% | -3.1% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | -0.172 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.009 | ok |
| abi_sink_wideselect_null_sink | 0.192 | ok |
| abi_sink_wideselect_per_record_sink | -0.251 | moderate- |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 3/6, lost 2/6
- **abi_sink_wideselect_batched_sink_decode**: won 5/6, lost 1/6
- **abi_sink_wideselect_per_record_sink**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6365250.8ns | 2098036.9ns | 303.4% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6252910.2ns | 2066414.8ns | 302.6% | HIGH |
| abi_sink_wideselect_null_sink | 6327430.0ns | 2081826.8ns | 303.9% | HIGH |
| abi_sink_wideselect_per_record_sink | 6534371.9ns | 2096571.3ns | 311.7% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2057277.1-2171293.5 ns)
  2057277.1 |########################################
  2062977.9 |#############
  2068678.7 |
  2074379.6 |
  2080080.4 |
  2085781.2 |
  2091482.0 |
  2097182.9 |
  2102883.7 |
  2108584.5 |
  2114285.3 |#############
  2119986.1 |
  2125687.0 |
  2131387.8 |
  2137088.6 |
  2142789.4 |
  2148490.3 |
  2154191.1 |
  2159891.9 |
  2165592.7 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2061807.1-2072463.9 ns)
  2061807.1 |####################
  2062339.9 |
  2062872.8 |
  2063405.6 |########################################
  2063938.5 |####################
  2064471.3 |
  2065004.2 |
  2065537.0 |
  2066069.8 |
  2066602.7 |
  2067135.5 |####################
  2067668.4 |
  2068201.2 |
  2068734.1 |
  2069266.9 |
  2069799.7 |
  2070332.6 |
  2070865.4 |
  2071398.3 |
  2071931.1 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2062696.2-2107318.0 ns)
  2062696.2 |########################################
  2064927.3 |
  2067158.4 |########################################
  2069389.5 |
  2071620.6 |########################################
  2073851.6 |########################################
  2076082.7 |
  2078313.8 |
  2080544.9 |
  2082776.0 |
  2085007.1 |########################################
  2087238.2 |
  2089469.2 |
  2091700.3 |
  2093931.4 |
  2096162.5 |
  2098393.6 |
  2100624.7 |
  2102855.8 |
  2105086.9 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2066410.4-2152409.8 ns)
  2066410.4 |########################################
  2070710.4 |##########
  2075010.3 |
  2079310.3 |
  2083610.3 |
  2087910.2 |
  2092210.2 |
  2096510.2 |
  2100810.2 |
  2105110.1 |
  2109410.1 |
  2113710.1 |
  2118010.0 |
  2122310.0 |
  2126610.0 |
  2130909.9 |
  2135209.9 |
  2139509.9 |
  2143809.9 |
  2148109.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=302.7% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=303.2% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=303.3% of algo (FFI overhead may distort results)

# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (abi_sink_wideselect_null_sink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline abi_sink_wideselect_null_sink has the worst median (2.07 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest abi_sink_wideselect_per_record_sink at 2.07 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.3% of the fastest

All 4 variants sit between 2.07 ms and 2.07 ms - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_per_record_sink** at 2067538.9 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 2067538.9 ns, slowest 2073155.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2075734ns | 2071274ns | 2063723ns | 2069046ns | 2091772ns | -1.18% |
| abi_sink_wideselect_batched_sink_decode | 2073662ns | 2072350ns | 2068455ns | 2071622ns | 2079324ns | -1.28% |
| abi_sink_wideselect_null_sink | 2100572ns | 2076168ns | 2072000ns | 2075025ns | 2153180ns | base |
| abi_sink_wideselect_per_record_sink | 2072734ns | 2070624ns | 2067318ns | 2069899ns | 2079695ns | -1.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2072572ns | 2060865ns | 2088130ns | -1.18% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2070676ns | 2065501ns | 2076185ns | -1.27% | 0.000 |
| abi_sink_wideselect_null_sink | 2097258ns | 2068736ns | 2149298ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2069716ns | 2064451ns | 2076679ns | -1.31% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 60933.3 | 2072018.2 | 2072572.4 | n/a |
| abi_sink_wideselect_batched_sink_decode | 57335.9 | 2070266.2 | 2070676.4 | n/a |
| abi_sink_wideselect_null_sink | 63526.8 | 2088617.7 | 2097258.5 | n/a |
| abi_sink_wideselect_per_record_sink | 56605.4 | 2072111.2 | 2069716.4 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 8

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.6% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.6% |
| abi_sink_wideselect_null_sink | 0.000 | 99.4% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2075734ns | 2075734ns | -1.18% |
| abi_sink_wideselect_batched_sink_decode | 2073662ns | 2073662ns | -1.28% |
| abi_sink_wideselect_null_sink | 2100572ns | 2100572ns | base |
| abi_sink_wideselect_per_record_sink | 2072734ns | 2072734ns | -1.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2073156ns | base | --- | [2069322, 2149298] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2068349ns | no significant difference | [-88060, +18381]ns | [2061239, 2088130] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2069434ns | no significant difference | [-77416, +3403]ns | [2066410, 2076185] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_wideselect_per_record_sink | 2067539ns | no significant difference | [-76754, +1925]ns | [2064932, 2076679] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2205565ns | -6.6% | -5.9% | -6.3% |
| 2 | 2093031ns | -1.5% | -1.2% | -0.7% |
| 3 | 2068736ns | +0.4% | -0.1% | +0.0% |
| 4 | 2069908ns | -0.1% | +0.1% | -0.2% |
| 5 | 2075550ns | -0.3% | -0.5% | -0.5% |
| 6 | 2070762ns | +1.3% | +0.3% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | -0.039 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.375 | moderate- |
| abi_sink_wideselect_null_sink | 0.111 | ok |
| abi_sink_wideselect_per_record_sink | -0.233 | moderate- |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 3/6, lost 2/6
- **abi_sink_wideselect_batched_sink_decode**: won 3/6, lost 1/6
- **abi_sink_wideselect_per_record_sink**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6294125.6ns | 2072572.4ns | 303.7% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6269767.9ns | 2070676.4ns | 302.8% | HIGH |
| abi_sink_wideselect_null_sink | 6350768.7ns | 2097258.5ns | 302.8% | HIGH |
| abi_sink_wideselect_per_record_sink | 6281207.6ns | 2069716.4ns | 303.5% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2060864.6-2088129.6 ns)
  2060864.6 |########################################
  2062227.9 |
  2063591.1 |
  2064954.4 |
  2066317.6 |
  2067680.9 |########################################
  2069044.1 |
  2070407.4 |
  2071770.6 |
  2073133.9 |
  2074497.1 |
  2075860.4 |
  2077223.6 |####################
  2078586.9 |
  2079950.1 |
  2081313.4 |
  2082676.6 |
  2084039.9 |
  2085403.1 |
  2086766.4 |
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2065500.8-2076184.8 ns)
  2065500.8 |####################
  2066035.0 |
  2066569.2 |
  2067103.4 |########################################
  2067637.6 |
  2068171.8 |
  2068706.0 |
  2069240.2 |
  2069774.4 |
  2070308.6 |
  2070842.8 |####################
  2071377.0 |
  2071911.2 |
  2072445.4 |
  2072979.6 |
  2073513.8 |
  2074048.0 |
  2074582.2 |
  2075116.4 |
  2075650.6 |####################
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2068735.8-2149298.1 ns)
  2068735.8 |########################################
  2072763.9 |#############
  2076792.0 |
  2080820.1 |
  2084848.3 |
  2088876.4 |
  2092904.5 |#############
  2096932.6 |
  2100960.7 |
  2104988.8 |
  2109017.0 |
  2113045.1 |
  2117073.2 |
  2121101.3 |
  2125129.4 |
  2129157.5 |
  2133185.6 |
  2137213.8 |
  2141241.9 |
  2145270.0 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2064450.8-2076678.8 ns)
  2064450.8 |########################################
  2065062.2 |########################################
  2065673.6 |########################################
  2066285.0 |
  2066896.4 |
  2067507.8 |
  2068119.2 |
  2068730.6 |########################################
  2069342.0 |
  2069953.4 |
  2070564.8 |
  2071176.2 |
  2071787.6 |
  2072399.0 |
  2073010.4 |
  2073621.8 |
  2074233.2 |########################################
  2074844.6 |
  2075456.0 |
  2076067.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=302.4% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=302.8% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=303.7% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=303.5% of algo (FFI overhead may distort results)

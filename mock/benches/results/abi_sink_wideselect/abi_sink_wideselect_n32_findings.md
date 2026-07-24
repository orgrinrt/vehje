# abi_sink (wideselect)

4 variants, 6 samples per variant.
Baseline: **abi_sink_wideselect_null_sink**

## Highlights

Baseline for all deltas below: **abi_sink_wideselect_null_sink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.4% of the fastest

All 4 variants sit between 2.07 ms and 2.07 ms - a 0.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: abi_sink_wideselect_batched_sink** at 2065216.9 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 2065216.9 ns, slowest 2073425.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2066963ns | 2068317ns | 2058108ns | 2066821ns | 2071604ns | -0.08% |
| abi_sink_wideselect_batched_sink_decode | 2082338ns | 2076604ns | 2061575ns | 2073451ns | 2106051ns | +0.66% |
| abi_sink_wideselect_null_sink | 2068621ns | 2069515ns | 2062338ns | 2068006ns | 2072685ns | base |
| abi_sink_wideselect_per_record_sink | 2088074ns | 2074721ns | 2069750ns | 2073644ns | 2118880ns | +0.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2064008ns | 2055439ns | 2068605ns | -0.08% | 0.000 |
| abi_sink_wideselect_batched_sink_decode | 2079117ns | 2058626ns | 2102458ns | +0.65% | 0.000 |
| abi_sink_wideselect_null_sink | 2065658ns | 2059182ns | 2069703ns | base | 0.000 |
| abi_sink_wideselect_per_record_sink | 2084987ns | 2066898ns | 2115576ns | +0.94% | 0.000 |

## Setup vs iteration cost (per call)

| Variant | setup S (ns) | first-touch (ns) | run I (ns) | k* vs base |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 56546.0 | 2067480.0 | 2064008.0 | 1 |
| abi_sink_wideselect_batched_sink_decode | 63397.4 | 2082253.9 | 2079117.2 | n/a |
| abi_sink_wideselect_null_sink | 54851.2 | 2067229.2 | 2065657.6 | n/a |
| abi_sink_wideselect_per_record_sink | 60932.3 | 2086948.2 | 2084986.9 | n/a |

Setup S is the one-time build cost (timed on every call by the matrix scaffold, so it cannot hide in untimed prep). Run I is the calibrated per-iteration cost. First-touch is the cold first pass before caches and the predictor warm. k* is the iteration count at which a higher-setup, lower-per-iteration variant repays its setup against the baseline.

## Performance model

- Peak throughput: **0.000 Gops/s** (abi_sink_wideselect_batched_sink; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.000 | 99.5% |
| abi_sink_wideselect_batched_sink_decode | 0.000 | 99.1% |
| abi_sink_wideselect_null_sink | 0.000 | 99.5% |
| abi_sink_wideselect_per_record_sink | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| abi_sink_wideselect_batched_sink | 2066963ns | 2066963ns | -0.08% |
| abi_sink_wideselect_batched_sink_decode | 2082338ns | 2082338ns | +0.66% |
| abi_sink_wideselect_null_sink | 2068621ns | 2068621ns | base |
| abi_sink_wideselect_per_record_sink | 2088074ns | 2088074ns | +0.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| abi_sink_wideselect_null_sink | 2066540ns | base | --- | [2060730, 2069703] | --- | --- | --- | --- |
| abi_sink_wideselect_batched_sink | 2065217ns | no significant difference | [-8820, +2987]ns | [2058202, 2068605] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_wideselect_batched_sink_decode | 2073425ns | no significant difference | [-5554, +38777]ns | [2061468, 2102458] | no | 0.6875 | 0.6875 | 0 |
| abi_sink_wideselect_per_record_sink | 2071691ns | +9540.7ns (+0.5%) | [+371, +48076]ns | [2067694, 2115576] | YES (adj: no) | 0.6563 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | abi_sink_wideselect_null_sink | abi_sink_wideselect_batched_sink | abi_sink_wideselect_batched_sink_decode | abi_sink_wideselect_per_record_sink |
|---|---|---|---|---|
| 1 | 2059182ns | +0.2% | +0.7% | +0.5% |
| 2 | 2065719ns | +0.1% | +3.1% | +0.3% |
| 3 | 2067360ns | +0.1% | +0.3% | +0.6% |
| 4 | 2067639ns | +0.0% | +0.4% | +4.0% |
| 5 | 2062278ns | -0.3% | -0.2% | +0.5% |
| 6 | 2071767ns | -0.5% | -0.4% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| abi_sink_wideselect_batched_sink | 0.169 | ok |
| abi_sink_wideselect_batched_sink_decode | -0.058 | ok |
| abi_sink_wideselect_null_sink | -0.249 | moderate- |
| abi_sink_wideselect_per_record_sink | -0.118 | ok |

**Consistency summary:**

- **abi_sink_wideselect_batched_sink**: won 2/6, lost 2/6
- **abi_sink_wideselect_batched_sink_decode**: won 2/6, lost 4/6
- **abi_sink_wideselect_per_record_sink**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| abi_sink_wideselect_batched_sink | 6260399.5ns | 2064008.0ns | 303.3% | HIGH |
| abi_sink_wideselect_batched_sink_decode | 6309651.6ns | 2079117.2ns | 303.5% | HIGH |
| abi_sink_wideselect_null_sink | 6256687.8ns | 2065657.6ns | 302.9% | HIGH |
| abi_sink_wideselect_per_record_sink | 6302177.9ns | 2084986.9ns | 302.3% | HIGH |

## Distribution (algo ns)

```
abi_sink_wideselect_batched_sink (n=6, range 2055439.2-2068604.8 ns)
  2055439.2 |####################
  2056097.5 |
  2056755.8 |
  2057414.0 |
  2058072.3 |
  2058730.6 |
  2059388.9 |
  2060047.2 |
  2060705.4 |####################
  2061363.7 |
  2062022.0 |####################
  2062680.3 |
  2063338.6 |
  2063996.8 |
  2064655.1 |
  2065313.4 |
  2065971.7 |
  2066630.0 |
  2067288.2 |
  2067946.5 |########################################
  (0 below, 1 above range)

abi_sink_wideselect_batched_sink_decode (n=6, range 2058625.8-2102458.5 ns)
  2058625.8 |####################
  2060817.4 |
  2063009.1 |####################
  2065200.7 |
  2067392.3 |
  2069584.0 |
  2071775.6 |########################################
  2073967.2 |####################
  2076158.9 |
  2078350.5 |
  2080542.1 |
  2082733.8 |
  2084925.4 |
  2087117.1 |
  2089308.7 |
  2091500.3 |
  2093692.0 |
  2095883.6 |
  2098075.2 |
  2100266.9 |
  (0 below, 1 above range)

abi_sink_wideselect_null_sink (n=6, range 2059182.5-2069702.9 ns)
  2059182.5 |########################################
  2059708.5 |
  2060234.5 |
  2060760.6 |
  2061286.6 |
  2061812.6 |########################################
  2062338.6 |
  2062864.7 |
  2063390.7 |
  2063916.7 |
  2064442.7 |
  2064968.7 |
  2065494.8 |########################################
  2066020.8 |
  2066546.8 |
  2067072.8 |########################################
  2067598.9 |########################################
  2068124.9 |
  2068650.9 |
  2069176.9 |
  (0 below, 1 above range)

abi_sink_wideselect_per_record_sink (n=6, range 2066897.9-2115576.2 ns)
  2066897.9 |########################################
  2069331.8 |####################
  2071765.7 |####################
  2074199.7 |
  2076633.6 |
  2079067.5 |####################
  2081501.4 |
  2083935.3 |
  2086369.2 |
  2088803.2 |
  2091237.1 |
  2093671.0 |
  2096104.9 |
  2098538.8 |
  2100972.7 |
  2103406.7 |
  2105840.6 |
  2108274.5 |
  2110708.4 |
  2113142.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **abi_sink_wideselect_batched_sink**: bridge=303.5% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_batched_sink_decode**: bridge=302.9% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_null_sink**: bridge=303.0% of algo (FFI overhead may distort results)
- **abi_sink_wideselect_per_record_sink**: bridge=302.9% of algo (FFI overhead may distort results)

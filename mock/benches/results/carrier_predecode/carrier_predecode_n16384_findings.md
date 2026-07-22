# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (2.56 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flat at 2.04 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flat is fastest but the noisiest (CV 5.2%)

carrier_predec_flat wins on median (2.04 ms) yet has the highest variance (CV 5.2%), while carrier_predec_wire is the steadiest (CV 1.0%, 2.56 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_predec_flat** at 2039440.0 ns median (-20.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.25x (fastest 2039440.0 ns, slowest 2555220.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 2084684ns | 2042298ns | 2014022ns | 2037721ns | 2190459ns | -18.50% |
| carrier_predec_flatthread | 2096311ns | 2068964ns | 2002885ns | 2067497ns | 2186243ns | -18.04% |
| carrier_predec_wire | 2557809ns | 2558762ns | 2516155ns | 2555121ns | 2582670ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 2081377ns | 2011535ns | 2186138ns | -18.51% | 0.008 |
| carrier_predec_flatthread | 2093637ns | 2000480ns | 2183391ns | -18.03% | 0.008 |
| carrier_predec_wire | 2554252ns | 2512435ns | 2579116ns | base | 0.006 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_predec_flatthread; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.008 | 98.1% |
| carrier_predec_flatthread | 0.008 | 96.8% |
| carrier_predec_wire | 0.006 | 78.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 2084684ns | 2084684ns | -18.50% |
| carrier_predec_flatthread | 2096311ns | 2096311ns | -18.04% |
| carrier_predec_wire | 2557809ns | 2557809ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 2555221ns | base | --- | [2528418, 2579116] | --- | --- | --- | --- |
| carrier_predec_flat | 2039440ns | -510829.6ns (-20.0%) | [-541915, -365879]ns | [2018553, 2186138] | YES | 0.0313 | 0.0313 | 0 |
| carrier_predec_flatthread | 2066303ns | -498482.0ns (-19.5%) | [-533608, -349754]ns | [2031217, 2183391] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat | carrier_predec_flatthread |
|---|---|---|---|
| 1 | 2512435ns | -19.9% | -17.9% |
| 2 | 2546738ns | -9.1% | -9.8% |
| 3 | 2592366ns | -21.9% | -20.2% |
| 4 | 2565867ns | -20.2% | -19.4% |
| 5 | 2563704ns | -19.8% | -19.5% |
| 6 | 2544401ns | -20.2% | -21.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | -0.380 | moderate- |
| carrier_predec_flatthread | -0.138 | ok |
| carrier_predec_wire | 0.136 | ok |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6
- **carrier_predec_flatthread**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 63703.8ns | 2081377.2ns | 3.1% |  |
| carrier_predec_flatthread | 57795.1ns | 2093637.3ns | 2.8% |  |
| carrier_predec_wire | 2884.0ns | 2554251.8ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 2011534.6-2186138.3 ns)
  2011534.6 |########################################
  2020264.8 |########################################
  2028995.0 |########################################
  2037725.2 |
  2046455.3 |########################################
  2055185.5 |########################################
  2063915.7 |
  2072645.9 |
  2081376.1 |
  2090106.3 |
  2098836.5 |
  2107566.6 |
  2116296.8 |
  2125027.0 |
  2133757.2 |
  2142487.4 |
  2151217.6 |
  2159947.7 |
  2168677.9 |
  2177408.1 |
  (0 below, 1 above range)

carrier_predec_flatthread (n=6, range 2000479.6-2183391.5 ns)
  2000479.6 |####################
  2009625.2 |
  2018770.8 |
  2027916.4 |
  2037062.0 |
  2046207.6 |
  2055353.2 |########################################
  2064498.7 |########################################
  2073644.3 |
  2082789.9 |
  2091935.5 |
  2101081.1 |
  2110226.7 |
  2119372.3 |
  2128517.9 |
  2137663.5 |
  2146809.1 |
  2155954.7 |
  2165100.3 |
  2174245.9 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 2512435.4-2579116.2 ns)
  2512435.4 |########################################
  2515769.4 |
  2519103.5 |
  2522437.5 |
  2525771.6 |
  2529105.6 |
  2532439.7 |
  2535773.7 |
  2539107.7 |
  2542441.8 |########################################
  2545775.8 |########################################
  2549109.9 |
  2552443.9 |
  2555778.0 |
  2559112.0 |
  2562446.0 |########################################
  2565780.1 |########################################
  2569114.1 |
  2572448.2 |
  2575782.2 |
  (0 below, 1 above range)

```

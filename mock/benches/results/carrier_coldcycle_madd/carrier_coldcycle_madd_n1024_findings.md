# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), madd profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_madd_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_madd_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_madd_null dominates: 10% faster than the next best (carrier_cold_madd_switch)

carrier_cold_madd_null (42.63 us) leads carrier_cold_madd_switch (47.03 us) by 10%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: carrier_cold_madd_null** at 42625.6 ns median (-9.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 42625.6 ns, slowest 48183.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_madd_fntable | 51099ns | 50462ns | 49834ns | 50342ns | 52867ns | +3.39% |
| carrier_cold_madd_null | 44980ns | 44916ns | 44187ns | 44834ns | 45594ns | -8.99% |
| carrier_cold_madd_switch | 49423ns | 49367ns | 48671ns | 49342ns | 49919ns | base |
| carrier_cold_madd_threaded | 49848ns | 49653ns | 49064ns | 49593ns | 50621ns | +0.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_madd_fntable | 48735ns | 47622ns | 50372ns | +3.53% | 0.021 |
| carrier_cold_madd_null | 42643ns | 41922ns | 43196ns | -9.41% | 0.024 |
| carrier_cold_madd_switch | 47073ns | 46281ns | 47634ns | base | 0.022 |
| carrier_cold_madd_threaded | 47491ns | 46898ns | 48149ns | +0.89% | 0.022 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_madd_fntable | 457130 | 1250591 | 0.366 | 1.02× |
| carrier_cold_madd_null | 497101 | 1356528 | 0.366 | 1.11× |
| carrier_cold_madd_switch | 449294 | 1020337 | 0.440 | 1.00× |
| carrier_cold_madd_threaded | 450594 | 1069636 | 0.421 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.024 Gops/s** (carrier_cold_madd_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_madd_fntable | 0.021 | 87.0% |
| carrier_cold_madd_null | 0.024 | 98.3% |
| carrier_cold_madd_switch | 0.022 | 89.1% |
| carrier_cold_madd_threaded | 0.022 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_madd_fntable | 51099ns | 51099ns | +3.39% |
| carrier_cold_madd_null | 44980ns | 44980ns | -8.99% |
| carrier_cold_madd_switch | 49423ns | 49423ns | base |
| carrier_cold_madd_threaded | 49848ns | 49848ns | +0.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_madd_switch | 47026ns | base | --- | [46559, 47634] | --- | --- | --- | --- |
| carrier_cold_madd_fntable | 48184ns | +1317.1ns (+2.8%) | [+931, +2738]ns | [47650, 50372] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_madd_null | 42626ns | -4683.5ns (-10.0%) | [-4892, -3715]ns | [42108, 43196] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_madd_threaded | 47264ns | no significant difference | [-554, +1543]ns | [47058, 48149] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_madd_switch | carrier_cold_madd_fntable | carrier_cold_madd_null | carrier_cold_madd_threaded |
|---|---|---|---|---|
| 1 | 47587ns | +6.7% | -9.9% | -1.4% |
| 2 | 46838ns | +2.6% | -10.5% | +0.8% |
| 3 | 47681ns | +4.8% | -10.2% | -0.9% |
| 4 | 47122ns | +2.5% | -7.6% | +0.3% |
| 5 | 46281ns | +3.0% | -8.3% | +2.3% |
| 6 | 46931ns | +1.5% | -9.9% | +4.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_madd_fntable | -0.121 | ok |
| carrier_cold_madd_null | -0.152 | ok |
| carrier_cold_madd_switch | -0.120 | ok |
| carrier_cold_madd_threaded | 0.029 | ok |

**Consistency summary:**

- **carrier_cold_madd_fntable**: won 0/6, lost 6/6
- **carrier_cold_madd_null**: won 6/6, lost 0/6
- **carrier_cold_madd_threaded**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_madd_fntable | 97748.5ns | 48735.4ns | 200.6% | HIGH |
| carrier_cold_madd_null | 114091.0ns | 42642.9ns | 267.5% | HIGH |
| carrier_cold_madd_switch | 94195.3ns | 47073.1ns | 200.1% | HIGH |
| carrier_cold_madd_threaded | 95188.8ns | 47490.6ns | 200.4% | HIGH |

## Distribution (algo ns)

```
carrier_cold_madd_fntable (n=6, range 47622.5-50372.1 ns)
  47622.5 |########################################
  47760.0 |
  47897.5 |
  48034.9 |####################
  48172.4 |####################
  48309.9 |
  48447.4 |
  48584.9 |
  48722.3 |
  48859.8 |
  48997.3 |
  49134.8 |
  49272.3 |
  49409.7 |
  49547.2 |
  49684.7 |
  49822.2 |####################
  49959.7 |
  50097.1 |
  50234.6 |
  (0 below, 1 above range)

carrier_cold_madd_null (n=6, range 41921.7-43195.6 ns)
  41921.7 |########################################
  41985.4 |
  42049.1 |
  42112.8 |
  42176.5 |
  42240.2 |########################################
  42303.9 |
  42367.6 |
  42431.3 |########################################
  42495.0 |
  42558.7 |
  42622.4 |
  42686.1 |
  42749.8 |########################################
  42813.5 |########################################
  42877.2 |
  42940.9 |
  43004.6 |
  43068.3 |
  43132.0 |
  (0 below, 1 above range)

carrier_cold_madd_switch (n=6, range 46281.2-47633.8 ns)
  46281.2 |########################################
  46348.8 |
  46416.5 |
  46484.1 |
  46551.7 |
  46619.3 |
  46687.0 |
  46754.6 |
  46822.2 |########################################
  46889.8 |########################################
  46957.5 |
  47025.1 |
  47092.7 |########################################
  47160.4 |
  47228.0 |
  47295.6 |
  47363.2 |
  47430.9 |
  47498.5 |
  47566.1 |########################################
  (0 below, 1 above range)

carrier_cold_madd_threaded (n=6, range 46898.3-48149.2 ns)
  46898.3 |#############
  46960.8 |
  47023.4 |
  47085.9 |
  47148.5 |
  47211.0 |########################################
  47273.6 |#############
  47336.1 |
  47398.7 |
  47461.2 |
  47523.8 |
  47586.3 |
  47648.8 |
  47711.4 |
  47773.9 |
  47836.5 |
  47899.0 |
  47961.6 |
  48024.1 |
  48086.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_madd_fntable**: bridge=200.5% of algo (FFI overhead may distort results)
- **carrier_cold_madd_null**: bridge=263.7% of algo (FFI overhead may distort results)
- **carrier_cold_madd_switch**: bridge=200.2% of algo (FFI overhead may distort results)
- **carrier_cold_madd_threaded**: bridge=200.4% of algo (FFI overhead may distort results)

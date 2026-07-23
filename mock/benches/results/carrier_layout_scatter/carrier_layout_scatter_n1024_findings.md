# Record layout (REC12..REC32) with fixed switch dispatch, scatter profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_scatter_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_scatter_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_lay_scatter_rec24) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_lay_scatter_rec24 has the worst median (41.91 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_lay_scatter_rec20 at 41.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 1.8% of the fastest

All 5 variants sit between 41.18 us and 41.91 us - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_scatter_rec20** at 41181.9 ns median (-1.7% vs baseline)
- Spread: 1.02x (fastest 41181.9 ns, slowest 41907.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 44425ns | 43509ns | 43082ns | 43470ns | 46530ns | -0.06% |
| carrier_lay_scatter_rec16 | 44552ns | 43775ns | 42831ns | 43477ns | 47024ns | +0.22% |
| carrier_lay_scatter_rec20 | 43564ns | 43531ns | 42805ns | 43383ns | 44215ns | -2.00% |
| carrier_lay_scatter_rec24 | 44453ns | 44217ns | 42886ns | 44064ns | 45819ns | base |
| carrier_lay_scatter_rec32 | 44314ns | 43623ns | 43099ns | 43488ns | 46161ns | -0.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 42178ns | 40973ns | 44209ns | +0.13% | 0.024 |
| carrier_lay_scatter_rec16 | 42314ns | 40712ns | 44698ns | +0.45% | 0.024 |
| carrier_lay_scatter_rec20 | 41322ns | 40682ns | 42027ns | -1.90% | 0.025 |
| carrier_lay_scatter_rec24 | 42123ns | 40770ns | 43443ns | base | 0.024 |
| carrier_lay_scatter_rec32 | 42001ns | 40968ns | 43667ns | -0.29% | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 488688 | 2195682 | 0.223 | 1.04× |
| carrier_lay_scatter_rec16 | 468669 | 2098280 | 0.223 | 1.00× |
| carrier_lay_scatter_rec20 | 497073 | 2224046 | 0.223 | 1.06× |
| carrier_lay_scatter_rec24 | 467884 | 2091572 | 0.224 | 1.00× |
| carrier_lay_scatter_rec32 | 481268 | 2149302 | 0.224 | 1.03× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.025 Gops/s** (carrier_lay_scatter_rec20; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_scatter_rec12 | 0.025 | 98.7% |
| carrier_lay_scatter_rec16 | 0.025 | 98.0% |
| carrier_lay_scatter_rec20 | 0.025 | 98.8% |
| carrier_lay_scatter_rec24 | 0.024 | 97.1% |
| carrier_lay_scatter_rec32 | 0.025 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_scatter_rec12 | 44425ns | 44425ns | -0.06% |
| carrier_lay_scatter_rec16 | 44552ns | 44552ns | +0.22% |
| carrier_lay_scatter_rec20 | 43564ns | 43564ns | -2.00% |
| carrier_lay_scatter_rec24 | 44453ns | 44453ns | base |
| carrier_lay_scatter_rec32 | 44314ns | 44314ns | -0.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_scatter_rec24 | 41908ns | base | --- | [41017, 43443] | --- | --- | --- | --- |
| carrier_lay_scatter_rec12 | 41238ns | no significant difference | [-1406, +2143]ns | [41087, 44209] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec16 | 41508ns | no significant difference | [-1127, +1561]ns | [40737, 44698] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_scatter_rec20 | 41182ns | no significant difference | [-1918, +76]ns | [40757, 42027] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_scatter_rec32 | 41322ns | no significant difference | [-2411, +2297]ns | [41015, 43667] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_scatter_rec24 | carrier_lay_scatter_rec12 | carrier_lay_scatter_rec16 | carrier_lay_scatter_rec20 | carrier_lay_scatter_rec32 |
|---|---|---|---|---|---|
| 1 | 40770ns | +1.1% | -0.1% | -0.2% | +0.7% |
| 2 | 42341ns | -3.2% | +5.2% | -1.2% | -3.2% |
| 3 | 44532ns | -3.2% | +0.8% | -5.2% | -7.7% |
| 4 | 42355ns | -2.7% | -3.6% | -3.6% | -1.9% |
| 5 | 41265ns | +0.0% | +2.3% | +0.6% | +8.3% |
| 6 | 41475ns | +9.3% | -1.7% | -1.5% | +2.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_scatter_rec12 | -0.187 | ok |
| carrier_lay_scatter_rec16 | -0.074 | ok |
| carrier_lay_scatter_rec20 | -0.245 | moderate- |
| carrier_lay_scatter_rec24 | 0.129 | ok |
| carrier_lay_scatter_rec32 | 0.269 | moderate+ |

**Consistency summary:**

- **carrier_lay_scatter_rec12**: won 3/6, lost 2/6
- **carrier_lay_scatter_rec16**: won 3/6, lost 3/6
- **carrier_lay_scatter_rec20**: won 5/6, lost 1/6
- **carrier_lay_scatter_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_scatter_rec12 | 113250.3ns | 42177.7ns | 268.5% | HIGH |
| carrier_lay_scatter_rec16 | 108374.5ns | 42314.1ns | 256.1% | HIGH |
| carrier_lay_scatter_rec20 | 115392.4ns | 41321.7ns | 279.3% | HIGH |
| carrier_lay_scatter_rec24 | 107707.6ns | 42122.8ns | 255.7% | HIGH |
| carrier_lay_scatter_rec32 | 111508.4ns | 42001.2ns | 265.5% | HIGH |

## Distribution (algo ns)

```
carrier_lay_scatter_rec12 (n=6, range 40972.9-44208.8 ns)
  40972.9 |#############
  41134.7 |########################################
  41296.5 |
  41458.3 |
  41620.1 |
  41781.9 |
  41943.7 |
  42105.4 |
  42267.2 |
  42429.0 |
  42590.8 |
  42752.6 |
  42914.4 |
  43076.2 |#############
  43238.0 |
  43399.8 |
  43561.6 |
  43723.4 |
  43885.2 |
  44047.0 |
  (0 below, 1 above range)

carrier_lay_scatter_rec16 (n=6, range 40712.1-44697.7 ns)
  40712.1 |########################################
  40911.4 |
  41110.7 |
  41309.9 |
  41509.2 |
  41708.5 |
  41907.8 |
  42107.1 |#############
  42306.3 |
  42505.6 |
  42704.9 |
  42904.2 |
  43103.5 |
  43302.7 |
  43502.0 |
  43701.3 |
  43900.6 |
  44099.9 |
  44299.1 |
  44498.4 |#############
  (0 below, 1 above range)

carrier_lay_scatter_rec20 (n=6, range 40682.1-42026.7 ns)
  40682.1 |####################
  40749.3 |
  40816.6 |########################################
  40883.8 |
  40951.0 |
  41018.2 |
  41085.5 |
  41152.7 |
  41219.9 |
  41287.1 |
  41354.4 |
  41421.6 |
  41488.8 |####################
  41556.1 |
  41623.3 |
  41690.5 |
  41757.7 |
  41825.0 |####################
  41892.2 |
  41959.4 |
  (0 below, 1 above range)

carrier_lay_scatter_rec24 (n=6, range 40769.6-43443.3 ns)
  40769.6 |####################
  40903.3 |
  41037.0 |
  41170.7 |####################
  41304.3 |
  41438.0 |####################
  41571.7 |
  41705.4 |
  41839.1 |
  41972.8 |
  42106.5 |
  42240.2 |########################################
  42373.8 |
  42507.5 |
  42641.2 |
  42774.9 |
  42908.6 |
  43042.3 |
  43176.0 |
  43309.7 |
  (0 below, 1 above range)

carrier_lay_scatter_rec32 (n=6, range 40968.3-43666.9 ns)
  40968.3 |########################################
  41103.2 |
  41238.2 |
  41373.1 |
  41508.0 |#############
  41642.9 |
  41777.9 |
  41912.8 |
  42047.7 |
  42182.6 |
  42317.6 |
  42452.5 |
  42587.4 |#############
  42722.4 |
  42857.3 |
  42992.2 |
  43127.1 |
  43262.1 |
  43397.0 |
  43531.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_scatter_rec12**: bridge=298.1% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec16**: bridge=271.9% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec20**: bridge=285.9% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec24**: bridge=255.3% of algo (FFI overhead may distort results)
- **carrier_lay_scatter_rec32**: bridge=284.6% of algo (FFI overhead may distort results)

# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (3.89 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 838.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 dominates: 62% faster than the next best (carrier_vert_real_vert4)

carrier_vert_real_vert8 (838.68 us) leads carrier_vert_real_vert4 (1.36 ms) by 62%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_real_vert8 beats baseline by 78% (significant)

carrier_vert_real_vert8 is -3.04 ms (78%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 4.6x slower than the field

carrier_vert_real_scalar (3.89 ms) is 4.6x the fastest (838.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.6x the fastest

Fastest carrier_vert_real_vert8 (838.68 us) to slowest carrier_vert_real_scalar (3.89 ms): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 838684.6 ns median (-78.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.64x (fastest 838684.6 ns, slowest 3889839.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 3891203ns | 3893401ns | 3791818ns | 3882010ns | 3954686ns | base |
| carrier_vert_real_vert4 | 1355092ns | 1359465ns | 1337795ns | 1352577ns | 1367514ns | -65.18% |
| carrier_vert_real_vert8 | 840534ns | 841719ns | 822057ns | 839283ns | 851650ns | -78.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 3887718ns | 3788650ns | 3950997ns | base | 0.001 |
| carrier_vert_real_vert4 | 1351865ns | 1334609ns | 1364253ns | -65.23% | 0.003 |
| carrier_vert_real_vert8 | 837388ns | 818615ns | 848330ns | -78.46% | 0.005 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_real_scalar | 23937418 | 21072176 | 1.136 | 1.00× |
| carrier_vert_real_vert4 | 8418656 | 7969483 | 1.056 | 0.35× |
| carrier_vert_real_vert8 | 5307962 | 5606288 | 0.947 | 0.22× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.005 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.001 | 21.0% |
| carrier_vert_real_vert4 | 0.003 | 60.4% |
| carrier_vert_real_vert8 | 0.005 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 3891203ns | 3891203ns | base |
| carrier_vert_real_vert4 | 1355092ns | 1355092ns | -65.18% |
| carrier_vert_real_vert8 | 840534ns | 840534ns | -78.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 3889839ns | base | --- | [3822319, 3950997] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 1356386ns | -2526237.3ns (-64.9%) | [-2606552, -2474771]ns | [1334956, 1364253] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 838685ns | -3041509.3ns (-78.2%) | [-3125848, -2983634]ns | [825149, 848330] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 3878578ns | -64.7% | -78.0% |
| 2 | 3990359ns | -66.1% | -79.5% |
| 3 | 3855988ns | -64.7% | -78.3% |
| 4 | 3911634ns | -65.9% | -78.7% |
| 5 | 3788650ns | -64.8% | -77.8% |
| 6 | 3901101ns | -65.2% | -78.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | -0.390 | moderate- |
| carrier_vert_real_vert4 | 0.102 | ok |
| carrier_vert_real_vert8 | -0.357 | moderate- |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 3874628.8ns | 3887718.3ns | 99.7% | HIGH |
| carrier_vert_real_vert4 | 1360227.2ns | 1351865.1ns | 100.6% | HIGH |
| carrier_vert_real_vert8 | 860827.2ns | 837387.7ns | 102.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 3788650.4-3950996.7 ns)
  3788650.4 |########################################
  3796767.7 |
  3804885.0 |
  3813002.3 |
  3821119.7 |
  3829237.0 |
  3837354.3 |
  3845471.6 |
  3853588.9 |########################################
  3861706.2 |
  3869823.5 |
  3877940.9 |########################################
  3886058.2 |
  3894175.5 |########################################
  3902292.8 |
  3910410.1 |########################################
  3918527.4 |
  3926644.8 |
  3934762.1 |
  3942879.4 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 1334608.8-1364253.1 ns)
  1334608.8 |########################################
  1336091.0 |
  1337573.2 |
  1339055.4 |
  1340537.7 |
  1342019.9 |
  1343502.1 |
  1344984.3 |
  1346466.5 |
  1347948.7 |
  1349431.0 |
  1350913.2 |
  1352395.4 |
  1353877.6 |####################
  1355359.8 |
  1356842.0 |
  1358324.2 |########################################
  1359806.5 |
  1361288.7 |
  1362770.9 |
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 818615.0-848330.0 ns)
  818615.0 |########################################
  820100.8 |
  821586.5 |
  823072.2 |
  824558.0 |
  826043.8 |
  827529.5 |
  829015.2 |
  830501.0 |########################################
  831986.8 |
  833472.5 |
  834958.2 |########################################
  836444.0 |
  837929.8 |
  839415.5 |
  840901.2 |########################################
  842387.0 |
  843872.8 |########################################
  845358.5 |
  846844.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: bridge=99.6% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=103.8% of algo (FFI overhead may distort results)

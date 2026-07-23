# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2.90 us) is smaller than the fastest variant's own run-to-run std-dev (3.87 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.5% of the fastest

All 5 variants sit between 191.13 us and 194.03 us - a 1.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_madd_rec16** at 191127.9 ns median (-0.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 191127.9 ns, slowest 194032.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 195558ns | 194625ns | 193424ns | 194265ns | 198565ns | -0.21% |
| carrier_lay_madd_rec16 | 194867ns | 193762ns | 191431ns | 193490ns | 198651ns | -0.56% |
| carrier_lay_madd_rec20 | 194447ns | 194094ns | 193642ns | 194049ns | 195448ns | -0.77% |
| carrier_lay_madd_rec24 | 195963ns | 195039ns | 193925ns | 194746ns | 198808ns | base |
| carrier_lay_madd_rec32 | 197844ns | 196614ns | 194928ns | 196428ns | 201425ns | +0.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 193014ns | 190926ns | 196083ns | -0.22% | 0.021 |
| carrier_lay_madd_rec16 | 192346ns | 189093ns | 196167ns | -0.57% | 0.021 |
| carrier_lay_madd_rec20 | 191830ns | 190950ns | 192776ns | -0.83% | 0.021 |
| carrier_lay_madd_rec24 | 193440ns | 191231ns | 196441ns | base | 0.021 |
| carrier_lay_madd_rec32 | 195274ns | 192258ns | 198878ns | +0.95% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 1200778 | 4801570 | 0.250 | 1.00× |
| carrier_lay_madd_rec16 | 1202224 | 4801774 | 0.250 | 1.00× |
| carrier_lay_madd_rec20 | 1200662 | 4801832 | 0.250 | 1.00× |
| carrier_lay_madd_rec24 | 1205845 | 4801933 | 0.251 | 1.00× |
| carrier_lay_madd_rec32 | 1219656 | 4801918 | 0.254 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_madd_rec16; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 98.5% |
| carrier_lay_madd_rec16 | 0.021 | 98.9% |
| carrier_lay_madd_rec20 | 0.021 | 98.7% |
| carrier_lay_madd_rec24 | 0.021 | 98.2% |
| carrier_lay_madd_rec32 | 0.021 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 195558ns | 195558ns | -0.21% |
| carrier_lay_madd_rec16 | 194867ns | 194867ns | -0.56% |
| carrier_lay_madd_rec20 | 194447ns | 194447ns | -0.77% |
| carrier_lay_madd_rec24 | 195963ns | 195963ns | base |
| carrier_lay_madd_rec32 | 197844ns | 197844ns | +0.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 192571ns | base | --- | [191309, 196441] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 191894ns | no significant difference | [-1144, +584]ns | [191067, 196083] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_madd_rec16 | 191128ns | no significant difference | [-2588, +547]ns | [189745, 196167] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 191566ns | no significant difference | [-5186, +676]ns | [191148, 192776] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 194032ns | +1877.4ns (+1.0%) | [+303, +3320]ns | [192911, 198878] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 199885ns | -0.3% | +0.5% | -4.5% | +1.6% |
| 2 | 191231ns | -0.2% | +0.1% | +0.2% | +1.8% |
| 3 | 192997ns | -0.6% | -0.8% | -0.7% | -0.4% |
| 4 | 192255ns | -0.5% | -1.6% | +0.5% | +0.7% |
| 5 | 192887ns | -0.4% | -1.0% | -0.3% | +0.8% |
| 6 | 191387ns | +0.8% | -0.5% | -0.0% | +1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.127 | ok |
| carrier_lay_madd_rec16 | 0.034 | ok |
| carrier_lay_madd_rec20 | 0.100 | ok |
| carrier_lay_madd_rec24 | -0.208 | moderate- |
| carrier_lay_madd_rec32 | 0.068 | ok |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 5/6, lost 1/6
- **carrier_lay_madd_rec16**: won 4/6, lost 1/6
- **carrier_lay_madd_rec20**: won 3/6, lost 2/6
- **carrier_lay_madd_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 193349.6ns | 193014.5ns | 100.2% | HIGH |
| carrier_lay_madd_rec16 | 192651.9ns | 192346.4ns | 100.2% | HIGH |
| carrier_lay_madd_rec20 | 192223.6ns | 191830.1ns | 100.2% | HIGH |
| carrier_lay_madd_rec24 | 193804.1ns | 193440.1ns | 100.2% | HIGH |
| carrier_lay_madd_rec32 | 195310.5ns | 195273.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 190926.2-196082.9 ns)
  190926.2 |########################################
  191184.0 |########################################
  191441.9 |
  191699.7 |########################################
  191957.5 |########################################
  192215.4 |
  192473.2 |
  192731.0 |########################################
  192988.9 |
  193246.7 |
  193504.6 |
  193762.4 |
  194020.2 |
  194278.1 |
  194535.9 |
  194793.7 |
  195051.6 |
  195309.4 |
  195567.2 |
  195825.1 |
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 189092.9-196166.6 ns)
  189092.9 |####################
  189446.6 |
  189800.3 |
  190154.0 |####################
  190507.6 |
  190861.3 |####################
  191215.0 |########################################
  191568.7 |
  191922.4 |
  192276.1 |
  192629.8 |
  192983.5 |
  193337.1 |
  193690.8 |
  194044.5 |
  194398.2 |
  194751.9 |
  195105.6 |
  195459.3 |
  195813.0 |
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 190950.0-192776.5 ns)
  190950.0 |####################
  191041.3 |
  191132.6 |
  191224.0 |
  191315.3 |####################
  191406.6 |
  191498.0 |########################################
  191589.3 |
  191680.6 |
  191771.9 |
  191863.2 |
  191954.6 |
  192045.9 |
  192137.2 |
  192228.5 |####################
  192319.9 |
  192411.2 |
  192502.5 |
  192593.9 |
  192685.2 |
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 191230.8-196440.7 ns)
  191230.8 |########################################
  191491.3 |
  191751.8 |
  192012.3 |####################
  192272.8 |
  192533.3 |
  192793.8 |########################################
  193054.2 |
  193314.7 |
  193575.2 |
  193835.7 |
  194096.2 |
  194356.7 |
  194617.2 |
  194877.7 |
  195138.2 |
  195398.7 |
  195659.2 |
  195919.7 |
  196180.2 |
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 192257.5-198878.1 ns)
  192257.5 |########################################
  192588.5 |
  192919.6 |
  193250.6 |########################################
  193581.6 |########################################
  193912.6 |
  194243.7 |########################################
  194574.7 |########################################
  194905.7 |
  195236.8 |
  195567.8 |
  195898.8 |
  196229.9 |
  196560.9 |
  196891.9 |
  197222.9 |
  197554.0 |
  197885.0 |
  198216.0 |
  198547.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=100.1% of algo (FFI overhead may distort results)

# Record layout (REC12..REC32) with fixed switch dispatch, wideselect profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_wideselect_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_wideselect_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_lay_wideselect_rec24)

The baseline carrier_lay_wideselect_rec24 is the fastest (9.73 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 3.0% of the fastest

All 5 variants sit between 9.73 us and 10.03 us - a 3.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_wideselect_rec24) is the fastest** at 9734.0 ns median
- Spread: 1.03x (fastest 9734.0 ns, slowest 10028.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 12485ns | 12276ns | 11995ns | 12227ns | 13117ns | +3.48% |
| carrier_lay_wideselect_rec16 | 12211ns | 12256ns | 11747ns | 12092ns | 12621ns | +1.21% |
| carrier_lay_wideselect_rec20 | 12196ns | 12216ns | 11730ns | 12092ns | 12586ns | +1.09% |
| carrier_lay_wideselect_rec24 | 12065ns | 12021ns | 11640ns | 11945ns | 12458ns | base |
| carrier_lay_wideselect_rec32 | 12240ns | 12188ns | 11787ns | 12058ns | 12740ns | +1.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 10208ns | 9796ns | 10728ns | +4.50% | 0.025 |
| carrier_lay_wideselect_rec16 | 9892ns | 9555ns | 10176ns | +1.26% | 0.026 |
| carrier_lay_wideselect_rec20 | 9911ns | 9565ns | 10234ns | +1.45% | 0.026 |
| carrier_lay_wideselect_rec24 | 9769ns | 9392ns | 10079ns | base | 0.026 |
| carrier_lay_wideselect_rec32 | 9917ns | 9573ns | 10276ns | +1.52% | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 313571 | 1603736 | 0.196 | 1.00× |
| carrier_lay_wideselect_rec16 | 310072 | 1524195 | 0.203 | 0.99× |
| carrier_lay_wideselect_rec20 | 311013 | 1529859 | 0.203 | 0.99× |
| carrier_lay_wideselect_rec24 | 314165 | 1545240 | 0.203 | 1.00× |
| carrier_lay_wideselect_rec32 | 312369 | 1535653 | 0.203 | 0.99× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.027 Gops/s** (carrier_lay_wideselect_rec24; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.026 | 93.7% |
| carrier_lay_wideselect_rec16 | 0.026 | 94.5% |
| carrier_lay_wideselect_rec20 | 0.026 | 94.9% |
| carrier_lay_wideselect_rec24 | 0.026 | 96.5% |
| carrier_lay_wideselect_rec32 | 0.026 | 94.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_wideselect_rec12 | 12485ns | 12485ns | +3.48% |
| carrier_lay_wideselect_rec16 | 12211ns | 12211ns | +1.21% |
| carrier_lay_wideselect_rec20 | 12196ns | 12196ns | +1.09% |
| carrier_lay_wideselect_rec24 | 12065ns | 12065ns | base |
| carrier_lay_wideselect_rec32 | 12240ns | 12240ns | +1.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_wideselect_rec24 | 9734ns | base | --- | [9493, 10079] | --- | --- | --- | --- |
| carrier_lay_wideselect_rec12 | 10028ns | no significant difference | [-139, +923]ns | [9868, 10728] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_wideselect_rec16 | 9936ns | no significant difference | [-244, +371]ns | [9563, 10176] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_wideselect_rec20 | 9899ns | no significant difference | [-65, +387]ns | [9600, 10234] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_wideselect_rec32 | 9901ns | no significant difference | [-90, +438]ns | [9575, 10276] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_wideselect_rec24 | carrier_lay_wideselect_rec12 | carrier_lay_wideselect_rec16 | carrier_lay_wideselect_rec20 | carrier_lay_wideselect_rec32 |
|---|---|---|---|---|---|
| 1 | 10280ns | -4.7% | -4.5% | -0.3% | +0.1% |
| 2 | 9735ns | +2.1% | +3.3% | -1.0% | -1.6% |
| 3 | 9392ns | +7.3% | +1.7% | +3.1% | +2.0% |
| 4 | 9877ns | +9.4% | +3.5% | +2.4% | +3.5% |
| 5 | 9733ns | +9.4% | +4.1% | +5.0% | +5.5% |
| 6 | 9595ns | +4.0% | -0.2% | -0.3% | -0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_wideselect_rec12 | 0.265 | moderate+ |
| carrier_lay_wideselect_rec16 | -0.426 | moderate- |
| carrier_lay_wideselect_rec20 | -0.241 | moderate- |
| carrier_lay_wideselect_rec24 | -0.096 | ok |
| carrier_lay_wideselect_rec32 | -0.182 | ok |

**Consistency summary:**

- **carrier_lay_wideselect_rec12**: won 1/6, lost 5/6
- **carrier_lay_wideselect_rec16**: won 2/6, lost 4/6
- **carrier_lay_wideselect_rec20**: won 3/6, lost 3/6
- **carrier_lay_wideselect_rec32**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_wideselect_rec12 | 90164.5ns | 10208.1ns | 883.3% | HIGH |
| carrier_lay_wideselect_rec16 | 90326.6ns | 9891.7ns | 913.2% | HIGH |
| carrier_lay_wideselect_rec20 | 90877.9ns | 9910.8ns | 917.0% | HIGH |
| carrier_lay_wideselect_rec24 | 90538.7ns | 9768.7ns | 926.8% | HIGH |
| carrier_lay_wideselect_rec32 | 91265.5ns | 9917.0ns | 920.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_wideselect_rec12 (n=6, range 9795.8-10727.7 ns)
   9795.8 |####################
   9842.4 |
   9889.0 |
   9935.6 |########################################
   9982.2 |
  10028.8 |
  10075.4 |####################
  10122.0 |
  10168.6 |
  10215.2 |
  10261.8 |
  10308.3 |
  10354.9 |
  10401.5 |
  10448.1 |
  10494.7 |
  10541.3 |
  10587.9 |
  10634.5 |####################
  10681.1 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec16 (n=6, range 9554.6-10175.8 ns)
   9554.6 |########################################
   9585.7 |
   9616.7 |
   9647.8 |
   9678.8 |
   9709.9 |
   9741.0 |
   9772.0 |
   9803.1 |####################
   9834.1 |
   9865.2 |
   9896.3 |
   9927.3 |
   9958.4 |
   9989.4 |
  10020.5 |
  10051.6 |####################
  10082.6 |
  10113.7 |####################
  10144.7 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec20 (n=6, range 9564.6-10233.5 ns)
   9564.6 |########################################
   9598.0 |
   9631.5 |########################################
   9664.9 |########################################
   9698.4 |
   9731.8 |
   9765.3 |
   9798.7 |
   9832.2 |
   9865.6 |
   9899.1 |
   9932.5 |
   9966.0 |
   9999.4 |
  10032.9 |
  10066.3 |
  10099.8 |########################################
  10133.2 |
  10166.7 |
  10200.1 |########################################
  (0 below, 1 above range)

carrier_lay_wideselect_rec24 (n=6, range 9392.1-10078.8 ns)
   9392.1 |####################
   9426.4 |
   9460.8 |
   9495.1 |
   9529.4 |
   9563.8 |####################
   9598.1 |
   9632.4 |
   9666.8 |
   9701.1 |########################################
   9735.4 |
   9769.8 |
   9804.1 |
   9838.4 |
   9872.8 |####################
   9907.1 |
   9941.4 |
   9975.8 |
  10010.1 |
  10044.4 |
  (0 below, 1 above range)

carrier_lay_wideselect_rec32 (n=6, range 9573.3-10275.6 ns)
   9573.3 |########################################
   9608.4 |
   9643.5 |
   9678.6 |
   9713.8 |
   9748.9 |
   9784.0 |
   9819.1 |
   9854.2 |
   9889.3 |
   9924.5 |
   9959.6 |
   9994.7 |
  10029.8 |
  10064.9 |
  10100.0 |
  10135.1 |
  10170.3 |
  10205.4 |#############
  10240.5 |#############
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_wideselect_rec12**: bridge=901.3% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec16**: bridge=906.4% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec20**: bridge=919.8% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec24**: bridge=917.5% of algo (FFI overhead may distort results)
- **carrier_lay_wideselect_rec32**: bridge=922.6% of algo (FFI overhead may distort results)

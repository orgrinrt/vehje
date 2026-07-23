# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec12's edge over baseline is significant but tiny (-13 ns, 0.14%)

carrier_lay_leaf_rec12 differs from baseline carrier_lay_leaf_rec24 by -13 ns (0.14%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_lay_leaf_rec20** at 9009.5 ns median (-3.1% vs baseline)
- Spread: 1.06x (fastest 9009.5 ns, slowest 9538.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 11604ns | 11942ns | 10587ns | 11509ns | 12256ns | +0.59% |
| carrier_lay_leaf_rec16 | 11416ns | 11662ns | 10650ns | 11352ns | 11896ns | -1.04% |
| carrier_lay_leaf_rec20 | 11330ns | 11248ns | 10680ns | 11140ns | 11941ns | -1.78% |
| carrier_lay_leaf_rec24 | 11536ns | 11676ns | 10627ns | 11336ns | 12292ns | base |
| carrier_lay_leaf_rec32 | 11646ns | 11958ns | 10615ns | 11534ns | 12331ns | +0.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 9237ns | 8453ns | 9756ns | +0.24% | 0.028 |
| carrier_lay_leaf_rec16 | 9120ns | 8467ns | 9539ns | -1.03% | 0.028 |
| carrier_lay_leaf_rec20 | 9044ns | 8490ns | 9529ns | -1.85% | 0.028 |
| carrier_lay_leaf_rec24 | 9215ns | 8469ns | 9869ns | base | 0.028 |
| carrier_lay_leaf_rec32 | 9311ns | 8459ns | 9922ns | +1.04% | 0.027 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 302865 | 1529443 | 0.198 | 1.00× |
| carrier_lay_leaf_rec16 | 302214 | 1535394 | 0.197 | 1.00× |
| carrier_lay_leaf_rec20 | 305421 | 1546588 | 0.197 | 1.01× |
| carrier_lay_leaf_rec24 | 302335 | 1534938 | 0.197 | 1.00× |
| carrier_lay_leaf_rec32 | 303306 | 1529845 | 0.198 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.030 Gops/s** (carrier_lay_leaf_rec12; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.027 | 89.1% |
| carrier_lay_leaf_rec16 | 0.027 | 90.8% |
| carrier_lay_leaf_rec20 | 0.028 | 93.8% |
| carrier_lay_leaf_rec24 | 0.028 | 90.9% |
| carrier_lay_leaf_rec32 | 0.027 | 88.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 11604ns | 11604ns | +0.59% |
| carrier_lay_leaf_rec16 | 11416ns | 11416ns | -1.04% |
| carrier_lay_leaf_rec20 | 11330ns | 11330ns | -1.78% |
| carrier_lay_leaf_rec24 | 11536ns | 11536ns | base |
| carrier_lay_leaf_rec32 | 11646ns | 11646ns | +0.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 9298ns | base | --- | [8478, 9869] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 9490ns | no significant difference | [-195, +274]ns | [8465, 9756] | no | 1.0000 | 0.6875 | 0 |
| carrier_lay_leaf_rec16 | 9310ns | no significant difference | [-484, +202]ns | [8513, 9539] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec20 | 9010ns | no significant difference | [-704, +444]ns | [8595, 9529] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec32 | 9539ns | no significant difference | [-331, +624]ns | [8471, 9922] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 8486ns | -0.1% | -0.2% | +5.4% | -0.3% |
| 2 | 9606ns | +0.6% | +0.2% | -5.5% | -0.7% |
| 3 | 9579ns | -1.1% | -1.3% | -9.2% | +7.4% |
| 4 | 8469ns | -0.2% | +1.1% | +0.2% | +0.2% |
| 5 | 9018ns | +5.4% | +3.5% | +4.8% | +5.9% |
| 6 | 10132ns | -2.8% | -8.3% | -5.2% | -5.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | -0.244 | moderate- |
| carrier_lay_leaf_rec16 | -0.366 | moderate- |
| carrier_lay_leaf_rec20 | 0.195 | ok |
| carrier_lay_leaf_rec24 | -0.199 | ok |
| carrier_lay_leaf_rec32 | -0.366 | moderate- |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 4/6, lost 2/6
- **carrier_lay_leaf_rec16**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec20**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec32**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 91563.2ns | 9236.9ns | 991.3% | HIGH |
| carrier_lay_leaf_rec16 | 90948.2ns | 9120.5ns | 997.2% | HIGH |
| carrier_lay_leaf_rec20 | 91108.0ns | 9044.3ns | 1007.4% | HIGH |
| carrier_lay_leaf_rec24 | 91697.5ns | 9215.1ns | 995.1% | HIGH |
| carrier_lay_leaf_rec32 | 92292.4ns | 9310.6ns | 991.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 8452.9-9755.8 ns)
   8452.9 |########################################
   8518.0 |
   8583.2 |
   8648.3 |
   8713.5 |
   8778.6 |
   8843.8 |
   8908.9 |
   8974.1 |
   9039.2 |
   9104.3 |
   9169.5 |
   9234.6 |
   9299.8 |
   9364.9 |
   9430.1 |####################
   9495.2 |####################
   9560.4 |
   9625.5 |####################
   9690.7 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 8466.7-9538.5 ns)
   8466.7 |########################################
   8520.3 |########################################
   8573.9 |
   8627.5 |
   8681.1 |
   8734.7 |
   8788.3 |
   8841.8 |
   8895.4 |
   8949.0 |
   9002.6 |
   9056.2 |
   9109.8 |
   9163.4 |
   9217.0 |
   9270.6 |########################################
   9324.2 |########################################
   9377.8 |
   9431.4 |########################################
   9485.0 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 8489.6-9528.8 ns)
   8489.6 |########################################
   8541.6 |
   8593.5 |
   8645.5 |
   8697.4 |########################################
   8749.4 |
   8801.3 |
   8853.3 |
   8905.3 |########################################
   8957.2 |
   9009.2 |
   9061.1 |########################################
   9113.1 |
   9165.0 |
   9217.0 |
   9269.0 |
   9320.9 |
   9372.9 |
   9424.8 |########################################
   9476.8 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 8469.2-9869.4 ns)
   8469.2 |########################################
   8539.2 |
   8609.2 |
   8679.2 |
   8749.2 |
   8819.2 |
   8889.2 |
   8959.3 |####################
   9029.3 |
   9099.3 |
   9169.3 |
   9239.3 |
   9309.3 |
   9379.3 |
   9449.3 |
   9519.3 |####################
   9589.3 |####################
   9659.3 |
   9729.3 |
   9799.3 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 8459.2-9921.7 ns)
   8459.2 |##########################
   8532.3 |
   8605.5 |
   8678.6 |
   8751.7 |
   8824.8 |
   8898.0 |
   8971.1 |
   9044.2 |
   9117.3 |
   9190.5 |
   9263.6 |
   9336.7 |
   9409.8 |
   9483.0 |########################################
   9556.1 |
   9629.2 |
   9702.3 |
   9775.5 |
   9848.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=965.9% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=981.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=1008.3% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=988.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=967.6% of algo (FFI overhead may distort results)

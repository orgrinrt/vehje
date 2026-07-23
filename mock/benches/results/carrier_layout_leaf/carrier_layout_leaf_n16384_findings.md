# Record layout (REC12..REC32) with fixed switch dispatch, leaf profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_leaf_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_leaf_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_leaf_rec20 shows alternating (throttle bounce) (autocorr -0.51)

carrier_lay_leaf_rec20's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (16.01 us) is smaller than the fastest variant's own run-to-run std-dev (16.24 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.4% of the fastest

All 5 variants sit between 1.14 ms and 1.15 ms - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_leaf_rec12** at 1135142.5 ns median (-0.8% vs baseline)
- Spread: 1.01x (fastest 1135142.5 ns, slowest 1151151.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1147635ns | 1138370ns | 1134655ns | 1137832ns | 1168829ns | -0.35% |
| carrier_lay_leaf_rec16 | 1144033ns | 1142651ns | 1129998ns | 1139303ns | 1158146ns | -0.66% |
| carrier_lay_leaf_rec20 | 1175025ns | 1154266ns | 1136773ns | 1149264ns | 1232794ns | +2.03% |
| carrier_lay_leaf_rec24 | 1151665ns | 1146714ns | 1135382ns | 1145191ns | 1169516ns | base |
| carrier_lay_leaf_rec32 | 1162864ns | 1145523ns | 1137905ns | 1143042ns | 1205076ns | +0.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1144706ns | 1131862ns | 1166180ns | -0.34% | 0.014 |
| carrier_lay_leaf_rec16 | 1141272ns | 1127578ns | 1155189ns | -0.64% | 0.014 |
| carrier_lay_leaf_rec20 | 1172038ns | 1134444ns | 1229314ns | +2.04% | 0.014 |
| carrier_lay_leaf_rec24 | 1148629ns | 1133085ns | 1166186ns | base | 0.014 |
| carrier_lay_leaf_rec32 | 1159758ns | 1134151ns | 1201902ns | +0.97% | 0.014 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 7107659 | 17913544 | 0.397 | 0.99× |
| carrier_lay_leaf_rec16 | 7099738 | 17911020 | 0.396 | 0.99× |
| carrier_lay_leaf_rec20 | 7157046 | 17917587 | 0.399 | 1.00× |
| carrier_lay_leaf_rec24 | 7146397 | 17912333 | 0.399 | 1.00× |
| carrier_lay_leaf_rec32 | 7151743 | 17918993 | 0.399 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.015 Gops/s** (carrier_lay_leaf_rec16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.014 | 99.3% |
| carrier_lay_leaf_rec16 | 0.014 | 98.9% |
| carrier_lay_leaf_rec20 | 0.014 | 98.0% |
| carrier_lay_leaf_rec24 | 0.014 | 98.6% |
| carrier_lay_leaf_rec32 | 0.014 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_leaf_rec12 | 1147635ns | 1147635ns | -0.35% |
| carrier_lay_leaf_rec16 | 1144033ns | 1144033ns | -0.66% |
| carrier_lay_leaf_rec20 | 1175025ns | 1175025ns | +2.03% |
| carrier_lay_leaf_rec24 | 1151665ns | 1151665ns | base |
| carrier_lay_leaf_rec32 | 1162864ns | 1162864ns | +0.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_leaf_rec24 | 1143941ns | base | --- | [1135760, 1166186] | --- | --- | --- | --- |
| carrier_lay_leaf_rec12 | 1135142ns | no significant difference | [-22056, +20181]ns | [1132797, 1166180] | no | 0.8750 | 0.2188 | 0 |
| carrier_lay_leaf_rec16 | 1139697ns | no significant difference | [-36531, +17455]ns | [1128930, 1155189] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_leaf_rec20 | 1151152ns | no significant difference | [-29370, +92775]ns | [1135646, 1229314] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_leaf_rec32 | 1142831ns | no significant difference | [-18539, +56601]ns | [1134540, 1201902] | no | 0.9167 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_leaf_rec24 | carrier_lay_leaf_rec12 | carrier_lay_leaf_rec16 | carrier_lay_leaf_rec20 | carrier_lay_leaf_rec32 |
|---|---|---|---|---|---|
| 1 | 1182146ns | -2.2% | -4.6% | -3.8% | -1.9% |
| 2 | 1133085ns | +3.8% | +1.3% | +13.0% | +9.8% |
| 3 | 1147888ns | -1.2% | -1.5% | -1.2% | +0.2% |
| 4 | 1139993ns | -0.5% | +1.0% | +3.4% | -0.5% |
| 5 | 1138435ns | -0.2% | +1.8% | +0.9% | -0.3% |
| 6 | 1150225ns | -1.6% | -1.6% | +0.3% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_leaf_rec12 | 0.231 | moderate+ |
| carrier_lay_leaf_rec16 | -0.302 | moderate- |
| carrier_lay_leaf_rec20 | -0.512 | HIGH- (thermal bounce) |
| carrier_lay_leaf_rec24 | -0.279 | moderate- |
| carrier_lay_leaf_rec32 | 0.076 | ok |

**Consistency summary:**

- **carrier_lay_leaf_rec12**: won 5/6, lost 1/6
- **carrier_lay_leaf_rec16**: won 3/6, lost 3/6
- **carrier_lay_leaf_rec20**: won 2/6, lost 4/6
- **carrier_lay_leaf_rec32**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_leaf_rec12 | 1148378.2ns | 1144706.4ns | 100.3% | HIGH |
| carrier_lay_leaf_rec16 | 1142997.7ns | 1141271.9ns | 100.2% | HIGH |
| carrier_lay_leaf_rec20 | 1168459.4ns | 1172037.5ns | 99.7% | HIGH |
| carrier_lay_leaf_rec24 | 1149405.5ns | 1148628.9ns | 100.1% | HIGH |
| carrier_lay_leaf_rec32 | 1160981.2ns | 1159757.9ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_leaf_rec12 (n=6, range 1131862.5-1166180.0 ns)
  1131862.5 |####################
  1133578.4 |########################################
  1135294.2 |####################
  1137010.1 |
  1138726.0 |
  1140441.9 |
  1142157.8 |
  1143873.6 |
  1145589.5 |
  1147305.4 |
  1149021.2 |
  1150737.1 |
  1152453.0 |
  1154168.9 |
  1155884.8 |####################
  1157600.6 |
  1159316.5 |
  1161032.4 |
  1162748.2 |
  1164464.1 |
  (0 below, 1 above range)

carrier_lay_leaf_rec16 (n=6, range 1127578.3-1155188.8 ns)
  1127578.3 |########################################
  1128958.8 |########################################
  1130339.3 |
  1131719.9 |########################################
  1133100.4 |
  1134480.9 |
  1135861.4 |
  1137242.0 |
  1138622.5 |
  1140003.0 |
  1141383.5 |
  1142764.0 |
  1144144.6 |
  1145525.1 |
  1146905.6 |########################################
  1148286.1 |
  1149666.7 |
  1151047.2 |########################################
  1152427.7 |
  1153808.2 |
  (0 below, 1 above range)

carrier_lay_leaf_rec20 (n=6, range 1134443.8-1229314.4 ns)
  1134443.8 |########################################
  1139187.3 |
  1143930.9 |####################
  1148674.4 |
  1153417.9 |####################
  1158161.4 |
  1162905.0 |
  1167648.5 |
  1172392.0 |
  1177135.6 |####################
  1181879.1 |
  1186622.6 |
  1191366.2 |
  1196109.7 |
  1200853.2 |
  1205596.8 |
  1210340.3 |
  1215083.8 |
  1219827.3 |
  1224570.9 |
  (0 below, 1 above range)

carrier_lay_leaf_rec24 (n=6, range 1133085.4-1166185.6 ns)
  1133085.4 |########################################
  1134740.4 |
  1136395.4 |
  1138050.4 |########################################
  1139705.4 |########################################
  1141360.4 |
  1143015.5 |
  1144670.5 |
  1146325.5 |########################################
  1147980.5 |
  1149635.5 |########################################
  1151290.5 |
  1152945.5 |
  1154600.5 |
  1156255.5 |
  1157910.6 |
  1159565.6 |
  1161220.6 |
  1162875.6 |
  1164530.6 |
  (0 below, 1 above range)

carrier_lay_leaf_rec32 (n=6, range 1134150.8-1201902.5 ns)
  1134150.8 |########################################
  1137538.4 |
  1140926.0 |
  1144313.6 |
  1147701.1 |#############
  1151088.7 |
  1154476.3 |
  1157863.9 |#############
  1161251.5 |
  1164639.1 |
  1168026.6 |
  1171414.2 |
  1174801.8 |
  1178189.4 |
  1181577.0 |
  1184964.6 |
  1188352.2 |
  1191739.7 |
  1195127.3 |
  1198514.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_leaf_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec20**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec24**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_leaf_rec32**: bridge=100.1% of algo (FFI overhead may distort results)

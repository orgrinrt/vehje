# Entropy x locality surface: op_correlation {0,500,900} x locality_window {4,64,unbounded}, fixed predecoded switch dispatch

9 variants, 6 samples per variant.
Baseline: **carrier_ent_c0_w64**

## Highlights

Baseline for all deltas below: **carrier_ent_c0_w64**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ent_c900_wmax beats baseline by 26% (significant)

carrier_ent_c900_wmax is -2.29 us (26%) faster than baseline carrier_ent_c0_w64, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_ent_c0_w64 shows alternating (throttle bounce) (autocorr -0.54)

carrier_ent_c0_w64's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_ent_c900_w64, carrier_ent_c900_wmax, carrier_ent_c500_w64, carrier_ent_c500_wmax, carrier_ent_c500_w4, carrier_ent_c0_w4, carrier_ent_c0_w64, carrier_ent_c0_wmax} vs {carrier_ent_c900_w4} (26% apart)

The field splits into a fast tier {carrier_ent_c900_w64, carrier_ent_c900_wmax, carrier_ent_c500_w64, carrier_ent_c500_wmax, carrier_ent_c500_w4, carrier_ent_c0_w4, carrier_ent_c0_w64, carrier_ent_c0_wmax} and a slow tier {carrier_ent_c900_w4} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### carrier_ent_c0_wmax's edge over baseline is significant but tiny (46 ns, 0.51%)

carrier_ent_c0_wmax differs from baseline carrier_ent_c0_w64 by 46 ns (0.51%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: carrier_ent_c900_w64** at 6589.8 ns median (-26.7% vs baseline)
- 4 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.76x (fastest 6589.8 ns, slowest 11585.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 11280ns | 11293ns | 10623ns | 11118ns | 11850ns | -1.02% |
| carrier_ent_c0_w64 | 11396ns | 11446ns | 10621ns | 11235ns | 12025ns | base |
| carrier_ent_c0_wmax | 11528ns | 11751ns | 10611ns | 11425ns | 12141ns | +1.16% |
| carrier_ent_c500_w4 | 10794ns | 11107ns | 9885ns | 10747ns | 11320ns | -5.28% |
| carrier_ent_c500_w64 | 10652ns | 10988ns | 9772ns | 10636ns | 11115ns | -6.53% |
| carrier_ent_c500_wmax | 10783ns | 11100ns | 9830ns | 10798ns | 11239ns | -5.38% |
| carrier_ent_c900_w4 | 13928ns | 14139ns | 12570ns | 14108ns | 14337ns | +22.21% |
| carrier_ent_c900_w64 | 9070ns | 8911ns | 8610ns | 8838ns | 9647ns | -20.42% |
| carrier_ent_c900_wmax | 9179ns | 9216ns | 8535ns | 9023ns | 9734ns | -19.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ent_c0_w4 | 8853ns | 8438ns | 9262ns | -1.57% | 0.029 |
| carrier_ent_c0_w64 | 8993ns | 8415ns | 9498ns | base | 0.028 |
| carrier_ent_c0_wmax | 9059ns | 8407ns | 9544ns | +0.73% | 0.028 |
| carrier_ent_c500_w4 | 8313ns | 7687ns | 8701ns | -7.56% | 0.031 |
| carrier_ent_c500_w64 | 8237ns | 7598ns | 8596ns | -8.41% | 0.031 |
| carrier_ent_c500_wmax | 8285ns | 7602ns | 8615ns | -7.88% | 0.031 |
| carrier_ent_c900_w4 | 11404ns | 10337ns | 11700ns | +26.81% | 0.022 |
| carrier_ent_c900_w64 | 6730ns | 6358ns | 7191ns | -25.16% | 0.038 |
| carrier_ent_c900_wmax | 6776ns | 6353ns | 7155ns | -24.66% | 0.038 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 304381 | 947026 | 0.321 | 1.00× |
| carrier_ent_c0_w64 | 305910 | 952017 | 0.321 | 1.00× |
| carrier_ent_c0_wmax | 303151 | 945254 | 0.321 | 0.99× |
| carrier_ent_c500_w4 | 291146 | 995040 | 0.293 | 0.95× |
| carrier_ent_c500_w64 | 289810 | 1005297 | 0.288 | 0.95× |
| carrier_ent_c500_wmax | 285713 | 989017 | 0.289 | 0.93× |
| carrier_ent_c900_w4 | 302687 | 789161 | 0.384 | 0.99× |
| carrier_ent_c900_w64 | 287621 | 1192772 | 0.241 | 0.94× |
| carrier_ent_c900_wmax | 286045 | 1188282 | 0.241 | 0.94× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_ent_c900_wmax; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ent_c0_w4 | 0.029 | 72.0% |
| carrier_ent_c0_w64 | 0.028 | 70.7% |
| carrier_ent_c0_wmax | 0.028 | 68.9% |
| carrier_ent_c500_w4 | 0.030 | 74.4% |
| carrier_ent_c500_w64 | 0.030 | 75.1% |
| carrier_ent_c500_wmax | 0.030 | 74.6% |
| carrier_ent_c900_w4 | 0.022 | 54.8% |
| carrier_ent_c900_w64 | 0.039 | 96.4% |
| carrier_ent_c900_wmax | 0.038 | 93.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ent_c0_w4 | 11280ns | 11280ns | -1.02% |
| carrier_ent_c0_w64 | 11396ns | 11396ns | base |
| carrier_ent_c0_wmax | 11528ns | 11528ns | +1.16% |
| carrier_ent_c500_w4 | 10794ns | 10794ns | -5.28% |
| carrier_ent_c500_w64 | 10652ns | 10652ns | -6.53% |
| carrier_ent_c500_wmax | 10783ns | 10783ns | -5.38% |
| carrier_ent_c900_w4 | 13928ns | 13928ns | +22.21% |
| carrier_ent_c900_w64 | 9070ns | 9070ns | -20.42% |
| carrier_ent_c900_wmax | 9179ns | 9179ns | -19.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ent_c0_w64 | 8985ns | base | --- | [8497, 9498] | --- | --- | --- | --- |
| carrier_ent_c0_w4 | 8820ns | no significant difference | [-623, +321]ns | [8476, 9262] | no | 0.2917 | 0.2188 | 0 |
| carrier_ent_c0_wmax | 9218ns | no significant difference | [-556, +705]ns | [8414, 9544] | no | 0.6875 | 0.6875 | 0 |
| carrier_ent_c500_w4 | 8534ns | -797.0ns (-8.9%) | [-890, -353]ns | [7705, 8701] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| carrier_ent_c500_w64 | 8456ns | -901.8ns (-10.0%) | [-1310, -57]ns | [7660, 8596] | YES (adj: no) | 0.2917 | 0.2188 | 0 |
| carrier_ent_c500_wmax | 8513ns | no significant difference | [-1341, +67]ns | [7727, 8615] | no | 0.6875 | 0.6875 | 0 |
| carrier_ent_c900_w4 | 11585ns | +2170.2ns (+24.2%) | [+1875, +3187]ns | [10928, 11700] | YES (adj: no) | 0.0833 | 0.0313 | 0 |
| carrier_ent_c900_w64 | 6590ns | -2203.0ns (-24.5%) | [-2908, -1678]ns | [6410, 7191] | YES (adj: no) | 0.0833 | 0.0313 | 0 |
| carrier_ent_c900_wmax | 6817ns | -2293.1ns (-25.5%) | [-2698, -1662]ns | [6354, 7155] | YES (adj: no) | 0.0833 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ent_c0_w64 | carrier_ent_c0_w4 | carrier_ent_c0_wmax | carrier_ent_c500_w4 | carrier_ent_c500_w64 | carrier_ent_c500_wmax | carrier_ent_c900_w4 | carrier_ent_c900_w64 | carrier_ent_c900_wmax |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 8610ns | -2.0% | +9.5% | -10.3% | -2.4% | -11.7% | +36.7% | -26.2% | -26.2% |
| 2 | 9471ns | -0.7% | +0.5% | -8.4% | -9.0% | -10.1% | +22.6% | -30.4% | -24.6% |
| 3 | 8415ns | +8.4% | +7.1% | +0.1% | +1.1% | +1.2% | +38.2% | -23.2% | -14.8% |
| 4 | 9359ns | -8.3% | -10.0% | -7.6% | -17.5% | -8.0% | +23.5% | -23.0% | -32.1% |
| 5 | 9525ns | -4.9% | +0.5% | -8.4% | -10.0% | -17.6% | +20.9% | -30.9% | -25.1% |
| 6 | 8580ns | -0.8% | -2.0% | -10.4% | -11.4% | +0.4% | +20.5% | -16.4% | -24.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ent_c0_w4 | -0.360 | moderate- |
| carrier_ent_c0_w64 | -0.537 | HIGH- (thermal bounce) |
| carrier_ent_c0_wmax | -0.337 | moderate- |
| carrier_ent_c500_w4 | -0.222 | moderate- |
| carrier_ent_c500_w64 | -0.349 | moderate- |
| carrier_ent_c500_wmax | -0.321 | moderate- |
| carrier_ent_c900_w4 | 0.037 | ok |
| carrier_ent_c900_w64 | -0.259 | moderate- |
| carrier_ent_c900_wmax | -0.501 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_ent_c0_w4**: won 5/6, lost 1/6
- **carrier_ent_c0_wmax**: won 2/6, lost 4/6
- **carrier_ent_c500_w4**: won 5/6, lost 1/6
- **carrier_ent_c500_w64**: won 5/6, lost 1/6
- **carrier_ent_c500_wmax**: won 4/6, lost 2/6
- **carrier_ent_c900_w4**: won 0/6, lost 6/6
- **carrier_ent_c900_w64**: won 6/6, lost 0/6
- **carrier_ent_c900_wmax**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ent_c0_w4 | 90643.3ns | 8852.6ns | 1023.9% | HIGH |
| carrier_ent_c0_w64 | 92510.9ns | 8993.4ns | 1028.7% | HIGH |
| carrier_ent_c0_wmax | 92382.7ns | 9058.6ns | 1019.8% | HIGH |
| carrier_ent_c500_w4 | 89255.3ns | 8313.4ns | 1073.6% | HIGH |
| carrier_ent_c500_w64 | 89583.1ns | 8237.3ns | 1087.5% | HIGH |
| carrier_ent_c500_wmax | 88639.2ns | 8284.8ns | 1069.9% | HIGH |
| carrier_ent_c900_w4 | 92547.1ns | 11404.2ns | 811.5% | HIGH |
| carrier_ent_c900_w64 | 87783.2ns | 6730.3ns | 1304.3% | HIGH |
| carrier_ent_c900_wmax | 87936.1ns | 6775.6ns | 1297.8% | HIGH |

## Distribution (algo ns)

```
carrier_ent_c0_w4 (n=6, range 8437.5-9262.1 ns)
   8437.5 |########################################
   8478.7 |########################################
   8520.0 |
   8561.2 |########################################
   8602.4 |
   8643.6 |
   8684.9 |
   8726.1 |
   8767.3 |
   8808.6 |
   8849.8 |
   8891.0 |
   8932.3 |
   8973.5 |
   9014.7 |
   9056.0 |########################################
   9097.2 |########################################
   9138.4 |
   9179.6 |
   9220.9 |
  (0 below, 1 above range)

carrier_ent_c0_w64 (n=6, range 8414.6-9498.1 ns)
   8414.6 |####################
   8468.8 |
   8523.0 |
   8577.1 |########################################
   8631.3 |
   8685.5 |
   8739.6 |
   8793.8 |
   8848.0 |
   8902.2 |
   8956.3 |
   9010.5 |
   9064.7 |
   9118.9 |
   9173.0 |
   9227.2 |
   9281.4 |
   9335.6 |####################
   9389.7 |
   9443.9 |####################
  (0 below, 1 above range)

carrier_ent_c0_wmax (n=6, range 8406.7-9544.2 ns)
   8406.7 |########################################
   8463.6 |
   8520.5 |
   8577.3 |
   8634.2 |
   8691.1 |
   8748.0 |
   8804.8 |
   8861.7 |
   8918.6 |
   8975.5 |####################
   9032.3 |
   9089.2 |
   9146.1 |
   9203.0 |
   9259.8 |
   9316.7 |
   9373.6 |####################
   9430.5 |
   9487.3 |####################
  (0 below, 1 above range)

carrier_ent_c500_w4 (n=6, range 7686.7-8701.0 ns)
   7686.7 |########################################
   7737.4 |
   7788.1 |
   7838.9 |
   7889.6 |
   7940.3 |
   7991.0 |
   8041.7 |
   8092.4 |
   8143.2 |
   8193.9 |
   8244.6 |
   8295.3 |
   8346.0 |
   8396.7 |####################
   8447.5 |
   8498.2 |
   8548.9 |
   8599.6 |####################
   8650.3 |####################
  (0 below, 1 above range)

carrier_ent_c500_w64 (n=6, range 7597.9-8596.2 ns)
   7597.9 |########################################
   7647.8 |
   7697.7 |########################################
   7747.7 |
   7797.6 |
   7847.5 |
   7897.4 |
   7947.3 |
   7997.2 |
   8047.2 |
   8097.1 |
   8147.0 |
   8196.9 |
   8246.8 |
   8296.7 |
   8346.7 |
   8396.6 |########################################
   8446.5 |
   8496.4 |########################################
   8546.3 |########################################
  (0 below, 1 above range)

carrier_ent_c500_wmax (n=6, range 7602.5-8614.6 ns)
   7602.5 |########################################
   7653.1 |
   7703.7 |
   7754.3 |
   7804.9 |########################################
   7855.5 |
   7906.1 |
   7956.7 |
   8007.3 |
   8057.9 |
   8108.6 |
   8159.2 |
   8209.8 |
   8260.4 |
   8311.0 |
   8361.6 |
   8412.2 |
   8462.8 |########################################
   8513.4 |########################################
   8564.0 |
  (0 below, 2 above range)

carrier_ent_c900_w4 (n=6, range 10336.7-11699.8 ns)
  10336.7 |####################
  10404.9 |
  10473.0 |
  10541.2 |
  10609.3 |
  10677.5 |
  10745.6 |
  10813.8 |
  10881.9 |
  10950.1 |
  11018.2 |
  11086.4 |
  11154.6 |
  11222.7 |
  11290.9 |
  11359.0 |
  11427.2 |
  11495.3 |########################################
  11563.5 |########################################
  11631.6 |
  (0 below, 1 above range)

carrier_ent_c900_w64 (n=6, range 6357.9-7190.8 ns)
   6357.9 |####################
   6399.5 |
   6441.2 |####################
   6482.8 |
   6524.5 |
   6566.1 |########################################
   6607.8 |
   6649.4 |
   6691.1 |
   6732.7 |
   6774.4 |
   6816.0 |
   6857.6 |
   6899.3 |
   6940.9 |
   6982.6 |
   7024.2 |
   7065.9 |
   7107.5 |
   7149.2 |####################
  (0 below, 1 above range)

carrier_ent_c900_wmax (n=6, range 6353.3-7155.2 ns)
   6353.3 |########################################
   6393.4 |
   6433.5 |
   6473.6 |####################
   6513.7 |
   6553.8 |
   6593.9 |
   6634.0 |
   6674.1 |
   6714.2 |
   6754.3 |
   6794.4 |
   6834.5 |
   6874.6 |
   6914.7 |
   6954.8 |
   6994.9 |
   7035.0 |
   7075.1 |
   7115.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_ent_c0_w4**: bridge=1025.9% of algo (FFI overhead may distort results)
- **carrier_ent_c0_w64**: bridge=1030.5% of algo (FFI overhead may distort results)
- **carrier_ent_c0_wmax**: bridge=1004.0% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w4**: bridge=1037.1% of algo (FFI overhead may distort results)
- **carrier_ent_c500_w64**: bridge=1061.8% of algo (FFI overhead may distort results)
- **carrier_ent_c500_wmax**: bridge=1037.7% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w4**: bridge=797.9% of algo (FFI overhead may distort results)
- **carrier_ent_c900_w64**: bridge=1326.3% of algo (FFI overhead may distort results)
- **carrier_ent_c900_wmax**: bridge=1281.0% of algo (FFI overhead may distort results)

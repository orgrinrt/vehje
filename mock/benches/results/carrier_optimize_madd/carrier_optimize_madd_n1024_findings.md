# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_madd_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_madd_none has the worst median (39.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_madd_all at 1.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_madd_all dominates: 1524% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (1.55 us) leads carrier_opt_madd_fold (25.17 us) by 1524%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 96% (significant)

carrier_opt_madd_all is -37.50 us (96%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_none is an outlier: 25.2x slower than the field

carrier_opt_madd_none (39.04 us) is 25.2x the fastest (1.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_madd_none shows alternating (throttle bounce) (autocorr -0.67)

carrier_opt_madd_none's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} (1524% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} with a 1524% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 25.2x the fastest

Fastest carrier_opt_madd_all (1.55 us) to slowest carrier_opt_madd_none (39.04 us): 25.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 1549.8 ns median (-96.0% vs baseline)
- 4 variants significantly faster than baseline
- Spread: 25.19x (fastest 1549.8 ns, slowest 39038.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 3696ns | 3706ns | 3628ns | 3690ns | 3739ns | -91.21% |
| carrier_opt_madd_canon | 39523ns | 39426ns | 38783ns | 39274ns | 40265ns | -5.96% |
| carrier_opt_madd_cse | 40298ns | 40083ns | 38702ns | 39926ns | 41655ns | -4.11% |
| carrier_opt_madd_dce | 40404ns | 40089ns | 39393ns | 40008ns | 41505ns | -3.86% |
| carrier_opt_madd_fold | 28452ns | 27361ns | 26958ns | 27311ns | 30909ns | -32.30% |
| carrier_opt_madd_none | 42027ns | 41253ns | 40530ns | 41140ns | 44105ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 1540ns | 1504ns | 1559ns | -96.13% | 0.665 |
| carrier_opt_madd_canon | 37349ns | 36647ns | 38053ns | -6.14% | 0.027 |
| carrier_opt_madd_cse | 38075ns | 36572ns | 39345ns | -4.32% | 0.027 |
| carrier_opt_madd_dce | 38182ns | 37242ns | 39196ns | -4.05% | 0.027 |
| carrier_opt_madd_fold | 26106ns | 24786ns | 28236ns | -34.39% | 0.039 |
| carrier_opt_madd_none | 39792ns | 38380ns | 41768ns | base | 0.026 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_madd_all | 277958 | 1450379 | 0.192 | 0.59× |
| carrier_opt_madd_canon | 471233 | 1992976 | 0.236 | 1.00× |
| carrier_opt_madd_cse | 470835 | 2052425 | 0.229 | 1.00× |
| carrier_opt_madd_dce | 474989 | 2159178 | 0.220 | 1.01× |
| carrier_opt_madd_fold | 396796 | 2446359 | 0.162 | 0.84× |
| carrier_opt_madd_none | 469918 | 2074390 | 0.227 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.681 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.661 | 97.1% |
| carrier_opt_madd_canon | 0.027 | 4.0% |
| carrier_opt_madd_cse | 0.027 | 4.0% |
| carrier_opt_madd_dce | 0.027 | 4.0% |
| carrier_opt_madd_fold | 0.041 | 6.0% |
| carrier_opt_madd_none | 0.026 | 3.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 3696ns | 3696ns | -91.21% |
| carrier_opt_madd_canon | 39523ns | 39523ns | -5.96% |
| carrier_opt_madd_cse | 40298ns | 40298ns | -4.11% |
| carrier_opt_madd_dce | 40404ns | 40404ns | -3.86% |
| carrier_opt_madd_fold | 28452ns | 28452ns | -32.30% |
| carrier_opt_madd_none | 42027ns | 42027ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 39038ns | base | --- | [38570, 41768] | --- | --- | --- | --- |
| carrier_opt_madd_all | 1550ns | -37497.7ns (-96.1%) | [-40236, -37020]ns | [1513, 1559] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_madd_canon | 37267ns | -2116.0ns (-5.4%) | [-4366, -846]ns | [36728, 38053] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_madd_cse | 37877ns | -1606.9ns (-4.1%) | [-2867, -678]ns | [37002, 39345] | YES | 0.0391 | 0.0313 | 0 |
| carrier_opt_madd_dce | 37900ns | no significant difference | [-3432, +185]ns | [37449, 39196] | no | 0.2188 | 0.2188 | 0 |
| carrier_opt_madd_fold | 25175ns | -13862.9ns (-35.5%) | [-16861, -10334]ns | [24906, 28236] | YES | 0.0391 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_canon | carrier_opt_madd_cse | carrier_opt_madd_dce | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|
| 1 | 39316ns | -96.0% | -5.4% | -7.0% | -4.2% | -35.9% |
| 2 | 38760ns | -96.1% | -5.5% | -3.4% | -3.9% | -29.1% |
| 3 | 41198ns | -96.2% | -7.8% | -7.3% | -6.5% | -39.3% |
| 4 | 38759ns | -96.0% | -3.7% | -3.1% | -2.8% | -35.1% |
| 5 | 42337ns | -96.4% | -13.1% | -4.5% | -9.9% | -41.5% |
| 6 | 38380ns | -96.0% | -0.7% | -0.4% | +3.8% | -24.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.500 | HIGH- (thermal bounce) |
| carrier_opt_madd_canon | -0.421 | moderate- |
| carrier_opt_madd_cse | -0.007 | ok |
| carrier_opt_madd_dce | -0.022 | ok |
| carrier_opt_madd_fold | -0.284 | moderate- |
| carrier_opt_madd_none | -0.674 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_canon**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_dce**: won 5/6, lost 1/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 85263.3ns | 1540.5ns | 5534.8% | HIGH |
| carrier_opt_madd_canon | 112254.7ns | 37349.4ns | 300.6% | HIGH |
| carrier_opt_madd_cse | 114411.5ns | 38074.5ns | 300.5% | HIGH |
| carrier_opt_madd_dce | 114349.5ns | 38181.7ns | 299.5% | HIGH |
| carrier_opt_madd_fold | 103492.5ns | 26105.9ns | 396.4% | HIGH |
| carrier_opt_madd_none | 112496.1ns | 39791.9ns | 282.7% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 1504.2-1558.8 ns)
   1504.2 |########################################
   1506.9 |
   1509.7 |
   1512.4 |
   1515.1 |
   1517.8 |
   1520.6 |########################################
   1523.3 |
   1526.0 |
   1528.7 |
   1531.5 |
   1534.2 |
   1536.9 |
   1539.7 |
   1542.4 |
   1545.1 |########################################
   1547.8 |
   1550.6 |
   1553.3 |########################################
   1556.0 |########################################
  (0 below, 1 above range)

carrier_opt_madd_canon (n=6, range 36647.1-38052.9 ns)
  36647.1 |########################################
  36717.4 |
  36787.7 |########################################
  36858.0 |
  36928.3 |
  36998.6 |
  37068.9 |
  37139.1 |########################################
  37209.4 |
  37279.7 |########################################
  37350.0 |
  37420.3 |
  37490.6 |
  37560.9 |
  37631.2 |
  37701.5 |
  37771.8 |
  37842.1 |
  37912.4 |
  37982.7 |########################################
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 36571.7-39344.6 ns)
  36571.7 |########################################
  36710.3 |
  36849.0 |
  36987.6 |
  37126.3 |
  37264.9 |
  37403.6 |########################################
  37542.2 |########################################
  37680.9 |
  37819.5 |
  37958.1 |
  38096.8 |########################################
  38235.4 |########################################
  38374.1 |
  38512.7 |
  38651.4 |
  38790.0 |
  38928.7 |
  39067.3 |
  39206.0 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 37242.5-39196.4 ns)
  37242.5 |####################
  37340.2 |
  37437.9 |
  37535.6 |
  37633.3 |########################################
  37731.0 |
  37828.7 |
  37926.4 |
  38024.1 |
  38121.8 |####################
  38219.5 |
  38317.2 |
  38414.9 |
  38512.6 |####################
  38610.3 |
  38708.0 |
  38805.7 |
  38903.4 |
  39001.1 |
  39098.8 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 24786.2-28236.4 ns)
  24786.2 |####################
  24958.7 |####################
  25131.2 |########################################
  25303.7 |
  25476.2 |
  25648.8 |
  25821.3 |
  25993.8 |
  26166.3 |
  26338.8 |
  26511.3 |
  26683.8 |
  26856.3 |
  27028.9 |
  27201.4 |
  27373.9 |####################
  27546.4 |
  27718.9 |
  27891.4 |
  28063.9 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 38380.4-41767.5 ns)
  38380.4 |####################
  38549.8 |
  38719.1 |########################################
  38888.5 |
  39057.8 |
  39227.2 |####################
  39396.5 |
  39565.9 |
  39735.2 |
  39904.6 |
  40073.9 |
  40243.3 |
  40412.7 |
  40582.0 |
  40751.4 |
  40920.7 |
  41090.1 |####################
  41259.4 |
  41428.8 |
  41598.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=5500.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_canon**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=300.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=300.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=400.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=296.6% of algo (FFI overhead may distort results)

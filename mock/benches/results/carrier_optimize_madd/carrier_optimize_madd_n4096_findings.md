# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_madd_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_madd_none has the worst median (165.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_madd_all at 7.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_madd_all dominates: 1297% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (7.36 us) leads carrier_opt_madd_fold (102.78 us) by 1297%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 96% (significant)

carrier_opt_madd_all is -158.39 us (96%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_none is an outlier: 22.5x slower than the field

carrier_opt_madd_none (165.78 us) is 22.5x the fastest (7.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_madd_none shows alternating (throttle bounce) (autocorr -0.60)

carrier_opt_madd_none's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} (1297% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} with a 1297% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 22.5x the fastest

Fastest carrier_opt_madd_all (7.36 us) to slowest carrier_opt_madd_none (165.78 us): 22.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 7355.9 ns median (-95.6% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 22.54x (fastest 7355.9 ns, slowest 165775.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 9585ns | 9608ns | 9115ns | 9485ns | 9971ns | -94.30% |
| carrier_opt_madd_canon | 155428ns | 155728ns | 151888ns | 155558ns | 157003ns | -7.55% |
| carrier_opt_madd_cse | 158366ns | 158228ns | 154050ns | 158073ns | 160965ns | -5.81% |
| carrier_opt_madd_dce | 160385ns | 159998ns | 159195ns | 159767ns | 161908ns | -4.60% |
| carrier_opt_madd_fold | 105433ns | 105042ns | 102939ns | 104556ns | 107995ns | -37.29% |
| carrier_opt_madd_none | 168126ns | 168000ns | 165954ns | 167896ns | 169556ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 7348ns | 6999ns | 7642ns | -95.57% | 0.557 |
| carrier_opt_madd_canon | 153174ns | 149736ns | 154732ns | -7.66% | 0.027 |
| carrier_opt_madd_cse | 156110ns | 151896ns | 158589ns | -5.89% | 0.026 |
| carrier_opt_madd_dce | 158143ns | 156912ns | 159669ns | -4.67% | 0.026 |
| carrier_opt_madd_fold | 103164ns | 100703ns | 105678ns | -37.81% | 0.040 |
| carrier_opt_madd_none | 165889ns | 163745ns | 167271ns | base | 0.025 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_madd_all | 297990 | 1391932 | 0.214 | 0.29× |
| carrier_opt_madd_canon | 957700 | 3910447 | 0.245 | 0.93× |
| carrier_opt_madd_cse | 972269 | 4035339 | 0.241 | 0.94× |
| carrier_opt_madd_dce | 984688 | 4305655 | 0.229 | 0.95× |
| carrier_opt_madd_fold | 634437 | 3918538 | 0.162 | 0.61× |
| carrier_opt_madd_none | 1033051 | 4305478 | 0.240 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.585 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.557 | 95.1% |
| carrier_opt_madd_canon | 0.027 | 4.6% |
| carrier_opt_madd_cse | 0.026 | 4.5% |
| carrier_opt_madd_dce | 0.026 | 4.4% |
| carrier_opt_madd_fold | 0.040 | 6.8% |
| carrier_opt_madd_none | 0.025 | 4.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 9585ns | 9585ns | -94.30% |
| carrier_opt_madd_canon | 155428ns | 155428ns | -7.55% |
| carrier_opt_madd_cse | 158366ns | 158366ns | -5.81% |
| carrier_opt_madd_dce | 160385ns | 160385ns | -4.60% |
| carrier_opt_madd_fold | 105433ns | 105433ns | -37.29% |
| carrier_opt_madd_none | 168126ns | 168126ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 165775ns | base | --- | [164620, 167271] | --- | --- | --- | --- |
| carrier_opt_madd_all | 7356ns | -158385.2ns (-95.5%) | [-159811, -157426]ns | [7046, 7642] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_canon | 153450ns | -12876.3ns (-7.8%) | [-14207, -11062]ns | [151340, 154732] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_cse | 156010ns | -10367.7ns (-6.3%) | [-11861, -7109]ns | [153730, 158589] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_dce | 157787ns | -7404.6ns (-4.5%) | [-9083, -6748]ns | [156974, 159669] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_fold | 102785ns | -64325.0ns (-38.8%) | [-64705, -59145]ns | [101029, 105678] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_canon | carrier_opt_madd_cse | carrier_opt_madd_dce | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|
| 1 | 165649ns | -95.4% | -7.1% | -6.1% | -4.5% | -38.8% |
| 2 | 168329ns | -95.4% | -8.6% | -7.1% | -4.3% | -38.2% |
| 3 | 163745ns | -95.7% | -8.6% | -7.2% | -3.9% | -34.8% |
| 4 | 166214ns | -95.7% | -8.0% | -6.4% | -5.5% | -38.9% |
| 5 | 165901ns | -95.5% | -6.2% | -4.9% | -5.4% | -37.0% |
| 6 | 165495ns | -95.7% | -7.5% | -3.7% | -4.4% | -39.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.182 | ok |
| carrier_opt_madd_canon | -0.117 | ok |
| carrier_opt_madd_cse | 0.163 | ok |
| carrier_opt_madd_dce | -0.001 | ok |
| carrier_opt_madd_fold | -0.355 | moderate- |
| carrier_opt_madd_none | -0.599 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_canon**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_dce**: won 6/6, lost 0/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 88974.7ns | 7347.9ns | 1210.9% | HIGH |
| carrier_opt_madd_canon | 154097.2ns | 153173.9ns | 100.6% | HIGH |
| carrier_opt_madd_cse | 157133.8ns | 156109.8ns | 100.7% | HIGH |
| carrier_opt_madd_dce | 158947.4ns | 158143.5ns | 100.5% | HIGH |
| carrier_opt_madd_fold | 103512.8ns | 103163.9ns | 100.3% | HIGH |
| carrier_opt_madd_none | 166718.6ns | 165888.9ns | 100.5% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 6998.8-7642.1 ns)
   6998.8 |####################
   7031.0 |
   7063.1 |####################
   7095.3 |
   7127.5 |
   7159.6 |####################
   7191.8 |
   7224.0 |
   7256.1 |
   7288.3 |
   7320.5 |
   7352.6 |
   7384.8 |
   7416.9 |
   7449.1 |
   7481.3 |
   7513.4 |########################################
   7545.6 |
   7577.8 |
   7609.9 |
  (0 below, 1 above range)

carrier_opt_madd_canon (n=6, range 149735.8-154732.3 ns)
  149735.8 |####################
  149985.6 |
  150235.4 |
  150485.3 |
  150735.1 |
  150984.9 |
  151234.8 |
  151484.6 |
  151734.4 |
  151984.2 |
  152234.0 |
  152483.9 |
  152733.7 |####################
  152983.5 |####################
  153233.3 |
  153483.2 |
  153733.0 |########################################
  153982.8 |
  154232.6 |
  154482.5 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 151895.8-158589.2 ns)
  151895.8 |####################
  152230.5 |
  152565.1 |
  152899.8 |
  153234.5 |
  153569.1 |
  153903.8 |
  154238.5 |
  154573.2 |
  154907.8 |
  155242.5 |########################################
  155577.2 |
  155911.8 |
  156246.5 |####################
  156581.2 |
  156915.9 |
  157250.5 |
  157585.2 |####################
  157919.9 |
  158254.5 |
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 156912.1-159668.8 ns)
  156912.1 |########################################
  157049.9 |
  157187.8 |
  157325.6 |####################
  157463.4 |
  157601.3 |
  157739.1 |
  157876.9 |
  158014.8 |
  158152.6 |########################################
  158290.4 |
  158428.3 |
  158566.1 |
  158703.9 |
  158841.8 |
  158979.6 |
  159117.4 |
  159255.3 |
  159393.1 |
  159530.9 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 100703.3-105678.3 ns)
  100703.3 |########################################
  100952.1 |
  101200.8 |########################################
  101449.6 |########################################
  101698.3 |
  101947.1 |
  102195.8 |
  102444.6 |
  102693.3 |
  102942.1 |
  103190.8 |
  103439.5 |
  103688.3 |
  103937.0 |########################################
  104185.8 |
  104434.5 |########################################
  104683.3 |
  104932.0 |
  105180.8 |
  105429.5 |
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 163745.0-167271.5 ns)
  163745.0 |########################################
  163921.3 |
  164097.6 |
  164274.0 |
  164450.3 |
  164626.6 |
  164802.9 |
  164979.3 |
  165155.6 |
  165331.9 |########################################
  165508.2 |########################################
  165684.5 |
  165860.9 |########################################
  166037.2 |
  166213.5 |########################################
  166389.8 |
  166566.2 |
  166742.5 |
  166918.8 |
  167095.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=1204.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_canon**: bridge=100.6% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=100.7% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=100.5% of algo (FFI overhead may distort results)

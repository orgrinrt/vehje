# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, madd profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_madd_none**

## Highlights

Baseline for all deltas below: **carrier_opt_madd_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_madd_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_madd_none has the worst median (650.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_madd_all at 26.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_madd_all dominates: 1440% faster than the next best (carrier_opt_madd_fold)

carrier_opt_madd_all (26.47 us) leads carrier_opt_madd_fold (407.56 us) by 1440%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_opt_madd_all beats baseline by 96% (significant)

carrier_opt_madd_all is -623.89 us (96%) faster than baseline carrier_opt_madd_none, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_opt_madd_none is an outlier: 24.6x slower than the field

carrier_opt_madd_none (650.32 us) is 24.6x the fastest (26.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_opt_madd_cse shows alternating (throttle bounce) (autocorr -0.62)

carrier_opt_madd_cse's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_opt_madd_all} vs {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} (1440% apart)

The field splits into a fast tier {carrier_opt_madd_all} and a slow tier {carrier_opt_madd_fold, carrier_opt_madd_canon, carrier_opt_madd_cse, carrier_opt_madd_dce, carrier_opt_madd_none} with a 1440% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 24.6x the fastest

Fastest carrier_opt_madd_all (26.47 us) to slowest carrier_opt_madd_none (650.32 us): 24.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_opt_madd_all** at 26467.5 ns median (-95.9% vs baseline)
- 5 variants significantly faster than baseline
- Spread: 24.57x (fastest 26467.5 ns, slowest 650322.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_madd_all | 29216ns | 28938ns | 28461ns | 28818ns | 30190ns | -95.54% |
| carrier_opt_madd_canon | 608253ns | 605055ns | 603429ns | 604694ns | 616002ns | -7.05% |
| carrier_opt_madd_cse | 614549ns | 614898ns | 603220ns | 613358ns | 622000ns | -6.09% |
| carrier_opt_madd_dce | 636502ns | 635430ns | 631798ns | 634219ns | 642278ns | -2.73% |
| carrier_opt_madd_fold | 407566ns | 409835ns | 400072ns | 407814ns | 410942ns | -37.72% |
| carrier_opt_madd_none | 654369ns | 652638ns | 651922ns | 652449ns | 658474ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_madd_all | 26862ns | 26246ns | 27825ns | -95.88% | 0.610 |
| carrier_opt_madd_canon | 606003ns | 601223ns | 613754ns | -7.06% | 0.027 |
| carrier_opt_madd_cse | 612158ns | 601050ns | 619460ns | -6.12% | 0.027 |
| carrier_opt_madd_dce | 634228ns | 629588ns | 640004ns | -2.73% | 0.026 |
| carrier_opt_madd_fold | 405314ns | 397898ns | 408701ns | -37.84% | 0.040 |
| carrier_opt_madd_none | 652050ns | 649695ns | 656111ns | base | 0.025 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_madd_all | 388351 | 1698717 | 0.229 | 0.09× |
| carrier_opt_madd_canon | 3798072 | 15385574 | 0.247 | 0.93× |
| carrier_opt_madd_cse | 3808548 | 15953922 | 0.239 | 0.93× |
| carrier_opt_madd_dce | 3974809 | 17170666 | 0.231 | 0.97× |
| carrier_opt_madd_fold | 2530103 | 15617619 | 0.162 | 0.62× |
| carrier_opt_madd_none | 4095574 | 17170616 | 0.239 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.624 Gops/s** (carrier_opt_madd_all; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_madd_all | 0.619 | 99.2% |
| carrier_opt_madd_canon | 0.027 | 4.4% |
| carrier_opt_madd_cse | 0.027 | 4.3% |
| carrier_opt_madd_dce | 0.026 | 4.1% |
| carrier_opt_madd_fold | 0.040 | 6.4% |
| carrier_opt_madd_none | 0.025 | 4.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_madd_all | 29216ns | 29216ns | -95.54% |
| carrier_opt_madd_canon | 608253ns | 608253ns | -7.05% |
| carrier_opt_madd_cse | 614549ns | 614549ns | -6.09% |
| carrier_opt_madd_dce | 636502ns | 636502ns | -2.73% |
| carrier_opt_madd_fold | 407566ns | 407566ns | -37.72% |
| carrier_opt_madd_none | 654369ns | 654369ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_madd_none | 650322ns | base | --- | [649716, 656111] | --- | --- | --- | --- |
| carrier_opt_madd_all | 26468ns | -623886.4ns (-95.9%) | [-628904, -622774]ns | [26293, 27825] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_canon | 602775ns | -47072.0ns (-7.2%) | [-50695, -40373]ns | [601481, 613754] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_cse | 612550ns | -38265.6ns (-5.9%) | [-49664, -31747]ns | [604463, 619460] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_dce | 633087ns | -20122.1ns (-3.1%) | [-23025, -10318]ns | [629594, 640004] | YES | 0.0313 | 0.0313 | 0 |
| carrier_opt_madd_fold | 407565ns | -247161.4ns (-38.0%) | [-251272, -241775]ns | [399675, 408701] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_madd_none | carrier_opt_madd_all | carrier_opt_madd_canon | carrier_opt_madd_cse | carrier_opt_madd_dce | carrier_opt_madd_fold |
|---|---|---|---|---|---|---|
| 1 | 657565ns | -96.0% | -6.8% | -8.6% | -3.8% | -38.0% |
| 2 | 654657ns | -95.7% | -8.2% | -5.4% | -3.3% | -37.5% |
| 3 | 650688ns | -95.9% | -5.5% | -6.6% | -1.5% | -38.8% |
| 4 | 649737ns | -96.0% | -7.3% | -5.1% | -3.1% | -37.3% |
| 5 | 649695ns | -95.7% | -7.4% | -6.4% | -3.1% | -37.1% |
| 6 | 649956ns | -95.9% | -7.2% | -4.7% | -1.6% | -38.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_madd_all | -0.548 | HIGH- (thermal bounce) |
| carrier_opt_madd_canon | -0.422 | moderate- |
| carrier_opt_madd_cse | -0.624 | HIGH- (thermal bounce) |
| carrier_opt_madd_dce | -0.326 | moderate- |
| carrier_opt_madd_fold | -0.371 | moderate- |
| carrier_opt_madd_none | 0.448 | moderate+ |

**Consistency summary:**

- **carrier_opt_madd_all**: won 6/6, lost 0/6
- **carrier_opt_madd_canon**: won 6/6, lost 0/6
- **carrier_opt_madd_cse**: won 6/6, lost 0/6
- **carrier_opt_madd_dce**: won 6/6, lost 0/6
- **carrier_opt_madd_fold**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_madd_all | 101294.5ns | 26861.8ns | 377.1% | HIGH |
| carrier_opt_madd_canon | 608548.8ns | 606003.2ns | 100.4% | HIGH |
| carrier_opt_madd_cse | 614347.5ns | 612157.6ns | 100.4% | HIGH |
| carrier_opt_madd_dce | 636166.6ns | 634228.4ns | 100.3% | HIGH |
| carrier_opt_madd_fold | 407700.9ns | 405313.8ns | 100.6% | HIGH |
| carrier_opt_madd_none | 654164.2ns | 652049.9ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_opt_madd_all (n=6, range 26245.8-27825.0 ns)
  26245.8 |####################
  26324.8 |########################################
  26403.7 |
  26482.7 |####################
  26561.6 |
  26640.6 |
  26719.6 |
  26798.5 |
  26877.5 |
  26956.4 |
  27035.4 |
  27114.4 |
  27193.3 |
  27272.3 |
  27351.2 |
  27430.2 |
  27509.2 |
  27588.1 |####################
  27667.1 |
  27746.0 |
  (0 below, 1 above range)

carrier_opt_madd_canon (n=6, range 601223.3-613753.6 ns)
  601223.3 |########################################
  601849.8 |####################
  602476.3 |
  603102.8 |####################
  603729.4 |
  604355.9 |
  604982.4 |
  605608.9 |
  606235.4 |
  606861.9 |
  607488.4 |
  608114.9 |
  608741.5 |
  609368.0 |
  609994.5 |
  610621.0 |
  611247.5 |
  611874.0 |
  612500.5 |####################
  613127.0 |
  (0 below, 1 above range)

carrier_opt_madd_cse (n=6, range 601050.0-619460.2 ns)
  601050.0 |####################
  601970.5 |
  602891.0 |
  603811.5 |
  604732.0 |
  605652.6 |
  606573.1 |
  607493.6 |########################################
  608414.1 |
  609334.6 |
  610255.1 |
  611175.6 |
  612096.1 |
  613016.6 |
  613937.1 |
  614857.6 |
  615778.2 |
  616698.7 |####################
  617619.2 |
  618539.7 |####################
  (0 below, 1 above range)

carrier_opt_madd_dce (n=6, range 629587.9-640004.4 ns)
  629587.9 |########################################
  630108.7 |
  630629.6 |
  631150.4 |
  631671.2 |
  632192.0 |
  632712.8 |####################
  633233.7 |####################
  633754.5 |
  634275.3 |
  634796.2 |
  635317.0 |
  635837.8 |
  636358.6 |
  636879.5 |
  637400.3 |
  637921.1 |
  638441.9 |
  638962.8 |####################
  639483.6 |
  (0 below, 1 above range)

carrier_opt_madd_fold (n=6, range 397898.3-408701.1 ns)
  397898.3 |########################################
  398438.4 |
  398978.6 |
  399518.7 |
  400058.8 |
  400599.0 |
  401139.1 |########################################
  401679.3 |
  402219.4 |
  402759.5 |
  403299.7 |
  403839.8 |
  404380.0 |
  404920.1 |
  405460.2 |
  406000.4 |
  406540.5 |
  407080.6 |########################################
  407620.8 |########################################
  408160.9 |########################################
  (0 below, 1 above range)

carrier_opt_madd_none (n=6, range 649695.4-656111.2 ns)
  649695.4 |########################################
  650016.2 |
  650337.0 |
  650657.8 |#############
  650978.6 |
  651299.4 |
  651620.2 |
  651940.9 |
  652261.7 |
  652582.5 |
  652903.3 |
  653224.1 |
  653544.9 |
  653865.7 |
  654186.5 |
  654507.3 |#############
  654828.1 |
  655148.9 |
  655469.7 |
  655790.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_madd_all**: bridge=398.1% of algo (FFI overhead may distort results)
- **carrier_opt_madd_canon**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_cse**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_opt_madd_dce**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_opt_madd_fold**: bridge=100.5% of algo (FFI overhead may distort results)
- **carrier_opt_madd_none**: bridge=100.4% of algo (FFI overhead may distort results)

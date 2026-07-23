# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (907.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 402.81 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 75% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (402.81 us) leads carrier_cfg_threaded (703.61 us) by 75%, a clear separation rather than a photo finish. CV 1.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 55% (significant)

carrier_cfg_trace is -502.03 us (55%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.3x slower than the field

carrier_cfg_switch (907.44 us) is 2.3x the fastest (402.81 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cfg_threaded shows alternating (throttle bounce) (autocorr -0.52)

carrier_cfg_threaded's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (75% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 75% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 402806.0 ns median (-55.6% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.25x (fastest 402806.0 ns, slowest 907444.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 875183ns | 872460ns | 863309ns | 870267ns | 888496ns | -5.26% |
| carrier_cfg_switch | 923818ns | 909898ns | 890890ns | 905386ns | 967931ns | base |
| carrier_cfg_threaded | 710646ns | 706508ns | 700640ns | 705069ns | 724013ns | -23.08% |
| carrier_cfg_trace | 404958ns | 405088ns | 398353ns | 402975ns | 411235ns | -56.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 872321ns | 860914ns | 885728ns | -5.29% | 0.001 |
| carrier_cfg_switch | 921057ns | 888494ns | 964617ns | base | 0.001 |
| carrier_cfg_threaded | 708007ns | 698379ns | 721534ns | -23.13% | 0.001 |
| carrier_cfg_trace | 402536ns | 395642ns | 408930ns | -56.30% | 0.003 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cfg_fntable | 5491091 | 12909858 | 0.425 | 0.97× |
| carrier_cfg_switch | 5657369 | 11830082 | 0.478 | 1.00× |
| carrier_cfg_threaded | 4470301 | 8612045 | 0.519 | 0.79× |
| carrier_cfg_trace | 2518645 | 9857439 | 0.256 | 0.45× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.5% |
| carrier_cfg_switch | 0.001 | 43.6% |
| carrier_cfg_threaded | 0.001 | 56.2% |
| carrier_cfg_trace | 0.003 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 875183ns | 875183ns | -5.26% |
| carrier_cfg_switch | 923818ns | 923818ns | base |
| carrier_cfg_threaded | 710646ns | 710646ns | -23.08% |
| carrier_cfg_trace | 404958ns | 404958ns | -56.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 907445ns | base | --- | [891110, 964617] | --- | --- | --- | --- |
| carrier_cfg_fntable | 869404ns | -40944.0ns (-4.5%) | [-96623, -8641]ns | [861832, 885728] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_cfg_threaded | 703608ns | -206335.2ns (-22.7%) | [-263239, -169576]ns | [698880, 721534] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cfg_trace | 402806ns | -502031.8ns (-55.3%) | [-568745, -484787]ns | [395873, 408930] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 898238ns | -4.2% | -22.3% | -54.4% |
| 2 | 888494ns | +1.0% | -19.3% | -54.1% |
| 3 | 1010760ns | -13.8% | -30.4% | -60.9% |
| 4 | 918475ns | -4.9% | -23.9% | -56.9% |
| 5 | 893726ns | -2.9% | -18.7% | -55.5% |
| 6 | 916651ns | -5.9% | -23.2% | -55.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.308 | moderate- |
| carrier_cfg_switch | -0.213 | moderate- |
| carrier_cfg_threaded | -0.519 | HIGH- (thermal bounce) |
| carrier_cfg_trace | 0.209 | moderate+ |

**Consistency summary:**

- **carrier_cfg_fntable**: won 5/6, lost 1/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 872097.5ns | 872321.2ns | 100.0% | HIGH |
| carrier_cfg_switch | 921830.7ns | 921057.3ns | 100.1% | HIGH |
| carrier_cfg_threaded | 709237.6ns | 708007.5ns | 100.2% | HIGH |
| carrier_cfg_trace | 402032.8ns | 402536.2ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 860913.8-885727.7 ns)
  860913.8 |########################################
  862154.5 |########################################
  863395.2 |
  864635.9 |
  865876.6 |
  867117.3 |########################################
  868358.0 |
  869598.7 |
  870839.4 |########################################
  872080.1 |
  873320.8 |########################################
  874561.4 |
  875802.1 |
  877042.8 |
  878283.5 |
  879524.2 |
  880764.9 |
  882005.6 |
  883246.3 |
  884487.0 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 888493.7-964617.3 ns)
  888493.7 |####################
  892299.9 |####################
  896106.1 |####################
  899912.2 |
  903718.4 |
  907524.6 |
  911330.8 |
  915137.0 |########################################
  918943.1 |
  922749.3 |
  926555.5 |
  930361.7 |
  934167.9 |
  937974.0 |
  941780.2 |
  945586.4 |
  949392.6 |
  953198.8 |
  957004.9 |
  960811.1 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 698379.2-721534.2 ns)
  698379.2 |########################################
  699536.9 |
  700694.7 |
  701852.4 |
  703010.2 |########################################
  704167.9 |
  705325.7 |
  706483.4 |
  707641.2 |
  708798.9 |
  709956.7 |
  711114.4 |
  712272.2 |
  713429.9 |
  714587.7 |
  715745.4 |####################
  716903.2 |
  718060.9 |
  719218.7 |
  720376.4 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 395642.1-408929.8 ns)
  395642.1 |########################################
  396306.5 |
  396970.9 |
  397635.3 |####################
  398299.6 |
  398964.0 |
  399628.4 |
  400292.8 |
  400957.2 |
  401621.6 |
  402285.9 |
  402950.3 |
  403614.7 |
  404279.1 |
  404943.5 |
  405607.9 |
  406272.3 |
  406936.6 |
  407601.0 |####################
  408265.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=99.9% of algo (FFI overhead may distort results)

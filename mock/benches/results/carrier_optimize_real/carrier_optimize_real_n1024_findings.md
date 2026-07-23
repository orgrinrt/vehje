# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, real profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_real_none**

## Highlights

Baseline for all deltas below: **carrier_opt_real_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_opt_real_none) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_opt_real_none has the worst median (33.67 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_opt_real_canon at 30.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_opt_real_fold shows alternating (throttle bounce) (autocorr -0.53)

carrier_opt_real_fold's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_real_canon** at 30250.6 ns median (-10.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.11x (fastest 30250.6 ns, slowest 33672.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_real_all | 33067ns | 33157ns | 31797ns | 32751ns | 34176ns | -7.99% |
| carrier_opt_real_canon | 32494ns | 32461ns | 31775ns | 32235ns | 33242ns | -9.58% |
| carrier_opt_real_cse | 33091ns | 33407ns | 31704ns | 32846ns | 34152ns | -7.92% |
| carrier_opt_real_dce | 35584ns | 35577ns | 34526ns | 35329ns | 36497ns | -0.98% |
| carrier_opt_real_fold | 35329ns | 35224ns | 34320ns | 34991ns | 36341ns | -1.69% |
| carrier_opt_real_none | 35937ns | 35946ns | 34280ns | 35426ns | 37533ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_real_all | 30773ns | 29614ns | 31759ns | -8.61% | 0.033 |
| carrier_opt_real_canon | 30273ns | 29596ns | 30968ns | -10.09% | 0.034 |
| carrier_opt_real_cse | 30842ns | 29546ns | 31845ns | -8.40% | 0.033 |
| carrier_opt_real_dce | 33340ns | 32359ns | 34189ns | -0.98% | 0.031 |
| carrier_opt_real_fold | 33099ns | 32163ns | 34035ns | -1.70% | 0.031 |
| carrier_opt_real_none | 33672ns | 32122ns | 35178ns | base | 0.030 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_real_all | 374880 | 1948158 | 0.192 | 0.92× |
| carrier_opt_real_canon | 377697 | 1959815 | 0.193 | 0.92× |
| carrier_opt_real_cse | 376899 | 1958832 | 0.192 | 0.92× |
| carrier_opt_real_dce | 409577 | 2096042 | 0.195 | 1.00× |
| carrier_opt_real_fold | 408666 | 2096481 | 0.195 | 1.00× |
| carrier_opt_real_none | 409438 | 2097035 | 0.195 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_opt_real_cse; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_real_all | 0.033 | 95.7% |
| carrier_opt_real_canon | 0.034 | 97.7% |
| carrier_opt_real_cse | 0.033 | 94.9% |
| carrier_opt_real_dce | 0.031 | 88.6% |
| carrier_opt_real_fold | 0.031 | 89.5% |
| carrier_opt_real_none | 0.030 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_real_all | 33067ns | 33067ns | -7.99% |
| carrier_opt_real_canon | 32494ns | 32494ns | -9.58% |
| carrier_opt_real_cse | 33091ns | 33091ns | -7.92% |
| carrier_opt_real_dce | 35584ns | 35584ns | -0.98% |
| carrier_opt_real_fold | 35329ns | 35329ns | -1.69% |
| carrier_opt_real_none | 35937ns | 35937ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_real_none | 33672ns | base | --- | [32164, 35178] | --- | --- | --- | --- |
| carrier_opt_real_all | 30879ns | -3289.1ns (-9.8%) | [-3938, -1468]ns | [29682, 31759] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_canon | 30251ns | -3408.1ns (-10.1%) | [-4765, -2022]ns | [29601, 30968] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_cse | 31125ns | -2591.5ns (-7.7%) | [-5071, -825]ns | [29557, 31845] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_real_dce | 33338ns | no significant difference | [-2058, +1174]ns | [32493, 34189] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_real_fold | 33006ns | no significant difference | [-1354, +92]ns | [32257, 34035] | no | 0.8594 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_real_none | carrier_opt_real_all | carrier_opt_real_canon | carrier_opt_real_cse | carrier_opt_real_dce | carrier_opt_real_fold |
|---|---|---|---|---|---|---|
| 1 | 32122ns | -7.8% | -7.9% | -8.0% | +1.6% | +0.1% |
| 2 | 34662ns | -11.5% | -12.7% | -7.5% | -1.0% | -2.0% |
| 3 | 32206ns | -1.3% | -5.0% | -1.9% | +5.7% | +0.4% |
| 4 | 34718ns | -10.5% | -14.7% | -14.8% | -6.8% | -3.3% |
| 5 | 35639ns | -10.9% | -12.0% | -14.0% | -4.9% | -4.4% |
| 6 | 32683ns | -9.0% | -7.4% | -3.2% | +0.3% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_real_all | -0.087 | ok |
| carrier_opt_real_canon | -0.432 | moderate- |
| carrier_opt_real_cse | -0.253 | moderate- |
| carrier_opt_real_dce | -0.431 | moderate- |
| carrier_opt_real_fold | -0.528 | HIGH- (thermal bounce) |
| carrier_opt_real_none | -0.384 | moderate- |

**Consistency summary:**

- **carrier_opt_real_all**: won 6/6, lost 0/6
- **carrier_opt_real_canon**: won 6/6, lost 0/6
- **carrier_opt_real_cse**: won 6/6, lost 0/6
- **carrier_opt_real_dce**: won 3/6, lost 3/6
- **carrier_opt_real_fold**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_real_all | 92484.2ns | 30773.3ns | 300.5% | HIGH |
| carrier_opt_real_canon | 90977.4ns | 30273.4ns | 300.5% | HIGH |
| carrier_opt_real_cse | 92759.7ns | 30842.5ns | 300.8% | HIGH |
| carrier_opt_real_dce | 100269.1ns | 33340.2ns | 300.7% | HIGH |
| carrier_opt_real_fold | 99456.1ns | 33099.4ns | 300.5% | HIGH |
| carrier_opt_real_none | 101002.3ns | 33671.7ns | 300.0% | HIGH |

## Distribution (algo ns)

```
carrier_opt_real_all (n=6, range 29614.2-31758.8 ns)
  29614.2 |########################################
  29721.4 |########################################
  29828.7 |
  29935.9 |
  30043.1 |
  30150.3 |
  30257.6 |
  30364.8 |
  30472.0 |
  30579.2 |########################################
  30686.5 |
  30793.7 |
  30900.9 |
  31008.2 |########################################
  31115.4 |
  31222.6 |
  31329.8 |
  31437.1 |
  31544.3 |
  31651.5 |########################################
  (0 below, 1 above range)

carrier_opt_real_canon (n=6, range 29596.2-30968.3 ns)
  29596.2 |########################################
  29664.8 |
  29733.4 |
  29802.0 |
  29870.6 |
  29939.2 |
  30007.8 |
  30076.5 |
  30145.1 |
  30213.7 |########################################
  30282.3 |
  30350.9 |
  30419.5 |
  30488.1 |
  30556.7 |####################
  30625.3 |
  30693.9 |
  30762.5 |
  30831.1 |
  30899.7 |
  (0 below, 1 above range)

carrier_opt_real_cse (n=6, range 29545.8-31845.0 ns)
  29545.8 |########################################
  29660.8 |
  29775.7 |
  29890.7 |
  30005.6 |
  30120.6 |
  30235.6 |
  30350.5 |
  30465.5 |
  30580.4 |####################
  30695.4 |
  30810.4 |
  30925.3 |
  31040.3 |
  31155.2 |
  31270.2 |
  31385.2 |
  31500.1 |####################
  31615.1 |####################
  31730.0 |
  (0 below, 1 above range)

carrier_opt_real_dce (n=6, range 32359.2-34188.9 ns)
  32359.2 |########################################
  32450.7 |
  32542.2 |########################################
  32633.7 |
  32725.2 |########################################
  32816.6 |
  32908.1 |
  32999.6 |
  33091.1 |
  33182.6 |
  33274.1 |
  33365.6 |
  33457.0 |
  33548.5 |
  33640.0 |
  33731.5 |
  33823.0 |########################################
  33914.5 |
  34006.0 |########################################
  34097.5 |
  (0 below, 1 above range)

carrier_opt_real_fold (n=6, range 32162.9-34035.2 ns)
  32162.9 |########################################
  32256.5 |
  32350.1 |########################################
  32443.8 |########################################
  32537.4 |
  32631.0 |
  32724.6 |
  32818.2 |
  32911.8 |
  33005.5 |
  33099.1 |
  33192.7 |
  33286.3 |
  33379.9 |
  33473.5 |########################################
  33567.2 |
  33660.8 |
  33754.4 |
  33848.0 |
  33941.6 |########################################
  (0 below, 1 above range)

carrier_opt_real_none (n=6, range 32122.5-35178.3 ns)
  32122.5 |########################################
  32275.3 |
  32428.1 |
  32580.9 |####################
  32733.7 |
  32886.5 |
  33039.3 |
  33192.0 |
  33344.8 |
  33497.6 |
  33650.4 |
  33803.2 |
  33956.0 |
  34108.8 |
  34261.6 |
  34414.4 |
  34567.2 |########################################
  34720.0 |
  34872.8 |
  35025.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_real_all**: bridge=300.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_canon**: bridge=300.4% of algo (FFI overhead may distort results)
- **carrier_opt_real_cse**: bridge=300.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_dce**: bridge=300.8% of algo (FFI overhead may distort results)
- **carrier_opt_real_fold**: bridge=300.0% of algo (FFI overhead may distort results)
- **carrier_opt_real_none**: bridge=300.3% of algo (FFI overhead may distort results)

# Optimize stage (none/CSE/eqsat/CSE+eqsat...), downstream interp, wideselect profile

6 variants, 6 samples per variant.
Baseline: **carrier_opt_wideselect_none**

## Highlights

Baseline for all deltas below: **carrier_opt_wideselect_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_opt_wideselect_canon shows alternating (throttle bounce) (autocorr -0.59)

carrier_opt_wideselect_canon's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_opt_wideselect_all** at 26248.8 ns median (-16.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.23x (fastest 26248.8 ns, slowest 32338.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 28984ns | 28508ns | 28410ns | 28498ns | 30000ns | -14.72% |
| carrier_opt_wideselect_canon | 30055ns | 29878ns | 29139ns | 29702ns | 31042ns | -11.56% |
| carrier_opt_wideselect_cse | 29650ns | 29185ns | 28926ns | 29125ns | 30801ns | -12.76% |
| carrier_opt_wideselect_dce | 34247ns | 34689ns | 32959ns | 34117ns | 35085ns | +0.77% |
| carrier_opt_wideselect_fold | 35002ns | 33767ns | 32826ns | 33570ns | 38238ns | +2.99% |
| carrier_opt_wideselect_none | 33985ns | 33565ns | 32902ns | 33391ns | 35418ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_opt_wideselect_all | 26669ns | 26113ns | 27621ns | -15.77% | 0.038 |
| carrier_opt_wideselect_canon | 27731ns | 26824ns | 28613ns | -12.42% | 0.037 |
| carrier_opt_wideselect_cse | 27380ns | 26594ns | 28478ns | -13.53% | 0.037 |
| carrier_opt_wideselect_dce | 31916ns | 30741ns | 32660ns | +0.80% | 0.032 |
| carrier_opt_wideselect_fold | 32681ns | 30660ns | 35801ns | +3.21% | 0.031 |
| carrier_opt_wideselect_none | 31663ns | 30607ns | 33007ns | base | 0.032 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 409530 | 2344986 | 0.175 | 1.04× |
| carrier_opt_wideselect_canon | 414999 | 2374789 | 0.175 | 1.06× |
| carrier_opt_wideselect_cse | 415714 | 2383626 | 0.174 | 1.06× |
| carrier_opt_wideselect_dce | 392821 | 2182690 | 0.180 | 1.00× |
| carrier_opt_wideselect_fold | 395623 | 2173283 | 0.182 | 1.01× |
| carrier_opt_wideselect_none | 393348 | 2183502 | 0.180 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.039 Gops/s** (carrier_opt_wideselect_all; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_opt_wideselect_all | 0.039 | 99.5% |
| carrier_opt_wideselect_canon | 0.037 | 94.6% |
| carrier_opt_wideselect_cse | 0.038 | 96.9% |
| carrier_opt_wideselect_dce | 0.032 | 80.7% |
| carrier_opt_wideselect_fold | 0.033 | 83.1% |
| carrier_opt_wideselect_none | 0.033 | 83.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_opt_wideselect_all | 28984ns | 28984ns | -14.72% |
| carrier_opt_wideselect_canon | 30055ns | 30055ns | -11.56% |
| carrier_opt_wideselect_cse | 29650ns | 29650ns | -12.76% |
| carrier_opt_wideselect_dce | 34247ns | 34247ns | +0.77% |
| carrier_opt_wideselect_fold | 35002ns | 35002ns | +2.99% |
| carrier_opt_wideselect_none | 33985ns | 33985ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_opt_wideselect_none | 31245ns | base | --- | [30738, 33007] | --- | --- | --- | --- |
| carrier_opt_wideselect_all | 26249ns | -4560.0ns (-14.6%) | [-6122, -4301]ns | [26136, 27621] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_wideselect_canon | 27613ns | -3728.1ns (-11.9%) | [-4626, -3441]ns | [26968, 28613] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_wideselect_cse | 26946ns | -3839.6ns (-12.3%) | [-5823, -3187]ns | [26715, 28478] | YES (adj: no) | 0.0521 | 0.0313 | 0 |
| carrier_opt_wideselect_dce | 32338ns | no significant difference | [-627, +1313]ns | [30749, 32660] | no | 1.0000 | 1.0000 | 0 |
| carrier_opt_wideselect_fold | 31441ns | no significant difference | [-481, +3472]ns | [30800, 35801] | no | 0.2734 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_opt_wideselect_none | carrier_opt_wideselect_all | carrier_opt_wideselect_canon | carrier_opt_wideselect_cse | carrier_opt_wideselect_dce | carrier_opt_wideselect_fold |
|---|---|---|---|---|---|---|
| 1 | 30988ns | -15.3% | -12.5% | -10.8% | -0.7% | +2.1% |
| 2 | 31502ns | -14.2% | -10.9% | -15.6% | +3.7% | +0.0% |
| 3 | 33671ns | -22.3% | -15.5% | -20.0% | -3.0% | +18.7% |
| 4 | 30607ns | -14.7% | -11.3% | -12.3% | +4.8% | +0.2% |
| 5 | 32342ns | -12.8% | -11.1% | -9.4% | +0.8% | -3.0% |
| 6 | 30868ns | -15.0% | -13.1% | -12.7% | -0.4% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_opt_wideselect_all | -0.457 | moderate- |
| carrier_opt_wideselect_canon | -0.589 | HIGH- (thermal bounce) |
| carrier_opt_wideselect_cse | -0.289 | moderate- |
| carrier_opt_wideselect_dce | -0.213 | moderate- |
| carrier_opt_wideselect_fold | -0.265 | moderate- |
| carrier_opt_wideselect_none | -0.534 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_opt_wideselect_all**: won 6/6, lost 0/6
- **carrier_opt_wideselect_canon**: won 6/6, lost 0/6
- **carrier_opt_wideselect_cse**: won 6/6, lost 0/6
- **carrier_opt_wideselect_dce**: won 3/6, lost 3/6
- **carrier_opt_wideselect_fold**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_opt_wideselect_all | 104231.0ns | 26668.7ns | 390.8% | HIGH |
| carrier_opt_wideselect_canon | 107010.4ns | 27731.5ns | 385.9% | HIGH |
| carrier_opt_wideselect_cse | 106266.5ns | 27379.9ns | 388.1% | HIGH |
| carrier_opt_wideselect_dce | 95769.2ns | 31915.5ns | 300.1% | HIGH |
| carrier_opt_wideselect_fold | 95695.9ns | 32681.0ns | 292.8% | HIGH |
| carrier_opt_wideselect_none | 94611.2ns | 31663.1ns | 298.8% | HIGH |

## Distribution (algo ns)

```
carrier_opt_wideselect_all (n=6, range 26112.9-27621.0 ns)
  26112.9 |########################################
  26188.3 |########################################
  26263.7 |
  26339.1 |
  26414.5 |
  26489.9 |
  26565.3 |
  26640.7 |
  26716.1 |
  26791.5 |
  26867.0 |
  26942.4 |
  27017.8 |####################
  27093.2 |
  27168.6 |
  27244.0 |
  27319.4 |
  27394.8 |
  27470.2 |
  27545.6 |
  (0 below, 1 above range)

carrier_opt_wideselect_canon (n=6, range 26823.7-28613.0 ns)
  26823.7 |####################
  26913.2 |
  27002.6 |
  27092.1 |########################################
  27181.5 |
  27271.0 |
  27360.5 |
  27449.9 |
  27539.4 |
  27628.9 |
  27718.3 |
  27807.8 |
  27897.2 |
  27986.7 |
  28076.2 |####################
  28165.6 |
  28255.1 |
  28344.6 |
  28434.0 |####################
  28523.5 |
  (0 below, 1 above range)

carrier_opt_wideselect_cse (n=6, range 26593.8-28478.3 ns)
  26593.8 |####################
  26688.0 |
  26782.2 |####################
  26876.5 |########################################
  26970.7 |
  27064.9 |
  27159.1 |
  27253.4 |
  27347.6 |
  27441.8 |
  27536.0 |
  27630.3 |####################
  27724.5 |
  27818.7 |
  27913.0 |
  28007.2 |
  28101.4 |
  28195.6 |
  28289.8 |
  28384.1 |
  (0 below, 1 above range)

carrier_opt_wideselect_dce (n=6, range 30741.2-32659.6 ns)
  30741.2 |########################################
  30837.1 |
  30933.0 |
  31029.0 |
  31124.9 |
  31220.8 |
  31316.7 |
  31412.6 |
  31508.6 |
  31604.5 |
  31700.4 |
  31796.3 |
  31892.2 |
  31988.2 |####################
  32084.1 |
  32180.0 |
  32275.9 |
  32371.8 |
  32467.8 |
  32563.7 |########################################
  (0 below, 1 above range)

carrier_opt_wideselect_fold (n=6, range 30659.6-35801.4 ns)
  30659.6 |####################
  30916.7 |####################
  31173.8 |####################
  31430.9 |########################################
  31688.0 |
  31945.1 |
  32202.2 |
  32459.2 |
  32716.3 |
  32973.4 |
  33230.5 |
  33487.6 |
  33744.7 |
  34001.8 |
  34258.9 |
  34516.0 |
  34773.1 |
  35030.2 |
  35287.3 |
  35544.4 |
  (0 below, 1 above range)

carrier_opt_wideselect_none (n=6, range 30606.7-33006.6 ns)
  30606.7 |########################################
  30726.7 |
  30846.7 |########################################
  30966.7 |########################################
  31086.7 |
  31206.7 |
  31326.7 |
  31446.7 |########################################
  31566.7 |
  31686.7 |
  31806.7 |
  31926.7 |
  32046.7 |
  32166.7 |
  32286.7 |########################################
  32406.7 |
  32526.7 |
  32646.7 |
  32766.7 |
  32886.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_opt_wideselect_all**: bridge=399.6% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_canon**: bridge=391.9% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_cse**: bridge=400.0% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_dce**: bridge=299.1% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_fold**: bridge=300.2% of algo (FFI overhead may distort results)
- **carrier_opt_wideselect_none**: bridge=300.8% of algo (FFI overhead may distort results)

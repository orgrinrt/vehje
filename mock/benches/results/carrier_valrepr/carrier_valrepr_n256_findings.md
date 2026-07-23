# Value representation (static / runtime-tagged / NaN-boxed)

3 variants, 6 samples per variant.
Baseline: **carrier_vr_static**

## Highlights

Baseline for all deltas below: **carrier_vr_static**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_vr_nanbox shows alternating (throttle bounce) (autocorr -0.71)

carrier_vr_nanbox's per-pass series has lag-1 autocorrelation -0.71, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 4.5% of the fastest

All 3 variants sit between 8.12 us and 8.48 us - a 4.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_vr_nanbox** at 8115.6 ns median (-4.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.04x (fastest 8115.6 ns, slowest 8477.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vr_nanbox | 10435ns | 10442ns | 10123ns | 10411ns | 10626ns | -3.74% |
| carrier_vr_static | 10840ns | 10865ns | 10424ns | 10760ns | 11168ns | base |
| carrier_vr_tagged | 10848ns | 10734ns | 10457ns | 10682ns | 11292ns | +0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vr_nanbox | 8108ns | 7890ns | 8282ns | -4.20% | 0.032 |
| carrier_vr_static | 8464ns | 8222ns | 8642ns | base | 0.030 |
| carrier_vr_tagged | 8557ns | 8310ns | 8841ns | +1.10% | 0.030 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vr_nanbox | 298377 | 1868253 | 0.160 | 0.96× |
| carrier_vr_static | 310993 | 1332948 | 0.233 | 1.00× |
| carrier_vr_tagged | 312943 | 1911358 | 0.164 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.032 Gops/s** (carrier_vr_nanbox; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vr_nanbox | 0.032 | 97.2% |
| carrier_vr_static | 0.030 | 93.1% |
| carrier_vr_tagged | 0.030 | 93.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vr_nanbox | 10435ns | 10435ns | -3.74% |
| carrier_vr_static | 10840ns | 10840ns | base |
| carrier_vr_tagged | 10848ns | 10848ns | +0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vr_static | 8474ns | base | --- | [8276, 8642] | --- | --- | --- | --- |
| carrier_vr_nanbox | 8116ns | -337.3ns (-4.0%) | [-478, -251]ns | [7928, 8282] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| carrier_vr_tagged | 8477ns | no significant difference | [-258, +428]ns | [8354, 8841] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vr_static | carrier_vr_nanbox | carrier_vr_tagged |
|---|---|---|---|
| 1 | 8482ns | -4.0% | -2.0% |
| 2 | 8330ns | -2.8% | +2.3% |
| 3 | 8222ns | -4.0% | +2.1% |
| 4 | 8781ns | -5.2% | -3.9% |
| 5 | 8466ns | -5.9% | +7.9% |
| 6 | 8504ns | -3.1% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vr_nanbox | -0.706 | HIGH- (thermal bounce) |
| carrier_vr_static | -0.256 | moderate- |
| carrier_vr_tagged | -0.092 | ok |

**Consistency summary:**

- **carrier_vr_nanbox**: won 6/6, lost 0/6
- **carrier_vr_tagged**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vr_nanbox | 88819.3ns | 8108.5ns | 1095.4% | HIGH |
| carrier_vr_static | 92068.0ns | 8464.1ns | 1087.7% | HIGH |
| carrier_vr_tagged | 90777.7ns | 8557.4ns | 1060.8% | HIGH |

## Distribution (algo ns)

```
carrier_vr_nanbox (n=6, range 7890.4-8282.3 ns)
   7890.4 |########################################
   7910.0 |
   7929.6 |
   7949.2 |########################################
   7968.8 |
   7988.4 |
   8008.0 |
   8027.6 |
   8047.2 |
   8066.8 |
   8086.3 |########################################
   8105.9 |
   8125.5 |########################################
   8145.1 |
   8164.7 |
   8184.3 |
   8203.9 |
   8223.5 |########################################
   8243.1 |
   8262.7 |
  (0 below, 1 above range)

carrier_vr_static (n=6, range 8222.1-8642.5 ns)
   8222.1 |########################################
   8243.1 |
   8264.1 |
   8285.2 |
   8306.2 |
   8327.2 |########################################
   8348.2 |
   8369.2 |
   8390.3 |
   8411.3 |
   8432.3 |
   8453.3 |########################################
   8474.3 |########################################
   8495.4 |########################################
   8516.4 |
   8537.4 |
   8558.4 |
   8579.4 |
   8600.5 |
   8621.5 |
  (0 below, 1 above range)

carrier_vr_tagged (n=6, range 8309.6-8841.2 ns)
   8309.6 |########################################
   8336.2 |
   8362.8 |
   8389.3 |########################################
   8415.9 |########################################
   8442.5 |
   8469.1 |
   8495.7 |########################################
   8522.3 |########################################
   8548.8 |
   8575.4 |
   8602.0 |
   8628.6 |
   8655.2 |
   8681.8 |
   8708.3 |
   8734.9 |
   8761.5 |
   8788.1 |
   8814.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vr_nanbox**: bridge=1089.6% of algo (FFI overhead may distort results)
- **carrier_vr_static**: bridge=1089.9% of algo (FFI overhead may distort results)
- **carrier_vr_tagged**: bridge=1071.6% of algo (FFI overhead may distort results)

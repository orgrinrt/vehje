# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), tight profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_tight_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_tight_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_tight_null dominates: 14% faster than the next best (carrier_cold_tight_threaded)

carrier_cold_tight_null (7.16 us) leads carrier_cold_tight_threaded (8.16 us) by 14%, a clear separation rather than a photo finish. CV 2.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### Two tiers: {carrier_cold_tight_null, carrier_cold_tight_threaded, carrier_cold_tight_switch} vs {carrier_cold_tight_fntable} (31% apart)

The field splits into a fast tier {carrier_cold_tight_null, carrier_cold_tight_threaded, carrier_cold_tight_switch} and a slow tier {carrier_cold_tight_fntable} with a 31% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cold_tight_null** at 7158.7 ns median (-14.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.54x (fastest 7158.7 ns, slowest 11014.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_tight_fntable | 13380ns | 13423ns | 12980ns | 13299ns | 13702ns | +25.23% |
| carrier_cold_tight_null | 9530ns | 9485ns | 9255ns | 9439ns | 9803ns | -10.80% |
| carrier_cold_tight_switch | 10684ns | 10746ns | 10340ns | 10712ns | 10815ns | base |
| carrier_cold_tight_threaded | 10569ns | 10404ns | 10249ns | 10371ns | 11025ns | -1.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_tight_fntable | 11024ns | 10691ns | 11322ns | +31.95% | 0.023 |
| carrier_cold_tight_null | 7215ns | 7064ns | 7405ns | -13.64% | 0.035 |
| carrier_cold_tight_switch | 8355ns | 8113ns | 8440ns | base | 0.031 |
| carrier_cold_tight_threaded | 8232ns | 7998ns | 8536ns | -1.46% | 0.031 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_tight_fntable | 298540 | 890970 | 0.335 | 1.03× |
| carrier_cold_tight_null | 296579 | 1197436 | 0.248 | 1.02× |
| carrier_cold_tight_switch | 289737 | 926096 | 0.313 | 1.00× |
| carrier_cold_tight_threaded | 305132 | 1040638 | 0.293 | 1.05× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.036 Gops/s** (carrier_cold_tight_null; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_tight_fntable | 0.023 | 64.1% |
| carrier_cold_tight_null | 0.036 | 98.7% |
| carrier_cold_tight_switch | 0.030 | 84.0% |
| carrier_cold_tight_threaded | 0.031 | 86.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_tight_fntable | 13380ns | 13380ns | +25.23% |
| carrier_cold_tight_null | 9530ns | 9530ns | -10.80% |
| carrier_cold_tight_switch | 10684ns | 10684ns | base |
| carrier_cold_tight_threaded | 10569ns | 10569ns | -1.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_tight_switch | 8412ns | base | --- | [8213, 8440] | --- | --- | --- | --- |
| carrier_cold_tight_fntable | 11015ns | +2619.0ns (+31.1%) | [+2503, +2887]ns | [10736, 11322] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_tight_null | 7159ns | -1180.8ns (-14.0%) | [-1281, -957]ns | [7082, 7405] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_tight_threaded | 8160ns | no significant difference | [-337, +131]ns | [8002, 8536] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_tight_switch | carrier_cold_tight_fntable | carrier_cold_tight_null | carrier_cold_tight_threaded |
|---|---|---|---|---|
| 1 | 8113ns | +31.8% | -12.9% | -1.3% |
| 2 | 8313ns | +29.7% | -14.6% | -2.8% |
| 3 | 8442ns | +30.1% | -14.6% | -5.3% |
| 4 | 8390ns | +31.7% | -10.3% | +4.4% |
| 5 | 8438ns | +36.8% | -15.8% | -1.5% |
| 6 | 8434ns | +31.6% | -13.6% | -2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_tight_fntable | 0.323 | moderate+ |
| carrier_cold_tight_null | -0.166 | ok |
| carrier_cold_tight_switch | 0.231 | moderate+ |
| carrier_cold_tight_threaded | -0.031 | ok |

**Consistency summary:**

- **carrier_cold_tight_fntable**: won 0/6, lost 6/6
- **carrier_cold_tight_null**: won 6/6, lost 0/6
- **carrier_cold_tight_threaded**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_tight_fntable | 86127.1ns | 11024.3ns | 781.2% | HIGH |
| carrier_cold_tight_null | 88162.8ns | 7215.3ns | 1221.9% | HIGH |
| carrier_cold_tight_switch | 84544.8ns | 8354.9ns | 1011.9% | HIGH |
| carrier_cold_tight_threaded | 89310.8ns | 8232.5ns | 1084.9% | HIGH |

## Distribution (algo ns)

```
carrier_cold_tight_fntable (n=6, range 10691.2-11322.3 ns)
  10691.2 |########################################
  10722.8 |
  10754.3 |########################################
  10785.9 |
  10817.4 |
  10849.0 |
  10880.5 |
  10912.1 |
  10943.6 |
  10975.2 |########################################
  11006.8 |
  11038.3 |########################################
  11069.9 |
  11101.4 |########################################
  11133.0 |
  11164.5 |
  11196.1 |
  11227.6 |
  11259.2 |
  11290.7 |
  (0 below, 1 above range)

carrier_cold_tight_null (n=6, range 7063.8-7405.2 ns)
   7063.8 |####################
   7080.9 |
   7097.9 |########################################
   7115.0 |
   7132.1 |
   7149.2 |
   7166.2 |
   7183.3 |
   7200.4 |####################
   7217.4 |
   7234.5 |
   7251.6 |
   7268.6 |####################
   7285.7 |
   7302.8 |
   7319.9 |
   7336.9 |
   7354.0 |
   7371.1 |
   7388.1 |
  (0 below, 1 above range)

carrier_cold_tight_switch (n=6, range 8113.3-8439.6 ns)
   8113.3 |####################
   8129.6 |
   8145.9 |
   8162.2 |
   8178.6 |
   8194.9 |
   8211.2 |
   8227.5 |
   8243.8 |
   8260.1 |
   8276.5 |
   8292.8 |
   8309.1 |####################
   8325.4 |
   8341.7 |
   8358.0 |
   8374.3 |####################
   8390.7 |
   8407.0 |
   8423.3 |########################################
  (0 below, 1 above range)

carrier_cold_tight_threaded (n=6, range 7998.3-8535.6 ns)
   7998.3 |########################################
   8025.2 |
   8052.0 |
   8078.9 |####################
   8105.8 |
   8132.6 |
   8159.5 |
   8186.4 |
   8213.2 |####################
   8240.1 |
   8267.0 |
   8293.8 |####################
   8320.7 |
   8347.5 |
   8374.4 |
   8401.3 |
   8428.1 |
   8455.0 |
   8481.9 |
   8508.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_tight_fntable**: bridge=782.3% of algo (FFI overhead may distort results)
- **carrier_cold_tight_null**: bridge=1231.9% of algo (FFI overhead may distort results)
- **carrier_cold_tight_switch**: bridge=1007.0% of algo (FFI overhead may distort results)
- **carrier_cold_tight_threaded**: bridge=1092.3% of algo (FFI overhead may distort results)

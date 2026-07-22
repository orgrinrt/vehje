# Residual encoding: register/SSA vs stack bytecode, real profile

2 variants, 6 samples per variant.
Baseline: **carrier_res_real_register**

## Highlights

Baseline for all deltas below: **carrier_res_real_register**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_res_real_register)

The baseline carrier_res_real_register is the fastest (2.61 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_res_real_register) is the fastest** at 2606839.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 2606839.2 ns, slowest 2806077.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_res_real_register | 2610416ns | 2610348ns | 2576852ns | 2610236ns | 2627467ns | base |
| carrier_res_real_stack | 2811970ns | 2808420ns | 2793460ns | 2807502ns | 2827928ns | +7.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_res_real_register | 2607055ns | 2574089ns | 2624052ns | base | 0.006 |
| carrier_res_real_stack | 2809550ns | 2791138ns | 2825508ns | +7.77% | 0.006 |

## Performance model

- Peak throughput: **0.006 Gops/s** (carrier_res_real_register; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_res_real_register | 0.006 | 98.7% |
| carrier_res_real_stack | 0.006 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_res_real_register | 2610416ns | 2610416ns | base |
| carrier_res_real_stack | 2811970ns | 2811970ns | +7.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_res_real_register | 2606839ns | base | --- | [2590273, 2624052] | --- | --- | --- | --- |
| carrier_res_real_stack | 2806077ns | +201201.2ns (+7.7%) | [+193146, +213139]ns | [2797066, 2825508] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_res_real_register | carrier_res_real_stack |
|---|---|---|
| 1 | 2606457ns | +7.5% |
| 2 | 2616632ns | +7.3% |
| 3 | 2631472ns | +8.0% |
| 4 | 2606904ns | +7.8% |
| 5 | 2606774ns | +7.6% |
| 6 | 2574089ns | +8.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_res_real_register | 0.132 | ok |
| carrier_res_real_stack | 0.009 | ok |

**Consistency summary:**

- **carrier_res_real_stack**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_res_real_register | 2610570.0ns | 2607054.9ns | 100.1% | HIGH |
| carrier_res_real_stack | 2767477.2ns | 2809550.2ns | 98.5% | HIGH |

## Distribution (algo ns)

```
carrier_res_real_register (n=6, range 2574089.2-2624052.3 ns)
  2574089.2 |####################
  2576587.4 |
  2579085.5 |
  2581583.7 |
  2584081.8 |
  2586580.0 |
  2589078.1 |
  2591576.3 |
  2594074.4 |
  2596572.6 |
  2599070.8 |
  2601568.9 |
  2604067.1 |####################
  2606565.2 |########################################
  2609063.4 |
  2611561.5 |
  2614059.7 |
  2616557.8 |####################
  2619056.0 |
  2621554.1 |
  (0 below, 1 above range)

carrier_res_real_stack (n=6, range 2791137.5-2825507.7 ns)
  2791137.5 |####################
  2792856.0 |
  2794574.5 |
  2796293.0 |
  2798011.5 |
  2799730.0 |
  2801448.6 |####################
  2803167.1 |
  2804885.6 |########################################
  2806604.1 |
  2808322.6 |
  2810041.1 |####################
  2811759.6 |
  2813478.1 |
  2815196.6 |
  2816915.2 |
  2818633.7 |
  2820352.2 |
  2822070.7 |
  2823789.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_res_real_register**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_res_real_stack**: bridge=98.7% of algo (FFI overhead may distort results)

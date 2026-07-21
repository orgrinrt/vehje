# Branch strategies, cheap-arm, runs: correlated long runs (flip when b<24)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_runs**

## Key findings

- **Baseline (br_branch_c_runs) is the fastest** at 32941.1 ns median
- Spread: 1.08x (fastest 32941.1 ns, slowest 35687.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_runs | 35612ns | 35379ns | 31581ns | 34320ns | 39565ns | base |
| br_lut_c_runs | 37101ns | 38098ns | 34426ns | 36885ns | 38763ns | +4.18% |
| br_mask_c_runs | 37083ns | 37610ns | 34715ns | 36740ns | 38781ns | +4.13% |
| br_predicate_c_runs | 37261ns | 36923ns | 34103ns | 36319ns | 40254ns | +4.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_runs | 33194ns | 29369ns | 36955ns | base | 0.494 |
| br_lut_c_runs | 34748ns | 32208ns | 36315ns | +4.68% | 0.472 |
| br_mask_c_runs | 34713ns | 32522ns | 36313ns | +4.58% | 0.472 |
| br_predicate_c_runs | 34843ns | 31955ns | 37541ns | +4.97% | 0.470 |

## Performance model

- Peak throughput: **0.558 Gops/s** (br_branch_c_runs; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_runs | 0.497 | 89.2% |
| br_lut_c_runs | 0.459 | 82.3% |
| br_mask_c_runs | 0.465 | 83.4% |
| br_predicate_c_runs | 0.474 | 84.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_runs | 35612ns | 35612ns | base |
| br_lut_c_runs | 37101ns | 37101ns | +4.18% |
| br_mask_c_runs | 37083ns | 37083ns | +4.13% |
| br_predicate_c_runs | 37261ns | 37261ns | +4.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_runs | 32941ns | base | --- | [29686, 36955] | --- | --- | --- | --- |
| br_lut_c_runs | 35688ns | no significant difference | [-1319, +3071]ns | [32241, 36315] | no | 0.2188 | 0.2188 | 0 |
| br_mask_c_runs | 35202ns | no significant difference | [-1456, +3416]ns | [32625, 36313] | no | 0.2188 | 0.2188 | 0 |
| br_predicate_c_runs | 34590ns | no significant difference | [-2776, +4690]ns | [32399, 37541] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_runs | br_lut_c_runs | br_mask_c_runs | br_predicate_c_runs |
|---|---|---|---|---|
| 1 | 40228ns | -12.0% | -9.8% | -20.6% |
| 2 | 29369ns | +9.9% | +11.4% | +11.8% |
| 3 | 30004ns | +7.3% | +8.4% | +9.7% |
| 4 | 33109ns | +8.8% | +3.2% | +9.5% |
| 5 | 32773ns | +9.8% | +10.6% | +18.0% |
| 6 | 33681ns | +8.6% | +7.9% | +8.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_runs | -0.195 | ok |
| br_lut_c_runs | 0.272 | moderate+ |
| br_mask_c_runs | 0.251 | moderate+ |
| br_predicate_c_runs | 0.521 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **br_lut_c_runs**: won 1/6, lost 5/6
- **br_mask_c_runs**: won 1/6, lost 5/6
- **br_predicate_c_runs**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_runs | 5.5ns | 33194.1ns | 0.0% |  |
| br_lut_c_runs | 3.5ns | 34748.0ns | 0.0% |  |
| br_mask_c_runs | 3.1ns | 34713.3ns | 0.0% |  |
| br_predicate_c_runs | 3.7ns | 34843.1ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_runs (n=6, range 29369.2-36954.8 ns)
  29369.2 |########################################
  29748.5 |########################################
  30127.8 |
  30507.0 |
  30886.3 |
  31265.6 |
  31644.9 |
  32024.1 |
  32403.4 |########################################
  32782.7 |########################################
  33162.0 |
  33541.3 |########################################
  33920.5 |
  34299.8 |
  34679.1 |
  35058.4 |
  35437.6 |
  35816.9 |
  36196.2 |
  36575.5 |
  (0 below, 1 above range)

br_lut_c_runs (n=6, range 32207.5-36314.6 ns)
  32207.5 |########################################
  32412.9 |
  32618.2 |
  32823.6 |
  33028.9 |
  33234.3 |
  33439.6 |
  33645.0 |
  33850.3 |
  34055.7 |
  34261.0 |
  34466.4 |
  34671.7 |
  34877.1 |
  35082.4 |
  35287.8 |####################
  35493.1 |
  35698.5 |
  35903.8 |########################################
  36109.2 |
  (0 below, 1 above range)

br_mask_c_runs (n=6, range 32521.7-36312.7 ns)
  32521.7 |####################
  32711.2 |####################
  32900.8 |
  33090.3 |
  33279.9 |
  33469.4 |
  33659.0 |
  33848.6 |
  34038.1 |####################
  34227.7 |
  34417.2 |
  34606.8 |
  34796.3 |
  34985.8 |
  35175.4 |
  35364.9 |
  35554.5 |
  35744.0 |
  35933.6 |
  36123.1 |########################################
  (0 below, 1 above range)

br_predicate_c_runs (n=6, range 31954.6-37540.8 ns)
  31954.6 |####################
  32233.9 |
  32513.2 |
  32792.5 |########################################
  33071.8 |
  33351.2 |
  33630.5 |
  33909.8 |
  34189.1 |
  34468.4 |
  34747.7 |
  35027.0 |
  35306.3 |
  35585.6 |
  35864.9 |
  36144.2 |########################################
  36423.6 |
  36702.9 |
  36982.2 |
  37261.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **br_predicate_c_runs**: autocorrelation=0.52 (measurement drift or warm-up artifact)

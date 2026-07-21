# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Key findings

- **Fastest: br_lut_c_rand50** at 32794.6 ns median (-51.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.06x (fastest 32794.6 ns, slowest 67596.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 68016ns | 70151ns | 62098ns | 68147ns | 70780ns | base |
| br_lut_c_rand50 | 34899ns | 35250ns | 31916ns | 34664ns | 36742ns | -48.69% |
| br_mask_c_rand50 | 41259ns | 41754ns | 36662ns | 41400ns | 43346ns | -39.34% |
| br_predicate_c_rand50 | 35601ns | 36100ns | 32168ns | 35317ns | 37743ns | -47.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 65554ns | 59860ns | 68238ns | base | 0.250 |
| br_lut_c_rand50 | 32502ns | 29766ns | 34254ns | -50.42% | 0.504 |
| br_mask_c_rand50 | 38759ns | 34450ns | 40699ns | -40.87% | 0.423 |
| br_predicate_c_rand50 | 33167ns | 29992ns | 35147ns | -49.41% | 0.494 |

## Performance model

- Peak throughput: **0.550 Gops/s** (br_lut_c_rand50; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.242 | 44.0% |
| br_lut_c_rand50 | 0.500 | 90.8% |
| br_mask_c_rand50 | 0.417 | 75.8% |
| br_predicate_c_rand50 | 0.487 | 88.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 68016ns | 68016ns | base |
| br_lut_c_rand50 | 34899ns | 34899ns | -48.69% |
| br_mask_c_rand50 | 41259ns | 41259ns | -39.34% |
| br_predicate_c_rand50 | 35601ns | 35601ns | -47.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 67596ns | base | --- | [60827, 68238] | --- | --- | --- | --- |
| br_lut_c_rand50 | 32795ns | -33447.3ns (-49.5%) | [-35338, -30370]ns | [30457, 34254] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 39252ns | -27732.4ns (-41.0%) | [-29760, -22893]ns | [36326, 40699] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 33640ns | -32485.4ns (-48.1%) | [-34597, -30079]ns | [30713, 35147] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 61795ns | -51.8% | -33.0% | -51.5% |
| 2 | 67597ns | -52.8% | -40.9% | -46.9% |
| 3 | 68667ns | -51.0% | -43.6% | -50.2% |
| 4 | 67595ns | -50.0% | -41.2% | -49.1% |
| 5 | 67808ns | -48.8% | -43.7% | -51.2% |
| 6 | 59860ns | -48.0% | -42.4% | -47.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | -0.046 | ok |
| br_lut_c_rand50 | 0.123 | ok |
| br_mask_c_rand50 | 0.179 | ok |
| br_predicate_c_rand50 | -0.189 | ok |

**Consistency summary:**

- **br_lut_c_rand50**: won 6/6, lost 0/6
- **br_mask_c_rand50**: won 6/6, lost 0/6
- **br_predicate_c_rand50**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 6.1ns | 65553.7ns | 0.0% |  |
| br_lut_c_rand50 | 3.5ns | 32502.0ns | 0.0% |  |
| br_mask_c_rand50 | 5.0ns | 38758.6ns | 0.0% |  |
| br_predicate_c_rand50 | 4.5ns | 33166.7ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 59859.6-68237.5 ns)
  59859.6 |#############
  60278.5 |
  60697.4 |
  61116.3 |
  61535.2 |#############
  61954.1 |
  62373.0 |
  62791.9 |
  63210.8 |
  63629.7 |
  64048.6 |
  64467.4 |
  64886.3 |
  65305.2 |
  65724.1 |
  66143.0 |
  66561.9 |
  66980.8 |
  67399.7 |########################################
  67818.6 |
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 29766.2-34254.3 ns)
  29766.2 |####################
  29990.6 |
  30215.0 |
  30439.4 |
  30663.8 |
  30888.2 |
  31112.6 |####################
  31337.1 |
  31561.5 |
  31785.9 |####################
  32010.3 |
  32234.7 |
  32459.1 |
  32683.5 |
  32907.9 |
  33132.3 |
  33356.7 |
  33581.1 |########################################
  33805.5 |
  34029.9 |
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 34450.4-40698.8 ns)
  34450.4 |########################################
  34762.8 |
  35075.2 |
  35387.7 |
  35700.1 |
  36012.5 |
  36324.9 |
  36637.3 |
  36949.7 |
  37262.2 |
  37574.6 |
  37887.0 |
  38199.4 |########################################
  38511.8 |########################################
  38824.2 |
  39136.7 |
  39449.1 |########################################
  39761.5 |########################################
  40073.9 |
  40386.3 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 29992.1-35146.7 ns)
  29992.1 |########################################
  30249.8 |
  30507.6 |
  30765.3 |
  31023.0 |
  31280.7 |########################################
  31538.5 |
  31796.2 |
  32053.9 |
  32311.6 |
  32569.4 |
  32827.1 |########################################
  33084.8 |
  33342.6 |
  33600.3 |
  33858.0 |
  34115.7 |########################################
  34373.5 |########################################
  34631.2 |
  34888.9 |
  (0 below, 1 above range)

```

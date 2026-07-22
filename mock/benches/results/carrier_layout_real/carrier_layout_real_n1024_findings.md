# Record layout (REC12..REC32) with fixed switch dispatch, real profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_real_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_real_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (carrier_lay_real_rec24)

The baseline carrier_lay_real_rec24 is the fastest (42.15 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.8% of the fastest

All 5 variants sit between 42.15 us and 42.89 us - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_lay_real_rec24) is the fastest** at 42151.2 ns median
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 42151.2 ns, slowest 42890.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 44888ns | 45213ns | 42057ns | 45118ns | 45958ns | +0.95% |
| carrier_lay_real_rec16 | 44869ns | 44688ns | 43972ns | 44497ns | 45876ns | +0.91% |
| carrier_lay_real_rec20 | 50922ns | 45379ns | 44093ns | 45358ns | 62683ns | +14.52% |
| carrier_lay_real_rec24 | 44466ns | 44653ns | 43090ns | 44423ns | 45219ns | base |
| carrier_lay_real_rec32 | 48900ns | 45294ns | 44415ns | 45139ns | 56784ns | +9.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_real_rec12 | 42406ns | 39773ns | 43445ns | +1.02% | 0.024 |
| carrier_lay_real_rec16 | 42444ns | 41523ns | 43376ns | +1.11% | 0.024 |
| carrier_lay_real_rec20 | 48459ns | 41811ns | 60153ns | +15.44% | 0.021 |
| carrier_lay_real_rec24 | 41979ns | 40810ns | 42644ns | base | 0.024 |
| carrier_lay_real_rec32 | 46381ns | 41993ns | 54268ns | +10.49% | 0.022 |

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_lay_real_rec12; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_real_rec12 | 0.024 | 93.3% |
| carrier_lay_real_rec16 | 0.024 | 94.1% |
| carrier_lay_real_rec20 | 0.024 | 92.7% |
| carrier_lay_real_rec24 | 0.024 | 94.4% |
| carrier_lay_real_rec32 | 0.024 | 93.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_real_rec12 | 44888ns | 44888ns | +0.95% |
| carrier_lay_real_rec16 | 44869ns | 44869ns | +0.91% |
| carrier_lay_real_rec20 | 50922ns | 50922ns | +14.52% |
| carrier_lay_real_rec24 | 44466ns | 44466ns | base |
| carrier_lay_real_rec32 | 48900ns | 48900ns | +9.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_real_rec24 | 42151ns | base | --- | [41141, 42644] | --- | --- | --- | --- |
| carrier_lay_real_rec12 | 42651ns | no significant difference | [-1351, +1512]ns | [41121, 43445] | no | 0.2917 | 0.2188 | 0 |
| carrier_lay_real_rec16 | 42266ns | no significant difference | [-378, +1658]ns | [41689, 43376] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_real_rec20 | 42890ns | +870.8ns (+2.1%) | [+120, +18450]ns | [42334, 60153] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| carrier_lay_real_rec32 | 42709ns | +548.5ns (+1.3%) | [+91, +12566]ns | [42165, 54268] | YES (adj: no) | 0.2917 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_real_rec24 | carrier_lay_real_rec12 | carrier_lay_real_rec16 | carrier_lay_real_rec20 | carrier_lay_real_rec32 |
|---|---|---|---|---|---|
| 1 | 42595ns | -6.6% | -1.3% | +37.1% | +47.6% |
| 2 | 41760ns | +2.9% | +5.4% | +0.1% | +1.4% |
| 3 | 40810ns | +4.1% | +2.6% | +51.7% | +11.9% |
| 4 | 42543ns | +3.2% | +0.4% | +0.7% | +0.7% |
| 5 | 42692ns | +0.3% | -0.5% | +0.4% | -0.3% |
| 6 | 41473ns | +2.5% | +0.1% | +3.4% | +1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_real_rec12 | -0.077 | ok |
| carrier_lay_real_rec16 | -0.449 | moderate- |
| carrier_lay_real_rec20 | -0.404 | moderate- |
| carrier_lay_real_rec24 | -0.173 | ok |
| carrier_lay_real_rec32 | -0.093 | ok |

**Consistency summary:**

- **carrier_lay_real_rec12**: won 1/6, lost 5/6
- **carrier_lay_real_rec16**: won 2/6, lost 4/6
- **carrier_lay_real_rec20**: won 0/6, lost 6/6
- **carrier_lay_real_rec32**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_real_rec12 | 109185.0ns | 42406.1ns | 257.5% | HIGH |
| carrier_lay_real_rec16 | 122606.4ns | 42443.5ns | 288.9% | HIGH |
| carrier_lay_real_rec20 | 122249.0ns | 48459.0ns | 252.3% | HIGH |
| carrier_lay_real_rec24 | 122739.8ns | 41978.8ns | 292.4% | HIGH |
| carrier_lay_real_rec32 | 121266.8ns | 46380.6ns | 261.5% | HIGH |

## Distribution (algo ns)

```
carrier_lay_real_rec12 (n=6, range 39772.9-43445.4 ns)
  39772.9 |####################
  39956.5 |
  40140.2 |
  40323.8 |
  40507.4 |
  40691.0 |
  40874.7 |
  41058.3 |
  41241.9 |
  41425.5 |
  41609.2 |
  41792.8 |
  41976.4 |
  42160.0 |
  42343.7 |########################################
  42527.3 |
  42710.9 |####################
  42894.5 |####################
  43078.2 |
  43261.8 |
  (0 below, 1 above range)

carrier_lay_real_rec16 (n=6, range 41523.3-43376.2 ns)
  41523.3 |########################################
  41615.9 |
  41708.6 |
  41801.2 |########################################
  41893.9 |
  41986.5 |########################################
  42079.2 |
  42171.8 |
  42264.5 |
  42357.1 |
  42449.8 |########################################
  42542.4 |
  42635.0 |########################################
  42727.7 |
  42820.3 |
  42913.0 |
  43005.6 |
  43098.3 |
  43190.9 |
  43283.6 |
  (0 below, 1 above range)

carrier_lay_real_rec20 (n=6, range 41810.8-60152.7 ns)
  41810.8 |#############
  42727.9 |########################################
  43645.0 |
  44562.1 |
  45479.2 |
  46396.3 |
  47313.4 |
  48230.5 |
  49147.6 |
  50064.7 |
  50981.8 |
  51898.8 |
  52815.9 |
  53733.0 |
  54650.1 |
  55567.2 |
  56484.3 |
  57401.4 |
  58318.5 |#############
  59235.6 |
  (0 below, 1 above range)

carrier_lay_real_rec24 (n=6, range 40810.0-42643.6 ns)
  40810.0 |########################################
  40901.7 |
  40993.4 |
  41085.0 |
  41176.7 |
  41268.4 |
  41360.1 |
  41451.7 |########################################
  41543.4 |
  41635.1 |
  41726.8 |########################################
  41818.5 |
  41910.1 |
  42001.8 |
  42093.5 |
  42185.2 |
  42276.8 |
  42368.5 |
  42460.2 |########################################
  42551.9 |########################################
  (0 below, 1 above range)

carrier_lay_real_rec32 (n=6, range 41992.9-54268.3 ns)
  41992.9 |########################################
  42606.7 |#############
  43220.4 |
  43834.2 |
  44448.0 |
  45061.8 |#############
  45675.5 |
  46289.3 |
  46903.1 |
  47516.9 |
  48130.6 |
  48744.4 |
  49358.2 |
  49971.9 |
  50585.7 |
  51199.5 |
  51813.3 |
  52427.0 |
  53040.8 |
  53654.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_real_rec12**: bridge=271.2% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec16**: bridge=289.6% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec20**: bridge=285.4% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec24**: bridge=290.8% of algo (FFI overhead may distort results)
- **carrier_lay_real_rec32**: bridge=287.3% of algo (FFI overhead may distort results)

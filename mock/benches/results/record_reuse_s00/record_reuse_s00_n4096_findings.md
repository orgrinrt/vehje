# Record update (0% shared): always-copy vs in-place-when-unique vs mutable ceiling

3 variants, 6 samples per variant.
Baseline: **rec_reuse_s00**

## Highlights

Baseline for all deltas below: **rec_reuse_s00**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rec_copy is an outlier: 7.1x slower than the field

rec_copy (115.46 us) is 7.1x the fastest (16.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rec_mut is fastest but the noisiest (CV 5.4%)

rec_mut wins on median (16.19 us) yet has the highest variance (CV 5.4%), while rec_copy is the steadiest (CV 2.2%, 115.46 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 7.1x the fastest

Fastest rec_mut (16.19 us) to slowest rec_copy (115.46 us): 7.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: rec_mut** at 16187.0 ns median (-9.0% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.13x (fastest 16187.0 ns, slowest 115462.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rec_copy | 118413ns | 117796ns | 116031ns | 117326ns | 121236ns | +480.92% |
| rec_mut | 18941ns | 18464ns | 17988ns | 18352ns | 20302ns | -7.08% |
| rec_reuse_s00 | 20384ns | 20111ns | 19014ns | 20008ns | 21633ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rec_copy | 116064ns | 113774ns | 118843ns | +542.95% | 0.035 |
| rec_mut | 16625ns | 15794ns | 17834ns | -7.90% | 0.246 |
| rec_reuse_s00 | 18052ns | 16811ns | 19181ns | base | 0.227 |

## Performance model

- Peak throughput: **0.259 Gops/s** (rec_mut; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rec_copy | 0.035 | 13.7% |
| rec_mut | 0.253 | 97.6% |
| rec_reuse_s00 | 0.230 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rec_copy | 118413ns | 118413ns | +480.92% |
| rec_mut | 18941ns | 18941ns | -7.08% |
| rec_reuse_s00 | 20384ns | 20384ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rec_reuse_s00 | 17787ns | base | --- | [17186, 19181] | --- | --- | --- | --- |
| rec_copy | 115462ns | +97046.1ns (+545.6%) | [+96099, +100891]ns | [113886, 118843] | YES | 0.0313 | 0.0313 | 0 |
| rec_mut | 16187ns | -1336.7ns (-7.5%) | [-2340, -603]ns | [15855, 17834] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rec_reuse_s00 | rec_copy | rec_mut |
|---|---|---|---|
| 1 | 19077ns | +511.6% | -16.6% |
| 2 | 19286ns | +505.0% | -7.0% |
| 3 | 17656ns | +545.7% | -8.6% |
| 4 | 17562ns | +550.6% | -7.5% |
| 5 | 16811ns | +619.7% | -6.0% |
| 6 | 17918ns | +535.0% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rec_copy | -0.459 | moderate- |
| rec_mut | -0.436 | moderate- |
| rec_reuse_s00 | 0.385 | moderate+ |

**Consistency summary:**

- **rec_copy**: won 0/6, lost 6/6
- **rec_mut**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rec_copy | 21478.1ns | 116063.8ns | 18.5% | HIGH |
| rec_mut | 22164.3ns | 16625.2ns | 133.3% | HIGH |
| rec_reuse_s00 | 22184.7ns | 18051.6ns | 122.9% | HIGH |

## Distribution (algo ns)

```
rec_copy (n=6, range 113773.8-118842.7 ns)
  113773.8 |########################################
  114027.2 |####################
  114280.7 |
  114534.1 |
  114787.6 |
  115041.0 |
  115294.5 |
  115547.9 |
  115801.4 |
  116054.8 |
  116308.2 |
  116561.7 |########################################
  116815.1 |
  117068.6 |
  117322.0 |
  117575.5 |
  117828.9 |
  118082.4 |
  118335.8 |
  118589.3 |
  (0 below, 1 above range)

rec_mut (n=6, range 15793.8-17834.0 ns)
  15793.8 |########################################
  15895.8 |########################################
  15997.8 |
  16099.8 |########################################
  16201.8 |########################################
  16303.8 |
  16405.8 |
  16507.9 |
  16609.9 |
  16711.9 |
  16813.9 |
  16915.9 |
  17017.9 |
  17119.9 |
  17221.9 |
  17323.9 |
  17425.9 |
  17527.9 |
  17629.9 |########################################
  17731.9 |
  (0 below, 1 above range)

rec_reuse_s00 (n=6, range 16810.8-19181.5 ns)
  16810.8 |########################################
  16929.3 |
  17047.9 |
  17166.4 |
  17284.9 |
  17403.5 |
  17522.0 |########################################
  17640.5 |########################################
  17759.1 |
  17877.6 |########################################
  17996.1 |
  18114.7 |
  18233.2 |
  18351.7 |
  18470.3 |
  18588.8 |
  18707.3 |
  18825.9 |
  18944.4 |
  19062.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **rec_copy**: bridge=18.5% of algo (FFI overhead may distort results)
- **rec_mut**: bridge=134.7% of algo (FFI overhead may distort results)
- **rec_reuse_s00**: bridge=123.2% of algo (FFI overhead may distort results)

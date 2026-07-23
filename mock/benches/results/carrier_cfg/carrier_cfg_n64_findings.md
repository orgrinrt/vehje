# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (57.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 26.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 78% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (26.23 us) leads carrier_cfg_threaded (46.82 us) by 78%, a clear separation rather than a photo finish. CV 3.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 54% (significant)

carrier_cfg_trace is -31.14 us (54%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (57.21 us) is 2.2x the fastest (26.23 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (78% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 26233.6 ns median (-54.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.18x (fastest 26233.6 ns, slowest 57208.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 59376ns | 59231ns | 57607ns | 59059ns | 60734ns | -0.91% |
| carrier_cfg_switch | 59920ns | 59410ns | 58667ns | 59221ns | 61595ns | base |
| carrier_cfg_threaded | 49309ns | 49049ns | 48313ns | 48848ns | 50497ns | -17.71% |
| carrier_cfg_trace | 28934ns | 28424ns | 28055ns | 28414ns | 30152ns | -51.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 57144ns | 55418ns | 58435ns | -0.99% | 0.001 |
| carrier_cfg_switch | 57718ns | 56531ns | 59353ns | base | 0.001 |
| carrier_cfg_threaded | 47060ns | 46118ns | 48170ns | -18.47% | 0.001 |
| carrier_cfg_trace | 26680ns | 25914ns | 27754ns | -53.78% | 0.002 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cfg_fntable | 528627 | 1235293 | 0.428 | 0.98× |
| carrier_cfg_switch | 541772 | 1132392 | 0.478 | 1.00× |
| carrier_cfg_threaded | 433478 | 829719 | 0.522 | 0.80× |
| carrier_cfg_trace | 399932 | 1523875 | 0.262 | 0.74× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.002 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.4% |
| carrier_cfg_switch | 0.001 | 45.3% |
| carrier_cfg_threaded | 0.001 | 55.3% |
| carrier_cfg_trace | 0.002 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 59376ns | 59376ns | -0.91% |
| carrier_cfg_switch | 59920ns | 59920ns | base |
| carrier_cfg_threaded | 49309ns | 49309ns | -17.71% |
| carrier_cfg_trace | 28934ns | 28934ns | -51.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 57208ns | base | --- | [56594, 59353] | --- | --- | --- | --- |
| carrier_cfg_fntable | 57020ns | no significant difference | [-1966, +1193]ns | [55977, 58435] | no | 0.6875 | 0.6875 | 0 |
| carrier_cfg_threaded | 46824ns | -10707.3ns (-18.7%) | [-12658, -8609]ns | [46186, 48170] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cfg_trace | 26234ns | -31143.9ns (-54.4%) | [-32086, -29886]ns | [26051, 27754] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 57726ns | -2.1% | -20.1% | -54.6% |
| 2 | 60946ns | -4.5% | -22.5% | -53.5% |
| 3 | 56531ns | -2.0% | -17.5% | -51.9% |
| 4 | 57760ns | -1.4% | -19.9% | -54.6% |
| 5 | 56690ns | +0.7% | -17.0% | -54.3% |
| 6 | 56656ns | +3.6% | -13.3% | -53.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.335 | moderate- |
| carrier_cfg_switch | -0.200 | moderate- |
| carrier_cfg_threaded | 0.011 | ok |
| carrier_cfg_trace | 0.121 | ok |

**Consistency summary:**

- **carrier_cfg_fntable**: won 4/6, lost 2/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 114311.5ns | 57144.1ns | 200.0% | HIGH |
| carrier_cfg_switch | 115446.0ns | 57718.2ns | 200.0% | HIGH |
| carrier_cfg_threaded | 94237.6ns | 47060.1ns | 200.2% | HIGH |
| carrier_cfg_trace | 102635.8ns | 26679.7ns | 384.7% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 55417.5-58435.4 ns)
  55417.5 |####################
  55568.4 |
  55719.3 |
  55870.2 |
  56021.1 |
  56172.0 |
  56322.9 |
  56473.8 |####################
  56624.7 |
  56775.6 |
  56926.5 |########################################
  57077.4 |
  57228.3 |
  57379.2 |
  57530.1 |
  57681.0 |
  57831.9 |
  57982.8 |
  58133.7 |####################
  58284.6 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 56531.2-59352.9 ns)
  56531.2 |########################################
  56672.3 |####################
  56813.4 |
  56954.5 |
  57095.5 |
  57236.6 |
  57377.7 |
  57518.8 |
  57659.9 |########################################
  57801.0 |
  57942.0 |
  58083.1 |
  58224.2 |
  58365.3 |
  58506.4 |
  58647.5 |
  58788.6 |
  58929.6 |
  59070.7 |
  59211.8 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 46118.3-48170.2 ns)
  46118.3 |########################################
  46220.9 |########################################
  46323.5 |
  46426.1 |
  46528.7 |########################################
  46631.3 |
  46733.9 |
  46836.5 |
  46939.1 |########################################
  47041.7 |
  47144.2 |########################################
  47246.8 |
  47349.4 |
  47452.0 |
  47554.6 |
  47657.2 |
  47759.8 |
  47862.4 |
  47965.0 |
  48067.6 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 25914.2-27754.2 ns)
  25914.2 |####################
  26006.2 |
  26098.2 |####################
  26190.2 |########################################
  26282.2 |
  26374.2 |
  26466.2 |
  26558.2 |
  26650.2 |
  26742.2 |
  26834.2 |
  26926.2 |
  27018.2 |
  27110.2 |####################
  27202.2 |
  27294.2 |
  27386.2 |
  27478.2 |
  27570.2 |
  27662.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=200.0% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=397.5% of algo (FFI overhead may distort results)

# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (14.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 6.42 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 78% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (6.42 ms) leads carrier_cfg_threaded (11.41 ms) by 78%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 55% (significant)

carrier_cfg_trace is -8.00 ms (55%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (14.45 ms) is 2.2x the fastest (6.42 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (78% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 6423318.2 ns median (-55.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.25x (fastest 6423318.2 ns, slowest 14450106.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 14057800ns | 14053679ns | 14018658ns | 14045115ns | 14096398ns | -2.72% |
| carrier_cfg_switch | 14451372ns | 14454553ns | 14401059ns | 14447525ns | 14482298ns | base |
| carrier_cfg_threaded | 11431939ns | 11415345ns | 11393510ns | 11410259ns | 11483673ns | -20.89% |
| carrier_cfg_trace | 6440380ns | 6426842ns | 6394277ns | 6424321ns | 6487519ns | -55.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 14053241ns | 14014523ns | 14091466ns | -2.73% | 0.001 |
| carrier_cfg_switch | 14447065ns | 14397131ns | 14477888ns | base | 0.001 |
| carrier_cfg_threaded | 11427846ns | 11389579ns | 11478652ns | -20.90% | 0.001 |
| carrier_cfg_trace | 6436684ns | 6391058ns | 6483120ns | -55.45% | 0.003 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cfg_fntable | 87419635 | 206118771 | 0.424 | 0.97× |
| carrier_cfg_switch | 90020813 | 188824436 | 0.477 | 1.00× |
| carrier_cfg_threaded | 71185931 | 137422038 | 0.518 | 0.79× |
| carrier_cfg_trace | 39998127 | 157332737 | 0.254 | 0.44× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.5% |
| carrier_cfg_switch | 0.001 | 44.2% |
| carrier_cfg_threaded | 0.001 | 56.0% |
| carrier_cfg_trace | 0.003 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 14057800ns | 14057800ns | -2.72% |
| carrier_cfg_switch | 14451372ns | 14451372ns | base |
| carrier_cfg_threaded | 11431939ns | 11431939ns | -20.89% |
| carrier_cfg_trace | 6440380ns | 6440380ns | -55.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 14450107ns | base | --- | [14413202, 14477888] | --- | --- | --- | --- |
| carrier_cfg_fntable | 14049031ns | -388827.3ns (-2.7%) | [-410795, -381850]ns | [14019227, 14091466] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_threaded | 11411603ns | -3010203.5ns (-20.8%) | [-3064405, -2983050]ns | [11393283, 11478652] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_trace | 6423318ns | -7995913.2ns (-55.3%) | [-8060911, -7974321]ns | [6403613, 6483120] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 14440597ns | -2.9% | -21.1% | -55.5% |
| 2 | 14463788ns | -2.6% | -20.6% | -55.2% |
| 3 | 14459617ns | -2.7% | -20.7% | -55.1% |
| 4 | 14491988ns | -2.7% | -21.2% | -55.9% |
| 5 | 14429272ns | -2.8% | -20.9% | -55.5% |
| 6 | 14397131ns | -2.7% | -20.8% | -55.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | 0.070 | ok |
| carrier_cfg_switch | 0.142 | ok |
| carrier_cfg_threaded | 0.036 | ok |
| carrier_cfg_trace | -0.059 | ok |

**Consistency summary:**

- **carrier_cfg_fntable**: won 6/6, lost 0/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 14049723.3ns | 14053241.4ns | 100.0% | HIGH |
| carrier_cfg_switch | 14448427.5ns | 14447065.5ns | 100.0% | HIGH |
| carrier_cfg_threaded | 11422848.4ns | 11427846.1ns | 100.0% | HIGH |
| carrier_cfg_trace | 6437376.4ns | 6436683.6ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 14014522.9-14091466.4 ns)
  14014522.9 |####################
  14018370.1 |
  14022217.3 |########################################
  14026064.4 |
  14029911.6 |
  14033758.8 |
  14037606.0 |
  14041453.1 |
  14045300.3 |
  14049147.5 |
  14052994.7 |
  14056841.9 |
  14060689.0 |
  14064536.2 |
  14068383.4 |
  14072230.6 |####################
  14076077.7 |
  14079924.9 |####################
  14083772.1 |
  14087619.3 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 14397131.2-14477887.9 ns)
  14397131.2 |########################################
  14401169.0 |
  14405206.9 |
  14409244.7 |
  14413282.5 |
  14417320.4 |
  14421358.2 |
  14425396.0 |########################################
  14429433.9 |
  14433471.7 |
  14437509.6 |########################################
  14441547.4 |
  14445585.2 |
  14449623.1 |
  14453660.9 |
  14457698.7 |########################################
  14461736.6 |########################################
  14465774.4 |
  14469812.2 |
  14473850.1 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 11389579.2-11478652.5 ns)
  11389579.2 |########################################
  11394032.9 |########################################
  11398486.5 |
  11402940.2 |
  11407393.9 |########################################
  11411847.5 |########################################
  11416301.2 |
  11420754.9 |
  11425208.5 |
  11429662.2 |
  11434115.8 |
  11438569.5 |
  11443023.2 |
  11447476.8 |
  11451930.5 |
  11456384.2 |
  11460837.8 |
  11465291.5 |########################################
  11469745.2 |
  11474198.8 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 6391058.3-6483119.8 ns)
  6391058.3 |########################################
  6395661.4 |
  6400264.4 |
  6404867.5 |
  6409470.6 |
  6414073.7 |########################################
  6418676.7 |########################################
  6423279.8 |########################################
  6427882.9 |
  6432486.0 |
  6437089.0 |
  6441692.1 |
  6446295.2 |
  6450898.2 |
  6455501.3 |
  6460104.4 |
  6464707.5 |
  6469310.5 |
  6473913.6 |########################################
  6478516.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=100.1% of algo (FFI overhead may distort results)

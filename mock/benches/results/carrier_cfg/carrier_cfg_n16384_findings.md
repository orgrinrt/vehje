# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (14.16 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 6.34 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 76% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (6.34 ms) leads carrier_cfg_threaded (11.15 ms) by 76%, a clear separation rather than a photo finish. CV 0.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 55% (significant)

carrier_cfg_trace is -7.82 ms (55%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (14.16 ms) is 2.2x the fastest (6.34 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (76% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 76% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 6342717.5 ns median (-55.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.23x (fastest 6342717.5 ns, slowest 14158469.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 13816610ns | 13788014ns | 13702577ns | 13765961ns | 13949598ns | -2.87% |
| carrier_cfg_switch | 14225242ns | 14162443ns | 14089133ns | 14148822ns | 14407928ns | base |
| carrier_cfg_threaded | 11272031ns | 11158113ns | 11129969ns | 11153384ns | 11521032ns | -20.76% |
| carrier_cfg_trace | 6345289ns | 6346608ns | 6311696ns | 6342273ns | 6366610ns | -55.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 13812394ns | 13698350ns | 13945161ns | -2.87% | 0.001 |
| carrier_cfg_switch | 14220710ns | 14084246ns | 14403045ns | base | 0.001 |
| carrier_cfg_threaded | 11267847ns | 11126304ns | 11517196ns | -20.76% | 0.001 |
| carrier_cfg_trace | 6341365ns | 6307744ns | 6362956ns | -55.41% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.8% |
| carrier_cfg_switch | 0.001 | 44.6% |
| carrier_cfg_threaded | 0.001 | 56.6% |
| carrier_cfg_trace | 0.003 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 13816610ns | 13816610ns | -2.87% |
| carrier_cfg_switch | 14225242ns | 14225242ns | base |
| carrier_cfg_threaded | 11272031ns | 11272031ns | -20.76% |
| carrier_cfg_trace | 6345289ns | 6345289ns | -55.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 14158469ns | base | --- | [14100616, 14403045] | --- | --- | --- | --- |
| carrier_cfg_fntable | 13783940ns | -432888.8ns (-3.1%) | [-588645, -203414]ns | [13708080, 13945161] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_threaded | 11153164ns | -2991955.8ns (-21.1%) | [-3018379, -2848253]ns | [11133182, 11517196] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_trace | 6342718ns | -7820988.6ns (-55.2%) | [-8067670, -7749377]ns | [6318420, 6362956] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 14084246ns | -0.5% | -20.9% | -54.8% |
| 2 | 14608596ns | -5.0% | -18.9% | -56.4% |
| 3 | 14197493ns | -3.2% | -21.2% | -55.6% |
| 4 | 14151987ns | -2.3% | -21.2% | -55.2% |
| 5 | 14164952ns | -3.2% | -21.4% | -55.3% |
| 6 | 14116986ns | -3.0% | -21.2% | -55.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | 0.266 | moderate+ |
| carrier_cfg_switch | -0.270 | moderate- |
| carrier_cfg_threaded | -0.188 | ok |
| carrier_cfg_trace | -0.199 | ok |

**Consistency summary:**

- **carrier_cfg_fntable**: won 6/6, lost 0/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 13920564.9ns | 13812393.9ns | 100.8% | HIGH |
| carrier_cfg_switch | 14227520.1ns | 14220710.0ns | 100.0% | HIGH |
| carrier_cfg_threaded | 11268577.2ns | 11267847.4ns | 100.0% | HIGH |
| carrier_cfg_trace | 6341484.2ns | 6341364.6ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 13698350.4-13945161.4 ns)
  13698350.4 |########################################
  13710691.0 |########################################
  13723031.5 |
  13735372.1 |########################################
  13747712.6 |
  13760053.2 |
  13772393.7 |
  13784734.3 |
  13797074.8 |
  13809415.4 |########################################
  13821755.9 |
  13834096.5 |
  13846437.0 |
  13858777.6 |
  13871118.1 |########################################
  13883458.7 |
  13895799.2 |
  13908139.8 |
  13920480.3 |
  13932820.9 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 14084246.2-14403044.8 ns)
  14084246.2 |########################################
  14100186.1 |
  14116126.1 |########################################
  14132066.0 |
  14148005.9 |########################################
  14163945.8 |########################################
  14179885.8 |
  14195825.7 |########################################
  14211765.6 |
  14227705.5 |
  14243645.5 |
  14259585.4 |
  14275525.3 |
  14291465.3 |
  14307405.2 |
  14323345.1 |
  14339285.0 |
  14355225.0 |
  14371164.9 |
  14387104.8 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 11126304.2-11517195.8 ns)
  11126304.2 |########################################
  11145848.8 |########################################
  11165393.4 |
  11184937.9 |####################
  11204482.5 |
  11224027.1 |
  11243571.7 |
  11263116.3 |
  11282660.9 |
  11302205.4 |
  11321750.0 |
  11341294.6 |
  11360839.2 |
  11380383.8 |
  11399928.4 |
  11419472.9 |
  11439017.5 |
  11458562.1 |
  11478106.7 |
  11497651.3 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 6307744.2-6362956.2 ns)
  6307744.2 |########################################
  6310504.8 |
  6313265.4 |
  6316026.0 |
  6318786.6 |
  6321547.2 |
  6324307.8 |
  6327068.4 |########################################
  6329829.0 |
  6332589.6 |
  6335350.2 |
  6338110.8 |########################################
  6340871.4 |
  6343632.0 |########################################
  6346392.6 |
  6349153.2 |
  6351913.8 |
  6354674.4 |
  6357435.0 |
  6360195.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=100.0% of algo (FFI overhead may distort results)

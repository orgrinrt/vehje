# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (228.74 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 104.75 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 74% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (104.75 us) leads carrier_cfg_threaded (182.70 us) by 74%, a clear separation rather than a photo finish. CV 3.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 54% (significant)

carrier_cfg_trace is -122.94 us (54%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (228.74 us) is 2.2x the fastest (104.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (74% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 74% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 104747.3 ns median (-54.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.18x (fastest 104747.3 ns, slowest 228739.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 224243ns | 224108ns | 223212ns | 223884ns | 225297ns | -2.75% |
| carrier_cfg_switch | 230580ns | 231342ns | 224102ns | 231002ns | 233186ns | base |
| carrier_cfg_threaded | 184219ns | 185272ns | 176450ns | 182735ns | 190330ns | -20.11% |
| carrier_cfg_trace | 107629ns | 107078ns | 101268ns | 106581ns | 112381ns | -53.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 221487ns | 220303ns | 222575ns | -2.82% | 0.001 |
| carrier_cfg_switch | 227913ns | 221314ns | 230680ns | base | 0.001 |
| carrier_cfg_threaded | 181753ns | 174108ns | 187912ns | -20.25% | 0.001 |
| carrier_cfg_trace | 105197ns | 98973ns | 109781ns | -53.84% | 0.002 |

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 44.7% |
| carrier_cfg_switch | 0.001 | 43.3% |
| carrier_cfg_threaded | 0.001 | 54.2% |
| carrier_cfg_trace | 0.002 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 224243ns | 224243ns | -2.75% |
| carrier_cfg_switch | 230580ns | 230580ns | base |
| carrier_cfg_threaded | 184219ns | 184219ns | -20.11% |
| carrier_cfg_trace | 107629ns | 107629ns | -53.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 228739ns | base | --- | [224321, 230680] | --- | --- | --- | --- |
| carrier_cfg_fntable | 221480ns | -7100.2ns (-3.1%) | [-9623, -2556]ns | [220405, 222575] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| carrier_cfg_threaded | 182702ns | -46777.8ns (-20.5%) | [-51572, -40131]ns | [174645, 187912] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cfg_trace | 104747ns | -122943.4ns (-53.7%) | [-127110, -118096]ns | [101063, 109781] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 221314ns | +0.4% | -21.3% | -55.3% |
| 2 | 227328ns | -2.6% | -15.2% | -51.5% |
| 3 | 231980ns | -4.5% | -21.1% | -55.5% |
| 4 | 229052ns | -2.7% | -20.2% | -54.7% |
| 5 | 228425ns | -3.6% | -20.0% | -52.1% |
| 6 | 229379ns | -3.9% | -23.6% | -53.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.105 | ok |
| carrier_cfg_switch | 0.116 | ok |
| carrier_cfg_threaded | -0.328 | moderate- |
| carrier_cfg_trace | -0.474 | moderate- |

**Consistency summary:**

- **carrier_cfg_fntable**: won 5/6, lost 1/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 221539.9ns | 221486.9ns | 100.0% | HIGH |
| carrier_cfg_switch | 227839.8ns | 227913.2ns | 100.0% | HIGH |
| carrier_cfg_threaded | 181719.0ns | 181752.8ns | 100.0% | HIGH |
| carrier_cfg_trace | 105161.8ns | 105196.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 220302.9-222575.4 ns)
  220302.9 |########################################
  220416.5 |########################################
  220530.1 |
  220643.8 |
  220757.4 |
  220871.0 |
  220984.6 |
  221098.3 |
  221211.9 |
  221325.5 |########################################
  221439.2 |
  221552.8 |########################################
  221666.4 |
  221780.0 |
  221893.7 |
  222007.3 |
  222120.9 |########################################
  222234.5 |
  222348.2 |
  222461.8 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 221314.2-230679.8 ns)
  221314.2 |########################################
  221782.5 |
  222250.8 |
  222719.0 |
  223187.3 |
  223655.6 |
  224123.9 |
  224592.2 |
  225060.4 |
  225528.7 |
  225997.0 |
  226465.3 |
  226933.6 |########################################
  227401.8 |
  227870.1 |
  228338.4 |########################################
  228806.7 |########################################
  229275.0 |########################################
  229743.2 |
  230211.5 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 174107.9-187912.3 ns)
  174107.9 |#############
  174798.1 |#############
  175488.3 |
  176178.6 |
  176868.8 |
  177559.0 |
  178249.2 |
  178939.4 |
  179629.7 |
  180319.9 |
  181010.1 |
  181700.3 |
  182390.5 |########################################
  183080.8 |
  183771.0 |
  184461.2 |
  185151.4 |
  185841.6 |
  186531.9 |
  187222.1 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 98972.9-109780.8 ns)
  98972.9 |########################################
  99513.3 |
  100053.7 |
  100594.1 |
  101134.5 |
  101674.9 |
  102215.3 |
  102755.7 |########################################
  103296.1 |########################################
  103836.5 |
  104376.8 |
  104917.2 |
  105457.6 |########################################
  105998.0 |
  106538.4 |
  107078.8 |
  107619.2 |
  108159.6 |
  108700.0 |
  109240.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=100.0% of algo (FFI overhead may distort results)

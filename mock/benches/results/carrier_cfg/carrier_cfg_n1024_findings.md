# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (881.18 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 400.30 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 74% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (400.30 us) leads carrier_cfg_threaded (697.15 us) by 74%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 55% (significant)

carrier_cfg_trace is -481.57 us (55%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (881.18 us) is 2.2x the fastest (400.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (74% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 74% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 400295.7 ns median (-54.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.20x (fastest 400295.7 ns, slowest 881184.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 872757ns | 865420ns | 855155ns | 864386ns | 894116ns | -1.29% |
| carrier_cfg_switch | 884180ns | 883985ns | 880710ns | 883069ns | 887581ns | base |
| carrier_cfg_threaded | 702520ns | 700462ns | 696901ns | 699495ns | 709866ns | -20.55% |
| carrier_cfg_trace | 403162ns | 403441ns | 400646ns | 402606ns | 405255ns | -54.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 869321ns | 851420ns | 890217ns | -1.32% | 0.001 |
| carrier_cfg_switch | 880946ns | 876818ns | 884527ns | base | 0.001 |
| carrier_cfg_threaded | 699243ns | 693444ns | 706975ns | -20.63% | 0.001 |
| carrier_cfg_trace | 400154ns | 397390ns | 402576ns | -54.58% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 46.1% |
| carrier_cfg_switch | 0.001 | 45.1% |
| carrier_cfg_threaded | 0.001 | 57.0% |
| carrier_cfg_trace | 0.003 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 872757ns | 872757ns | -1.29% |
| carrier_cfg_switch | 884180ns | 884180ns | base |
| carrier_cfg_threaded | 702520ns | 702520ns | -20.55% |
| carrier_cfg_trace | 403162ns | 403162ns | -54.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 881185ns | base | --- | [877125, 884527] | --- | --- | --- | --- |
| carrier_cfg_fntable | 862517ns | no significant difference | [-23035, +7424]ns | [855230, 890217] | no | 0.2188 | 0.2188 | 0 |
| carrier_cfg_threaded | 697149ns | -181176.7ns (-20.6%) | [-187989, -175943]ns | [693605, 706975] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cfg_trace | 400296ns | -481574.2ns (-54.7%) | [-485818, -474983]ns | [397590, 402576] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 882575ns | -2.3% | -20.3% | -54.5% |
| 2 | 883261ns | -2.3% | -19.6% | -54.8% |
| 3 | 877432ns | -2.1% | -20.4% | -54.0% |
| 4 | 885792ns | -2.0% | -21.7% | -55.1% |
| 5 | 879795ns | +3.7% | -20.9% | -54.8% |
| 6 | 876818ns | -2.9% | -20.9% | -54.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.297 | moderate- |
| carrier_cfg_switch | -0.357 | moderate- |
| carrier_cfg_threaded | 0.359 | moderate+ |
| carrier_cfg_trace | -0.212 | moderate- |

**Consistency summary:**

- **carrier_cfg_fntable**: won 5/6, lost 1/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 869590.2ns | 869321.4ns | 100.0% | HIGH |
| carrier_cfg_switch | 881176.5ns | 880945.5ns | 100.0% | HIGH |
| carrier_cfg_threaded | 699022.3ns | 699242.7ns | 100.0% | HIGH |
| carrier_cfg_trace | 400066.3ns | 400153.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 851419.6-890217.3 ns)
  851419.6 |####################
  853359.5 |
  855299.4 |
  857239.3 |####################
  859179.1 |
  861119.0 |########################################
  863058.9 |
  864998.8 |
  866938.7 |####################
  868878.6 |
  870818.4 |
  872758.3 |
  874698.2 |
  876638.1 |
  878578.0 |
  880517.9 |
  882457.8 |
  884397.6 |
  886337.5 |
  888277.4 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 876817.9-884526.8 ns)
  876817.9 |########################################
  877203.3 |########################################
  877588.8 |
  877974.2 |
  878359.7 |
  878745.1 |
  879130.6 |
  879516.0 |########################################
  879901.5 |
  880286.9 |
  880672.4 |
  881057.8 |
  881443.3 |
  881828.7 |
  882214.2 |########################################
  882599.6 |
  882985.1 |########################################
  883370.5 |
  883756.0 |
  884141.4 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 693443.8-706975.0 ns)
  693443.8 |########################################
  694120.4 |
  694796.9 |
  695473.5 |
  696150.0 |####################
  696826.6 |
  697503.2 |####################
  698179.7 |
  698856.3 |
  699532.8 |
  700209.4 |
  700886.0 |
  701562.5 |
  702239.1 |
  702915.6 |
  703592.2 |####################
  704268.8 |
  704945.3 |
  705621.9 |
  706298.4 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 397389.6-402576.5 ns)
  397389.6 |########################################
  397648.9 |########################################
  397908.3 |
  398167.6 |
  398427.0 |
  398686.3 |
  398945.7 |
  399205.0 |
  399464.3 |########################################
  399723.7 |
  399983.0 |
  400242.4 |
  400501.7 |
  400761.1 |########################################
  401020.4 |
  401279.7 |
  401539.1 |
  401798.4 |########################################
  402057.8 |
  402317.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=100.1% of algo (FFI overhead may distort results)

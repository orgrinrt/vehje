# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (26.24 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 7.77 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 238% faster than the next best (r_chain128_whole)

r_chain128_semi (7.77 ms) leads r_chain128_whole (26.24 ms) by 238%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 70% (significant)

r_chain128_semi is -18.47 ms (70%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Wide spread: slowest is 3.4x the fastest

Fastest r_chain128_semi (7.77 ms) to slowest r_chain128_whole (26.24 ms): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain128_semi** at 7766006.8 ns median (-70.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.38x (fastest 7766006.8 ns, slowest 26237105.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 7781453ns | 7769200ns | 7693825ns | 7765245ns | 7849579ns | -70.68% |
| r_chain128_whole | 26540856ns | 26241237ns | 26214022ns | 26236866ns | 27160260ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 7778284ns | 7690731ns | 7846384ns | -70.69% | 0.001 |
| r_chain128_whole | 26536419ns | 26210021ns | 27155244ns | base | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.001 | 99.0% |
| r_chain128_whole | 0.000 | 29.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 7781453ns | 7781453ns | -70.68% |
| r_chain128_whole | 26540856ns | 26540856ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 26237105ns | base | --- | [26216908, 27155244] | --- | --- | --- | --- |
| r_chain128_semi | 7766007ns | -18470048.1ns (-70.4%) | [-19364026, -18440331]ns | [7722462, 7846384] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 28061971ns | -71.9% |
| 2 | 26240229ns | -70.4% |
| 3 | 26248518ns | -70.7% |
| 4 | 26233982ns | -70.3% |
| 5 | 26210021ns | -70.4% |
| 6 | 26223795ns | -70.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | -0.139 | ok |
| r_chain128_whole | -0.028 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 2383.2ns | 7778284.2ns | 0.0% |  |
| r_chain128_whole | 2193.5ns | 26536419.2ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 7690731.2-7846383.8 ns)
  7690731.2 |####################
  7698513.8 |
  7706296.5 |
  7714079.1 |
  7721861.7 |
  7729644.3 |
  7737427.0 |
  7745209.6 |
  7752992.2 |########################################
  7760774.8 |
  7768557.5 |####################
  7776340.1 |
  7784122.7 |
  7791905.4 |
  7799688.0 |####################
  7807470.6 |
  7815253.2 |
  7823035.9 |
  7830818.5 |
  7838601.1 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 26210021.2-27155244.1 ns)
  26210021.2 |########################################
  26257282.3 |
  26304543.5 |
  26351804.6 |
  26399065.8 |
  26446326.9 |
  26493588.1 |
  26540849.2 |
  26588110.4 |
  26635371.5 |
  26682632.7 |
  26729893.8 |
  26777155.0 |
  26824416.1 |
  26871677.3 |
  26918938.4 |
  26966199.6 |
  27013460.7 |
  27060721.9 |
  27107983.0 |
  (0 below, 1 above range)

```

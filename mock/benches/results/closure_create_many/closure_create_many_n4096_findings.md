# Closure representation: create-many-call-once, flat vs linked (creation cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_create_many_flat**

## Highlights

Baseline for all deltas below: **closure_create_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (closure_create_many_flat) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline closure_create_many_flat has the worst median (600.39 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest closure_create_many_linked at 290.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### closure_create_many_linked dominates: 107% faster than the next best (closure_create_many_flat)

closure_create_many_linked (290.68 us) leads closure_create_many_flat (600.39 us) by 107%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### closure_create_many_linked beats baseline by 52% (significant)

closure_create_many_linked is -310.70 us (52%) faster than baseline closure_create_many_flat, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: closure_create_many_linked** at 290678.5 ns median (-51.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.07x (fastest 290678.5 ns, slowest 600392.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_create_many_flat | 602846ns | 603588ns | 596037ns | 602573ns | 606659ns | base |
| closure_create_many_linked | 292033ns | 292975ns | 285587ns | 291189ns | 296523ns | -51.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_create_many_flat | 599945ns | 593190ns | 603692ns | base | 0.007 |
| closure_create_many_linked | 289547ns | 283211ns | 293826ns | -51.74% | 0.014 |

## Performance model

- Peak throughput: **0.014 Gops/s** (closure_create_many_linked; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_create_many_flat | 0.007 | 47.2% |
| closure_create_many_linked | 0.014 | 97.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_create_many_flat | 602846ns | 602846ns | base |
| closure_create_many_linked | 292033ns | 292033ns | -51.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_create_many_flat | 600392ns | base | --- | [595749, 603692] | --- | --- | --- | --- |
| closure_create_many_linked | 290679ns | -310698.3ns (-51.7%) | [-316770, -303724]ns | [284137, 293826] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_create_many_flat | closure_create_many_linked |
|---|---|---|
| 1 | 601714ns | -52.9% |
| 2 | 605670ns | -51.5% |
| 3 | 600685ns | -51.6% |
| 4 | 593190ns | -51.0% |
| 5 | 598309ns | -50.9% |
| 6 | 600100ns | -52.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_create_many_flat | 0.238 | moderate+ |
| closure_create_many_linked | -0.358 | moderate- |

**Consistency summary:**

- **closure_create_many_linked**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_create_many_flat | 8.4ns | 599944.6ns | 0.0% |  |
| closure_create_many_linked | 5.3ns | 289547.3ns | 0.0% |  |

## Distribution (algo ns)

```
closure_create_many_flat (n=6, range 593190.0-603692.1 ns)
  593190.0 |########################################
  593715.1 |
  594240.2 |
  594765.3 |
  595290.4 |
  595815.5 |
  596340.6 |
  596865.7 |
  597390.8 |
  597915.9 |########################################
  598441.1 |
  598966.2 |
  599491.3 |
  600016.4 |########################################
  600541.5 |########################################
  601066.6 |
  601591.7 |########################################
  602116.8 |
  602641.9 |
  603167.0 |
  (0 below, 1 above range)

closure_create_many_linked (n=6, range 283211.2-293826.5 ns)
  283211.2 |########################################
  283742.0 |
  284272.7 |
  284803.5 |########################################
  285334.3 |
  285865.0 |
  286395.8 |
  286926.6 |
  287457.3 |
  287988.1 |
  288518.8 |
  289049.6 |
  289580.4 |
  290111.1 |########################################
  290641.9 |########################################
  291172.7 |
  291703.4 |
  292234.2 |
  292765.0 |
  293295.7 |########################################
  (0 below, 1 above range)

```

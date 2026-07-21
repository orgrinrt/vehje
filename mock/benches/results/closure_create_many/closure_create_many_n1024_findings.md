# Closure representation: create-many-call-once, flat vs linked (creation cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_create_many_flat**

## Highlights

Baseline for all deltas below: **closure_create_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (closure_create_many_flat) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline closure_create_many_flat has the worst median (161.04 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest closure_create_many_linked at 80.23 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### closure_create_many_linked dominates: 101% faster than the next best (closure_create_many_flat)

closure_create_many_linked (80.23 us) leads closure_create_many_flat (161.04 us) by 101%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### closure_create_many_linked beats baseline by 52% (significant)

closure_create_many_linked is -83.12 us (52%) faster than baseline closure_create_many_flat, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### closure_create_many_flat shows alternating (throttle bounce) (autocorr -0.51)

closure_create_many_flat's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: closure_create_many_linked** at 80231.7 ns median (-50.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.01x (fastest 80231.7 ns, slowest 161042.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_create_many_flat | 162663ns | 163669ns | 156985ns | 161769ns | 166844ns | base |
| closure_create_many_linked | 81225ns | 82792ns | 73182ns | 82753ns | 82952ns | -50.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_create_many_flat | 160056ns | 154410ns | 164234ns | base | 0.006 |
| closure_create_many_linked | 78712ns | 70922ns | 80362ns | -50.82% | 0.013 |

## Performance model

- Peak throughput: **0.014 Gops/s** (closure_create_many_linked; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_create_many_flat | 0.006 | 44.0% |
| closure_create_many_linked | 0.013 | 88.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_create_many_flat | 162663ns | 162663ns | base |
| closure_create_many_linked | 81225ns | 81225ns | -50.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_create_many_flat | 161043ns | base | --- | [154891, 164234] | --- | --- | --- | --- |
| closure_create_many_linked | 80232ns | -83124.6ns (-51.6%) | [-84258, -76649]ns | [75543, 80362] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_create_many_flat | closure_create_many_linked |
|---|---|---|
| 1 | 155373ns | -54.4% |
| 2 | 164336ns | -51.2% |
| 3 | 162652ns | -50.7% |
| 4 | 154410ns | -48.1% |
| 5 | 164132ns | -51.0% |
| 6 | 159434ns | -49.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_create_many_flat | -0.513 | HIGH- (thermal bounce) |
| closure_create_many_linked | -0.036 | ok |

**Consistency summary:**

- **closure_create_many_linked**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_create_many_flat | 6.3ns | 160055.9ns | 0.0% |  |
| closure_create_many_linked | 4.2ns | 78712.2ns | 0.0% |  |

## Distribution (algo ns)

```
closure_create_many_flat (n=6, range 154409.6-164233.8 ns)
  154409.6 |########################################
  154900.8 |########################################
  155392.0 |
  155883.2 |
  156374.4 |
  156865.6 |
  157356.8 |
  157848.1 |
  158339.3 |
  158830.5 |
  159321.7 |########################################
  159812.9 |
  160304.1 |
  160795.3 |
  161286.5 |
  161777.7 |
  162268.9 |########################################
  162760.1 |
  163251.3 |
  163742.5 |########################################
  (0 below, 1 above range)

closure_create_many_linked (n=6, range 70921.7-80362.1 ns)
  70921.7 |##########
  71393.7 |
  71865.7 |
  72337.8 |
  72809.8 |
  73281.8 |
  73753.8 |
  74225.8 |
  74697.9 |
  75169.9 |
  75641.9 |
  76113.9 |
  76585.9 |
  77058.0 |
  77530.0 |
  78002.0 |
  78474.0 |
  78946.0 |
  79418.1 |
  79890.1 |########################################
  (0 below, 1 above range)

```

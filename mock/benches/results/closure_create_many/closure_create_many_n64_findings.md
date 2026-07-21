# Closure representation: create-many-call-once, flat vs linked (creation cost dominates)

2 variants, 6 samples per variant.
Baseline: **closure_create_many_flat**

## Highlights

Baseline for all deltas below: **closure_create_many_flat**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (closure_create_many_flat) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline closure_create_many_flat has the worst median (11.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest closure_create_many_linked at 5.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### closure_create_many_linked dominates: 108% faster than the next best (closure_create_many_flat)

closure_create_many_linked (5.36 us) leads closure_create_many_flat (11.17 us) by 108%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### closure_create_many_linked beats baseline by 52% (significant)

closure_create_many_linked is -5.86 us (52%) faster than baseline closure_create_many_flat, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### closure_create_many_linked is fastest but the noisiest (CV 6.7%)

closure_create_many_linked wins on median (5.36 us) yet has the highest variance (CV 6.7%), while closure_create_many_flat is the steadiest (CV 1.9%, 11.17 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: closure_create_many_linked** at 5357.9 ns median (-52.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.08x (fastest 5357.9 ns, slowest 11166.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| closure_create_many_flat | 13582ns | 13743ns | 13089ns | 13600ns | 13802ns | base |
| closure_create_many_linked | 7706ns | 7936ns | 6518ns | 7934ns | 7957ns | -43.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| closure_create_many_flat | 11051ns | 10691ns | 11228ns | base | 0.006 |
| closure_create_many_linked | 5200ns | 4402ns | 5366ns | -52.94% | 0.012 |

## Performance model

- Peak throughput: **0.015 Gops/s** (closure_create_many_linked; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| closure_create_many_flat | 0.006 | 39.4% |
| closure_create_many_linked | 0.012 | 82.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| closure_create_many_flat | 13582ns | 13582ns | base |
| closure_create_many_linked | 7706ns | 7706ns | -43.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| closure_create_many_flat | 11167ns | base | --- | [10758, 11228] | --- | --- | --- | --- |
| closure_create_many_linked | 5358ns | -5855.0ns (-52.4%) | [-6080, -5617]ns | [4877, 5366] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | closure_create_many_flat | closure_create_many_linked |
|---|---|---|
| 1 | 10691ns | -58.8% |
| 2 | 11208ns | -52.1% |
| 3 | 10825ns | -50.6% |
| 4 | 11126ns | -51.8% |
| 5 | 11218ns | -52.3% |
| 6 | 11239ns | -52.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| closure_create_many_flat | -0.237 | moderate- |
| closure_create_many_linked | -0.039 | ok |

**Consistency summary:**

- **closure_create_many_linked**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| closure_create_many_flat | 4.7ns | 11050.8ns | 0.0% |  |
| closure_create_many_linked | 4.1ns | 5200.3ns | 0.1% |  |

## Distribution (algo ns)

```
closure_create_many_flat (n=6, range 10690.8-11228.1 ns)
  10690.8 |####################
  10717.7 |
  10744.5 |
  10771.4 |
  10798.3 |####################
  10825.1 |
  10852.0 |
  10878.9 |
  10905.7 |
  10932.6 |
  10959.5 |
  10986.3 |
  11013.2 |
  11040.1 |
  11066.9 |
  11093.8 |
  11120.7 |####################
  11147.5 |
  11174.4 |
  11201.3 |########################################
  (0 below, 1 above range)

closure_create_many_linked (n=6, range 4402.1-5366.0 ns)
   4402.1 |##########
   4450.3 |
   4498.5 |
   4546.7 |
   4594.9 |
   4643.1 |
   4691.3 |
   4739.5 |
   4787.7 |
   4835.9 |
   4884.1 |
   4932.3 |
   4980.5 |
   5028.7 |
   5076.9 |
   5125.1 |
   5173.3 |
   5221.5 |
   5269.7 |
   5317.9 |########################################
  (0 below, 1 above range)

```

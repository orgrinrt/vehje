# Partial-eval specialization: fold ratio on a random static/dynamic mix (reduction metric)

4 variants, 6 samples per variant.
Baseline: **pe_rand_sf30**

## Highlights

Baseline for all deltas below: **pe_rand_sf30**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_rand_sf30) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_rand_sf30 has the worst median (74.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_rand_sf90 at 31.19 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### pe_rand_sf90 dominates: 82% faster than the next best (pe_rand_sf70)

pe_rand_sf90 (31.19 us) leads pe_rand_sf70 (56.83 us) by 82%, a clear separation rather than a photo finish. CV 7.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### pe_rand_sf90 beats baseline by 57% (significant)

pe_rand_sf90 is -42.34 us (57%) faster than baseline pe_rand_sf30, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### pe_rand_sf30 is an outlier: 2.4x slower than the field

pe_rand_sf30 (74.59 us) is 2.4x the fastest (31.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### pe_rand_sf90 is fastest but the noisiest (CV 7.0%)

pe_rand_sf90 wins on median (31.19 us) yet has the highest variance (CV 7.0%), while pe_rand_sf70 is the steadiest (CV 3.3%, 56.83 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {pe_rand_sf90} vs {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} (82% apart)

The field splits into a fast tier {pe_rand_sf90} and a slow tier {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} with a 82% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: pe_rand_sf90** at 31191.7 ns median (-58.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.39x (fastest 31191.7 ns, slowest 74593.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_rand_sf30 | 77621ns | 76900ns | 74092ns | 76198ns | 81521ns | base |
| pe_rand_sf50 | 71830ns | 71653ns | 67080ns | 70571ns | 76094ns | -7.46% |
| pe_rand_sf70 | 59267ns | 59108ns | 57145ns | 58492ns | 61490ns | -23.65% |
| pe_rand_sf90 | 34142ns | 33450ns | 32134ns | 33231ns | 36512ns | -56.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_rand_sf30 | 75336ns | 71916ns | 79158ns | base | 0.014 |
| pe_rand_sf50 | 69498ns | 64902ns | 73654ns | -7.75% | 0.015 |
| pe_rand_sf70 | 57007ns | 54968ns | 59161ns | -24.33% | 0.018 |
| pe_rand_sf90 | 31852ns | 29969ns | 34064ns | -57.72% | 0.032 |

## Performance model

- Peak throughput: **0.034 Gops/s** (pe_rand_sf90; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_rand_sf30 | 0.014 | 40.2% |
| pe_rand_sf50 | 0.015 | 43.3% |
| pe_rand_sf70 | 0.018 | 52.7% |
| pe_rand_sf90 | 0.033 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_rand_sf30 | 77621ns | 77621ns | base |
| pe_rand_sf50 | 71830ns | 71830ns | -7.46% |
| pe_rand_sf70 | 59267ns | 59267ns | -23.65% |
| pe_rand_sf90 | 34142ns | 34142ns | -56.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_rand_sf30 | 74593ns | base | --- | [72256, 79158] | --- | --- | --- | --- |
| pe_rand_sf50 | 69271ns | -4415.4ns (-5.9%) | [-10971, -2127]ns | [65569, 73654] | YES | 0.0313 | 0.0313 | 0 |
| pe_rand_sf70 | 56827ns | -17788.1ns (-23.8%) | [-23238, -13959]ns | [55034, 59161] | YES | 0.0313 | 0.0313 | 0 |
| pe_rand_sf90 | 31192ns | -42340.1ns (-56.8%) | [-47966, -40146]ns | [30299, 34064] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_rand_sf30 | pe_rand_sf50 | pe_rand_sf70 | pe_rand_sf90 |
|---|---|---|---|---|
| 1 | 73362ns | -11.5% | -17.6% | -58.2% |
| 2 | 75824ns | -2.9% | -23.7% | -51.8% |
| 3 | 76145ns | -3.2% | -25.5% | -58.6% |
| 4 | 82171ns | -16.4% | -32.9% | -62.5% |
| 5 | 72595ns | -8.8% | -24.3% | -56.5% |
| 6 | 71916ns | -2.9% | -20.9% | -58.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_rand_sf30 | -0.062 | ok |
| pe_rand_sf50 | -0.054 | ok |
| pe_rand_sf70 | 0.359 | moderate+ |
| pe_rand_sf90 | -0.214 | moderate- |

**Consistency summary:**

- **pe_rand_sf50**: won 6/6, lost 0/6
- **pe_rand_sf70**: won 6/6, lost 0/6
- **pe_rand_sf90**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_rand_sf30 | 4.5ns | 75335.7ns | 0.0% |  |
| pe_rand_sf50 | 3.4ns | 69497.9ns | 0.0% |  |
| pe_rand_sf70 | 4.2ns | 57007.3ns | 0.0% |  |
| pe_rand_sf90 | 3.5ns | 31851.6ns | 0.0% |  |

## Distribution (algo ns)

```
pe_rand_sf30 (n=6, range 71916.2-79158.1 ns)
  71916.2 |########################################
  72278.3 |########################################
  72640.4 |
  73002.5 |########################################
  73364.6 |
  73726.7 |
  74088.8 |
  74450.9 |
  74813.0 |
  75175.1 |
  75537.1 |########################################
  75899.2 |########################################
  76261.3 |
  76623.4 |
  76985.5 |
  77347.6 |
  77709.7 |
  78071.8 |
  78433.9 |
  78796.0 |
  (0 below, 1 above range)

pe_rand_sf50 (n=6, range 64902.5-73653.6 ns)
  64902.5 |########################################
  65340.1 |
  65777.6 |
  66215.2 |########################################
  66652.7 |
  67090.3 |
  67527.8 |
  67965.4 |
  68402.9 |########################################
  68840.5 |
  69278.0 |
  69715.6 |########################################
  70153.1 |
  70590.7 |
  71028.2 |
  71465.8 |
  71903.3 |
  72340.9 |
  72778.4 |
  73216.0 |########################################
  (0 below, 1 above range)

pe_rand_sf70 (n=6, range 54968.3-59161.4 ns)
  54968.3 |########################################
  55178.0 |
  55387.6 |
  55597.3 |
  55806.9 |
  56016.6 |
  56226.2 |
  56435.9 |
  56645.6 |####################
  56855.2 |####################
  57064.9 |
  57274.5 |
  57484.2 |
  57693.8 |####################
  57903.5 |
  58113.2 |
  58322.8 |
  58532.5 |
  58742.1 |
  58951.8 |
  (0 below, 1 above range)

pe_rand_sf90 (n=6, range 29968.8-34063.9 ns)
  29968.8 |####################
  30173.6 |
  30378.3 |
  30583.1 |####################
  30787.8 |####################
  30992.6 |
  31197.3 |
  31402.1 |########################################
  31606.9 |
  31811.6 |
  32016.4 |
  32221.1 |
  32425.9 |
  32630.6 |
  32835.4 |
  33040.2 |
  33244.9 |
  33449.7 |
  33654.4 |
  33859.2 |
  (0 below, 1 above range)

```

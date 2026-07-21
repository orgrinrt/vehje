# Effect inference: thermometer OR-join vs naive branch-max lattice join

2 variants, 6 samples per variant.
Baseline: **ei_thermo**

## Highlights

Baseline for all deltas below: **ei_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ei_thermo dominates: 401% faster than the next best (ei_branchmax)

ei_thermo (5.03 us) leads ei_branchmax (25.20 us) by 401%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (ei_thermo)

The baseline ei_thermo is the fastest (5.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.0x the fastest

Fastest ei_thermo (5.03 us) to slowest ei_branchmax (25.20 us): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ei_thermo) is the fastest** at 5025.6 ns median
- 1 variant significantly slower than baseline
- Spread: 5.01x (fastest 5025.6 ns, slowest 25200.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ei_branchmax | 27009ns | 27659ns | 24021ns | 26702ns | 28964ns | +270.14% |
| ei_thermo | 7297ns | 7624ns | 6620ns | 7291ns | 7644ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ei_branchmax | 24615ns | 21898ns | 26407ns | +412.00% | 0.010 |
| ei_thermo | 4808ns | 4358ns | 5036ns | base | 0.053 |

## Performance model

- Peak throughput: **0.059 Gops/s** (ei_thermo; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ei_branchmax | 0.010 | 17.3% |
| ei_thermo | 0.051 | 86.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ei_branchmax | 27009ns | 27009ns | +270.14% |
| ei_thermo | 7297ns | 7297ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ei_thermo | 5026ns | base | --- | [4361, 5036] | --- | --- | --- | --- |
| ei_branchmax | 25200ns | +20169.3ns (+401.3%) | [+17875, +21376]ns | [22237, 26407] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ei_thermo | ei_branchmax |
|---|---|---|
| 1 | 4365ns | +401.7% |
| 2 | 5028ns | +442.0% |
| 3 | 5033ns | +407.9% |
| 4 | 5023ns | +407.0% |
| 5 | 5039ns | +394.8% |
| 6 | 4358ns | +418.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ei_branchmax | -0.209 | moderate- |
| ei_thermo | -0.089 | ok |

**Consistency summary:**

- **ei_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ei_branchmax | 85.0ns | 24614.6ns | 0.3% |  |
| ei_thermo | 89.2ns | 4807.6ns | 1.9% |  |

## Distribution (algo ns)

```
ei_branchmax (n=6, range 21897.5-26406.9 ns)
  21897.5 |########################################
  22123.0 |
  22348.4 |
  22573.9 |########################################
  22799.4 |
  23024.8 |
  23250.3 |
  23475.8 |
  23701.3 |
  23926.7 |
  24152.2 |
  24377.7 |
  24603.1 |
  24828.6 |########################################
  25054.1 |
  25279.6 |########################################
  25505.0 |########################################
  25730.5 |
  25956.0 |
  26181.4 |
  (0 below, 1 above range)

ei_thermo (n=6, range 4357.5-5035.9 ns)
   4357.5 |##########################
   4391.4 |
   4425.3 |
   4459.3 |
   4493.2 |
   4527.1 |
   4561.0 |
   4594.9 |
   4628.8 |
   4662.8 |
   4696.7 |
   4730.6 |
   4764.5 |
   4798.4 |
   4832.3 |
   4866.3 |
   4900.2 |
   4934.1 |
   4968.0 |
   5001.9 |########################################
  (0 below, 1 above range)

```

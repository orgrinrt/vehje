# Effect inference: thermometer OR-join vs naive branch-max lattice join

2 variants, 6 samples per variant.
Baseline: **ei_thermo**

## Highlights

Baseline for all deltas below: **ei_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ei_thermo dominates: 467% faster than the next best (ei_branchmax)

ei_thermo (1.12 us) leads ei_branchmax (6.32 us) by 467%, a clear separation rather than a photo finish. CV 6.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ei_branchmax shows warm-up / thermal drift (autocorr +0.53)

ei_branchmax's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ei_thermo)

The baseline ei_thermo is the fastest (1.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.7x the fastest

Fastest ei_thermo (1.12 us) to slowest ei_branchmax (6.32 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ei_thermo) is the fastest** at 1115.2 ns median
- 1 variant significantly slower than baseline
- Spread: 5.67x (fastest 1115.2 ns, slowest 6323.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ei_branchmax | 8685ns | 8733ns | 7839ns | 8529ns | 9343ns | +145.05% |
| ei_thermo | 3544ns | 3712ns | 3123ns | 3560ns | 3731ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ei_branchmax | 6288ns | 5684ns | 6762ns | +489.67% | 0.010 |
| ei_thermo | 1066ns | 944ns | 1122ns | base | 0.060 |

## Performance model

- Peak throughput: **0.068 Gops/s** (ei_thermo; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ei_branchmax | 0.010 | 14.9% |
| ei_thermo | 0.057 | 84.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ei_branchmax | 8685ns | 8685ns | +145.05% |
| ei_thermo | 3544ns | 3544ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ei_thermo | 1115ns | base | --- | [962, 1122] | --- | --- | --- | --- |
| ei_branchmax | 6323ns | +5273.7ns (+472.9%) | [+4745, +5646]ns | [5779, 6762] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ei_thermo | ei_branchmax |
|---|---|---|
| 1 | 980ns | +502.1% |
| 2 | 944ns | +502.0% |
| 3 | 1124ns | +422.6% |
| 4 | 1120ns | +505.2% |
| 5 | 1119ns | +502.7% |
| 6 | 1111ns | +507.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ei_branchmax | 0.532 | HIGH+ (drift/warm-up) |
| ei_thermo | 0.353 | moderate+ |

**Consistency summary:**

- **ei_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ei_branchmax | 52.0ns | 6287.9ns | 0.8% |  |
| ei_thermo | 57.9ns | 1066.3ns | 5.4% | HIGH |

## Distribution (algo ns)

```
ei_branchmax (n=6, range 5684.2-6761.9 ns)
   5684.2 |####################
   5738.1 |
   5792.0 |
   5845.8 |####################
   5899.7 |####################
   5953.6 |
   6007.5 |
   6061.4 |
   6115.3 |
   6169.1 |
   6223.0 |
   6276.9 |
   6330.8 |
   6384.7 |
   6438.6 |
   6492.4 |
   6546.3 |
   6600.2 |
   6654.1 |
   6708.0 |########################################
  (0 below, 1 above range)

ei_thermo (n=6, range 944.2-1121.7 ns)
    944.2 |####################
    953.1 |
    962.0 |
    970.8 |
    979.7 |####################
    988.6 |
    997.4 |
   1006.3 |
   1015.2 |
   1024.1 |
   1032.9 |
   1041.8 |
   1050.7 |
   1059.6 |
   1068.4 |
   1077.3 |
   1086.2 |
   1095.1 |
   1103.9 |####################
   1112.8 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **ei_branchmax**: autocorrelation=0.53 (measurement drift or warm-up artifact)

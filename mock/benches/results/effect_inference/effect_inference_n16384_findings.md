# Effect inference: thermometer OR-join vs naive branch-max lattice join

2 variants, 6 samples per variant.
Baseline: **ei_thermo**

## Highlights

Baseline for all deltas below: **ei_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ei_thermo dominates: 408% faster than the next best (ei_branchmax)

ei_thermo (278.13 us) leads ei_branchmax (1.41 ms) by 408%, a clear separation rather than a photo finish. CV 1.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### ei_branchmax shows alternating (throttle bounce) (autocorr -0.55)

ei_branchmax's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (ei_thermo)

The baseline ei_thermo is the fastest (278.13 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.1x the fastest

Fastest ei_thermo (278.13 us) to slowest ei_branchmax (1.41 ms): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ei_thermo) is the fastest** at 278128.8 ns median
- 1 variant significantly slower than baseline
- Spread: 5.08x (fastest 278128.8 ns, slowest 1413667.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ei_branchmax | 1418270ns | 1417189ns | 1409988ns | 1417021ns | 1424283ns | +403.81% |
| ei_thermo | 281511ns | 280422ns | 277384ns | 279974ns | 285879ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ei_branchmax | 1415006ns | 1406650ns | 1421325ns | +407.32% | 0.012 |
| ei_thermo | 278917ns | 274525ns | 283174ns | base | 0.059 |

## Performance model

- Peak throughput: **0.060 Gops/s** (ei_thermo; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ei_branchmax | 0.012 | 19.4% |
| ei_thermo | 0.059 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ei_branchmax | 1418270ns | 1418270ns | +403.81% |
| ei_thermo | 281511ns | 281511ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ei_thermo | 278129ns | base | --- | [275449, 283174] | --- | --- | --- | --- |
| ei_branchmax | 1413668ns | +1134801.7ns (+408.0%) | [+1129637, +1143827]ns | [1410026, 1421325] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ei_thermo | ei_branchmax |
|---|---|---|
| 1 | 274525ns | +412.4% |
| 2 | 276373ns | +415.6% |
| 3 | 277636ns | +409.2% |
| 4 | 278622ns | +408.8% |
| 5 | 279933ns | +404.9% |
| 6 | 286415ns | +393.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ei_branchmax | -0.553 | HIGH- (thermal bounce) |
| ei_thermo | 0.261 | moderate+ |

**Consistency summary:**

- **ei_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ei_branchmax | 2784.2ns | 1415006.1ns | 0.2% |  |
| ei_thermo | 2270.1ns | 278917.4ns | 0.8% |  |

## Distribution (algo ns)

```
ei_branchmax (n=6, range 1406650.4-1421324.8 ns)
  1406650.4 |#############
  1407384.1 |
  1408117.8 |
  1408851.6 |
  1409585.3 |
  1410319.0 |
  1411052.7 |
  1411786.4 |
  1412520.2 |
  1413253.9 |########################################
  1413987.6 |
  1414721.3 |
  1415455.0 |
  1416188.8 |
  1416922.5 |
  1417656.2 |#############
  1418389.9 |
  1419123.6 |
  1419857.4 |
  1420591.1 |
  (0 below, 1 above range)

ei_thermo (n=6, range 274525.0-283174.3 ns)
  274525.0 |########################################
  274957.5 |
  275389.9 |
  275822.4 |
  276254.9 |########################################
  276687.3 |
  277119.8 |
  277552.3 |########################################
  277984.7 |
  278417.2 |########################################
  278849.7 |
  279282.1 |
  279714.6 |########################################
  280147.1 |
  280579.5 |
  281012.0 |
  281444.5 |
  281876.9 |
  282309.4 |
  282741.9 |
  (0 below, 1 above range)

```

# Effect inference: thermometer OR-join vs naive branch-max lattice join

2 variants, 6 samples per variant.
Baseline: **ei_thermo**

## Highlights

Baseline for all deltas below: **ei_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ei_thermo dominates: 410% faster than the next best (ei_branchmax)

ei_thermo (19.32 us) leads ei_branchmax (98.61 us) by 410%, a clear separation rather than a photo finish. CV 4.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (ei_thermo)

The baseline ei_thermo is the fastest (19.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.1x the fastest

Fastest ei_thermo (19.32 us) to slowest ei_branchmax (98.61 us): 5.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (ei_thermo) is the fastest** at 19324.6 ns median
- 1 variant significantly slower than baseline
- Spread: 5.10x (fastest 19324.6 ns, slowest 98614.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ei_branchmax | 99623ns | 101129ns | 91078ns | 101075ns | 101716ns | +360.41% |
| ei_thermo | 21638ns | 21799ns | 19912ns | 21693ns | 22417ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ei_branchmax | 97045ns | 88877ns | 98949ns | +406.14% | 0.011 |
| ei_thermo | 19173ns | 17617ns | 19872ns | base | 0.053 |

## Performance model

- Peak throughput: **0.058 Gops/s** (ei_thermo; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ei_branchmax | 0.010 | 17.9% |
| ei_thermo | 0.053 | 91.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ei_branchmax | 99623ns | 99623ns | +360.41% |
| ei_thermo | 21638ns | 21638ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ei_thermo | 19325ns | base | --- | [18324, 19872] | --- | --- | --- | --- |
| ei_branchmax | 98614ns | +79050.6ns (+409.1%) | [+74888, +79675]ns | [93570, 98949] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | ei_thermo | ei_branchmax |
|---|---|---|
| 1 | 17617ns | +404.5% |
| 2 | 19130ns | +413.7% |
| 3 | 19519ns | +407.8% |
| 4 | 19525ns | +404.5% |
| 5 | 19030ns | +419.1% |
| 6 | 20219ns | +388.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ei_branchmax | 0.013 | ok |
| ei_thermo | -0.007 | ok |

**Consistency summary:**

- **ei_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ei_branchmax | 164.4ns | 97044.6ns | 0.2% |  |
| ei_thermo | 175.1ns | 19173.3ns | 0.9% |  |

## Distribution (algo ns)

```
ei_branchmax (n=6, range 88877.1-98949.4 ns)
  88877.1 |#############
  89380.7 |
  89884.3 |
  90387.9 |
  90891.6 |
  91395.2 |
  91898.8 |
  92402.4 |
  92906.0 |
  93409.6 |
  93913.2 |
  94416.8 |
  94920.5 |
  95424.1 |
  95927.7 |
  96431.3 |
  96934.9 |
  97438.5 |
  97942.1 |#############
  98445.7 |########################################
  (0 below, 1 above range)

ei_thermo (n=6, range 17616.7-19871.9 ns)
  17616.7 |####################
  17729.5 |
  17842.2 |
  17955.0 |
  18067.7 |
  18180.5 |
  18293.3 |
  18406.0 |
  18518.8 |
  18631.5 |
  18744.3 |
  18857.1 |
  18969.8 |####################
  19082.6 |####################
  19195.3 |
  19308.1 |
  19420.9 |########################################
  19533.6 |
  19646.4 |
  19759.1 |
  (0 below, 1 above range)

```

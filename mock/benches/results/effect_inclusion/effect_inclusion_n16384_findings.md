# Effect inclusion gate: thermometer subset test vs naive per-family compare

2 variants, 6 samples per variant.
Baseline: **eg_thermo**

## Highlights

Baseline for all deltas below: **eg_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### eg_thermo dominates: 6843% faster than the next best (eg_branchmax)

eg_thermo (23.03 us) leads eg_branchmax (1.60 ms) by 6843%, a clear separation rather than a photo finish. CV 5.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### eg_thermo is fastest but the noisiest (CV 5.3%)

eg_thermo wins on median (23.03 us) yet has the highest variance (CV 5.3%), while eg_branchmax is the steadiest (CV 1.8%, 1.60 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (eg_thermo)

The baseline eg_thermo is the fastest (23.03 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 69.4x the fastest

Fastest eg_thermo (23.03 us) to slowest eg_branchmax (1.60 ms): 69.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (eg_thermo) is the fastest** at 23026.2 ns median
- 1 variant significantly slower than baseline
- Spread: 69.43x (fastest 23026.2 ns, slowest 1598756.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| eg_branchmax | 1592725ns | 1602115ns | 1532460ns | 1599550ns | 1612622ns | +6248.75% |
| eg_thermo | 25087ns | 25385ns | 23218ns | 24806ns | 26444ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| eg_branchmax | 1589451ns | 1528769ns | 1609662ns | +6889.31% | 0.010 |
| eg_thermo | 22741ns | 21038ns | 23951ns | base | 0.720 |

## Performance model

- Peak throughput: **0.779 Gops/s** (eg_thermo; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| eg_branchmax | 0.010 | 1.3% |
| eg_thermo | 0.712 | 91.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| eg_branchmax | 1592725ns | 1592725ns | +6248.75% |
| eg_thermo | 25087ns | 25087ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| eg_thermo | 23026ns | base | --- | [21247, 23951] | --- | --- | --- | --- |
| eg_branchmax | 1598757ns | +1576471.1ns (+6846.4%) | [+1537117, +1586540]ns | [1559933, 1609662] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | eg_thermo | eg_branchmax |
|---|---|---|
| 1 | 21038ns | +7462.9% |
| 2 | 22200ns | +7131.3% |
| 3 | 23853ns | +6309.2% |
| 4 | 21455ns | +7377.7% |
| 5 | 23858ns | +6577.8% |
| 6 | 24043ns | +6612.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| eg_branchmax | -0.355 | moderate- |
| eg_thermo | -0.121 | ok |

**Consistency summary:**

- **eg_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| eg_branchmax | 19.5ns | 1589450.6ns | 0.0% |  |
| eg_thermo | 2.6ns | 22741.2ns | 0.0% |  |

## Distribution (algo ns)

```
eg_branchmax (n=6, range 1528769.2-1609661.9 ns)
  1528769.2 |####################
  1532813.8 |
  1536858.5 |
  1540903.1 |
  1544947.7 |
  1548992.4 |
  1553037.0 |
  1557081.6 |
  1561126.3 |
  1565170.9 |
  1569215.5 |
  1573260.2 |
  1577304.8 |
  1581349.5 |
  1585394.1 |
  1589438.7 |########################################
  1593483.4 |
  1597528.0 |
  1601572.6 |########################################
  1605617.3 |
  (0 below, 1 above range)

eg_thermo (n=6, range 21038.3-23950.6 ns)
  21038.3 |####################
  21183.9 |
  21329.5 |####################
  21475.1 |
  21620.8 |
  21766.4 |
  21912.0 |
  22057.6 |####################
  22203.2 |
  22348.8 |
  22494.4 |
  22640.1 |
  22785.7 |
  22931.3 |
  23076.9 |
  23222.5 |
  23368.1 |
  23513.8 |
  23659.4 |
  23805.0 |########################################
  (0 below, 1 above range)

```

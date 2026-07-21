# Effect inclusion gate: thermometer subset test vs naive per-family compare

2 variants, 6 samples per variant.
Baseline: **eg_thermo**

## Highlights

Baseline for all deltas below: **eg_thermo**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### eg_thermo dominates: 6104% faster than the next best (eg_branchmax)

eg_thermo (5.54 us) leads eg_branchmax (343.61 us) by 6104%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### eg_thermo is fastest but the noisiest (CV 5.2%)

eg_thermo wins on median (5.54 us) yet has the highest variance (CV 5.2%), while eg_branchmax is the steadiest (CV 4.3%, 343.61 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (eg_thermo)

The baseline eg_thermo is the fastest (5.54 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 62.0x the fastest

Fastest eg_thermo (5.54 us) to slowest eg_branchmax (343.61 us): 62.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (eg_thermo) is the fastest** at 5538.8 ns median
- 1 variant significantly slower than baseline
- Spread: 62.04x (fastest 5538.8 ns, slowest 343614.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| eg_branchmax | 338435ns | 345899ns | 315663ns | 337117ns | 351798ns | +4172.27% |
| eg_thermo | 7922ns | 7746ns | 7484ns | 7709ns | 8460ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| eg_branchmax | 336037ns | 313474ns | 349062ns | +5840.05% | 0.012 |
| eg_thermo | 5657ns | 5348ns | 6051ns | base | 0.724 |

## Performance model

- Peak throughput: **0.766 Gops/s** (eg_thermo; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| eg_branchmax | 0.012 | 1.6% |
| eg_thermo | 0.740 | 96.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| eg_branchmax | 338435ns | 338435ns | +4172.27% |
| eg_thermo | 7922ns | 7922ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| eg_thermo | 5539ns | base | --- | [5382, 6051] | --- | --- | --- | --- |
| eg_branchmax | 343615ns | +337781.9ns (+6098.5%) | [+309676, +343681]ns | [315433, 349062] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | eg_thermo | eg_branchmax |
|---|---|---|
| 1 | 5348ns | +6481.6% |
| 2 | 5416ns | +6291.9% |
| 3 | 6049ns | +5147.3% |
| 4 | 5612ns | +6027.4% |
| 5 | 6053ns | +5571.7% |
| 6 | 5465ns | +5636.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| eg_branchmax | -0.212 | moderate- |
| eg_thermo | -0.260 | moderate- |

**Consistency summary:**

- **eg_branchmax**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| eg_branchmax | 10.8ns | 336036.8ns | 0.0% |  |
| eg_thermo | 3.4ns | 5657.1ns | 0.1% |  |

## Distribution (algo ns)

```
eg_branchmax (n=6, range 313474.2-349062.5 ns)
  313474.2 |########################################
  315253.6 |
  317033.0 |########################################
  318812.4 |
  320591.9 |
  322371.3 |
  324150.7 |
  325930.1 |
  327709.5 |
  329488.9 |
  331268.3 |
  333047.7 |
  334827.2 |
  336606.6 |
  338386.0 |
  340165.4 |
  341944.8 |########################################
  343724.2 |########################################
  345503.6 |########################################
  347283.0 |
  (0 below, 1 above range)

eg_thermo (n=6, range 5347.5-6051.0 ns)
   5347.5 |########################################
   5382.7 |########################################
   5417.9 |
   5453.0 |########################################
   5488.2 |
   5523.4 |
   5558.6 |
   5593.7 |########################################
   5628.9 |
   5664.1 |
   5699.2 |
   5734.4 |
   5769.6 |
   5804.8 |
   5839.9 |
   5875.1 |
   5910.3 |
   5945.5 |
   5980.6 |
   6015.8 |########################################
  (0 below, 1 above range)

```

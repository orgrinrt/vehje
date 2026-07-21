# Interpreter dispatch shapes: switch vs fnptr-table vs computed

2 variants, 6 samples per variant.
Baseline: **hx_dispatch__switch**

## Highlights

Baseline for all deltas below: **hx_dispatch__switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hx_dispatch__switch dominates: 475% faster than the next best (hx_dispatch__fnptr)

hx_dispatch__switch (5.49 us) leads hx_dispatch__fnptr (31.55 us) by 475%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hx_dispatch__switch is fastest but the noisiest (CV 5.2%)

hx_dispatch__switch wins on median (5.49 us) yet has the highest variance (CV 5.2%), while hx_dispatch__fnptr is the steadiest (CV 4.7%, 31.55 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (hx_dispatch__switch)

The baseline hx_dispatch__switch is the fastest (5.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 5.7x the fastest

Fastest hx_dispatch__switch (5.49 us) to slowest hx_dispatch__fnptr (31.55 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (hx_dispatch__switch) is the fastest** at 5489.2 ns median
- 1 variant significantly slower than baseline
- Spread: 5.75x (fastest 5489.2 ns, slowest 31554.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_dispatch__fnptr | 33791ns | 34100ns | 30555ns | 33939ns | 35186ns | +332.77% |
| hx_dispatch__switch | 7808ns | 7900ns | 7274ns | 7694ns | 8247ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_dispatch__fnptr | 31288ns | 28304ns | 32593ns | +477.98% | 0.131 |
| hx_dispatch__switch | 5413ns | 5033ns | 5716ns | base | 0.757 |

## Performance model

- Peak throughput: **0.814 Gops/s** (hx_dispatch__switch; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_dispatch__fnptr | 0.130 | 16.0% |
| hx_dispatch__switch | 0.746 | 91.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_dispatch__fnptr | 33791ns | 33791ns | +332.77% |
| hx_dispatch__switch | 7808ns | 7808ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_dispatch__switch | 5489ns | base | --- | [5035, 5716] | --- | --- | --- | --- |
| hx_dispatch__fnptr | 31555ns | +26092.7ns (+475.3%) | [+24382, +27150]ns | [29717, 32593] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_dispatch__switch | hx_dispatch__fnptr |
|---|---|---|
| 1 | 5033ns | +462.3% |
| 2 | 5795ns | +453.0% |
| 3 | 5637ns | +452.3% |
| 4 | 5529ns | +469.1% |
| 5 | 5037ns | +528.3% |
| 6 | 5450ns | +508.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_dispatch__fnptr | -0.129 | ok |
| hx_dispatch__switch | -0.184 | ok |

**Consistency summary:**

- **hx_dispatch__fnptr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_dispatch__fnptr | 4.0ns | 31288.3ns | 0.0% |  |
| hx_dispatch__switch | 3.1ns | 5413.4ns | 0.1% |  |

## Distribution (algo ns)

```
hx_dispatch__fnptr (n=6, range 28303.7-32592.7 ns)
  28303.7 |########################################
  28518.2 |
  28732.6 |
  28947.0 |
  29161.5 |
  29376.0 |
  29590.4 |
  29804.9 |
  30019.3 |
  30233.8 |
  30448.2 |
  30662.7 |
  30877.1 |
  31091.5 |########################################
  31306.0 |########################################
  31520.5 |########################################
  31734.9 |
  31949.3 |########################################
  32163.8 |
  32378.2 |
  (0 below, 1 above range)

hx_dispatch__switch (n=6, range 5033.3-5716.0 ns)
   5033.3 |########################################
   5067.4 |
   5101.6 |
   5135.7 |
   5169.9 |
   5204.0 |
   5238.1 |
   5272.3 |
   5306.4 |
   5340.5 |
   5374.7 |
   5408.8 |
   5442.9 |####################
   5477.1 |
   5511.2 |####################
   5545.4 |
   5579.5 |
   5613.6 |####################
   5647.8 |
   5681.9 |
  (0 below, 1 above range)

```

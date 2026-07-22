# Zig dispatch: switch vs tail-threaded @call(.always_tail) (the Deegen question)

2 variants, 6 samples per variant.
Baseline: **zig_switch**

## Highlights

Baseline for all deltas below: **zig_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### zig_switch dominates: 301% faster than the next best (zig_tail)

zig_switch (76.37 us) leads zig_tail (306.48 us) by 301%, a clear separation rather than a photo finish. CV 6.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (zig_switch)

The baseline zig_switch is the fastest (76.37 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 4.0x the fastest

Fastest zig_switch (76.37 us) to slowest zig_tail (306.48 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (zig_switch) is the fastest** at 76367.7 ns median
- 1 variant significantly slower than baseline
- Spread: 4.01x (fastest 76367.7 ns, slowest 306483.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| zig_switch | 80977ns | 78792ns | 75560ns | 78379ns | 87582ns | base |
| zig_tail | 303805ns | 309125ns | 264792ns | 296581ns | 334148ns | +275.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| zig_switch | 78584ns | 73370ns | 85089ns | base | 0.039 |
| zig_tail | 301319ns | 262514ns | 331584ns | +283.43% | 0.010 |

## Performance model

- Peak throughput: **0.042 Gops/s** (zig_switch; best 20% batches)
- Ops per call: 3072

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| zig_switch | 0.040 | 96.1% |
| zig_tail | 0.010 | 23.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| zig_switch | 80977ns | 80977ns | base |
| zig_tail | 303805ns | 303805ns | +275.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| zig_switch | 76368ns | base | --- | [74296, 85089] | --- | --- | --- | --- |
| zig_tail | 306483ns | +225658.0ns (+295.5%) | [+191468, +251080]ns | [265891, 331584] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | zig_switch | zig_tail |
|---|---|---|
| 1 | 73370ns | +267.0% |
| 2 | 77258ns | +272.4% |
| 3 | 75223ns | +340.0% |
| 4 | 84393ns | +285.4% |
| 5 | 75478ns | +247.8% |
| 6 | 85785ns | +287.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| zig_switch | -0.358 | moderate- |
| zig_tail | -0.270 | moderate- |

**Consistency summary:**

- **zig_tail**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| zig_switch | 2.5ns | 78584.3ns | 0.0% |  |
| zig_tail | 3.6ns | 301319.4ns | 0.0% |  |

## Distribution (algo ns)

```
zig_switch (n=6, range 73369.6-85088.8 ns)
  73369.6 |####################
  73955.6 |
  74541.5 |
  75127.5 |########################################
  75713.4 |
  76299.4 |
  76885.3 |####################
  77471.3 |
  78057.3 |
  78643.2 |
  79229.2 |
  79815.1 |
  80401.1 |
  80987.0 |
  81573.0 |
  82159.0 |
  82744.9 |
  83330.9 |
  83916.8 |####################
  84502.8 |
  (0 below, 1 above range)

zig_tail (n=6, range 262514.2-331583.8 ns)
  262514.2 |########################################
  265967.7 |########################################
  269421.2 |
  272874.6 |
  276328.1 |
  279781.6 |
  283235.1 |
  286688.5 |########################################
  290142.0 |
  293595.5 |
  297049.0 |
  300502.5 |
  303955.9 |
  307409.4 |
  310862.9 |
  314316.4 |
  317769.8 |
  321223.3 |
  324676.8 |########################################
  328130.3 |########################################
  (0 below, 1 above range)

```

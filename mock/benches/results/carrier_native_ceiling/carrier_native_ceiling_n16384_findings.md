# Native ceiling: switch vs fn-table interp vs shape-specialized native, opaque program (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_ceiling_native**

## Highlights

Baseline for all deltas below: **carrier_ceiling_native**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_ceiling_native dominates: 104% faster than the next best (carrier_ceiling_switch)

carrier_ceiling_native (263.41 us) leads carrier_ceiling_switch (537.79 us) by 104%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_ceiling_fntable is an outlier: 2.2x slower than the field

carrier_ceiling_fntable (584.33 us) is 2.2x the fastest (263.41 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (carrier_ceiling_native)

The baseline carrier_ceiling_native is the fastest (263.41 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (carrier_ceiling_native) is the fastest** at 263406.9 ns median
- 2 variants significantly slower than baseline
- Spread: 2.22x (fastest 263406.9 ns, slowest 584327.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_ceiling_fntable | 603258ns | 586658ns | 580186ns | 586505ns | 639922ns | +124.04% |
| carrier_ceiling_native | 269265ns | 265673ns | 256154ns | 263701ns | 284168ns | base |
| carrier_ceiling_switch | 545940ns | 540172ns | 537791ns | 539399ns | 559826ns | +102.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_ceiling_fntable | 600740ns | 577978ns | 637308ns | +125.06% | 0.027 |
| carrier_ceiling_native | 266920ns | 253819ns | 281779ns | base | 0.061 |
| carrier_ceiling_switch | 543570ns | 535445ns | 557395ns | +103.65% | 0.030 |

## Performance model

- Peak throughput: **0.065 Gops/s** (carrier_ceiling_native; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_ceiling_fntable | 0.028 | 43.4% |
| carrier_ceiling_native | 0.062 | 96.4% |
| carrier_ceiling_switch | 0.030 | 47.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_ceiling_fntable | 603258ns | 603258ns | +124.04% |
| carrier_ceiling_native | 269265ns | 269265ns | base |
| carrier_ceiling_switch | 545940ns | 545940ns | +102.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_ceiling_native | 263407ns | base | --- | [255575, 281779] | --- | --- | --- | --- |
| carrier_ceiling_fntable | 584328ns | +325153.8ns (+123.4%) | [+310207, +366098]ns | [580583, 637308] | YES | 0.0313 | 0.0313 | 0 |
| carrier_ceiling_switch | 537795ns | +276271.1ns (+104.9%) | [+263011, +290668]ns | [535521, 557395] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_ceiling_native | carrier_ceiling_fntable | carrier_ceiling_switch |
|---|---|---|---|
| 1 | 280470ns | +108.4% | +90.9% |
| 2 | 253819ns | +130.1% | +112.6% |
| 3 | 265229ns | +122.3% | +111.4% |
| 4 | 261585ns | +121.0% | +104.8% |
| 5 | 257331ns | +126.6% | +108.2% |
| 6 | 283087ns | +142.0% | +95.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_ceiling_fntable | -0.044 | ok |
| carrier_ceiling_native | -0.338 | moderate- |
| carrier_ceiling_switch | -0.314 | moderate- |

**Consistency summary:**

- **carrier_ceiling_fntable**: won 0/6, lost 6/6
- **carrier_ceiling_switch**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_ceiling_fntable | 2062.3ns | 600739.7ns | 0.3% |  |
| carrier_ceiling_native | 9.7ns | 266920.3ns | 0.0% |  |
| carrier_ceiling_switch | 1932.8ns | 543570.3ns | 0.4% |  |

## Distribution (algo ns)

```
carrier_ceiling_fntable (n=6, range 577977.9-637308.3 ns)
  577977.9 |####################
  580944.4 |####################
  583910.9 |########################################
  586877.5 |####################
  589844.0 |
  592810.5 |
  595777.0 |
  598743.6 |
  601710.1 |
  604676.6 |
  607643.1 |
  610609.6 |
  613576.2 |
  616542.7 |
  619509.2 |
  622475.7 |
  625442.3 |
  628408.8 |
  631375.3 |
  634341.8 |
  (0 below, 1 above range)

carrier_ceiling_native (n=6, range 253819.2-281778.8 ns)
  253819.2 |########################################
  255217.2 |
  256615.2 |########################################
  258013.1 |
  259411.1 |
  260809.1 |########################################
  262207.1 |
  263605.0 |
  265003.0 |########################################
  266401.0 |
  267799.0 |
  269197.0 |
  270594.9 |
  271992.9 |
  273390.9 |
  274788.9 |
  276186.8 |
  277584.8 |
  278982.8 |
  280380.8 |########################################
  (0 below, 1 above range)

carrier_ceiling_switch (n=6, range 535445.0-557395.0 ns)
  535445.0 |########################################
  536542.5 |
  537640.0 |
  538737.5 |#############
  539835.0 |
  540932.5 |
  542030.0 |
  543127.5 |
  544225.0 |
  545322.5 |
  546420.0 |
  547517.5 |
  548615.0 |
  549712.5 |
  550810.0 |
  551907.5 |
  553005.0 |
  554102.5 |#############
  555200.0 |
  556297.5 |
  (0 below, 1 above range)

```

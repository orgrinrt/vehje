# Partial-eval specialization: fold ratio on a random static/dynamic mix (reduction metric)

4 variants, 6 samples per variant.
Baseline: **pe_rand_sf30**

## Highlights

Baseline for all deltas below: **pe_rand_sf30**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_rand_sf30) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_rand_sf30 has the worst median (20.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_rand_sf90 at 10.20 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### pe_rand_sf90 dominates: 60% faster than the next best (pe_rand_sf70)

pe_rand_sf90 (10.20 us) leads pe_rand_sf70 (16.35 us) by 60%, a clear separation rather than a photo finish. CV 7.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### pe_rand_sf90 beats baseline by 52% (significant)

pe_rand_sf90 is -10.56 us (52%) faster than baseline pe_rand_sf30, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### pe_rand_sf30 is an outlier: 2.0x slower than the field

pe_rand_sf30 (20.46 us) is 2.0x the fastest (10.20 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {pe_rand_sf90} vs {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} (60% apart)

The field splits into a fast tier {pe_rand_sf90} and a slow tier {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} with a 60% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: pe_rand_sf90** at 10198.4 ns median (-50.2% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.01x (fastest 10198.4 ns, slowest 20461.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_rand_sf30 | 22632ns | 22812ns | 21082ns | 22275ns | 23943ns | base |
| pe_rand_sf50 | 21003ns | 21070ns | 18825ns | 20516ns | 22822ns | -7.20% |
| pe_rand_sf70 | 18317ns | 18839ns | 16466ns | 18057ns | 19632ns | -19.07% |
| pe_rand_sf90 | 12128ns | 12796ns | 10766ns | 12125ns | 12815ns | -46.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_rand_sf30 | 20283ns | 18899ns | 21437ns | base | 0.013 |
| pe_rand_sf50 | 18620ns | 16675ns | 20226ns | -8.20% | 0.014 |
| pe_rand_sf70 | 15895ns | 14299ns | 17025ns | -21.63% | 0.016 |
| pe_rand_sf90 | 9670ns | 8590ns | 10219ns | -52.32% | 0.026 |

## Performance model

- Peak throughput: **0.030 Gops/s** (pe_rand_sf90; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_rand_sf30 | 0.013 | 42.0% |
| pe_rand_sf50 | 0.014 | 46.0% |
| pe_rand_sf70 | 0.016 | 52.5% |
| pe_rand_sf90 | 0.025 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_rand_sf30 | 22632ns | 22632ns | base |
| pe_rand_sf50 | 21003ns | 21003ns | -7.20% |
| pe_rand_sf70 | 18317ns | 18317ns | -19.07% |
| pe_rand_sf90 | 12128ns | 12128ns | -46.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_rand_sf30 | 20461ns | base | --- | [18952, 21437] | --- | --- | --- | --- |
| pe_rand_sf50 | 18691ns | -1649.0ns (-8.1%) | [-3030, -311]ns | [16943, 20226] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| pe_rand_sf70 | 16353ns | -4466.9ns (-21.8%) | [-5255, -3442]ns | [14309, 17025] | YES | 0.0469 | 0.0313 | 0 |
| pe_rand_sf90 | 10198ns | -10564.0ns (-51.6%) | [-11229, -10045]ns | [8594, 10219] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_rand_sf30 | pe_rand_sf50 | pe_rand_sf70 | pe_rand_sf90 |
|---|---|---|---|---|
| 1 | 20941ns | -20.4% | -18.8% | -51.3% |
| 2 | 20931ns | -4.2% | -20.7% | -51.2% |
| 3 | 21933ns | -7.9% | -26.6% | -53.4% |
| 4 | 18899ns | -8.3% | -24.3% | -54.5% |
| 5 | 19005ns | -9.4% | -24.7% | -54.8% |
| 6 | 19991ns | +1.3% | -14.8% | -49.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_rand_sf30 | 0.188 | ok |
| pe_rand_sf50 | -0.208 | moderate- |
| pe_rand_sf70 | 0.162 | ok |
| pe_rand_sf90 | 0.167 | ok |

**Consistency summary:**

- **pe_rand_sf50**: won 5/6, lost 1/6
- **pe_rand_sf70**: won 6/6, lost 0/6
- **pe_rand_sf90**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_rand_sf30 | 4.2ns | 20283.3ns | 0.0% |  |
| pe_rand_sf50 | 3.9ns | 18619.9ns | 0.0% |  |
| pe_rand_sf70 | 4.1ns | 15895.4ns | 0.0% |  |
| pe_rand_sf90 | 5.0ns | 9670.5ns | 0.1% |  |

## Distribution (algo ns)

```
pe_rand_sf30 (n=6, range 18898.8-21437.1 ns)
  18898.8 |########################################
  19025.7 |
  19152.6 |
  19279.5 |
  19406.5 |
  19533.4 |
  19660.3 |
  19787.2 |
  19914.1 |####################
  20041.0 |
  20167.9 |
  20294.8 |
  20421.8 |
  20548.7 |
  20675.6 |
  20802.5 |
  20929.4 |########################################
  21056.3 |
  21183.2 |
  21310.1 |
  (0 below, 1 above range)

pe_rand_sf50 (n=6, range 16675.4-20226.1 ns)
  16675.4 |####################
  16852.9 |
  17030.5 |
  17208.0 |########################################
  17385.5 |
  17563.1 |
  17740.6 |
  17918.1 |
  18095.7 |
  18273.2 |
  18450.7 |
  18628.3 |
  18805.8 |
  18983.3 |
  19160.9 |
  19338.4 |
  19515.9 |
  19693.5 |
  19871.0 |####################
  20048.5 |####################
  (0 below, 1 above range)

pe_rand_sf70 (n=6, range 14299.2-17024.6 ns)
  14299.2 |########################################
  14435.5 |
  14571.7 |
  14708.0 |
  14844.3 |
  14980.5 |
  15116.8 |
  15253.1 |
  15389.4 |
  15525.6 |
  15661.9 |
  15798.2 |
  15934.4 |
  16070.7 |####################
  16207.0 |
  16343.2 |
  16479.5 |####################
  16615.8 |
  16752.1 |
  16888.3 |####################
  (0 below, 1 above range)

pe_rand_sf90 (n=6, range 8590.0-10219.3 ns)
   8590.0 |##########################
   8671.5 |
   8752.9 |
   8834.4 |
   8915.9 |
   8997.3 |
   9078.8 |
   9160.3 |
   9241.7 |
   9323.2 |
   9404.7 |
   9486.1 |
   9567.6 |
   9649.1 |
   9730.5 |
   9812.0 |
   9893.5 |
   9974.9 |
  10056.4 |
  10137.9 |########################################
  (0 below, 1 above range)

```

# Partial-eval specialization: fold ratio on a random static/dynamic mix (reduction metric)

4 variants, 6 samples per variant.
Baseline: **pe_rand_sf30**

## Highlights

Baseline for all deltas below: **pe_rand_sf30**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_rand_sf30) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_rand_sf30 has the worst median (5.93 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_rand_sf90 at 2.37 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### pe_rand_sf90 dominates: 90% faster than the next best (pe_rand_sf70)

pe_rand_sf90 (2.37 us) leads pe_rand_sf70 (4.52 us) by 90%, a clear separation rather than a photo finish. CV 15.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### pe_rand_sf90 beats baseline by 59% (significant)

pe_rand_sf90 is -3.50 us (59%) faster than baseline pe_rand_sf30, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### pe_rand_sf30 is an outlier: 2.5x slower than the field

pe_rand_sf30 (5.93 us) is 2.5x the fastest (2.37 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### pe_rand_sf90 is fastest but the noisiest (CV 15.1%)

pe_rand_sf90 wins on median (2.37 us) yet has the highest variance (CV 15.1%), while pe_rand_sf70 is the steadiest (CV 11.4%, 4.52 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {pe_rand_sf90} vs {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} (90% apart)

The field splits into a fast tier {pe_rand_sf90} and a slow tier {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} with a 90% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: pe_rand_sf90** at 2373.5 ns median (-60.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.50x (fastest 2373.5 ns, slowest 5929.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_rand_sf30 | 8418ns | 8473ns | 6982ns | 8055ns | 9679ns | base |
| pe_rand_sf50 | 8087ns | 7949ns | 6748ns | 7663ns | 9392ns | -3.93% |
| pe_rand_sf70 | 7033ns | 7066ns | 6046ns | 6802ns | 7872ns | -16.45% |
| pe_rand_sf90 | 4976ns | 4968ns | 4172ns | 4716ns | 5769ns | -40.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_rand_sf30 | 5854ns | 4812ns | 6719ns | base | 0.011 |
| pe_rand_sf50 | 5502ns | 4532ns | 6375ns | -6.01% | 0.012 |
| pe_rand_sf70 | 4502ns | 3883ns | 5030ns | -23.09% | 0.014 |
| pe_rand_sf90 | 2407ns | 1993ns | 2834ns | -58.88% | 0.027 |

## Performance model

- Peak throughput: **0.032 Gops/s** (pe_rand_sf90; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_rand_sf30 | 0.011 | 33.6% |
| pe_rand_sf50 | 0.012 | 36.9% |
| pe_rand_sf70 | 0.014 | 44.1% |
| pe_rand_sf90 | 0.027 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_rand_sf30 | 8418ns | 8418ns | base |
| pe_rand_sf50 | 8087ns | 8087ns | -3.93% |
| pe_rand_sf70 | 7033ns | 7033ns | -16.45% |
| pe_rand_sf90 | 4976ns | 4976ns | -40.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_rand_sf30 | 5929ns | base | --- | [4914, 6719] | --- | --- | --- | --- |
| pe_rand_sf50 | 5407ns | no significant difference | [-834, +11]ns | [4724, 6375] | no | 0.2188 | 0.2188 | 0 |
| pe_rand_sf70 | 4520ns | -1266.7ns (-21.4%) | [-1831, -958]ns | [3956, 5030] | YES | 0.0469 | 0.0313 | 0 |
| pe_rand_sf90 | 2374ns | -3495.4ns (-59.0%) | [-3945, -2901]ns | [2013, 2834] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_rand_sf30 | pe_rand_sf50 | pe_rand_sf70 | pe_rand_sf90 |
|---|---|---|---|---|
| 1 | 6982ns | +1.7% | -21.5% | -56.5% |
| 2 | 6456ns | -12.5% | -29.1% | -59.3% |
| 3 | 6359ns | -13.5% | -28.0% | -62.1% |
| 4 | 4812ns | -5.8% | -19.3% | -58.6% |
| 5 | 5015ns | -2.0% | -19.7% | -59.5% |
| 6 | 5499ns | -3.4% | -18.8% | -57.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_rand_sf30 | 0.428 | moderate+ |
| pe_rand_sf50 | 0.233 | moderate+ |
| pe_rand_sf70 | 0.216 | moderate+ |
| pe_rand_sf90 | 0.418 | moderate+ |

**Consistency summary:**

- **pe_rand_sf50**: won 5/6, lost 1/6
- **pe_rand_sf70**: won 6/6, lost 0/6
- **pe_rand_sf90**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_rand_sf30 | 4.8ns | 5853.8ns | 0.1% |  |
| pe_rand_sf50 | 4.6ns | 5502.1ns | 0.1% |  |
| pe_rand_sf70 | 4.1ns | 4501.9ns | 0.1% |  |
| pe_rand_sf90 | 4.6ns | 2406.9ns | 0.2% |  |

## Distribution (algo ns)

```
pe_rand_sf30 (n=6, range 4812.1-6718.8 ns)
   4812.1 |########################################
   4907.4 |
   5002.8 |########################################
   5098.1 |
   5193.4 |
   5288.8 |
   5384.1 |
   5479.4 |########################################
   5574.8 |
   5670.1 |
   5765.4 |
   5860.8 |
   5956.1 |
   6051.4 |
   6146.8 |
   6242.1 |
   6337.4 |########################################
   6432.8 |########################################
   6528.1 |
   6623.4 |
  (0 below, 1 above range)

pe_rand_sf50 (n=6, range 4531.7-6374.5 ns)
   4531.7 |########################################
   4623.8 |
   4716.0 |
   4808.1 |
   4900.3 |########################################
   4992.4 |
   5084.6 |
   5176.7 |
   5268.8 |########################################
   5361.0 |
   5453.1 |########################################
   5545.3 |
   5637.4 |########################################
   5729.6 |
   5821.7 |
   5913.8 |
   6006.0 |
   6098.1 |
   6190.3 |
   6282.4 |
  (0 below, 1 above range)

pe_rand_sf70 (n=6, range 3882.9-5029.8 ns)
   3882.9 |####################
   3940.2 |
   3997.6 |####################
   4054.9 |
   4112.3 |
   4169.6 |
   4227.0 |
   4284.3 |
   4341.7 |
   4399.0 |
   4456.3 |####################
   4513.7 |
   4571.0 |########################################
   4628.4 |
   4685.7 |
   4743.1 |
   4800.4 |
   4857.8 |
   4915.1 |
   4972.5 |
  (0 below, 1 above range)

pe_rand_sf90 (n=6, range 1992.9-2834.2 ns)
   1992.9 |########################################
   2035.0 |
   2077.0 |
   2119.1 |
   2161.2 |
   2203.2 |
   2245.3 |
   2287.3 |
   2329.4 |####################
   2371.5 |####################
   2413.5 |
   2455.6 |
   2497.7 |
   2539.7 |
   2581.8 |
   2623.8 |####################
   2665.9 |
   2708.0 |
   2750.0 |
   2792.1 |
  (0 below, 1 above range)

```

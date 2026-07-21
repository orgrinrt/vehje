# Partial-eval specialization: fold ratio on a random static/dynamic mix (reduction metric)

4 variants, 6 samples per variant.
Baseline: **pe_rand_sf30**

## Highlights

Baseline for all deltas below: **pe_rand_sf30**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_rand_sf30) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_rand_sf30 has the worst median (335.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_rand_sf90 at 132.91 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### pe_rand_sf90 dominates: 83% faster than the next best (pe_rand_sf70)

pe_rand_sf90 (132.91 us) leads pe_rand_sf70 (243.21 us) by 83%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### pe_rand_sf90 beats baseline by 61% (significant)

pe_rand_sf90 is -205.48 us (61%) faster than baseline pe_rand_sf30, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### pe_rand_sf30 is an outlier: 2.5x slower than the field

pe_rand_sf30 (335.58 us) is 2.5x the fastest (132.91 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {pe_rand_sf90} vs {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} (83% apart)

The field splits into a fast tier {pe_rand_sf90} and a slow tier {pe_rand_sf70, pe_rand_sf50, pe_rand_sf30} with a 83% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: pe_rand_sf90** at 132909.8 ns median (-60.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.52x (fastest 132909.8 ns, slowest 335575.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_rand_sf30 | 340518ns | 338027ns | 309385ns | 335368ns | 363809ns | base |
| pe_rand_sf50 | 328227ns | 317181ns | 289450ns | 308156ns | 377723ns | -3.61% |
| pe_rand_sf70 | 248673ns | 245711ns | 228948ns | 240577ns | 270679ns | -26.97% |
| pe_rand_sf90 | 134865ns | 135415ns | 128685ns | 134905ns | 137894ns | -60.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_rand_sf30 | 337995ns | 307156ns | 361082ns | base | 0.012 |
| pe_rand_sf50 | 325868ns | 287095ns | 375470ns | -3.59% | 0.013 |
| pe_rand_sf70 | 246282ns | 226686ns | 268267ns | -27.13% | 0.017 |
| pe_rand_sf90 | 132431ns | 126415ns | 135462ns | -60.82% | 0.031 |

## Performance model

- Peak throughput: **0.032 Gops/s** (pe_rand_sf90; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_rand_sf30 | 0.012 | 37.7% |
| pe_rand_sf50 | 0.013 | 40.1% |
| pe_rand_sf70 | 0.017 | 52.0% |
| pe_rand_sf90 | 0.031 | 95.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_rand_sf30 | 340518ns | 340518ns | base |
| pe_rand_sf50 | 328227ns | 328227ns | -3.61% |
| pe_rand_sf70 | 248673ns | 248673ns | -26.97% |
| pe_rand_sf90 | 134865ns | 134865ns | -60.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_rand_sf30 | 335575ns | base | --- | [317326, 361082] | --- | --- | --- | --- |
| pe_rand_sf50 | 314958ns | no significant difference | [-70301, +39184]ns | [287175, 375470] | no | 1.0000 | 1.0000 | 0 |
| pe_rand_sf70 | 243206ns | -80399.4ns (-24.0%) | [-130103, -64635]ns | [227372, 268267] | YES | 0.0469 | 0.0313 | 0 |
| pe_rand_sf90 | 132910ns | -205484.2ns (-61.2%) | [-228554, -182652]ns | [128922, 135462] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_rand_sf30 | pe_rand_sf50 | pe_rand_sf70 | pe_rand_sf90 |
|---|---|---|---|---|
| 1 | 333287ns | -6.5% | -18.2% | -60.0% |
| 2 | 307156ns | +3.6% | -22.3% | -56.8% |
| 3 | 345077ns | +5.8% | -23.5% | -61.1% |
| 4 | 337864ns | -15.0% | -32.5% | -62.6% |
| 5 | 377088ns | -23.8% | -39.9% | -65.1% |
| 6 | 327496ns | +17.8% | -24.3% | -58.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_rand_sf30 | -0.184 | ok |
| pe_rand_sf50 | -0.302 | moderate- |
| pe_rand_sf70 | -0.186 | ok |
| pe_rand_sf90 | -0.139 | ok |

**Consistency summary:**

- **pe_rand_sf50**: won 3/6, lost 3/6
- **pe_rand_sf70**: won 6/6, lost 0/6
- **pe_rand_sf90**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_rand_sf30 | 11.1ns | 337994.6ns | 0.0% |  |
| pe_rand_sf50 | 7.9ns | 325867.7ns | 0.0% |  |
| pe_rand_sf70 | 5.0ns | 246282.1ns | 0.0% |  |
| pe_rand_sf90 | 5.1ns | 132431.2ns | 0.0% |  |

## Distribution (algo ns)

```
pe_rand_sf30 (n=6, range 307155.8-361082.3 ns)
  307155.8 |########################################
  309852.1 |
  312548.5 |
  315244.8 |
  317941.1 |
  320637.4 |
  323333.8 |
  326030.1 |########################################
  328726.4 |
  331422.7 |########################################
  334119.1 |
  336815.4 |########################################
  339511.7 |
  342208.0 |
  344904.4 |########################################
  347600.7 |
  350297.0 |
  352993.3 |
  355689.7 |
  358386.0 |
  (0 below, 1 above range)

pe_rand_sf50 (n=6, range 287095.0-375470.4 ns)
  287095.0 |########################################
  291513.8 |
  295932.5 |
  300351.3 |
  304770.1 |
  309188.8 |####################
  313607.6 |
  318026.4 |####################
  322445.2 |
  326863.9 |
  331282.7 |
  335701.5 |
  340120.2 |
  344539.0 |
  348957.8 |
  353376.6 |
  357795.3 |
  362214.1 |####################
  366632.9 |
  371051.6 |
  (0 below, 1 above range)

pe_rand_sf70 (n=6, range 226686.2-268267.2 ns)
  226686.2 |########################################
  228765.3 |
  230844.3 |
  232923.4 |
  235002.4 |
  237081.5 |####################
  239160.5 |
  241239.6 |
  243318.6 |
  245397.7 |
  247476.7 |####################
  249555.8 |
  251634.8 |
  253713.9 |
  255792.9 |
  257872.0 |
  259951.0 |
  262030.1 |####################
  264109.1 |
  266188.2 |
  (0 below, 1 above range)

pe_rand_sf90 (n=6, range 126415.0-135462.0 ns)
  126415.0 |########################################
  126867.4 |
  127319.7 |
  127772.1 |
  128224.4 |
  128676.8 |
  129129.1 |
  129581.5 |
  130033.8 |
  130486.2 |
  130938.5 |
  131390.9 |########################################
  131843.2 |
  132295.6 |########################################
  132747.9 |
  133200.3 |########################################
  133652.6 |
  134105.0 |########################################
  134557.3 |
  135009.7 |
  (0 below, 1 above range)

```

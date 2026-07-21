# Reachability fixpoint: whole-column vs real semi-naive (chain8)

2 variants, 6 samples per variant.
Baseline: **r_chain8_whole**

## Highlights

Baseline for all deltas below: **r_chain8_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain8_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain8_whole has the worst median (374.07 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain8_semi at 100.43 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain8_semi dominates: 272% faster than the next best (r_chain8_whole)

r_chain8_semi (100.43 us) leads r_chain8_whole (374.07 us) by 272%, a clear separation rather than a photo finish. CV 14.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain8_semi beats baseline by 73% (significant)

r_chain8_semi is -272.47 us (73%) faster than baseline r_chain8_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_chain8_semi is fastest but the noisiest (CV 14.4%)

r_chain8_semi wins on median (100.43 us) yet has the highest variance (CV 14.4%), while r_chain8_whole is the steadiest (CV 1.3%, 374.07 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.7x the fastest

Fastest r_chain8_semi (100.43 us) to slowest r_chain8_whole (374.07 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain8_semi** at 100427.1 ns median (-73.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.72x (fastest 100427.1 ns, slowest 374072.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain8_semi | 106236ns | 102703ns | 86678ns | 101862ns | 122577ns | -71.74% |
| r_chain8_whole | 375875ns | 376589ns | 367306ns | 375168ns | 381221ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain8_semi | 103954ns | 84452ns | 120330ns | -72.15% | 0.010 |
| r_chain8_whole | 373287ns | 364728ns | 378504ns | base | 0.003 |

## Performance model

- Peak throughput: **0.012 Gops/s** (r_chain8_semi; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain8_semi | 0.010 | 84.1% |
| r_chain8_whole | 0.003 | 22.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain8_semi | 106236ns | 106236ns | -71.74% |
| r_chain8_whole | 375875ns | 375875ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain8_whole | 374073ns | base | --- | [367285, 378504] | --- | --- | --- | --- |
| r_chain8_semi | 100427ns | -272472.9ns (-72.8%) | [-287399, -248128]ns | [91105, 120330] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain8_whole | r_chain8_semi |
|---|---|---|
| 1 | 364728ns | -63.8% |
| 2 | 372188ns | -70.9% |
| 3 | 375958ns | -73.3% |
| 4 | 379669ns | -77.8% |
| 5 | 369842ns | -72.8% |
| 6 | 377339ns | -74.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain8_semi | 0.214 | moderate+ |
| r_chain8_whole | -0.083 | ok |

**Consistency summary:**

- **r_chain8_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain8_semi | 363.8ns | 103953.8ns | 0.3% |  |
| r_chain8_whole | 179.9ns | 373287.3ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain8_semi (n=6, range 84451.7-120329.6 ns)
  84451.7 |####################
  86245.6 |
  88039.5 |
  89833.4 |
  91627.3 |
  93421.2 |
  95215.1 |
  97009.0 |####################
  98802.9 |########################################
  100596.8 |
  102390.6 |
  104184.5 |
  105978.4 |
  107772.3 |####################
  109566.2 |
  111360.1 |
  113154.0 |
  114947.9 |
  116741.8 |
  118535.7 |
  (0 below, 1 above range)

r_chain8_whole (n=6, range 364728.3-378503.7 ns)
  364728.3 |########################################
  365417.1 |
  366105.8 |
  366794.6 |
  367483.4 |
  368172.2 |
  368860.9 |
  369549.7 |########################################
  370238.5 |
  370927.2 |
  371616.0 |########################################
  372304.8 |
  372993.5 |
  373682.3 |
  374371.1 |
  375059.8 |
  375748.6 |########################################
  376437.4 |
  377126.2 |########################################
  377814.9 |
  (0 below, 1 above range)

```

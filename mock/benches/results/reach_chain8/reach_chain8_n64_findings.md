# Reachability fixpoint: whole-column vs real semi-naive (chain8)

2 variants, 6 samples per variant.
Baseline: **r_chain8_whole**

## Highlights

Baseline for all deltas below: **r_chain8_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain8_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain8_whole has the worst median (26.65 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain8_semi at 12.08 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain8_semi dominates: 121% faster than the next best (r_chain8_whole)

r_chain8_semi (12.08 us) leads r_chain8_whole (26.65 us) by 121%, a clear separation rather than a photo finish. CV 9.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain8_semi beats baseline by 56% (significant)

r_chain8_semi is -14.88 us (56%) faster than baseline r_chain8_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_chain8_semi is fastest but the noisiest (CV 9.0%)

r_chain8_semi wins on median (12.08 us) yet has the highest variance (CV 9.0%), while r_chain8_whole is the steadiest (CV 8.1%, 26.65 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: r_chain8_semi** at 12076.7 ns median (-54.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.21x (fastest 12076.7 ns, slowest 26648.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain8_semi | 14990ns | 14663ns | 13534ns | 14572ns | 16344ns | -49.87% |
| r_chain8_whole | 29904ns | 29235ns | 27863ns | 28938ns | 32374ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain8_semi | 12341ns | 11118ns | 13474ns | -54.74% | 0.005 |
| r_chain8_whole | 27270ns | 25419ns | 29524ns | base | 0.002 |

## Performance model

- Peak throughput: **0.006 Gops/s** (r_chain8_semi; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain8_semi | 0.005 | 92.1% |
| r_chain8_whole | 0.002 | 41.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain8_semi | 14990ns | 14990ns | -49.87% |
| r_chain8_whole | 29904ns | 29904ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain8_whole | 26649ns | base | --- | [25637, 29524] | --- | --- | --- | --- |
| r_chain8_semi | 12077ns | -14880.5ns (-55.8%) | [-16318, -13588]ns | [11474, 13474] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain8_whole | r_chain8_semi |
|---|---|---|
| 1 | 31898ns | -54.2% |
| 2 | 27151ns | -54.6% |
| 3 | 26833ns | -55.7% |
| 4 | 26464ns | -58.0% |
| 5 | 25856ns | -54.3% |
| 6 | 25419ns | -51.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain8_semi | 0.171 | ok |
| r_chain8_whole | 0.130 | ok |

**Consistency summary:**

- **r_chain8_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain8_semi | 185.1ns | 12341.3ns | 1.5% |  |
| r_chain8_whole | 69.1ns | 27270.2ns | 0.3% |  |

## Distribution (algo ns)

```
r_chain8_semi (n=6, range 11117.9-13473.8 ns)
  11117.9 |####################
  11235.7 |
  11353.5 |
  11471.3 |
  11589.1 |
  11706.9 |
  11824.7 |########################################
  11942.4 |
  12060.2 |
  12178.0 |####################
  12295.8 |####################
  12413.6 |
  12531.4 |
  12649.2 |
  12767.0 |
  12884.8 |
  13002.6 |
  13120.4 |
  13238.2 |
  13356.0 |
  (0 below, 1 above range)

r_chain8_whole (n=6, range 25418.7-29524.3 ns)
  25418.7 |########################################
  25624.0 |
  25829.3 |########################################
  26034.5 |
  26239.8 |
  26445.1 |########################################
  26650.4 |########################################
  26855.7 |
  27061.0 |########################################
  27266.2 |
  27471.5 |
  27676.8 |
  27882.1 |
  28087.4 |
  28292.7 |
  28497.9 |
  28703.2 |
  28908.5 |
  29113.8 |
  29319.1 |
  (0 below, 1 above range)

```

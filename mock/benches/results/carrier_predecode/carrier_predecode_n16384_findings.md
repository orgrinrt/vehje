# Predecode: zero-copy wire decode vs flat predecoded form (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_predec_wire**

## Highlights

Baseline for all deltas below: **carrier_predec_wire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_predec_wire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_predec_wire has the worst median (2.52 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_predec_flat at 2.00 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_predec_flat dominates: 26% faster than the next best (carrier_predec_wire)

carrier_predec_flat (2.00 ms) leads carrier_predec_wire (2.52 ms) by 26%, a clear separation rather than a photo finish. CV 1.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_predec_flat beats baseline by 21% (significant)

carrier_predec_flat is -523.66 us (21%) faster than baseline carrier_predec_wire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: carrier_predec_flat** at 2000248.3 ns median (-20.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.26x (fastest 2000248.3 ns, slowest 2521366.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_predec_flat | 2008701ns | 2002752ns | 1984910ns | 1999749ns | 2034024ns | -20.73% |
| carrier_predec_wire | 2533845ns | 2525179ns | 2497165ns | 2523797ns | 2567257ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_predec_flat | 2006191ns | 1982618ns | 2031344ns | -20.72% | 0.008 |
| carrier_predec_wire | 2530511ns | 2494162ns | 2564405ns | base | 0.006 |

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_predec_flat; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_predec_flat | 0.008 | 99.1% |
| carrier_predec_wire | 0.006 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_predec_flat | 2008701ns | 2008701ns | -20.73% |
| carrier_predec_wire | 2533845ns | 2533845ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_predec_wire | 2521366ns | base | --- | [2505760, 2564405] | --- | --- | --- | --- |
| carrier_predec_flat | 2000248ns | -523659.2ns (-20.8%) | [-560165, -489136]ns | [1986979, 2031344] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_predec_wire | carrier_predec_flat |
|---|---|---|
| 1 | 2557508ns | -22.1% |
| 2 | 2494162ns | -20.0% |
| 3 | 2517358ns | -21.2% |
| 4 | 2517374ns | -20.4% |
| 5 | 2571302ns | -21.6% |
| 6 | 2525359ns | -19.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_predec_flat | 0.333 | moderate+ |
| carrier_predec_wire | -0.263 | moderate- |

**Consistency summary:**

- **carrier_predec_flat**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_predec_flat | 56418.7ns | 2006190.7ns | 2.8% |  |
| carrier_predec_wire | 2426.2ns | 2530510.6ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_predec_flat (n=6, range 1982618.3-2031344.4 ns)
  1982618.3 |########################################
  1985054.6 |
  1987490.9 |
  1989927.2 |########################################
  1992363.5 |
  1994799.8 |########################################
  1997236.1 |
  1999672.4 |
  2002108.7 |
  2004545.0 |########################################
  2006981.4 |
  2009417.7 |
  2011854.0 |
  2014290.3 |
  2016726.6 |########################################
  2019162.9 |
  2021599.2 |
  2024035.5 |
  2026471.8 |
  2028908.1 |
  (0 below, 1 above range)

carrier_predec_wire (n=6, range 2494162.5-2564405.0 ns)
  2494162.5 |####################
  2497674.6 |
  2501186.8 |
  2504698.9 |
  2508211.0 |
  2511723.1 |
  2515235.2 |########################################
  2518747.4 |
  2522259.5 |####################
  2525771.6 |
  2529283.8 |
  2532795.9 |
  2536308.0 |
  2539820.1 |
  2543332.2 |
  2546844.4 |
  2550356.5 |
  2553868.6 |
  2557380.8 |####################
  2560892.9 |
  (0 below, 1 above range)

```

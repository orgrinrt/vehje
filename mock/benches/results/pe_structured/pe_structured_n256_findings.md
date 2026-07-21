# Partial-eval specialization: fold ratio on a block-structured template (reduction metric)

3 variants, 6 samples per variant.
Baseline: **pe_struct_sf50**

## Highlights

Baseline for all deltas below: **pe_struct_sf50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_struct_sf50) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_struct_sf50 has the worst median (26.92 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_struct_sf90 at 24.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: pe_struct_sf90** at 24267.3 ns median (-9.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.11x (fastest 24267.3 ns, slowest 26924.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_struct_sf50 | 28845ns | 29434ns | 26537ns | 28735ns | 30164ns | base |
| pe_struct_sf70 | 26705ns | 27562ns | 24164ns | 26766ns | 27883ns | -7.42% |
| pe_struct_sf90 | 26402ns | 26771ns | 23259ns | 26757ns | 27442ns | -8.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_struct_sf50 | 26430ns | 24378ns | 27646ns | base | 0.010 |
| pe_struct_sf70 | 24284ns | 22001ns | 25356ns | -8.12% | 0.011 |
| pe_struct_sf90 | 23924ns | 21089ns | 24892ns | -9.48% | 0.011 |

## Performance model

- Peak throughput: **0.012 Gops/s** (pe_struct_sf90; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_struct_sf50 | 0.010 | 78.3% |
| pe_struct_sf70 | 0.010 | 84.1% |
| pe_struct_sf90 | 0.011 | 86.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_struct_sf50 | 28845ns | 28845ns | base |
| pe_struct_sf70 | 26705ns | 26705ns | -7.42% |
| pe_struct_sf90 | 26402ns | 26402ns | -8.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_struct_sf50 | 26924ns | base | --- | [24721, 27646] | --- | --- | --- | --- |
| pe_struct_sf70 | 25082ns | -2239.2ns (-8.3%) | [-2360, -1840]ns | [22414, 25356] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| pe_struct_sf90 | 24267ns | -3030.4ns (-11.3%) | [-3576, -912]ns | [22614, 24892] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_struct_sf50 | pe_struct_sf70 | pe_struct_sf90 |
|---|---|---|---|
| 1 | 25065ns | -8.9% | -15.9% |
| 2 | 24378ns | -9.7% | +2.6% |
| 3 | 26590ns | -5.7% | -9.2% |
| 4 | 27258ns | -7.9% | -10.8% |
| 5 | 27338ns | -8.2% | -11.4% |
| 6 | 27955ns | -8.4% | -11.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_struct_sf50 | 0.477 | moderate+ |
| pe_struct_sf70 | 0.355 | moderate+ |
| pe_struct_sf90 | -0.233 | moderate- |

**Consistency summary:**

- **pe_struct_sf70**: won 6/6, lost 0/6
- **pe_struct_sf90**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_struct_sf50 | 4.6ns | 26430.5ns | 0.0% |  |
| pe_struct_sf70 | 4.7ns | 24284.2ns | 0.0% |  |
| pe_struct_sf90 | 4.5ns | 23924.2ns | 0.0% |  |

## Distribution (algo ns)

```
pe_struct_sf50 (n=6, range 24377.5-27646.2 ns)
  24377.5 |########################################
  24540.9 |
  24704.4 |
  24867.8 |
  25031.2 |########################################
  25194.7 |
  25358.1 |
  25521.6 |
  25685.0 |
  25848.4 |
  26011.9 |
  26175.3 |
  26338.8 |
  26502.2 |########################################
  26665.6 |
  26829.1 |
  26992.5 |
  27155.9 |########################################
  27319.4 |########################################
  27482.8 |
  (0 below, 1 above range)

pe_struct_sf70 (n=6, range 22001.2-25356.5 ns)
  22001.2 |#############
  22169.0 |
  22336.7 |
  22504.5 |
  22672.2 |#############
  22840.0 |
  23007.8 |
  23175.5 |
  23343.3 |
  23511.1 |
  23678.8 |
  23846.6 |
  24014.4 |
  24182.1 |
  24349.9 |
  24517.6 |
  24685.4 |
  24853.2 |
  25020.9 |########################################
  25188.7 |
  (0 below, 1 above range)

pe_struct_sf90 (n=6, range 21089.2-24891.7 ns)
  21089.2 |#############
  21279.3 |
  21469.4 |
  21659.6 |
  21849.7 |
  22039.8 |
  22229.9 |
  22420.1 |
  22610.2 |
  22800.3 |
  22990.4 |
  23180.5 |
  23370.7 |
  23560.8 |
  23750.9 |
  23941.0 |
  24131.2 |########################################
  24321.3 |
  24511.4 |
  24701.5 |#############
  (0 below, 1 above range)

```

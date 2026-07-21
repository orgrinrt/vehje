# Partial-eval specialization: fold ratio on a block-structured template (reduction metric)

3 variants, 6 samples per variant.
Baseline: **pe_struct_sf50**

## Highlights

Baseline for all deltas below: **pe_struct_sf50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_struct_sf50) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_struct_sf50 has the worst median (140.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_struct_sf70 at 118.53 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### pe_struct_sf70 dominates: 16% faster than the next best (pe_struct_sf90)

pe_struct_sf70 (118.53 us) leads pe_struct_sf90 (136.90 us) by 16%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

## Key findings

- **Fastest: pe_struct_sf70** at 118528.5 ns median (-15.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.18x (fastest 118528.5 ns, slowest 140318.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_struct_sf50 | 141624ns | 142634ns | 133726ns | 141648ns | 145536ns | base |
| pe_struct_sf70 | 120715ns | 120837ns | 116718ns | 120711ns | 122718ns | -14.76% |
| pe_struct_sf90 | 138866ns | 139215ns | 133465ns | 137562ns | 143522ns | -1.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_struct_sf50 | 139305ns | 131484ns | 143158ns | base | 0.007 |
| pe_struct_sf70 | 118427ns | 114525ns | 120397ns | -14.99% | 0.009 |
| pe_struct_sf90 | 136577ns | 131284ns | 141170ns | -1.96% | 0.007 |

## Performance model

- Peak throughput: **0.009 Gops/s** (pe_struct_sf70; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_struct_sf50 | 0.007 | 81.6% |
| pe_struct_sf70 | 0.009 | 96.6% |
| pe_struct_sf90 | 0.007 | 83.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_struct_sf50 | 141624ns | 141624ns | base |
| pe_struct_sf70 | 120715ns | 120715ns | -14.76% |
| pe_struct_sf90 | 138866ns | 138866ns | -1.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_struct_sf50 | 140318ns | base | --- | [134440, 143158] | --- | --- | --- | --- |
| pe_struct_sf70 | 118529ns | -21580.9ns (-15.4%) | [-25143, -15911]ns | [116355, 120397] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| pe_struct_sf90 | 136903ns | -1043.6ns (-0.7%) | [-7060, -80]ns | [131659, 141170] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_struct_sf50 | pe_struct_sf70 | pe_struct_sf90 |
|---|---|---|---|
| 1 | 139535ns | -17.9% | -5.4% |
| 2 | 131484ns | -9.8% | -0.2% |
| 3 | 137395ns | -13.8% | -0.3% |
| 4 | 143462ns | -17.6% | -4.6% |
| 5 | 141101ns | -14.6% | +0.0% |
| 6 | 142854ns | -15.8% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_struct_sf50 | 0.194 | ok |
| pe_struct_sf70 | 0.112 | ok |
| pe_struct_sf90 | 0.488 | moderate+ |

**Consistency summary:**

- **pe_struct_sf70**: won 6/6, lost 0/6
- **pe_struct_sf90**: won 5/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_struct_sf50 | 4.0ns | 139305.4ns | 0.0% |  |
| pe_struct_sf70 | 5.7ns | 118426.9ns | 0.0% |  |
| pe_struct_sf90 | 6.2ns | 136577.4ns | 0.0% |  |

## Distribution (algo ns)

```
pe_struct_sf50 (n=6, range 131483.8-143158.2 ns)
  131483.8 |########################################
  132067.5 |
  132651.2 |
  133235.0 |
  133818.7 |
  134402.4 |
  134986.1 |
  135569.8 |
  136153.5 |
  136737.3 |
  137321.0 |########################################
  137904.7 |
  138488.4 |
  139072.1 |########################################
  139655.8 |
  140239.6 |
  140823.3 |########################################
  141407.0 |
  141990.7 |
  142574.4 |########################################
  (0 below, 1 above range)

pe_struct_sf70 (n=6, range 114524.6-120396.9 ns)
  114524.6 |####################
  114818.2 |
  115111.8 |
  115405.4 |
  115699.1 |
  115992.7 |
  116286.3 |
  116579.9 |
  116873.5 |
  117167.1 |
  117460.7 |
  117754.3 |
  118048.0 |####################
  118341.6 |########################################
  118635.2 |
  118928.8 |
  119222.4 |
  119516.0 |
  119809.6 |
  120103.2 |####################
  (0 below, 1 above range)

pe_struct_sf90 (n=6, range 131284.2-141170.2 ns)
  131284.2 |####################
  131778.5 |####################
  132272.8 |
  132767.1 |
  133261.4 |
  133755.7 |
  134250.0 |
  134744.3 |
  135238.6 |
  135732.9 |
  136227.2 |
  136721.5 |########################################
  137215.8 |
  137710.1 |
  138204.4 |
  138698.7 |
  139193.0 |
  139687.3 |
  140181.6 |
  140675.9 |####################
  (0 below, 1 above range)

```

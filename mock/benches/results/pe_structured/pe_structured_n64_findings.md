# Partial-eval specialization: fold ratio on a block-structured template (reduction metric)

3 variants, 6 samples per variant.
Baseline: **pe_struct_sf50**

## Highlights

Baseline for all deltas below: **pe_struct_sf50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (pe_struct_sf50) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline pe_struct_sf50 has the worst median (7.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest pe_struct_sf90 at 6.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: pe_struct_sf90** at 6481.6 ns median (-14.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.17x (fastest 6481.6 ns, slowest 7596.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| pe_struct_sf50 | 10133ns | 10207ns | 8807ns | 10138ns | 10787ns | base |
| pe_struct_sf70 | 9618ns | 9692ns | 9065ns | 9681ns | 9802ns | -5.08% |
| pe_struct_sf90 | 9198ns | 9089ns | 8925ns | 9078ns | 9515ns | -9.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| pe_struct_sf50 | 7541ns | 6597ns | 8086ns | base | 0.008 |
| pe_struct_sf70 | 7029ns | 6643ns | 7139ns | -6.79% | 0.009 |
| pe_struct_sf90 | 6546ns | 6327ns | 6761ns | -13.19% | 0.010 |

## Performance model

- Peak throughput: **0.010 Gops/s** (pe_struct_sf90; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| pe_struct_sf50 | 0.008 | 83.3% |
| pe_struct_sf70 | 0.009 | 89.2% |
| pe_struct_sf90 | 0.010 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| pe_struct_sf50 | 10133ns | 10133ns | base |
| pe_struct_sf70 | 9618ns | 9618ns | -5.08% |
| pe_struct_sf90 | 9198ns | 9198ns | -9.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| pe_struct_sf50 | 7596ns | base | --- | [6941, 8086] | --- | --- | --- | --- |
| pe_struct_sf70 | 7091ns | no significant difference | [-994, +77]ns | [6857, 7139] | no | 0.2188 | 0.2188 | 0 |
| pe_struct_sf90 | 6482ns | -1118.6ns (-14.7%) | [-1449, -417]ns | [6396, 6761] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | pe_struct_sf50 | pe_struct_sf70 | pe_struct_sf90 |
|---|---|---|---|
| 1 | 8358ns | -15.3% | -16.9% |
| 2 | 7813ns | -9.1% | -19.0% |
| 3 | 7286ns | -8.8% | -9.8% |
| 4 | 6597ns | +7.2% | -1.9% |
| 5 | 7719ns | -7.7% | -16.2% |
| 6 | 7474ns | -4.3% | -13.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| pe_struct_sf50 | 0.123 | ok |
| pe_struct_sf70 | -0.131 | ok |
| pe_struct_sf90 | -0.383 | moderate- |

**Consistency summary:**

- **pe_struct_sf70**: won 5/6, lost 1/6
- **pe_struct_sf90**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| pe_struct_sf50 | 4.2ns | 7541.1ns | 0.1% |  |
| pe_struct_sf70 | 5.1ns | 7029.2ns | 0.1% |  |
| pe_struct_sf90 | 4.6ns | 6546.2ns | 0.1% |  |

## Distribution (algo ns)

```
pe_struct_sf50 (n=6, range 6596.7-8085.6 ns)
   6596.7 |########################################
   6671.1 |
   6745.6 |
   6820.0 |
   6894.5 |
   6968.9 |
   7043.4 |
   7117.8 |
   7192.3 |
   7266.7 |########################################
   7341.1 |
   7415.6 |########################################
   7490.0 |
   7564.5 |
   7638.9 |
   7713.4 |########################################
   7787.8 |########################################
   7862.3 |
   7936.7 |
   8011.2 |
  (0 below, 1 above range)

pe_struct_sf70 (n=6, range 6642.9-7138.9 ns)
   6642.9 |####################
   6667.7 |
   6692.5 |
   6717.3 |
   6742.1 |
   6766.9 |
   6791.7 |
   6816.5 |
   6841.3 |
   6866.1 |
   6890.9 |
   6915.7 |
   6940.5 |
   6965.3 |
   6990.1 |
   7014.9 |
   7039.7 |
   7064.5 |########################################
   7089.3 |####################
   7114.1 |####################
  (0 below, 1 above range)

pe_struct_sf90 (n=6, range 6326.7-6760.6 ns)
   6326.7 |####################
   6348.4 |
   6370.1 |
   6391.8 |
   6413.5 |
   6435.2 |
   6456.9 |########################################
   6478.6 |####################
   6500.3 |
   6522.0 |
   6543.7 |
   6565.4 |####################
   6587.1 |
   6608.8 |
   6630.5 |
   6652.2 |
   6673.9 |
   6695.6 |
   6717.3 |
   6739.0 |
  (0 below, 1 above range)

```

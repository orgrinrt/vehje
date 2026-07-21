# Closure representation: flat-capture vs linked-env

2 variants, 6 samples per variant.
Baseline: **hx_closure__linked**

## Highlights

Baseline for all deltas below: **hx_closure__linked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hx_closure__linked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hx_closure__linked has the worst median (4.61 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hx_closure__flat at 4.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (160 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: hx_closure__flat** at 4603.8 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 4603.8 ns, slowest 4606.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hx_closure__flat | 7080ns | 7184ns | 6733ns | 7042ns | 7310ns | +2.51% |
| hx_closure__linked | 6907ns | 7182ns | 5941ns | 7030ns | 7206ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hx_closure__flat | 4526ns | 4307ns | 4667ns | +2.34% | 3.620 |
| hx_closure__linked | 4423ns | 3794ns | 4613ns | base | 3.704 |

## Performance model

- Peak throughput: **4.318 Gops/s** (hx_closure__linked; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hx_closure__flat | 3.559 | 82.4% |
| hx_closure__linked | 3.557 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hx_closure__flat | 7080ns | 7080ns | +2.51% |
| hx_closure__linked | 6907ns | 6907ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hx_closure__linked | 4607ns | base | --- | [4050, 4613] | --- | --- | --- | --- |
| hx_closure__flat | 4604ns | no significant difference | [-157, +406]ns | [4309, 4667] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | hx_closure__linked | hx_closure__flat |
|---|---|---|
| 1 | 3794ns | +13.5% |
| 2 | 4610ns | -0.2% |
| 3 | 4305ns | +7.0% |
| 4 | 4604ns | +2.7% |
| 5 | 4609ns | -0.1% |
| 6 | 4616ns | -6.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hx_closure__flat | 0.027 | ok |
| hx_closure__linked | -0.166 | ok |

**Consistency summary:**

- **hx_closure__flat**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hx_closure__flat | 3.2ns | 4526.4ns | 0.1% |  |
| hx_closure__linked | 3.4ns | 4423.1ns | 0.1% |  |

## Distribution (algo ns)

```
hx_closure__flat (n=6, range 4306.7-4667.1 ns)
   4306.7 |##########################
   4324.7 |
   4342.7 |
   4360.8 |
   4378.8 |
   4396.8 |
   4414.8 |
   4432.8 |
   4450.8 |
   4468.9 |
   4486.9 |
   4504.9 |
   4522.9 |
   4540.9 |
   4558.9 |
   4577.0 |
   4595.0 |########################################
   4613.0 |
   4631.0 |
   4649.0 |
  (0 below, 1 above range)

hx_closure__linked (n=6, range 3794.2-4612.9 ns)
   3794.2 |#############
   3835.1 |
   3876.1 |
   3917.0 |
   3957.9 |
   3998.9 |
   4039.8 |
   4080.7 |
   4121.7 |
   4162.6 |
   4203.5 |
   4244.5 |
   4285.4 |#############
   4326.4 |
   4367.3 |
   4408.2 |
   4449.2 |
   4490.1 |
   4531.0 |
   4572.0 |########################################
  (0 below, 1 above range)

```

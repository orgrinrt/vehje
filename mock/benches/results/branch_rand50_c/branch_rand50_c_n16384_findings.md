# Branch strategies, cheap-arm, rand50: ~50% taken, UNPREDICTABLE (b&1)

4 variants, 6 samples per variant.
Baseline: **br_branch_c_rand50**

## Highlights

Baseline for all deltas below: **br_branch_c_rand50**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (br_branch_c_rand50) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline br_branch_c_rand50 has the worst median (64.81 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest br_predicate_c_rand50 at 30.89 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### br_predicate_c_rand50 beats baseline by 50% (significant)

br_predicate_c_rand50 is -32.45 us (50%) faster than baseline br_branch_c_rand50, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### br_branch_c_rand50 is an outlier: 2.1x slower than the field

br_branch_c_rand50 (64.81 us) is 2.1x the fastest (30.89 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {br_predicate_c_rand50, br_lut_c_rand50, br_mask_c_rand50} vs {br_branch_c_rand50} (78% apart)

The field splits into a fast tier {br_predicate_c_rand50, br_lut_c_rand50, br_mask_c_rand50} and a slow tier {br_branch_c_rand50} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: br_predicate_c_rand50** at 30894.0 ns median (-52.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.10x (fastest 30894.0 ns, slowest 64808.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 67382ns | 67130ns | 62770ns | 65838ns | 72005ns | base |
| br_lut_c_rand50 | 34811ns | 34597ns | 32317ns | 33945ns | 37359ns | -48.34% |
| br_mask_c_rand50 | 39288ns | 38671ns | 36301ns | 37919ns | 42836ns | -41.69% |
| br_predicate_c_rand50 | 33984ns | 33248ns | 32320ns | 33037ns | 36236ns | -49.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| br_branch_c_rand50 | 65034ns | 60572ns | 69515ns | base | 0.252 |
| br_lut_c_rand50 | 32402ns | 30155ns | 34666ns | -50.18% | 0.506 |
| br_mask_c_rand50 | 36912ns | 34048ns | 40254ns | -43.24% | 0.444 |
| br_predicate_c_rand50 | 31660ns | 30157ns | 33799ns | -51.32% | 0.518 |

## Performance model

- Peak throughput: **0.543 Gops/s** (br_lut_c_rand50; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| br_branch_c_rand50 | 0.253 | 46.5% |
| br_lut_c_rand50 | 0.508 | 93.5% |
| br_mask_c_rand50 | 0.451 | 83.0% |
| br_predicate_c_rand50 | 0.530 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| br_branch_c_rand50 | 67382ns | 67382ns | base |
| br_lut_c_rand50 | 34811ns | 34811ns | -48.34% |
| br_mask_c_rand50 | 39288ns | 39288ns | -41.69% |
| br_predicate_c_rand50 | 33984ns | 33984ns | -49.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| br_branch_c_rand50 | 64808ns | base | --- | [60779, 69515] | --- | --- | --- | --- |
| br_lut_c_rand50 | 32237ns | -31899.8ns (-49.2%) | [-35856, -30140]ns | [30304, 34666] | YES | 0.0313 | 0.0313 | 0 |
| br_mask_c_rand50 | 36347ns | -27566.2ns (-42.5%) | [-30580, -26218]ns | [34136, 40254] | YES | 0.0313 | 0.0313 | 0 |
| br_predicate_c_rand50 | 30894ns | -32453.9ns (-50.1%) | [-37518, -30150]ns | [30287, 33799] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | br_branch_c_rand50 | br_lut_c_rand50 | br_mask_c_rand50 | br_predicate_c_rand50 |
|---|---|---|---|---|
| 1 | 60985ns | -49.5% | -43.9% | -50.6% |
| 2 | 61515ns | -51.0% | -42.1% | -49.1% |
| 3 | 69800ns | -51.8% | -41.6% | -56.4% |
| 4 | 60572ns | -49.7% | -43.8% | -49.7% |
| 5 | 69229ns | -51.4% | -46.4% | -51.5% |
| 6 | 68101ns | -47.6% | -41.7% | -50.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| br_branch_c_rand50 | -0.301 | moderate- |
| br_lut_c_rand50 | -0.001 | ok |
| br_mask_c_rand50 | -0.315 | moderate- |
| br_predicate_c_rand50 | 0.315 | moderate+ |

**Consistency summary:**

- **br_lut_c_rand50**: won 6/6, lost 0/6
- **br_mask_c_rand50**: won 6/6, lost 0/6
- **br_predicate_c_rand50**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| br_branch_c_rand50 | 4.5ns | 65033.9ns | 0.0% |  |
| br_lut_c_rand50 | 3.9ns | 32402.2ns | 0.0% |  |
| br_mask_c_rand50 | 3.5ns | 36912.4ns | 0.0% |  |
| br_predicate_c_rand50 | 3.9ns | 31659.8ns | 0.0% |  |

## Distribution (algo ns)

```
br_branch_c_rand50 (n=6, range 60572.5-69514.6 ns)
  60572.5 |########################################
  61019.6 |
  61466.7 |####################
  61913.8 |
  62360.9 |
  62808.0 |
  63255.1 |
  63702.2 |
  64149.3 |
  64596.4 |
  65043.6 |
  65490.7 |
  65937.8 |
  66384.9 |
  66832.0 |
  67279.1 |
  67726.2 |####################
  68173.3 |
  68620.4 |
  69067.5 |####################
  (0 below, 1 above range)

br_lut_c_rand50 (n=6, range 30155.0-34666.2 ns)
  30155.0 |####################
  30380.6 |####################
  30606.1 |####################
  30831.7 |
  31057.2 |
  31282.8 |
  31508.4 |
  31733.9 |
  31959.5 |
  32185.1 |
  32410.6 |
  32636.2 |
  32861.8 |
  33087.3 |
  33312.9 |
  33538.4 |########################################
  33764.0 |
  33989.6 |
  34215.1 |
  34440.7 |
  (0 below, 1 above range)

br_mask_c_rand50 (n=6, range 34047.9-40253.9 ns)
  34047.9 |########################################
  34358.2 |
  34668.5 |
  34978.8 |
  35289.1 |
  35599.4 |####################
  35909.7 |
  36220.0 |
  36530.3 |
  36840.6 |####################
  37150.9 |
  37461.2 |
  37771.5 |
  38081.8 |
  38392.1 |
  38702.4 |
  39012.7 |
  39323.0 |
  39633.3 |####################
  39943.6 |
  (0 below, 1 above range)

br_predicate_c_rand50 (n=6, range 30156.7-33798.8 ns)
  30156.7 |####################
  30338.8 |########################################
  30520.9 |
  30703.0 |
  30885.1 |
  31067.2 |
  31249.3 |####################
  31431.4 |
  31613.5 |
  31795.6 |
  31977.7 |
  32159.8 |
  32341.9 |
  32524.0 |
  32706.1 |
  32888.2 |
  33070.3 |
  33252.4 |
  33434.5 |####################
  33616.6 |
  (0 below, 1 above range)

```

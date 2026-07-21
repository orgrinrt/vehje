# Reachability fixpoint: whole-column vs real semi-naive (chain128)

2 variants, 6 samples per variant.
Baseline: **r_chain128_whole**

## Highlights

Baseline for all deltas below: **r_chain128_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain128_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain128_whole has the worst median (6.55 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain128_semi at 1.05 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain128_semi dominates: 526% faster than the next best (r_chain128_whole)

r_chain128_semi (1.05 ms) leads r_chain128_whole (6.55 ms) by 526%, a clear separation rather than a photo finish. CV 0.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain128_semi beats baseline by 84% (significant)

r_chain128_semi is -5.51 ms (84%) faster than baseline r_chain128_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_chain128_semi shows warm-up / thermal drift (autocorr +0.58)

r_chain128_semi's per-pass series has lag-1 autocorrelation +0.58, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.3x the fastest

Fastest r_chain128_semi (1.05 ms) to slowest r_chain128_whole (6.55 ms): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: r_chain128_semi** at 1046659.6 ns median (-84.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 6.26x (fastest 1046659.6 ns, slowest 6547379.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain128_semi | 1049443ns | 1048932ns | 1038103ns | 1045778ns | 1060610ns | -83.97% |
| r_chain128_whole | 6547244ns | 6550784ns | 6519672ns | 6541239ns | 6570037ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain128_semi | 1047127ns | 1035806ns | 1058262ns | -84.00% | 0.001 |
| r_chain128_whole | 6543737ns | 6515957ns | 6566538ns | base | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (r_chain128_semi; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain128_semi | 0.001 | 99.0% |
| r_chain128_whole | 0.000 | 15.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain128_semi | 1049443ns | 1049443ns | -83.97% |
| r_chain128_whole | 6547244ns | 6547244ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain128_whole | 6547380ns | base | --- | [6517294, 6566538] | --- | --- | --- | --- |
| r_chain128_semi | 1046660ns | -5505463.1ns (-84.1%) | [-5524175, -5460193]ns | [1036459, 1058262] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain128_whole | r_chain128_semi |
|---|---|---|
| 1 | 6555176ns | -84.2% |
| 2 | 6566091ns | -84.2% |
| 3 | 6539583ns | -84.1% |
| 4 | 6566984ns | -83.9% |
| 5 | 6518631ns | -83.7% |
| 6 | 6515957ns | -83.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain128_semi | 0.579 | HIGH+ (drift/warm-up) |
| r_chain128_whole | 0.070 | ok |

**Consistency summary:**

- **r_chain128_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain128_semi | 964.5ns | 1047126.7ns | 0.1% |  |
| r_chain128_whole | 854.0ns | 6543737.0ns | 0.0% |  |

## Distribution (algo ns)

```
r_chain128_semi (n=6, range 1035806.2-1058261.9 ns)
  1035806.2 |########################################
  1036929.0 |########################################
  1038051.8 |
  1039174.6 |########################################
  1040297.3 |
  1041420.1 |
  1042542.9 |
  1043665.7 |
  1044788.5 |
  1045911.3 |
  1047034.0 |
  1048156.8 |
  1049279.6 |
  1050402.4 |
  1051525.2 |
  1052648.0 |########################################
  1053770.8 |
  1054893.5 |########################################
  1056016.3 |
  1057139.1 |
  (0 below, 1 above range)

r_chain128_whole (n=6, range 6515956.7-6566537.5 ns)
  6515956.7 |########################################
  6518485.7 |########################################
  6521014.8 |
  6523543.8 |
  6526072.9 |
  6528601.9 |
  6531130.9 |
  6533660.0 |
  6536189.0 |
  6538718.1 |########################################
  6541247.1 |
  6543776.1 |
  6546305.2 |
  6548834.2 |
  6551363.3 |
  6553892.3 |########################################
  6556421.3 |
  6558950.4 |
  6561479.4 |
  6564008.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **r_chain128_semi**: autocorrelation=0.58 (measurement drift or warm-up artifact)

# Reachability fixpoint: whole-column vs real semi-naive (randomdag)

2 variants, 6 samples per variant.
Baseline: **r_randomdag_whole**

## Highlights

Baseline for all deltas below: **r_randomdag_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_randomdag_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_randomdag_whole has the worst median (4.15 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_randomdag_semi at 1.54 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_randomdag_semi dominates: 169% faster than the next best (r_randomdag_whole)

r_randomdag_semi (1.54 ms) leads r_randomdag_whole (4.15 ms) by 169%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_randomdag_semi beats baseline by 63% (significant)

r_randomdag_semi is -2.60 ms (63%) faster than baseline r_randomdag_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_randomdag_whole shows alternating (throttle bounce) (autocorr -0.50)

r_randomdag_whole's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: r_randomdag_semi** at 1544872.8 ns median (-62.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.69x (fastest 1544872.8 ns, slowest 4148377.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_randomdag_semi | 1561320ns | 1547202ns | 1520855ns | 1541845ns | 1610765ns | -62.38% |
| r_randomdag_whole | 4150148ns | 4151604ns | 4124543ns | 4144776ns | 4171008ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_randomdag_semi | 1558950ns | 1518646ns | 1608305ns | -62.41% | 0.003 |
| r_randomdag_whole | 4146743ns | 4120806ns | 4167657ns | base | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (r_randomdag_semi; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_randomdag_semi | 0.003 | 98.3% |
| r_randomdag_whole | 0.001 | 36.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_randomdag_semi | 1561320ns | 1561320ns | -62.38% |
| r_randomdag_whole | 4150148ns | 4150148ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_randomdag_whole | 4148377ns | base | --- | [4124196, 4167657] | --- | --- | --- | --- |
| r_randomdag_semi | 1544873ns | -2596780.0ns (-62.6%) | [-2628460, -2538140]ns | [1523672, 1608305] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_randomdag_whole | r_randomdag_semi |
|---|---|---|
| 1 | 4127585ns | -62.3% |
| 2 | 4158459ns | -63.1% |
| 3 | 4149957ns | -63.4% |
| 4 | 4120806ns | -60.8% |
| 5 | 4176856ns | -61.7% |
| 6 | 4146798ns | -63.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_randomdag_semi | -0.007 | ok |
| r_randomdag_whole | -0.501 | HIGH- (thermal bounce) |

**Consistency summary:**

- **r_randomdag_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_randomdag_semi | 1656.0ns | 1558949.9ns | 0.1% |  |
| r_randomdag_whole | 731.3ns | 4146743.4ns | 0.0% |  |

## Distribution (algo ns)

```
r_randomdag_semi (n=6, range 1518645.8-1608305.2 ns)
  1518645.8 |########################################
  1523128.8 |
  1527611.7 |########################################
  1532094.7 |########################################
  1536577.7 |
  1541060.7 |
  1545543.6 |
  1550026.6 |
  1554509.6 |########################################
  1558992.5 |
  1563475.5 |
  1567958.5 |
  1572441.4 |
  1576924.4 |
  1581407.4 |
  1585890.4 |
  1590373.3 |
  1594856.3 |
  1599339.3 |########################################
  1603822.2 |
  (0 below, 1 above range)

r_randomdag_whole (n=6, range 4120806.2-4167657.2 ns)
  4120806.2 |########################################
  4123148.8 |
  4125491.3 |########################################
  4127833.9 |
  4130176.4 |
  4132519.0 |
  4134861.5 |
  4137204.1 |
  4139546.6 |
  4141889.2 |
  4144231.7 |
  4146574.3 |########################################
  4148916.8 |########################################
  4151259.4 |
  4153601.9 |
  4155944.5 |
  4158287.0 |########################################
  4160629.6 |
  4162972.1 |
  4165314.7 |
  (0 below, 1 above range)

```

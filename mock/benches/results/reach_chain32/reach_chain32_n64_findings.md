# Reachability fixpoint: whole-column vs real semi-naive (chain32)

2 variants, 6 samples per variant.
Baseline: **r_chain32_whole**

## Highlights

Baseline for all deltas below: **r_chain32_whole**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (r_chain32_whole) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline r_chain32_whole has the worst median (48.92 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest r_chain32_semi at 19.38 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### r_chain32_semi dominates: 152% faster than the next best (r_chain32_whole)

r_chain32_semi (19.38 us) leads r_chain32_whole (48.92 us) by 152%, a clear separation rather than a photo finish. CV 9.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### r_chain32_semi beats baseline by 59% (significant)

r_chain32_semi is -28.88 us (59%) faster than baseline r_chain32_whole, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### r_chain32_semi is fastest but the noisiest (CV 9.2%)

r_chain32_semi wins on median (19.38 us) yet has the highest variance (CV 9.2%), while r_chain32_whole is the steadiest (CV 3.4%, 48.92 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: r_chain32_semi** at 19381.5 ns median (-60.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.52x (fastest 19381.5 ns, slowest 48917.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| r_chain32_semi | 22631ns | 21675ns | 20832ns | 21610ns | 25062ns | -56.25% |
| r_chain32_whole | 51728ns | 51376ns | 49361ns | 51149ns | 53779ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| r_chain32_semi | 20284ns | 18623ns | 22543ns | -58.88% | 0.003 |
| r_chain32_whole | 49325ns | 47120ns | 51273ns | base | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (r_chain32_semi; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| r_chain32_semi | 0.003 | 96.1% |
| r_chain32_whole | 0.001 | 38.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| r_chain32_semi | 22631ns | 22631ns | -56.25% |
| r_chain32_whole | 51728ns | 51728ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| r_chain32_whole | 48918ns | base | --- | [47785, 51273] | --- | --- | --- | --- |
| r_chain32_semi | 19381ns | -28884.8ns (-59.0%) | [-30020, -28219]ns | [18927, 22543] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | r_chain32_whole | r_chain32_semi |
|---|---|---|
| 1 | 48450ns | -60.3% |
| 2 | 48653ns | -60.3% |
| 3 | 47120ns | -60.5% |
| 4 | 50151ns | -61.2% |
| 5 | 49182ns | -56.8% |
| 6 | 52396ns | -54.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| r_chain32_semi | 0.347 | moderate+ |
| r_chain32_whole | -0.019 | ok |

**Consistency summary:**

- **r_chain32_semi**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| r_chain32_semi | 164.2ns | 20283.7ns | 0.8% |  |
| r_chain32_whole | 61.1ns | 49325.3ns | 0.1% |  |

## Distribution (algo ns)

```
r_chain32_semi (n=6, range 18623.3-22542.7 ns)
  18623.3 |####################
  18819.3 |
  19015.2 |
  19211.2 |########################################
  19407.2 |####################
  19603.1 |
  19799.1 |
  19995.1 |
  20191.1 |
  20387.0 |
  20583.0 |
  20779.0 |
  20974.9 |
  21170.9 |####################
  21366.9 |
  21562.8 |
  21758.8 |
  21954.8 |
  22150.8 |
  22346.7 |
  (0 below, 1 above range)

r_chain32_whole (n=6, range 47119.6-51273.3 ns)
  47119.6 |########################################
  47327.3 |
  47535.0 |
  47742.7 |
  47950.3 |
  48158.0 |
  48365.7 |########################################
  48573.4 |########################################
  48781.1 |
  48988.8 |########################################
  49196.4 |
  49404.1 |
  49611.8 |
  49819.5 |
  50027.2 |########################################
  50234.9 |
  50442.6 |
  50650.2 |
  50857.9 |
  51065.6 |
  (0 below, 1 above range)

```

# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, scatter profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (2.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_stencil at 577 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 74% (significant)

carrier_nat_scatter_copypatch is -1.70 us (74%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 4.0x slower than the field

carrier_nat_scatter_interp (2.28 us) is 4.0x the fastest (577 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_scatter_stencil is fastest but the noisiest (CV 38.7%)

carrier_nat_scatter_stencil wins on median (577 ns) yet has the highest variance (CV 38.7%), while carrier_nat_scatter_copypatch is the steadiest (CV 2.6%, 593 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.0x the fastest

Fastest carrier_nat_scatter_stencil (577 ns) to slowest carrier_nat_scatter_interp (2.28 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader carrier_nat_scatter_stencil vs stability leader carrier_nat_scatter_copypatch (+3% speed for 15.1x steadier)

carrier_nat_scatter_stencil is fastest (577 ns, CV 38.7%); carrier_nat_scatter_copypatch gives up 2.9% median for 15.1x lower variance (CV 2.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### carrier_nat_scatter_stencil is inconsistent: worst-20% is 1.7x its best-20%

carrier_nat_scatter_stencil's best 20% of batches run at 554 ns but its worst 20% at 934 ns (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_nat_scatter_stencil** at 576.6 ns median (-74.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.96x (fastest 576.6 ns, slowest 2281.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 2939ns | 2921ns | 2835ns | 2893ns | 3062ns | -39.24% |
| carrier_nat_scatter_interp | 4838ns | 4516ns | 4437ns | 4493ns | 5555ns | base |
| carrier_nat_scatter_stencil | 3181ns | 2870ns | 2755ns | 2845ns | 3897ns | -34.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 590ns | 566ns | 606ns | -75.81% | 0.109 |
| carrier_nat_scatter_interp | 2438ns | 2248ns | 2783ns | base | 0.026 |
| carrier_nat_scatter_stencil | 689ns | 554ns | 934ns | -71.75% | 0.093 |

## Performance model

- Peak throughput: **0.116 Gops/s** (carrier_nat_scatter_stencil; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.108 | 93.3% |
| carrier_nat_scatter_interp | 0.028 | 24.3% |
| carrier_nat_scatter_stencil | 0.111 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 2939ns | 2939ns | -39.24% |
| carrier_nat_scatter_interp | 4838ns | 4838ns | base |
| carrier_nat_scatter_stencil | 3181ns | 3181ns | -34.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 2282ns | base | --- | [2249, 2783] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 593ns | -1697.2ns (-74.4%) | [-2181, -1666]ns | [570, 606] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_stencil | 577ns | -1692.8ns (-74.2%) | [-1899, -1656]ns | [555, 934] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_stencil |
|---|---|---|---|
| 1 | 2248ns | -73.7% | -74.3% |
| 2 | 2299ns | -73.8% | -75.8% |
| 3 | 2250ns | -74.5% | -75.4% |
| 4 | 2820ns | -78.4% | -58.3% |
| 5 | 2746ns | -78.3% | -74.9% |
| 6 | 2264ns | -75.0% | -74.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | -0.372 | moderate- |
| carrier_nat_scatter_interp | 0.125 | ok |
| carrier_nat_scatter_stencil | -0.110 | ok |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 68976.5ns | 589.6ns | 11698.2% | HIGH |
| carrier_nat_scatter_interp | 88494.3ns | 2437.7ns | 3630.2% | HIGH |
| carrier_nat_scatter_stencil | 68948.1ns | 688.6ns | 10012.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 565.8-605.8 ns)
    565.8 |########################################
    567.8 |
    569.8 |
    571.8 |
    573.8 |########################################
    575.8 |
    577.8 |
    579.8 |
    581.8 |
    583.8 |
    585.8 |
    587.8 |
    589.8 |########################################
    591.8 |
    593.8 |########################################
    595.8 |
    597.8 |
    599.8 |
    601.8 |########################################
    603.8 |
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 2247.5-2782.9 ns)
   2247.5 |########################################
   2274.3 |#############
   2301.0 |
   2327.8 |
   2354.6 |
   2381.3 |
   2408.1 |
   2434.9 |
   2461.7 |
   2488.4 |
   2515.2 |
   2542.0 |
   2568.7 |
   2595.5 |
   2622.3 |
   2649.0 |
   2675.8 |
   2702.6 |
   2729.4 |#############
   2756.1 |
  (0 below, 1 above range)

carrier_nat_scatter_stencil (n=6, range 553.7-933.8 ns)
    553.7 |########################################
    572.7 |########################################
    591.7 |
    610.7 |
    629.7 |
    648.7 |
    667.7 |
    686.7 |####################
    705.7 |
    724.7 |
    743.7 |
    762.7 |
    781.7 |
    800.7 |
    819.7 |
    838.7 |
    857.7 |
    876.7 |
    895.7 |
    914.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=11715.8% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=3789.1% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_stencil**: CV=32.4% (high variance, measurements may be unstable)
- **carrier_nat_scatter_stencil**: bridge=11786.2% of algo (FFI overhead may distort results)

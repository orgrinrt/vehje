# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, real profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (2.34 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_copypatch at 549 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_copypatch dominates: 11% faster than the next best (carrier_nat_real_stencil)

carrier_nat_real_copypatch (549 ns) leads carrier_nat_real_stencil (608 ns) by 11%, a clear separation rather than a photo finish. CV 72.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_nat_real_stencil beats baseline by 74% (significant)

carrier_nat_real_stencil is -1.72 us (74%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 4.3x slower than the field

carrier_nat_real_interp (2.34 us) is 4.3x the fastest (549 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_real_copypatch is fastest but the noisiest (CV 72.3%)

carrier_nat_real_copypatch wins on median (549 ns) yet has the highest variance (CV 72.3%), while carrier_nat_real_interp is the steadiest (CV 7.9%, 2.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.3x the fastest

Fastest carrier_nat_real_copypatch (549 ns) to slowest carrier_nat_real_interp (2.34 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### carrier_nat_real_copypatch is inconsistent: worst-20% is 2.0x its best-20%

carrier_nat_real_copypatch's best 20% of batches run at 542 ns but its worst 20% at 1.10 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: carrier_nat_real_copypatch** at 549.4 ns median (-76.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.26x (fastest 549.4 ns, slowest 2339.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 3491ns | 2806ns | 2746ns | 2799ns | 4900ns | -29.04% |
| carrier_nat_real_interp | 4919ns | 4700ns | 4458ns | 4658ns | 5541ns | base |
| carrier_nat_real_stencil | 3039ns | 2994ns | 2716ns | 2927ns | 3368ns | -38.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 731ns | 542ns | 1101ns | -69.52% | 0.088 |
| carrier_nat_real_interp | 2398ns | 2230ns | 2613ns | base | 0.027 |
| carrier_nat_real_stencil | 607ns | 542ns | 669ns | -74.68% | 0.105 |

## Performance model

- Peak throughput: **0.118 Gops/s** (carrier_nat_real_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.117 | 98.6% |
| carrier_nat_real_interp | 0.027 | 23.2% |
| carrier_nat_real_stencil | 0.105 | 89.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 3491ns | 3491ns | -29.04% |
| carrier_nat_real_interp | 4919ns | 4919ns | base |
| carrier_nat_real_stencil | 3039ns | 3039ns | -38.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 2340ns | base | --- | [2241, 2613] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 549ns | -1715.6ns (-73.3%) | [-1874, -1412]ns | [542, 1101] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_stencil | 608ns | -1720.8ns (-73.6%) | [-2004, -1646]ns | [545, 669] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_stencil |
|---|---|---|---|
| 1 | 2385ns | -77.0% | -72.6% |
| 2 | 2252ns | -74.1% | -75.9% |
| 3 | 2230ns | -75.3% | -75.5% |
| 4 | 2294ns | -76.4% | -70.2% |
| 5 | 2773ns | -41.6% | -77.8% |
| 6 | 2452ns | -77.9% | -75.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | -0.261 | moderate- |
| carrier_nat_real_interp | 0.123 | ok |
| carrier_nat_real_stencil | -0.197 | ok |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 79570.6ns | 730.8ns | 10887.9% | HIGH |
| carrier_nat_real_interp | 88127.8ns | 2397.8ns | 3675.4% | HIGH |
| carrier_nat_real_stencil | 70887.3ns | 607.2ns | 11673.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 541.7-1100.8 ns)
    541.7 |########################################
    569.7 |##########
    597.6 |
    625.6 |
    653.5 |
    681.5 |
    709.4 |
    737.4 |
    765.3 |
    793.3 |
    821.2 |
    849.2 |
    877.2 |
    905.1 |
    933.1 |
    961.0 |
    989.0 |
   1016.9 |
   1044.9 |
   1072.8 |
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 2230.4-2612.7 ns)
   2230.4 |########################################
   2249.5 |########################################
   2268.6 |
   2287.7 |########################################
   2306.9 |
   2326.0 |
   2345.1 |
   2364.2 |
   2383.3 |########################################
   2402.4 |
   2421.6 |
   2440.7 |########################################
   2459.8 |
   2478.9 |
   2498.0 |
   2517.1 |
   2536.2 |
   2555.4 |
   2574.5 |
   2593.6 |
  (0 below, 1 above range)

carrier_nat_real_stencil (n=6, range 542.1-668.8 ns)
    542.1 |########################################
    548.4 |
    554.8 |
    561.1 |
    567.4 |
    573.8 |
    580.1 |
    586.4 |
    592.8 |
    599.1 |####################
    605.4 |
    611.8 |####################
    618.1 |
    624.4 |
    630.8 |
    637.1 |
    643.4 |
    649.8 |####################
    656.1 |
    662.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: CV=54.3% (high variance, measurements may be unstable)
- **carrier_nat_real_copypatch**: bridge=12398.1% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=3708.7% of algo (FFI overhead may distort results)
- **carrier_nat_real_stencil**: bridge=11765.6% of algo (FFI overhead may distort results)

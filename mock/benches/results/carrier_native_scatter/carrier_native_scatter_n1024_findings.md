# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (42.71 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_copypatch at 14.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 66% (significant)

carrier_nat_scatter_copypatch is -28.23 us (66%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 2.9x slower than the field

carrier_nat_scatter_interp (42.71 us) is 2.9x the fastest (14.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_nat_scatter_copypatch** at 14546.7 ns median (-65.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.94x (fastest 14546.7 ns, slowest 42706.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 16850ns | 16785ns | 16249ns | 16691ns | 17388ns | -62.93% |
| carrier_nat_scatter_direct | 17070ns | 16972ns | 16408ns | 16801ns | 17806ns | -62.45% |
| carrier_nat_scatter_interp | 45458ns | 45110ns | 43629ns | 44634ns | 47609ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 14571ns | 14101ns | 14975ns | -66.20% | 0.070 |
| carrier_nat_scatter_direct | 14799ns | 14226ns | 15428ns | -65.67% | 0.069 |
| carrier_nat_scatter_interp | 43110ns | 41465ns | 45136ns | base | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 378953 | 1122852 | 0.337 | 0.85× |
| carrier_nat_scatter_direct | 335630 | 1013232 | 0.331 | 0.75× |
| carrier_nat_scatter_interp | 446058 | 1991882 | 0.224 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.073 Gops/s** (carrier_nat_scatter_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.070 | 96.9% |
| carrier_nat_scatter_direct | 0.070 | 95.8% |
| carrier_nat_scatter_interp | 0.024 | 33.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 16850ns | 16850ns | -62.93% |
| carrier_nat_scatter_direct | 17070ns | 17070ns | -62.45% |
| carrier_nat_scatter_interp | 45458ns | 45458ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 42706ns | base | --- | [41489, 45136] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 14547ns | -28229.6ns (-66.1%) | [-30232, -27156]ns | [14191, 14975] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_direct | 14719ns | -27969.6ns (-65.5%) | [-29765, -27200]ns | [14250, 15428] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_direct |
|---|---|---|---|
| 1 | 42820ns | -67.1% | -66.8% |
| 2 | 43981ns | -65.7% | -64.8% |
| 3 | 41465ns | -65.3% | -65.6% |
| 4 | 41512ns | -65.6% | -65.5% |
| 5 | 46290ns | -68.2% | -66.8% |
| 6 | 42592ns | -65.1% | -64.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | -0.410 | moderate- |
| carrier_nat_scatter_direct | -0.340 | moderate- |
| carrier_nat_scatter_interp | -0.351 | moderate- |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 106956.1ns | 14570.9ns | 734.0% | HIGH |
| carrier_nat_scatter_direct | 94186.3ns | 14798.6ns | 636.5% | HIGH |
| carrier_nat_scatter_interp | 102753.5ns | 43110.1ns | 238.4% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 14101.2-14975.0 ns)
  14101.2 |########################################
  14144.9 |
  14188.6 |
  14232.3 |
  14276.0 |########################################
  14319.7 |
  14363.3 |########################################
  14407.0 |
  14450.7 |
  14494.4 |
  14538.1 |
  14581.8 |
  14625.5 |
  14669.2 |########################################
  14712.9 |
  14756.5 |
  14800.2 |
  14843.9 |########################################
  14887.6 |
  14931.3 |
  (0 below, 1 above range)

carrier_nat_scatter_direct (n=6, range 14225.8-15427.5 ns)
  14225.8 |########################################
  14285.9 |####################
  14346.0 |
  14406.1 |
  14466.1 |
  14526.2 |
  14586.3 |
  14646.4 |
  14706.5 |
  14766.6 |
  14826.6 |
  14886.7 |
  14946.8 |
  15006.9 |
  15067.0 |
  15127.1 |####################
  15187.2 |
  15247.2 |
  15307.3 |####################
  15367.4 |
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 41465.0-45135.8 ns)
  41465.0 |########################################
  41648.5 |
  41832.1 |
  42015.6 |
  42199.2 |
  42382.7 |
  42566.2 |####################
  42749.8 |####################
  42933.3 |
  43116.9 |
  43300.4 |
  43483.9 |
  43667.5 |
  43851.0 |####################
  44034.6 |
  44218.1 |
  44401.6 |
  44585.2 |
  44768.7 |
  44952.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=728.6% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_direct**: bridge=634.6% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=220.3% of algo (FFI overhead may distort results)

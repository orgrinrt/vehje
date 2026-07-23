# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_scatter_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_scatter_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_scatter_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_scatter_scalar has the worst median (16.18 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_scatter_vert8 at 6.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_scatter_vert8 dominates: 11% faster than the next best (carrier_vert_scatter_vert4)

carrier_vert_scatter_vert8 (6.66 us) leads carrier_vert_scatter_vert4 (7.38 us) by 11%, a clear separation rather than a photo finish. CV 4.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_scatter_vert8 beats baseline by 58% (significant)

carrier_vert_scatter_vert8 is -9.40 us (58%) faster than baseline carrier_vert_scatter_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_scatter_scalar is an outlier: 2.4x slower than the field

carrier_vert_scatter_scalar (16.18 us) is 2.4x the fastest (6.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: carrier_vert_scatter_vert8** at 6664.6 ns median (-58.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.43x (fastest 6664.6 ns, slowest 16185.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 18492ns | 18445ns | 17859ns | 18287ns | 19115ns | base |
| carrier_vert_scatter_vert4 | 9770ns | 9720ns | 9342ns | 9648ns | 10167ns | -47.17% |
| carrier_vert_scatter_vert8 | 9076ns | 9055ns | 8601ns | 8927ns | 9537ns | -50.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 16218ns | 15623ns | 16764ns | base | 0.004 |
| carrier_vert_scatter_vert4 | 7447ns | 7163ns | 7749ns | -54.08% | 0.009 |
| carrier_vert_scatter_vert8 | 6762ns | 6459ns | 7136ns | -58.31% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 334747 | 1123720 | 0.298 | 1.00× |
| carrier_vert_scatter_vert4 | 297347 | 820595 | 0.362 | 0.89× |
| carrier_vert_scatter_vert8 | 299666 | 632676 | 0.474 | 0.90× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_scatter_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_scatter_scalar | 0.004 | 39.9% |
| carrier_vert_scatter_vert4 | 0.009 | 87.5% |
| carrier_vert_scatter_vert8 | 0.010 | 96.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_scatter_scalar | 18492ns | 18492ns | base |
| carrier_vert_scatter_vert4 | 9770ns | 9770ns | -47.17% |
| carrier_vert_scatter_vert8 | 9076ns | 9076ns | -50.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_scatter_scalar | 16185ns | base | --- | [15706, 16764] | --- | --- | --- | --- |
| carrier_vert_scatter_vert4 | 7384ns | -8703.5ns (-53.8%) | [-9220, -8390]ns | [7209, 7749] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_scatter_vert8 | 6665ns | -9398.8ns (-58.1%) | [-9768, -9203]ns | [6485, 7136] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_scatter_scalar | carrier_vert_scatter_vert4 | carrier_vert_scatter_vert8 |
|---|---|---|---|
| 1 | 15623ns | -54.1% | -58.7% |
| 2 | 16658ns | -53.3% | -57.3% |
| 3 | 16871ns | -56.3% | -57.6% |
| 4 | 16033ns | -51.9% | -57.6% |
| 5 | 16337ns | -54.7% | -60.1% |
| 6 | 15789ns | -54.0% | -58.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_scatter_scalar | -0.140 | ok |
| carrier_vert_scatter_vert4 | -0.477 | moderate- |
| carrier_vert_scatter_vert8 | 0.193 | ok |

**Consistency summary:**

- **carrier_vert_scatter_vert4**: won 6/6, lost 0/6
- **carrier_vert_scatter_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_scatter_scalar | 93383.7ns | 16218.5ns | 575.8% | HIGH |
| carrier_vert_scatter_vert4 | 89032.3ns | 7447.3ns | 1195.5% | HIGH |
| carrier_vert_scatter_vert8 | 89539.2ns | 6761.8ns | 1324.2% | HIGH |

## Distribution (algo ns)

```
carrier_vert_scatter_scalar (n=6, range 15623.3-16764.3 ns)
  15623.3 |########################################
  15680.4 |
  15737.4 |########################################
  15794.5 |
  15851.5 |
  15908.6 |
  15965.6 |
  16022.7 |########################################
  16079.7 |
  16136.8 |
  16193.8 |
  16250.9 |
  16307.9 |########################################
  16365.0 |
  16422.0 |
  16479.1 |
  16536.1 |
  16593.2 |
  16650.2 |########################################
  16707.3 |
  (0 below, 1 above range)

carrier_vert_scatter_vert4 (n=6, range 7163.3-7748.9 ns)
   7163.3 |########################################
   7192.6 |
   7221.9 |
   7251.1 |########################################
   7280.4 |
   7309.7 |
   7339.0 |########################################
   7368.3 |
   7397.6 |########################################
   7426.8 |
   7456.1 |
   7485.4 |
   7514.7 |
   7544.0 |
   7573.3 |
   7602.5 |
   7631.8 |
   7661.1 |
   7690.4 |########################################
   7719.7 |
  (0 below, 1 above range)

carrier_vert_scatter_vert8 (n=6, range 6458.7-7136.1 ns)
   6458.7 |########################################
   6492.6 |########################################
   6526.4 |########################################
   6560.3 |
   6594.2 |
   6628.0 |
   6661.9 |
   6695.8 |
   6729.6 |
   6763.5 |########################################
   6797.4 |
   6831.2 |
   6865.1 |
   6899.0 |
   6932.8 |
   6966.7 |
   7000.6 |
   7034.4 |
   7068.3 |
   7102.2 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_scatter_scalar**: bridge=578.7% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert4**: bridge=1203.9% of algo (FFI overhead may distort results)
- **carrier_vert_scatter_vert8**: bridge=1340.3% of algo (FFI overhead may distort results)

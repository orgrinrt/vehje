# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), real profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_real_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_real_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_real_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_real_scalar has the worst median (15.69 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_real_vert8 at 6.68 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_real_vert8 beats baseline by 57% (significant)

carrier_vert_real_vert8 is -8.98 us (57%) faster than baseline carrier_vert_real_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_real_scalar is an outlier: 2.3x slower than the field

carrier_vert_real_scalar (15.69 us) is 2.3x the fastest (6.68 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_real_scalar shows warm-up / thermal drift (autocorr +0.53)

carrier_vert_real_scalar's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_real_vert8** at 6676.5 ns median (-57.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.35x (fastest 6676.5 ns, slowest 15686.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 17947ns | 17976ns | 17634ns | 17944ns | 18108ns | base |
| carrier_vert_real_vert4 | 9516ns | 9416ns | 9291ns | 9395ns | 9808ns | -46.98% |
| carrier_vert_real_vert8 | 8982ns | 8982ns | 8797ns | 8938ns | 9140ns | -49.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_real_scalar | 15664ns | 15412ns | 15844ns | base | 0.004 |
| carrier_vert_real_vert4 | 7202ns | 7102ns | 7334ns | -54.02% | 0.009 |
| carrier_vert_real_vert8 | 6660ns | 6543ns | 6739ns | -57.48% | 0.010 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_real_scalar | 340573 | 1156039 | 0.295 | 1.00× |
| carrier_vert_real_vert4 | 306346 | 843098 | 0.363 | 0.90× |
| carrier_vert_real_vert8 | 300474 | 631009 | 0.476 | 0.88× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.010 Gops/s** (carrier_vert_real_vert8; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_real_scalar | 0.004 | 41.7% |
| carrier_vert_real_vert4 | 0.009 | 91.4% |
| carrier_vert_real_vert8 | 0.010 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_real_scalar | 17947ns | 17947ns | base |
| carrier_vert_real_vert4 | 9516ns | 9516ns | -46.98% |
| carrier_vert_real_vert8 | 8982ns | 8982ns | -49.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_real_scalar | 15686ns | base | --- | [15463, 15844] | --- | --- | --- | --- |
| carrier_vert_real_vert4 | 7162ns | -8434.0ns (-53.8%) | [-8644, -8308]ns | [7111, 7334] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_real_vert8 | 6676ns | -8982.6ns (-57.3%) | [-9155, -8875]ns | [6565, 6739] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_real_scalar | carrier_vert_real_vert4 | carrier_vert_real_vert8 |
|---|---|---|---|
| 1 | 15733ns | -54.2% | -57.5% |
| 2 | 15880ns | -55.1% | -58.1% |
| 3 | 15808ns | -53.6% | -57.5% |
| 4 | 15640ns | -53.1% | -56.8% |
| 5 | 15514ns | -54.1% | -57.5% |
| 6 | 15412ns | -53.9% | -57.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_real_scalar | 0.529 | HIGH+ (drift/warm-up) |
| carrier_vert_real_vert4 | 0.074 | ok |
| carrier_vert_real_vert8 | 0.212 | moderate+ |

**Consistency summary:**

- **carrier_vert_real_vert4**: won 6/6, lost 0/6
- **carrier_vert_real_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_real_scalar | 94174.0ns | 15664.3ns | 601.2% | HIGH |
| carrier_vert_real_vert4 | 89817.2ns | 7202.5ns | 1247.0% | HIGH |
| carrier_vert_real_vert8 | 89115.2ns | 6660.2ns | 1338.0% | HIGH |

## Distribution (algo ns)

```
carrier_vert_real_scalar (n=6, range 15412.1-15843.8 ns)
  15412.1 |########################################
  15433.7 |
  15455.3 |
  15476.8 |
  15498.4 |########################################
  15520.0 |
  15541.6 |
  15563.2 |
  15584.8 |
  15606.3 |
  15627.9 |########################################
  15649.5 |
  15671.1 |
  15692.7 |
  15714.3 |########################################
  15735.8 |
  15757.4 |
  15779.0 |
  15800.6 |########################################
  15822.2 |
  (0 below, 1 above range)

carrier_vert_real_vert4 (n=6, range 7102.5-7333.5 ns)
   7102.5 |####################
   7114.1 |########################################
   7125.6 |
   7137.2 |
   7148.7 |
   7160.3 |
   7171.8 |
   7183.4 |
   7194.9 |####################
   7206.5 |
   7218.0 |
   7229.6 |
   7241.1 |
   7252.7 |
   7264.2 |
   7275.8 |
   7287.3 |
   7298.9 |
   7310.4 |
   7322.0 |####################
  (0 below, 1 above range)

carrier_vert_real_vert8 (n=6, range 6543.3-6738.6 ns)
   6543.3 |########################################
   6553.1 |
   6562.8 |
   6572.6 |
   6582.4 |########################################
   6592.1 |
   6601.9 |
   6611.6 |
   6621.4 |
   6631.2 |
   6640.9 |
   6650.7 |########################################
   6660.4 |
   6670.2 |
   6680.0 |
   6689.7 |########################################
   6699.5 |
   6709.3 |########################################
   6719.0 |
   6728.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_real_scalar**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **carrier_vert_real_scalar**: bridge=600.2% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert4**: bridge=1259.3% of algo (FFI overhead may distort results)
- **carrier_vert_real_vert8**: bridge=1330.4% of algo (FFI overhead may distort results)

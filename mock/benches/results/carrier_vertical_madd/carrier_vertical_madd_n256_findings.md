# Vertical/SoA SIMD, 8 inputs per call for every cell (scalar 8x1 vs vert4 2x4 vs vert8 1x8; directly comparable), madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_vert_madd_scalar**

## Highlights

Baseline for all deltas below: **carrier_vert_madd_scalar**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_vert_madd_scalar) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_vert_madd_scalar has the worst median (86.62 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_vert_madd_vert8 at 35.14 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_vert_madd_vert8 dominates: 44% faster than the next best (carrier_vert_madd_vert4)

carrier_vert_madd_vert8 (35.14 us) leads carrier_vert_madd_vert4 (50.67 us) by 44%, a clear separation rather than a photo finish. CV 2.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_vert_madd_vert8 beats baseline by 60% (significant)

carrier_vert_madd_vert8 is -51.90 us (60%) faster than baseline carrier_vert_madd_scalar, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_vert_madd_scalar is an outlier: 2.5x slower than the field

carrier_vert_madd_scalar (86.62 us) is 2.5x the fastest (35.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_vert_madd_scalar shows alternating (throttle bounce) (autocorr -0.68)

carrier_vert_madd_scalar's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_vert_madd_vert8** at 35141.7 ns median (-59.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.46x (fastest 35141.7 ns, slowest 86616.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 89140ns | 88938ns | 87035ns | 88654ns | 90921ns | base |
| carrier_vert_madd_vert4 | 52701ns | 53242ns | 50971ns | 52642ns | 53654ns | -40.88% |
| carrier_vert_madd_vert8 | 37487ns | 37543ns | 36017ns | 37189ns | 38670ns | -57.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 86772ns | 84845ns | 88470ns | base | 0.003 |
| carrier_vert_madd_vert4 | 50311ns | 48780ns | 51314ns | -42.02% | 0.005 |
| carrier_vert_madd_vert8 | 35152ns | 33864ns | 36307ns | -59.49% | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 640074 | 1592578 | 0.402 | 1.00× |
| carrier_vert_madd_vert4 | 467548 | 826144 | 0.566 | 0.73× |
| carrier_vert_madd_vert8 | 436273 | 798808 | 0.546 | 0.68× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.008 Gops/s** (carrier_vert_madd_vert8; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_vert_madd_scalar | 0.003 | 39.1% |
| carrier_vert_madd_vert4 | 0.005 | 66.8% |
| carrier_vert_madd_vert8 | 0.007 | 96.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_vert_madd_scalar | 89140ns | 89140ns | base |
| carrier_vert_madd_vert4 | 52701ns | 52701ns | -40.88% |
| carrier_vert_madd_vert8 | 37487ns | 37487ns | -57.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_vert_madd_scalar | 86616ns | base | --- | [85229, 88470] | --- | --- | --- | --- |
| carrier_vert_madd_vert4 | 50671ns | -35958.8ns (-41.5%) | [-37969, -35455]ns | [48947, 51314] | YES | 0.0313 | 0.0313 | 0 |
| carrier_vert_madd_vert8 | 35142ns | -51904.4ns (-59.9%) | [-52667, -50287]ns | [34008, 36307] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_vert_madd_scalar | carrier_vert_madd_vert4 | carrier_vert_madd_vert8 |
|---|---|---|---|
| 1 | 85614ns | -41.1% | -60.1% |
| 2 | 89822ns | -42.8% | -58.9% |
| 3 | 84845ns | -42.1% | -57.9% |
| 4 | 87117ns | -41.5% | -59.4% |
| 5 | 86975ns | -41.1% | -59.9% |
| 6 | 86257ns | -43.4% | -60.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_vert_madd_scalar | -0.684 | HIGH- (thermal bounce) |
| carrier_vert_madd_vert4 | -0.447 | moderate- |
| carrier_vert_madd_vert8 | -0.049 | ok |

**Consistency summary:**

- **carrier_vert_madd_vert4**: won 6/6, lost 0/6
- **carrier_vert_madd_vert8**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_vert_madd_scalar | 119933.5ns | 86771.7ns | 138.2% | HIGH |
| carrier_vert_madd_vert4 | 100591.5ns | 50310.6ns | 199.9% | HIGH |
| carrier_vert_madd_vert8 | 105381.6ns | 35152.3ns | 299.8% | HIGH |

## Distribution (algo ns)

```
carrier_vert_madd_scalar (n=6, range 84844.6-88469.6 ns)
  84844.6 |########################################
  85025.9 |
  85207.1 |
  85388.4 |
  85569.6 |########################################
  85750.9 |
  85932.1 |
  86113.4 |########################################
  86294.6 |
  86475.9 |
  86657.1 |
  86838.4 |########################################
  87019.6 |########################################
  87200.9 |
  87382.1 |
  87563.4 |
  87744.6 |
  87925.9 |
  88107.1 |
  88288.4 |
  (0 below, 1 above range)

carrier_vert_madd_vert4 (n=6, range 48780.4-51313.8 ns)
  48780.4 |########################################
  48907.1 |
  49033.7 |########################################
  49160.4 |
  49287.1 |
  49413.7 |
  49540.4 |
  49667.1 |
  49793.7 |
  49920.4 |
  50047.1 |
  50173.7 |
  50300.4 |########################################
  50427.1 |
  50553.7 |
  50680.4 |
  50807.1 |########################################
  50933.7 |
  51060.4 |
  51187.1 |########################################
  (0 below, 1 above range)

carrier_vert_madd_vert8 (n=6, range 33864.2-36307.1 ns)
  33864.2 |########################################
  33986.3 |
  34108.5 |########################################
  34230.6 |
  34352.8 |
  34474.9 |
  34597.1 |
  34719.2 |
  34841.3 |########################################
  34963.5 |
  35085.6 |
  35207.8 |
  35329.9 |########################################
  35452.1 |
  35574.2 |
  35696.3 |########################################
  35818.5 |
  35940.6 |
  36062.8 |
  36184.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_vert_madd_scalar**: bridge=134.7% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert4**: bridge=199.3% of algo (FFI overhead may distort results)
- **carrier_vert_madd_vert8**: bridge=300.8% of algo (FFI overhead may distort results)

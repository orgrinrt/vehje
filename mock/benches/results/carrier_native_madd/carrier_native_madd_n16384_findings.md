# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, madd profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (780.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_direct at 643.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (carrier_nat_madd_direct, carrier_nat_madd_copypatch) are a dead heat (<1%)

carrier_nat_madd_direct (643.57 us) and carrier_nat_madd_copypatch (644.71 us) differ by 0.18%, inside the noise, even though the wider field spreads 21.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_madd_copypatch shows alternating (throttle bounce) (autocorr -0.63)

carrier_nat_madd_copypatch's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_nat_madd_direct** at 643567.9 ns median (-17.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.21x (fastest 643567.9 ns, slowest 780775.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 647965ns | 647136ns | 640084ns | 646231ns | 654505ns | -17.35% |
| carrier_nat_madd_direct | 647447ns | 646016ns | 639594ns | 645277ns | 654629ns | -17.42% |
| carrier_nat_madd_interp | 784003ns | 784238ns | 776356ns | 783480ns | 788613ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 645530ns | 637781ns | 651955ns | -17.33% | 0.025 |
| carrier_nat_madd_direct | 645018ns | 637301ns | 652120ns | -17.40% | 0.025 |
| carrier_nat_madd_interp | 780873ns | 772867ns | 786056ns | base | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 5189217 | 9512928 | 0.545 | 1.07× |
| carrier_nat_madd_direct | 4757323 | 7811414 | 0.609 | 0.98× |
| carrier_nat_madd_interp | 4870772 | 19149739 | 0.254 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.026 Gops/s** (carrier_nat_madd_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.025 | 98.9% |
| carrier_nat_madd_direct | 0.025 | 99.0% |
| carrier_nat_madd_interp | 0.021 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 647965ns | 647965ns | -17.35% |
| carrier_nat_madd_direct | 647447ns | 647447ns | -17.42% |
| carrier_nat_madd_interp | 784003ns | 784003ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 780775ns | base | --- | [775786, 786056] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 644713ns | -134983.5ns (-17.3%) | [-143445, -127599]ns | [639924, 651955] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_direct | 643568ns | -135734.8ns (-17.4%) | [-144063, -127766]ns | [639367, 652120] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_direct |
|---|---|---|---|
| 1 | 784786ns | -17.2% | -18.3% |
| 2 | 779411ns | -17.6% | -15.9% |
| 3 | 772867ns | -16.9% | -17.0% |
| 4 | 778705ns | -16.0% | -17.1% |
| 5 | 787326ns | -19.0% | -17.6% |
| 6 | 782140ns | -17.2% | -18.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | -0.632 | HIGH- (thermal bounce) |
| carrier_nat_madd_direct | -0.499 | moderate- |
| carrier_nat_madd_interp | 0.135 | ok |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 1020762.7ns | 645530.3ns | 158.1% | HIGH |
| carrier_nat_madd_direct | 882388.8ns | 645018.2ns | 136.8% | HIGH |
| carrier_nat_madd_interp | 781895.3ns | 780872.6ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 637780.8-651954.6 ns)
  637780.8 |####################
  638489.5 |
  639198.2 |
  639906.9 |
  640615.6 |
  641324.2 |
  642032.9 |########################################
  642741.6 |
  643450.3 |
  644159.0 |
  644867.7 |
  645576.4 |
  646285.1 |
  646993.8 |####################
  647702.5 |
  648411.2 |
  649119.8 |####################
  649828.5 |
  650537.2 |
  651245.9 |
  (0 below, 1 above range)

carrier_nat_madd_direct (n=6, range 637301.2-652119.6 ns)
  637301.2 |####################
  638042.1 |
  638783.0 |
  639524.0 |
  640264.9 |
  641005.8 |########################################
  641746.7 |
  642487.6 |
  643228.5 |
  643969.5 |
  644710.4 |
  645451.3 |####################
  646192.2 |
  646933.1 |
  647674.0 |
  648415.0 |####################
  649155.9 |
  649896.8 |
  650637.7 |
  651378.6 |
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 772867.1-786056.2 ns)
  772867.1 |########################################
  773526.6 |
  774186.0 |
  774845.5 |
  775504.9 |
  776164.4 |
  776823.8 |
  777483.3 |
  778142.7 |########################################
  778802.2 |########################################
  779461.6 |
  780121.1 |
  780780.6 |
  781440.0 |
  782099.5 |########################################
  782758.9 |
  783418.4 |
  784077.8 |
  784737.3 |########################################
  785396.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=158.2% of algo (FFI overhead may distort results)
- **carrier_nat_madd_direct**: bridge=136.7% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=99.9% of algo (FFI overhead may distort results)

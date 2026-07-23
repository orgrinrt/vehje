# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, real profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (559.83 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_direct at 58.18 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_direct beats baseline by 90% (significant)

carrier_nat_real_direct is -501.93 us (90%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 9.6x slower than the field

carrier_nat_real_interp (559.83 us) is 9.6x the fastest (58.18 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_real_direct, carrier_nat_real_copypatch) are a dead heat (<1%)

carrier_nat_real_direct (58.18 us) and carrier_nat_real_copypatch (58.74 us) differ by 0.98%, inside the noise, even though the wider field spreads 862.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_real_copypatch shows warm-up / thermal drift (autocorr +0.53)

carrier_nat_real_copypatch's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.6x the fastest

Fastest carrier_nat_real_direct (58.18 us) to slowest carrier_nat_real_interp (559.83 us): 9.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_real_direct** at 58175.2 ns median (-89.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.62x (fastest 58175.2 ns, slowest 559830.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 60825ns | 60985ns | 60191ns | 60746ns | 61259ns | -89.19% |
| carrier_nat_real_direct | 60531ns | 60406ns | 59940ns | 60304ns | 61168ns | -89.24% |
| carrier_nat_real_interp | 562726ns | 562129ns | 546566ns | 561147ns | 573176ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 58585ns | 57932ns | 59019ns | -89.54% | 0.070 |
| carrier_nat_real_direct | 58297ns | 57736ns | 58896ns | -89.59% | 0.070 |
| carrier_nat_real_interp | 560192ns | 543870ns | 570859ns | base | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 901301 | 2562426 | 0.352 | 0.26× |
| carrier_nat_real_direct | 771718 | 2135878 | 0.361 | 0.22× |
| carrier_nat_real_interp | 3501018 | 4597332 | 0.762 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.071 Gops/s** (carrier_nat_real_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.070 | 98.3% |
| carrier_nat_real_direct | 0.070 | 99.2% |
| carrier_nat_real_interp | 0.007 | 10.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 60825ns | 60825ns | -89.19% |
| carrier_nat_real_direct | 60531ns | 60531ns | -89.24% |
| carrier_nat_real_interp | 562726ns | 562726ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 559830ns | base | --- | [549887, 570859] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 58743ns | -501046.3ns (-89.5%) | [-512865, -490910]ns | [57995, 59019] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_direct | 58175ns | -501929.5ns (-89.7%) | [-512402, -491354]ns | [57820, 58896] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_direct |
|---|---|---|---|
| 1 | 555905ns | -89.4% | -89.6% |
| 2 | 561097ns | -89.5% | -89.7% |
| 3 | 543870ns | -89.1% | -89.1% |
| 4 | 558564ns | -89.5% | -89.7% |
| 5 | 565595ns | -89.7% | -89.7% |
| 6 | 576123ns | -89.9% | -89.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | 0.534 | HIGH+ (drift/warm-up) |
| carrier_nat_real_direct | -0.428 | moderate- |
| carrier_nat_real_interp | 0.149 | ok |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 229181.0ns | 58585.3ns | 391.2% | HIGH |
| carrier_nat_real_direct | 186143.1ns | 58297.1ns | 319.3% | HIGH |
| carrier_nat_real_interp | 560689.2ns | 560192.4ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 57931.7-59018.6 ns)
  57931.7 |########################################
  57986.0 |
  58040.4 |########################################
  58094.7 |
  58149.1 |
  58203.4 |
  58257.8 |
  58312.1 |
  58366.4 |
  58420.8 |
  58475.1 |
  58529.5 |
  58583.8 |########################################
  58638.2 |
  58692.5 |
  58746.8 |
  58801.2 |
  58855.5 |########################################
  58909.9 |########################################
  58964.2 |
  (0 below, 1 above range)

carrier_nat_real_direct (n=6, range 57736.2-58895.8 ns)
  57736.2 |########################################
  57794.2 |
  57852.2 |########################################
  57910.1 |
  57968.1 |
  58026.1 |########################################
  58084.1 |
  58142.1 |
  58200.1 |
  58258.0 |########################################
  58316.0 |
  58374.0 |
  58432.0 |
  58490.0 |
  58548.0 |
  58605.9 |########################################
  58663.9 |
  58721.9 |
  58779.9 |
  58837.9 |
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 543869.6-570859.4 ns)
  543869.6 |########################################
  545219.1 |
  546568.6 |
  547918.1 |
  549267.6 |
  550617.0 |
  551966.5 |
  553316.0 |
  554665.5 |########################################
  556015.0 |
  557364.5 |########################################
  558714.0 |
  560063.5 |########################################
  561412.9 |
  562762.4 |
  564111.9 |
  565461.4 |########################################
  566810.9 |
  568160.4 |
  569509.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **carrier_nat_real_copypatch**: bridge=389.7% of algo (FFI overhead may distort results)
- **carrier_nat_real_direct**: bridge=316.7% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=100.2% of algo (FFI overhead may distort results)

# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (10.32 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_copypatch at 3.66 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_direct beats baseline by 61% (significant)

carrier_nat_wideselect_direct is -6.34 us (61%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 2.8x slower than the field

carrier_nat_wideselect_interp (10.32 us) is 2.8x the fastest (3.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_wideselect_copypatch is fastest but the noisiest (CV 5.2%)

carrier_nat_wideselect_copypatch wins on median (3.66 us) yet has the highest variance (CV 5.2%), while carrier_nat_wideselect_interp is the steadiest (CV 3.7%, 10.32 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_nat_wideselect_copypatch** at 3659.3 ns median (-64.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.82x (fastest 3659.3 ns, slowest 10316.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 6062ns | 5891ns | 5788ns | 5858ns | 6504ns | -51.73% |
| carrier_nat_wideselect_direct | 6251ns | 6371ns | 5854ns | 6232ns | 6476ns | -50.23% |
| carrier_nat_wideselect_interp | 12559ns | 12787ns | 11882ns | 12551ns | 12908ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 3759ns | 3594ns | 4021ns | -62.91% | 0.068 |
| carrier_nat_wideselect_direct | 3863ns | 3583ns | 4012ns | -61.88% | 0.066 |
| carrier_nat_wideselect_interp | 10133ns | 9601ns | 10477ns | base | 0.025 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 280396 | 914118 | 0.307 | 0.91× |
| carrier_nat_wideselect_direct | 258190 | 852028 | 0.303 | 0.84× |
| carrier_nat_wideselect_interp | 307800 | 1510414 | 0.204 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.071 Gops/s** (carrier_nat_wideselect_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.070 | 97.9% |
| carrier_nat_wideselect_direct | 0.065 | 91.0% |
| carrier_nat_wideselect_interp | 0.025 | 34.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 6062ns | 6062ns | -51.73% |
| carrier_nat_wideselect_direct | 6251ns | 6251ns | -50.23% |
| carrier_nat_wideselect_interp | 12559ns | 12559ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 10317ns | base | --- | [9606, 10477] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 3659ns | -6295.2ns (-61.0%) | [-6847, -5981]ns | [3595, 4021] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_direct | 3937ns | -6343.1ns (-61.5%) | [-6716, -5751]ns | [3641, 4012] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_direct |
|---|---|---|---|
| 1 | 10559ns | -65.3% | -66.1% |
| 2 | 10318ns | -61.7% | -62.6% |
| 3 | 10315ns | -60.4% | -61.1% |
| 4 | 9601ns | -61.9% | -58.2% |
| 5 | 9611ns | -62.6% | -61.5% |
| 6 | 10395ns | -65.4% | -61.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.251 | moderate+ |
| carrier_nat_wideselect_direct | -0.156 | ok |
| carrier_nat_wideselect_interp | 0.180 | ok |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 88396.4ns | 3758.7ns | 2351.7% | HIGH |
| carrier_nat_wideselect_direct | 82071.6ns | 3863.1ns | 2124.5% | HIGH |
| carrier_nat_wideselect_interp | 91548.3ns | 10133.2ns | 903.5% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 3593.8-4021.4 ns)
   3593.8 |########################################
   3615.2 |
   3636.6 |####################
   3657.9 |####################
   3679.3 |
   3700.7 |
   3722.1 |
   3743.5 |
   3764.9 |
   3786.2 |
   3807.6 |
   3829.0 |
   3850.4 |
   3871.8 |
   3893.2 |
   3914.5 |
   3935.9 |####################
   3957.3 |
   3978.7 |
   4000.1 |
  (0 below, 1 above range)

carrier_nat_wideselect_direct (n=6, range 3582.9-4011.9 ns)
   3582.9 |####################
   3604.3 |
   3625.8 |
   3647.2 |
   3668.7 |
   3690.2 |####################
   3711.6 |
   3733.0 |
   3754.5 |
   3775.9 |
   3797.4 |
   3818.8 |
   3840.3 |
   3861.8 |####################
   3883.2 |
   3904.6 |
   3926.1 |
   3947.5 |
   3969.0 |
   3990.4 |########################################
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 9601.2-10476.9 ns)
   9601.2 |########################################
   9645.0 |
   9688.8 |
   9732.6 |
   9776.3 |
   9820.1 |
   9863.9 |
   9907.7 |
   9951.5 |
   9995.3 |
  10039.1 |
  10082.8 |
  10126.6 |
  10170.4 |
  10214.2 |
  10258.0 |
  10301.8 |########################################
  10345.5 |
  10389.3 |####################
  10433.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=2362.0% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_direct**: bridge=2070.3% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=893.4% of algo (FFI overhead may distort results)

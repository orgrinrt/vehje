# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (10.55 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_copypatch at 3.48 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 66% (significant)

carrier_nat_scatter_copypatch is -6.95 us (66%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 3.0x slower than the field

carrier_nat_scatter_interp (10.55 us) is 3.0x the fastest (3.48 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.0x the fastest

Fastest carrier_nat_scatter_copypatch (3.48 us) to slowest carrier_nat_scatter_interp (10.55 us): 3.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_scatter_copypatch** at 3476.1 ns median (-67.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.03x (fastest 3476.1 ns, slowest 10545.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 5851ns | 5701ns | 5560ns | 5665ns | 6277ns | -54.39% |
| carrier_nat_scatter_direct | 5958ns | 5951ns | 5567ns | 5828ns | 6349ns | -53.56% |
| carrier_nat_scatter_interp | 12830ns | 12839ns | 11918ns | 12596ns | 13637ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 3567ns | 3392ns | 3830ns | -66.16% | 0.072 |
| carrier_nat_scatter_direct | 3626ns | 3391ns | 3865ns | -65.60% | 0.071 |
| carrier_nat_scatter_interp | 10542ns | 9802ns | 11207ns | base | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 269482 | 880883 | 0.306 | 0.89× |
| carrier_nat_scatter_direct | 256631 | 843763 | 0.304 | 0.85× |
| carrier_nat_scatter_interp | 303492 | 1397925 | 0.217 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.075 Gops/s** (carrier_nat_scatter_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.074 | 97.6% |
| carrier_nat_scatter_direct | 0.071 | 93.7% |
| carrier_nat_scatter_interp | 0.024 | 32.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 5851ns | 5851ns | -54.39% |
| carrier_nat_scatter_direct | 5958ns | 5958ns | -53.56% |
| carrier_nat_scatter_interp | 12830ns | 12830ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 10546ns | base | --- | [9873, 11207] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 3476ns | -6954.0ns (-65.9%) | [-7506, -6464]ns | [3396, 3830] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_direct | 3619ns | -6940.6ns (-65.8%) | [-7349, -6457]ns | [3395, 3865] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_direct |
|---|---|---|---|
| 1 | 9945ns | -65.6% | -65.9% |
| 2 | 9802ns | -65.3% | -65.3% |
| 3 | 11132ns | -65.9% | -66.0% |
| 4 | 11212ns | -65.5% | -65.6% |
| 5 | 11202ns | -68.5% | -65.4% |
| 6 | 9960ns | -65.9% | -65.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.226 | moderate+ |
| carrier_nat_scatter_direct | 0.237 | moderate+ |
| carrier_nat_scatter_interp | 0.185 | ok |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 84290.1ns | 3567.3ns | 2362.9% | HIGH |
| carrier_nat_scatter_direct | 81263.0ns | 3626.2ns | 2241.0% | HIGH |
| carrier_nat_scatter_interp | 89494.8ns | 10542.0ns | 848.9% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 3391.7-3830.0 ns)
   3391.7 |########################################
   3413.6 |####################
   3435.5 |
   3457.4 |
   3479.4 |
   3501.3 |
   3523.2 |####################
   3545.1 |
   3567.0 |
   3588.9 |
   3610.8 |
   3632.8 |
   3654.7 |
   3676.6 |
   3698.5 |
   3720.4 |
   3742.3 |
   3764.3 |
   3786.2 |####################
   3808.1 |
  (0 below, 1 above range)

carrier_nat_scatter_direct (n=6, range 3391.2-3864.8 ns)
   3391.2 |########################################
   3414.9 |
   3438.6 |####################
   3462.2 |
   3485.9 |
   3509.6 |
   3533.3 |
   3557.0 |
   3580.6 |
   3604.3 |
   3628.0 |
   3651.7 |
   3675.4 |
   3699.0 |
   3722.7 |
   3746.4 |
   3770.1 |####################
   3793.8 |
   3817.4 |
   3841.1 |####################
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 9801.7-11207.1 ns)
   9801.7 |####################
   9872.0 |
   9942.2 |########################################
  10012.5 |
  10082.8 |
  10153.1 |
  10223.3 |
  10293.6 |
  10363.9 |
  10434.1 |
  10504.4 |
  10574.7 |
  10644.9 |
  10715.2 |
  10785.5 |
  10855.8 |
  10926.0 |
  10996.3 |
  11066.6 |####################
  11136.8 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=2418.3% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_direct**: bridge=2248.8% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=850.6% of algo (FFI overhead may distort results)

# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (2.63 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_copypatch at 388.40 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 85% (significant)

carrier_nat_scatter_copypatch is -2.25 ms (85%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 6.8x slower than the field

carrier_nat_scatter_interp (2.63 ms) is 6.8x the fastest (388.40 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_scatter_direct shows alternating (throttle bounce) (autocorr -0.59)

carrier_nat_scatter_direct's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.8x the fastest

Fastest carrier_nat_scatter_copypatch (388.40 us) to slowest carrier_nat_scatter_interp (2.63 ms): 6.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_scatter_copypatch** at 388399.8 ns median (-85.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.78x (fastest 388399.8 ns, slowest 2634433.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 391003ns | 390747ns | 387300ns | 389878ns | 394543ns | -85.16% |
| carrier_nat_scatter_direct | 396438ns | 396836ns | 390322ns | 395498ns | 400904ns | -84.96% |
| carrier_nat_scatter_interp | 2635215ns | 2637486ns | 2615304ns | 2631822ns | 2650259ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 388654ns | 385033ns | 392144ns | -85.24% | 0.042 |
| carrier_nat_scatter_direct | 394022ns | 388011ns | 398391ns | -85.03% | 0.042 |
| carrier_nat_scatter_interp | 2632393ns | 2612670ns | 2647420ns | base | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 4030422 | 9064313 | 0.445 | 0.25× |
| carrier_nat_scatter_direct | 3578096 | 7317049 | 0.489 | 0.22× |
| carrier_nat_scatter_interp | 16302957 | 18761013 | 0.869 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.043 Gops/s** (carrier_nat_scatter_copypatch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.042 | 99.1% |
| carrier_nat_scatter_direct | 0.042 | 97.6% |
| carrier_nat_scatter_interp | 0.006 | 14.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 391003ns | 391003ns | -85.16% |
| carrier_nat_scatter_direct | 396438ns | 396438ns | -84.96% |
| carrier_nat_scatter_interp | 2635215ns | 2635215ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 2634434ns | base | --- | [2615325, 2647420] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 388400ns | -2245427.7ns (-85.2%) | [-2258856, -2226933]ns | [385418, 392144] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_direct | 394420ns | -2241802.1ns (-85.1%) | [-2254124, -2219185]ns | [389256, 398391] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_direct |
|---|---|---|---|
| 1 | 2634332ns | -85.3% | -85.2% |
| 2 | 2634535ns | -85.4% | -85.0% |
| 3 | 2612670ns | -85.0% | -84.8% |
| 4 | 2637909ns | -85.1% | -85.3% |
| 5 | 2656930ns | -85.4% | -85.0% |
| 6 | 2617981ns | -85.3% | -84.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.095 | ok |
| carrier_nat_scatter_direct | -0.593 | HIGH- (thermal bounce) |
| carrier_nat_scatter_interp | -0.295 | moderate- |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 894665.6ns | 388654.0ns | 230.2% | HIGH |
| carrier_nat_scatter_direct | 754642.7ns | 394022.5ns | 191.5% | HIGH |
| carrier_nat_scatter_interp | 2635563.8ns | 2632392.8ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 385033.3-392144.2 ns)
  385033.3 |########################################
  385388.8 |
  385744.4 |########################################
  386099.9 |
  386455.5 |
  386811.0 |
  387166.6 |
  387522.1 |
  387877.7 |########################################
  388233.2 |
  388588.8 |########################################
  388944.3 |
  389299.8 |
  389655.4 |
  390010.9 |
  390366.5 |
  390722.0 |########################################
  391077.6 |
  391433.1 |
  391788.7 |
  (0 below, 1 above range)

carrier_nat_scatter_direct (n=6, range 388010.8-398391.5 ns)
  388010.8 |########################################
  388529.8 |
  389048.9 |
  389567.9 |
  390086.9 |########################################
  390606.0 |
  391125.0 |
  391644.0 |
  392163.1 |
  392682.1 |
  393201.1 |
  393720.2 |########################################
  394239.2 |
  394758.2 |########################################
  395277.3 |
  395796.3 |
  396315.3 |
  396834.4 |
  397353.4 |
  397872.4 |########################################
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 2612670.0-2647419.5 ns)
  2612670.0 |####################
  2614407.5 |
  2616145.0 |
  2617882.4 |####################
  2619619.9 |
  2621357.4 |
  2623094.9 |
  2624832.3 |
  2626569.8 |
  2628307.3 |
  2630044.8 |
  2631782.3 |
  2633519.7 |########################################
  2635257.2 |
  2636994.7 |####################
  2638732.2 |
  2640469.6 |
  2642207.1 |
  2643944.6 |
  2645682.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=230.4% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_direct**: bridge=190.6% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=100.1% of algo (FFI overhead may distort results)

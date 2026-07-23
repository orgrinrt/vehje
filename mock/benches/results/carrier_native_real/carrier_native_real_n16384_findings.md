# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, real profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (2.65 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_copypatch at 409.58 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_copypatch beats baseline by 85% (significant)

carrier_nat_real_copypatch is -2.24 ms (85%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 6.5x slower than the field

carrier_nat_real_interp (2.65 ms) is 6.5x the fastest (409.58 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_real_copypatch, carrier_nat_real_direct) are a dead heat (<1%)

carrier_nat_real_copypatch (409.58 us) and carrier_nat_real_direct (410.07 us) differ by 0.12%, inside the noise, even though the wider field spreads 547.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 6.5x the fastest

Fastest carrier_nat_real_copypatch (409.58 us) to slowest carrier_nat_real_interp (2.65 ms): 6.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_real_copypatch** at 409577.5 ns median (-84.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.48x (fastest 409577.5 ns, slowest 2653109.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 411974ns | 411930ns | 410608ns | 411566ns | 413269ns | -84.49% |
| carrier_nat_real_direct | 412734ns | 412371ns | 409297ns | 411443ns | 416389ns | -84.46% |
| carrier_nat_real_interp | 2655968ns | 2656479ns | 2635113ns | 2654432ns | 2668699ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 409649ns | 408379ns | 410896ns | -84.56% | 0.040 |
| carrier_nat_real_direct | 410383ns | 407040ns | 413981ns | -84.53% | 0.040 |
| carrier_nat_real_interp | 2652672ns | 2632080ns | 2665326ns | base | 0.006 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 4045144 | 9352469 | 0.433 | 0.25× |
| carrier_nat_real_direct | 3581443 | 7623799 | 0.470 | 0.22× |
| carrier_nat_real_interp | 16466075 | 18293600 | 0.900 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.040 Gops/s** (carrier_nat_real_direct; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.040 | 99.4% |
| carrier_nat_real_direct | 0.040 | 99.3% |
| carrier_nat_real_interp | 0.006 | 15.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 411974ns | 411974ns | -84.49% |
| carrier_nat_real_direct | 412734ns | 412734ns | -84.46% |
| carrier_nat_real_interp | 2655968ns | 2655968ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 2653109ns | base | --- | [2639580, 2665326] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 409578ns | -2243378.9ns (-84.6%) | [-2256170, -2229519]ns | [408473, 410896] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_direct | 410070ns | -2242378.8ns (-84.5%) | [-2257280, -2227207]ns | [407098, 413981] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_direct |
|---|---|---|---|
| 1 | 2632080ns | -84.4% | -84.4% |
| 2 | 2647080ns | -84.5% | -84.5% |
| 3 | 2650295ns | -84.6% | -84.3% |
| 4 | 2655923ns | -84.5% | -84.7% |
| 5 | 2661072ns | -84.6% | -84.7% |
| 6 | 2669579ns | -84.7% | -84.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | -0.288 | moderate- |
| carrier_nat_real_direct | -0.002 | ok |
| carrier_nat_real_interp | 0.350 | moderate+ |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 878806.5ns | 409649.0ns | 214.5% | HIGH |
| carrier_nat_real_direct | 731279.9ns | 410383.0ns | 178.2% | HIGH |
| carrier_nat_real_interp | 2654468.3ns | 2652671.6ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 408379.2-410896.5 ns)
  408379.2 |########################################
  408505.1 |########################################
  408630.9 |
  408756.8 |
  408882.7 |
  409008.5 |
  409134.4 |
  409260.2 |
  409386.1 |########################################
  409512.0 |
  409637.8 |########################################
  409763.7 |
  409889.5 |
  410015.4 |
  410141.3 |
  410267.1 |
  410393.0 |
  410518.9 |
  410644.7 |########################################
  410770.6 |
  (0 below, 1 above range)

carrier_nat_real_direct (n=6, range 407039.6-413980.8 ns)
  407039.6 |########################################
  407386.7 |
  407733.7 |
  408080.8 |
  408427.8 |
  408774.9 |####################
  409122.0 |
  409469.0 |
  409816.1 |
  410163.2 |
  410510.2 |
  410857.3 |####################
  411204.3 |####################
  411551.4 |
  411898.5 |
  412245.5 |
  412592.6 |
  412939.7 |
  413286.7 |
  413633.8 |
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 2632080.0-2665325.6 ns)
  2632080.0 |########################################
  2633742.3 |
  2635404.6 |
  2637066.8 |
  2638729.1 |
  2640391.4 |
  2642053.7 |
  2643716.0 |
  2645378.3 |
  2647040.5 |########################################
  2648702.8 |########################################
  2650365.1 |
  2652027.4 |
  2653689.7 |
  2655352.0 |########################################
  2657014.2 |
  2658676.5 |
  2660338.8 |########################################
  2662001.1 |
  2663663.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: bridge=214.0% of algo (FFI overhead may distort results)
- **carrier_nat_real_direct**: bridge=178.6% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=100.1% of algo (FFI overhead may distort results)

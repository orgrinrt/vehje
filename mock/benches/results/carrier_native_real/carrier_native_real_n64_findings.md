# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, real profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (2.19 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_direct at 587 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_copypatch beats baseline by 75% (significant)

carrier_nat_real_copypatch is -1.64 us (75%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 3.7x slower than the field

carrier_nat_real_interp (2.19 us) is 3.7x the fastest (587 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest carrier_nat_real_direct (587 ns) to slowest carrier_nat_real_interp (2.19 us): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_real_direct** at 587.2 ns median (-73.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.73x (fastest 587.2 ns, slowest 2192.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 3020ns | 3074ns | 2741ns | 3011ns | 3172ns | -34.29% |
| carrier_nat_real_direct | 2985ns | 2931ns | 2769ns | 2888ns | 3238ns | -35.04% |
| carrier_nat_real_interp | 4595ns | 4488ns | 4373ns | 4451ns | 4924ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 606ns | 542ns | 648ns | -73.26% | 0.106 |
| carrier_nat_real_direct | 597ns | 560ns | 636ns | -73.64% | 0.107 |
| carrier_nat_real_interp | 2266ns | 2165ns | 2438ns | base | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 220780 | 1019432 | 0.217 | 0.81× |
| carrier_nat_real_direct | 222929 | 1018650 | 0.219 | 0.82× |
| carrier_nat_real_interp | 272746 | 1368345 | 0.199 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.118 Gops/s** (carrier_nat_real_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.102 | 86.6% |
| carrier_nat_real_direct | 0.109 | 92.4% |
| carrier_nat_real_interp | 0.029 | 24.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 3020ns | 3020ns | -34.29% |
| carrier_nat_real_direct | 2985ns | 2985ns | -35.04% |
| carrier_nat_real_interp | 4595ns | 4595ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 2192ns | base | --- | [2168, 2438] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 626ns | -1635.7ns (-74.6%) | [-1799, -1545]ns | [544, 648] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_direct | 587ns | -1616.7ns (-73.8%) | [-1803, -1586]ns | [569, 636] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_direct |
|---|---|---|---|
| 1 | 2195ns | -70.7% | -73.1% |
| 2 | 2351ns | -73.3% | -72.8% |
| 3 | 2170ns | -75.0% | -73.3% |
| 4 | 2189ns | -75.1% | -74.4% |
| 5 | 2165ns | -71.1% | -73.1% |
| 6 | 2526ns | -74.2% | -75.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | 0.250 | moderate+ |
| carrier_nat_real_direct | -0.061 | ok |
| carrier_nat_real_interp | -0.240 | moderate- |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 73646.4ns | 605.8ns | 12156.2% | HIGH |
| carrier_nat_real_direct | 72216.8ns | 597.4ns | 12089.5% | HIGH |
| carrier_nat_real_interp | 85850.7ns | 2266.0ns | 3788.7% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 542.5-647.5 ns)
    542.5 |########################################
    547.8 |
    553.0 |
    558.2 |
    563.5 |
    568.8 |
    574.0 |
    579.2 |
    584.5 |
    589.8 |
    595.0 |
    600.2 |
    605.5 |
    610.8 |
    616.0 |
    621.2 |####################
    626.5 |####################
    631.8 |
    637.0 |
    642.2 |####################
  (0 below, 1 above range)

carrier_nat_real_direct (n=6, range 559.6-635.6 ns)
    559.6 |########################################
    563.4 |
    567.2 |
    571.0 |
    574.8 |
    578.6 |########################################
    582.4 |########################################
    586.2 |
    590.0 |########################################
    593.8 |
    597.6 |
    601.4 |
    605.2 |
    609.0 |
    612.8 |
    616.6 |
    620.4 |
    624.2 |
    628.0 |########################################
    631.8 |
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 2165.0-2438.3 ns)
   2165.0 |########################################
   2178.7 |####################
   2192.3 |####################
   2206.0 |
   2219.7 |
   2233.3 |
   2247.0 |
   2260.7 |
   2274.3 |
   2288.0 |
   2301.7 |
   2315.3 |
   2329.0 |
   2342.6 |####################
   2356.3 |
   2370.0 |
   2383.6 |
   2397.3 |
   2411.0 |
   2424.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: bridge=12110.2% of algo (FFI overhead may distort results)
- **carrier_nat_real_direct**: bridge=11868.6% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=3910.1% of algo (FFI overhead may distort results)

# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (2.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_direct at 456.36 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_direct beats baseline by 79% (significant)

carrier_nat_wideselect_direct is -1.72 ms (79%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 4.8x slower than the field

carrier_nat_wideselect_interp (2.18 ms) is 4.8x the fastest (456.36 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_wideselect_direct, carrier_nat_wideselect_copypatch) are a dead heat (<1%)

carrier_nat_wideselect_direct (456.36 us) and carrier_nat_wideselect_copypatch (459.48 us) differ by 0.68%, inside the noise, even though the wider field spreads 376.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 4.8x the fastest

Fastest carrier_nat_wideselect_direct (456.36 us) to slowest carrier_nat_wideselect_interp (2.18 ms): 4.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_wideselect_direct** at 456355.8 ns median (-79.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.77x (fastest 456355.8 ns, slowest 2176267.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 464177ns | 462023ns | 450856ns | 458330ns | 479607ns | -78.79% |
| carrier_nat_wideselect_direct | 464246ns | 458918ns | 456749ns | 458261ns | 476972ns | -78.79% |
| carrier_nat_wideselect_interp | 2188693ns | 2179740ns | 2169920ns | 2178268ns | 2213717ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 461619ns | 448488ns | 476855ns | -78.88% | 0.035 |
| carrier_nat_wideselect_direct | 461696ns | 454365ns | 474309ns | -78.87% | 0.035 |
| carrier_nat_wideselect_interp | 2185183ns | 2166761ns | 2210149ns | base | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 4454882 | 10710143 | 0.416 | 0.33× |
| carrier_nat_wideselect_direct | 3896253 | 8408032 | 0.463 | 0.29× |
| carrier_nat_wideselect_interp | 13544514 | 19261391 | 0.703 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.037 Gops/s** (carrier_nat_wideselect_copypatch; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.036 | 97.6% |
| carrier_nat_wideselect_direct | 0.036 | 98.3% |
| carrier_nat_wideselect_interp | 0.008 | 20.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 464177ns | 464177ns | -78.79% |
| carrier_nat_wideselect_direct | 464246ns | 464246ns | -78.79% |
| carrier_nat_wideselect_interp | 2188693ns | 2188693ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 2176268ns | base | --- | [2169133, 2210149] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 459480ns | -1718271.9ns (-79.0%) | [-1751719, -1700701]ns | [448523, 476855] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_direct | 456356ns | -1718870.0ns (-79.0%) | [-1744015, -1707576]ns | [454424, 474309] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_direct |
|---|---|---|---|
| 1 | 2171504ns | -78.8% | -79.1% |
| 2 | 2177452ns | -78.9% | -79.1% |
| 3 | 2179623ns | -77.5% | -78.3% |
| 4 | 2240674ns | -79.3% | -78.8% |
| 5 | 2166761ns | -79.3% | -78.9% |
| 6 | 2175083ns | -79.4% | -79.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.120 | ok |
| carrier_nat_wideselect_direct | 0.213 | moderate+ |
| carrier_nat_wideselect_interp | -0.262 | moderate- |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 1000740.5ns | 461619.0ns | 216.8% | HIGH |
| carrier_nat_wideselect_direct | 805366.6ns | 461696.3ns | 174.4% | HIGH |
| carrier_nat_wideselect_interp | 2191134.1ns | 2185183.1ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 448487.9-476854.8 ns)
  448487.9 |########################################
  449906.2 |
  451324.6 |
  452742.9 |
  454161.3 |
  455579.6 |
  456998.0 |
  458416.3 |########################################
  459834.7 |
  461253.0 |
  462671.4 |####################
  464089.7 |
  465508.0 |
  466926.4 |
  468344.7 |
  469763.1 |
  471181.4 |
  472599.8 |
  474018.1 |
  475436.5 |
  (0 below, 1 above range)

carrier_nat_wideselect_direct (n=6, range 454365.0-474309.4 ns)
  454365.0 |########################################
  455362.2 |
  456359.4 |
  457356.7 |#############
  458353.9 |
  459351.1 |
  460348.3 |
  461345.5 |
  462342.8 |
  463340.0 |
  464337.2 |
  465334.4 |
  466331.6 |
  467328.9 |
  468326.1 |
  469323.3 |
  470320.5 |
  471317.7 |
  472315.0 |
  473312.2 |#############
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 2166761.2-2210148.8 ns)
  2166761.2 |########################################
  2168930.6 |
  2171100.0 |########################################
  2173269.3 |########################################
  2175438.7 |########################################
  2177608.1 |########################################
  2179777.5 |
  2181946.8 |
  2184116.2 |
  2186285.6 |
  2188455.0 |
  2190624.4 |
  2192793.7 |
  2194963.1 |
  2197132.5 |
  2199301.9 |
  2201471.2 |
  2203640.6 |
  2205810.0 |
  2207979.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=214.7% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_direct**: bridge=174.3% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=100.2% of algo (FFI overhead may distort results)

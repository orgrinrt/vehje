# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, real profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (42.35 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_direct at 14.63 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_direct beats baseline by 66% (significant)

carrier_nat_real_direct is -27.80 us (66%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 2.9x slower than the field

carrier_nat_real_interp (42.35 us) is 2.9x the fastest (14.63 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_real_direct shows alternating (throttle bounce) (autocorr -0.58)

carrier_nat_real_direct's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_nat_real_direct vs stability leader carrier_nat_real_copypatch (+1% speed for 1.1x steadier)

carrier_nat_real_direct is fastest (14.63 us, CV 3.1%); carrier_nat_real_copypatch gives up 1.5% median for 1.1x lower variance (CV 2.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_real_direct** at 14626.2 ns median (-65.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.90x (fastest 14626.2 ns, slowest 42353.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 17105ns | 17089ns | 16363ns | 16946ns | 17715ns | -61.97% |
| carrier_nat_real_direct | 17090ns | 16839ns | 16477ns | 16808ns | 17820ns | -62.00% |
| carrier_nat_real_interp | 44978ns | 44666ns | 43255ns | 44325ns | 46820ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 14847ns | 14211ns | 15351ns | -65.20% | 0.069 |
| carrier_nat_real_direct | 14817ns | 14272ns | 15416ns | -65.27% | 0.069 |
| carrier_nat_real_interp | 42659ns | 41043ns | 44438ns | base | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 366120 | 1099934 | 0.333 | 0.76× |
| carrier_nat_real_direct | 329207 | 996492 | 0.330 | 0.68× |
| carrier_nat_real_interp | 482418 | 2123928 | 0.227 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.072 Gops/s** (carrier_nat_real_copypatch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.069 | 95.7% |
| carrier_nat_real_direct | 0.070 | 97.2% |
| carrier_nat_real_interp | 0.024 | 33.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 17105ns | 17105ns | -61.97% |
| carrier_nat_real_direct | 17090ns | 17090ns | -62.00% |
| carrier_nat_real_interp | 44978ns | 44978ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 42354ns | base | --- | [41186, 44438] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 14844ns | -27606.1ns (-65.2%) | [-29263, -26568]ns | [14346, 15351] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_direct | 14626ns | -27797.1ns (-65.6%) | [-29530, -26200]ns | [14408, 15416] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_direct |
|---|---|---|---|
| 1 | 44298ns | -64.9% | -67.8% |
| 2 | 44578ns | -66.7% | -65.1% |
| 3 | 41043ns | -64.7% | -64.2% |
| 4 | 41438ns | -64.1% | -64.9% |
| 5 | 41328ns | -65.6% | -63.0% |
| 6 | 43270ns | -64.9% | -66.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | -0.211 | moderate- |
| carrier_nat_real_direct | -0.585 | HIGH- (thermal bounce) |
| carrier_nat_real_interp | 0.224 | moderate+ |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 103168.2ns | 14846.8ns | 694.9% | HIGH |
| carrier_nat_real_direct | 90842.7ns | 14816.6ns | 613.1% | HIGH |
| carrier_nat_real_interp | 115105.2ns | 42659.1ns | 269.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 14211.2-15350.9 ns)
  14211.2 |########################################
  14268.2 |
  14325.2 |
  14382.1 |
  14439.1 |########################################
  14496.1 |
  14553.1 |
  14610.1 |
  14667.1 |
  14724.0 |
  14781.0 |########################################
  14838.0 |########################################
  14895.0 |
  14952.0 |
  15009.0 |
  15065.9 |
  15122.9 |########################################
  15179.9 |
  15236.9 |
  15293.9 |
  (0 below, 1 above range)

carrier_nat_real_direct (n=6, range 14272.1-15416.0 ns)
  14272.1 |########################################
  14329.3 |
  14386.5 |
  14443.7 |
  14500.9 |########################################
  14558.1 |########################################
  14615.3 |
  14672.5 |########################################
  14729.7 |
  14786.9 |
  14844.1 |
  14901.3 |
  14958.5 |
  15015.7 |
  15072.9 |
  15130.1 |
  15187.3 |
  15244.5 |########################################
  15301.7 |
  15358.9 |
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 41043.3-44437.7 ns)
  41043.3 |########################################
  41213.0 |########################################
  41382.7 |########################################
  41552.5 |
  41722.2 |
  41891.9 |
  42061.6 |
  42231.3 |
  42401.1 |
  42570.8 |
  42740.5 |
  42910.2 |
  43079.9 |
  43249.7 |########################################
  43419.4 |
  43589.1 |
  43758.8 |
  43928.5 |
  44098.3 |
  44268.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: bridge=694.9% of algo (FFI overhead may distort results)
- **carrier_nat_real_direct**: bridge=605.3% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=283.0% of algo (FFI overhead may distort results)

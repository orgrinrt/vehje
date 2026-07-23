# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (450.68 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_copypatch at 61.47 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_copypatch beats baseline by 86% (significant)

carrier_nat_wideselect_copypatch is -388.75 us (86%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 7.3x slower than the field

carrier_nat_wideselect_interp (450.68 us) is 7.3x the fastest (61.47 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_wideselect_copypatch, carrier_nat_wideselect_direct) are a dead heat (<1%)

carrier_nat_wideselect_copypatch (61.47 us) and carrier_nat_wideselect_direct (61.93 us) differ by 0.75%, inside the noise, even though the wider field spreads 633.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 7.3x the fastest

Fastest carrier_nat_wideselect_copypatch (61.47 us) to slowest carrier_nat_wideselect_interp (450.68 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_wideselect_copypatch** at 61474.8 ns median (-86.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.33x (fastest 61474.8 ns, slowest 450680.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 63891ns | 63712ns | 62926ns | 63679ns | 64691ns | -85.84% |
| carrier_nat_wideselect_direct | 64089ns | 64201ns | 62680ns | 63768ns | 65275ns | -85.79% |
| carrier_nat_wideselect_interp | 451056ns | 453527ns | 424660ns | 449740ns | 466229ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 61608ns | 60740ns | 62277ns | -86.26% | 0.066 |
| carrier_nat_wideselect_direct | 61833ns | 60510ns | 62968ns | -86.21% | 0.066 |
| carrier_nat_wideselect_interp | 448334ns | 422370ns | 463525ns | base | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 945595 | 2868265 | 0.330 | 0.34× |
| carrier_nat_wideselect_direct | 779807 | 2301969 | 0.339 | 0.28× |
| carrier_nat_wideselect_interp | 2798524 | 4827905 | 0.580 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.068 Gops/s** (carrier_nat_wideselect_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.067 | 98.4% |
| carrier_nat_wideselect_direct | 0.066 | 97.7% |
| carrier_nat_wideselect_interp | 0.009 | 13.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 63891ns | 63891ns | -85.84% |
| carrier_nat_wideselect_direct | 64089ns | 64089ns | -85.79% |
| carrier_nat_wideselect_interp | 451056ns | 451056ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 450680ns | base | --- | [430797, 463525] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 61475ns | -388752.3ns (-86.3%) | [-402050, -369377]ns | [61071, 62277] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_direct | 61935ns | -388699.3ns (-86.2%) | [-401942, -368862]ns | [60596, 62968] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_direct |
|---|---|---|---|
| 1 | 440073ns | -85.7% | -86.3% |
| 2 | 439224ns | -86.0% | -85.9% |
| 3 | 461288ns | -86.7% | -86.2% |
| 4 | 422370ns | -85.6% | -85.3% |
| 5 | 464706ns | -86.8% | -86.9% |
| 6 | 462344ns | -86.7% | -86.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.094 | ok |
| carrier_nat_wideselect_direct | -0.106 | ok |
| carrier_nat_wideselect_interp | -0.394 | moderate- |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 241245.2ns | 61607.5ns | 391.6% | HIGH |
| carrier_nat_wideselect_direct | 188367.5ns | 61833.0ns | 304.6% | HIGH |
| carrier_nat_wideselect_interp | 448344.4ns | 448334.1ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 60739.6-62276.7 ns)
  60739.6 |####################
  60816.5 |
  60893.3 |
  60970.2 |
  61047.0 |
  61123.9 |
  61200.7 |
  61277.6 |
  61354.4 |########################################
  61431.3 |
  61508.1 |####################
  61585.0 |####################
  61661.9 |
  61738.7 |
  61815.6 |
  61892.4 |
  61969.3 |
  62046.1 |
  62123.0 |
  62199.8 |
  (0 below, 1 above range)

carrier_nat_wideselect_direct (n=6, range 60510.0-62967.9 ns)
  60510.0 |########################################
  60632.9 |########################################
  60755.8 |
  60878.7 |
  61001.6 |
  61124.5 |
  61247.4 |
  61370.3 |
  61493.2 |
  61616.1 |
  61738.9 |########################################
  61861.8 |
  61984.7 |########################################
  62107.6 |
  62230.5 |
  62353.4 |
  62476.3 |########################################
  62599.2 |
  62722.1 |
  62845.0 |
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 422369.6-463525.2 ns)
  422369.6 |####################
  424427.4 |
  426485.2 |
  428542.9 |
  430600.7 |
  432658.5 |
  434716.3 |
  436774.1 |
  438831.8 |########################################
  440889.6 |
  442947.4 |
  445005.2 |
  447063.0 |
  449120.7 |
  451178.5 |
  453236.3 |
  455294.1 |
  457351.9 |
  459409.6 |####################
  461467.4 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=391.4% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_direct**: bridge=305.2% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=100.0% of algo (FFI overhead may distort results)

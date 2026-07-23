# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (571.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_direct at 57.84 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_copypatch beats baseline by 90% (significant)

carrier_nat_scatter_copypatch is -513.15 us (90%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 9.9x slower than the field

carrier_nat_scatter_interp (571.44 us) is 9.9x the fastest (57.84 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_scatter_direct is fastest but the noisiest (CV 16.1%)

carrier_nat_scatter_direct wins on median (57.84 us) yet has the highest variance (CV 16.1%), while carrier_nat_scatter_copypatch is the steadiest (CV 1.2%, 58.33 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_nat_scatter_direct, carrier_nat_scatter_copypatch) are a dead heat (<1%)

carrier_nat_scatter_direct (57.84 us) and carrier_nat_scatter_copypatch (58.33 us) differ by 0.84%, inside the noise, even though the wider field spreads 887.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_scatter_interp shows alternating (throttle bounce) (autocorr -0.80)

carrier_nat_scatter_interp's per-pass series has lag-1 autocorrelation -0.80, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.9x the fastest

Fastest carrier_nat_scatter_direct (57.84 us) to slowest carrier_nat_scatter_interp (571.44 us): 9.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader carrier_nat_scatter_direct vs stability leader carrier_nat_scatter_copypatch (+1% speed for 13.9x steadier)

carrier_nat_scatter_direct is fastest (57.84 us, CV 16.1%); carrier_nat_scatter_copypatch gives up 0.8% median for 13.9x lower variance (CV 1.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_scatter_direct** at 57842.7 ns median (-89.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.88x (fastest 57842.7 ns, slowest 571440.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 60721ns | 60563ns | 59946ns | 60503ns | 61435ns | -89.36% |
| carrier_nat_scatter_direct | 64142ns | 60061ns | 59122ns | 59761ns | 73223ns | -88.76% |
| carrier_nat_scatter_interp | 570855ns | 574114ns | 541386ns | 565693ns | 593332ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 58483ns | 57748ns | 59176ns | -89.71% | 0.070 |
| carrier_nat_scatter_direct | 61749ns | 56944ns | 70436ns | -89.13% | 0.066 |
| carrier_nat_scatter_interp | 568224ns | 539053ns | 590414ns | base | 0.007 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 900529 | 2601972 | 0.346 | 0.25× |
| carrier_nat_scatter_direct | 769226 | 2141711 | 0.359 | 0.22× |
| carrier_nat_scatter_interp | 3541358 | 4704715 | 0.753 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.072 Gops/s** (carrier_nat_scatter_direct; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.070 | 97.6% |
| carrier_nat_scatter_direct | 0.071 | 98.4% |
| carrier_nat_scatter_interp | 0.007 | 10.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 60721ns | 60721ns | -89.36% |
| carrier_nat_scatter_direct | 64142ns | 64142ns | -88.76% |
| carrier_nat_scatter_interp | 570855ns | 570855ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 571441ns | base | --- | [542819, 590414] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 58328ns | -513150.2ns (-89.8%) | [-531282, -484792]ns | [57945, 59176] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_direct | 57843ns | -502833.5ns (-88.0%) | [-531616, -484976]ns | [56968, 70436] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_direct |
|---|---|---|---|
| 1 | 587800ns | -90.1% | -90.3% |
| 2 | 539053ns | -89.2% | -89.3% |
| 3 | 591459ns | -89.9% | -86.0% |
| 4 | 555081ns | -89.5% | -89.5% |
| 5 | 589368ns | -90.1% | -90.3% |
| 6 | 546584ns | -89.4% | -89.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | -0.216 | moderate- |
| carrier_nat_scatter_direct | -0.187 | ok |
| carrier_nat_scatter_interp | -0.800 | HIGH- (thermal bounce) |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 232476.7ns | 58482.9ns | 397.5% | HIGH |
| carrier_nat_scatter_direct | 196047.3ns | 61749.0ns | 317.5% | HIGH |
| carrier_nat_scatter_interp | 568629.3ns | 568224.2ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 57748.3-59175.8 ns)
  57748.3 |########################################
  57819.7 |
  57891.1 |
  57962.4 |
  58033.8 |
  58105.2 |########################################
  58176.6 |
  58247.9 |########################################
  58319.3 |########################################
  58390.7 |########################################
  58462.1 |
  58533.5 |
  58604.8 |
  58676.2 |
  58747.6 |
  58819.0 |
  58890.3 |
  58961.7 |
  59033.1 |
  59104.5 |
  (0 below, 1 above range)

carrier_nat_scatter_direct (n=6, range 56943.8-70436.5 ns)
  56943.8 |##########################
  57618.4 |########################################
  58293.1 |
  58967.7 |
  59642.3 |
  60317.0 |
  60991.6 |
  61666.2 |
  62340.9 |
  63015.5 |
  63690.2 |
  64364.8 |
  65039.4 |
  65714.1 |
  66388.7 |
  67063.3 |
  67738.0 |
  68412.6 |
  69087.2 |
  69761.9 |
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 539052.9-590413.6 ns)
  539052.9 |########################################
  541620.9 |
  544189.0 |########################################
  546757.0 |
  549325.0 |
  551893.1 |
  554461.1 |########################################
  557029.1 |
  559597.2 |
  562165.2 |
  564733.2 |
  567301.3 |
  569869.3 |
  572437.3 |
  575005.4 |
  577573.4 |
  580141.4 |
  582709.5 |
  585277.5 |########################################
  587845.5 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=396.1% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_direct**: bridge=319.6% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=100.1% of algo (FFI overhead may distort results)

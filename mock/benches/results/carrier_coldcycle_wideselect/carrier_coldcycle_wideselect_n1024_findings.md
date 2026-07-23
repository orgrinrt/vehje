# Cold/aliased-predictor dispatch: 16 distinct programs cycled per pass (defeats predictor memorization), wideselect profile

4 variants, 6 samples per variant.
Baseline: **carrier_cold_wideselect_switch**

## Highlights

Baseline for all deltas below: **carrier_cold_wideselect_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_cold_wideselect_null dominates: 259% faster than the next best (carrier_cold_wideselect_switch)

carrier_cold_wideselect_null (30.02 us) leads carrier_cold_wideselect_switch (107.84 us) by 259%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cold_wideselect_null beats baseline by 72% (significant)

carrier_cold_wideselect_null is -77.42 us (72%) faster than baseline carrier_cold_wideselect_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cold_wideselect_fntable is an outlier: 4.1x slower than the field

carrier_cold_wideselect_fntable (122.48 us) is 4.1x the fastest (30.02 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cold_wideselect_null shows alternating (throttle bounce) (autocorr -0.75)

carrier_cold_wideselect_null's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cold_wideselect_null} vs {carrier_cold_wideselect_switch, carrier_cold_wideselect_threaded, carrier_cold_wideselect_fntable} (259% apart)

The field splits into a fast tier {carrier_cold_wideselect_null} and a slow tier {carrier_cold_wideselect_switch, carrier_cold_wideselect_threaded, carrier_cold_wideselect_fntable} with a 259% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 4.1x the fastest

Fastest carrier_cold_wideselect_null (30.02 us) to slowest carrier_cold_wideselect_fntable (122.48 us): 4.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_cold_wideselect_null** at 30022.7 ns median (-72.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 4.08x (fastest 30022.7 ns, slowest 122483.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 124817ns | 124771ns | 123987ns | 124538ns | 125650ns | +13.13% |
| carrier_cold_wideselect_null | 32416ns | 32348ns | 31370ns | 32104ns | 33409ns | -70.62% |
| carrier_cold_wideselect_switch | 110333ns | 110360ns | 108719ns | 109963ns | 111694ns | base |
| carrier_cold_wideselect_threaded | 112511ns | 111869ns | 109889ns | 111370ns | 115535ns | +1.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 122405ns | 121385ns | 123231ns | +13.49% | 0.008 |
| carrier_cold_wideselect_null | 30150ns | 29173ns | 31140ns | -72.05% | 0.034 |
| carrier_cold_wideselect_switch | 107853ns | 106151ns | 109194ns | base | 0.009 |
| carrier_cold_wideselect_threaded | 110098ns | 107610ns | 112924ns | +2.08% | 0.009 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 774616 | 913651 | 0.848 | 1.12× |
| carrier_cold_wideselect_null | 399644 | 1412859 | 0.283 | 0.58× |
| carrier_cold_wideselect_switch | 691034 | 692993 | 0.997 | 1.00× |
| carrier_cold_wideselect_threaded | 692507 | 777699 | 0.890 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.035 Gops/s** (carrier_cold_wideselect_null; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cold_wideselect_fntable | 0.008 | 23.8% |
| carrier_cold_wideselect_null | 0.034 | 97.2% |
| carrier_cold_wideselect_switch | 0.009 | 27.1% |
| carrier_cold_wideselect_threaded | 0.009 | 26.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cold_wideselect_fntable | 124817ns | 124817ns | +13.13% |
| carrier_cold_wideselect_null | 32416ns | 32416ns | -70.62% |
| carrier_cold_wideselect_switch | 110333ns | 110333ns | base |
| carrier_cold_wideselect_threaded | 112511ns | 112511ns | +1.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cold_wideselect_switch | 107836ns | base | --- | [106531, 109194] | --- | --- | --- | --- |
| carrier_cold_wideselect_fntable | 122484ns | +14517.3ns (+13.5%) | [+12714, +16422]ns | [121499, 123231] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_wideselect_null | 30023ns | -77416.4ns (-71.8%) | [-79171, -76523]ns | [29287, 31140] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cold_wideselect_threaded | 109493ns | +803.2ns (+0.7%) | [+63, +5867]ns | [107876, 112924] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cold_wideselect_switch | carrier_cold_wideselect_fntable | carrier_cold_wideselect_null | carrier_cold_wideselect_threaded |
|---|---|---|---|---|
| 1 | 107735ns | +14.3% | -71.6% | -0.1% |
| 2 | 106151ns | +16.2% | -72.3% | +9.3% |
| 3 | 107936ns | +12.7% | -70.7% | +1.7% |
| 4 | 106911ns | +14.6% | -72.7% | +1.2% |
| 5 | 108935ns | +11.4% | -72.2% | +0.2% |
| 6 | 109452ns | +11.9% | -72.8% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cold_wideselect_fntable | -0.119 | ok |
| carrier_cold_wideselect_null | -0.747 | HIGH- (thermal bounce) |
| carrier_cold_wideselect_switch | 0.092 | ok |
| carrier_cold_wideselect_threaded | -0.301 | moderate- |

**Consistency summary:**

- **carrier_cold_wideselect_fntable**: won 0/6, lost 6/6
- **carrier_cold_wideselect_null**: won 6/6, lost 0/6
- **carrier_cold_wideselect_threaded**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cold_wideselect_fntable | 122767.8ns | 122404.6ns | 100.3% | HIGH |
| carrier_cold_wideselect_null | 94573.1ns | 30149.9ns | 313.7% | HIGH |
| carrier_cold_wideselect_switch | 111576.6ns | 107853.4ns | 103.5% | HIGH |
| carrier_cold_wideselect_threaded | 110386.1ns | 110097.6ns | 100.3% | HIGH |

## Distribution (algo ns)

```
carrier_cold_wideselect_fntable (n=6, range 121384.6-123231.0 ns)
  121384.6 |########################################
  121476.9 |
  121569.2 |########################################
  121661.6 |
  121753.9 |
  121846.2 |
  121938.5 |
  122030.9 |
  122123.2 |
  122215.5 |
  122307.8 |
  122400.1 |########################################
  122492.5 |########################################
  122584.8 |
  122677.1 |
  122769.4 |
  122861.8 |
  122954.1 |
  123046.4 |########################################
  123138.7 |
  (0 below, 1 above range)

carrier_cold_wideselect_null (n=6, range 29172.9-31140.4 ns)
  29172.9 |########################################
  29271.3 |
  29369.7 |########################################
  29468.0 |
  29566.4 |
  29664.8 |
  29763.2 |########################################
  29861.5 |
  29959.9 |
  30058.3 |
  30156.7 |########################################
  30255.0 |
  30353.4 |
  30451.8 |
  30550.2 |########################################
  30648.5 |
  30746.9 |
  30845.3 |
  30943.7 |
  31042.0 |
  (0 below, 1 above range)

carrier_cold_wideselect_switch (n=6, range 106151.2-109193.5 ns)
  106151.2 |########################################
  106303.3 |
  106455.4 |
  106607.6 |
  106759.7 |########################################
  106911.8 |
  107063.9 |
  107216.0 |
  107368.1 |
  107520.3 |
  107672.4 |########################################
  107824.5 |########################################
  107976.6 |
  108128.7 |
  108280.8 |
  108433.0 |
  108585.1 |
  108737.2 |
  108889.3 |########################################
  109041.4 |
  (0 below, 1 above range)

carrier_cold_wideselect_threaded (n=6, range 107610.0-112923.9 ns)
  107610.0 |####################
  107875.7 |
  108141.4 |####################
  108407.1 |
  108672.8 |
  108938.5 |####################
  109204.2 |
  109469.9 |
  109735.6 |########################################
  110001.3 |
  110267.0 |
  110532.7 |
  110798.4 |
  111064.1 |
  111329.8 |
  111595.5 |
  111861.2 |
  112126.9 |
  112392.6 |
  112658.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cold_wideselect_fntable**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_null**: bridge=313.3% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_switch**: bridge=103.9% of algo (FFI overhead may distort results)
- **carrier_cold_wideselect_threaded**: bridge=100.1% of algo (FFI overhead may distort results)

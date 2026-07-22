# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (60.22 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 27.25 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 77% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (27.25 us) leads carrier_cfg_threaded (48.11 us) by 77%, a clear separation rather than a photo finish. CV 3.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 55% (significant)

carrier_cfg_trace is -32.88 us (55%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (60.22 us) is 2.2x the fastest (27.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cfg_threaded shows alternating (throttle bounce) (autocorr -0.60)

carrier_cfg_threaded's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (77% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 77% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 27252.3 ns median (-54.7% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.21x (fastest 27252.3 ns, slowest 60224.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 59595ns | 58941ns | 58532ns | 58888ns | 61186ns | -4.17% |
| carrier_cfg_switch | 62189ns | 62773ns | 60032ns | 62080ns | 63432ns | base |
| carrier_cfg_threaded | 51200ns | 50477ns | 48622ns | 49916ns | 54414ns | -17.67% |
| carrier_cfg_trace | 29577ns | 29669ns | 27935ns | 29444ns | 30596ns | -52.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 57194ns | 56017ns | 58790ns | -4.25% | 0.001 |
| carrier_cfg_switch | 59733ns | 57718ns | 60989ns | base | 0.001 |
| carrier_cfg_threaded | 48693ns | 46063ns | 51779ns | -18.48% | 0.001 |
| carrier_cfg_trace | 27175ns | 25708ns | 28119ns | -54.51% | 0.002 |

## Performance model

- Peak throughput: **0.002 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.4% |
| carrier_cfg_switch | 0.001 | 42.7% |
| carrier_cfg_threaded | 0.001 | 53.4% |
| carrier_cfg_trace | 0.002 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 59595ns | 59595ns | -4.17% |
| carrier_cfg_switch | 62189ns | 62189ns | base |
| carrier_cfg_threaded | 51200ns | 51200ns | -17.67% |
| carrier_cfg_trace | 29577ns | 29577ns | -52.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 60224ns | base | --- | [57986, 60989] | --- | --- | --- | --- |
| carrier_cfg_fntable | 56579ns | -1762.7ns (-2.9%) | [-4667, -1189]ns | [56213, 58790] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_threaded | 48108ns | -10405.8ns (-17.3%) | [-14140, -8575]ns | [46192, 51779] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_trace | 27252ns | -32880.6ns (-54.6%) | [-34070, -30724]ns | [26154, 28119] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 60422ns | -3.0% | -23.8% | -57.5% |
| 2 | 57718ns | -2.3% | -14.4% | -53.9% |
| 3 | 60027ns | -1.8% | -14.8% | -55.7% |
| 4 | 58254ns | -3.0% | -20.5% | -52.1% |
| 5 | 61250ns | -8.5% | -14.4% | -53.7% |
| 6 | 60728ns | -6.8% | -22.9% | -54.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.282 | moderate- |
| carrier_cfg_switch | -0.312 | moderate- |
| carrier_cfg_threaded | -0.597 | HIGH- (thermal bounce) |
| carrier_cfg_trace | 0.461 | moderate+ |

**Consistency summary:**

- **carrier_cfg_fntable**: won 6/6, lost 0/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 114432.9ns | 57193.8ns | 200.1% | HIGH |
| carrier_cfg_switch | 119576.4ns | 59733.2ns | 200.2% | HIGH |
| carrier_cfg_threaded | 97477.4ns | 48693.1ns | 200.2% | HIGH |
| carrier_cfg_trace | 100124.8ns | 27175.1ns | 368.4% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 56017.1-58789.8 ns)
  56017.1 |########################################
  56155.7 |
  56294.4 |########################################
  56433.0 |########################################
  56571.6 |########################################
  56710.3 |
  56848.9 |
  56987.5 |
  57126.2 |
  57264.8 |
  57403.4 |
  57542.1 |
  57680.7 |
  57819.3 |
  57958.0 |
  58096.6 |
  58235.2 |
  58373.9 |
  58512.5 |########################################
  58651.1 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 57717.9-60989.4 ns)
  57717.9 |########################################
  57881.5 |
  58045.0 |
  58208.6 |########################################
  58372.2 |
  58535.8 |
  58699.3 |
  58862.9 |
  59026.5 |
  59190.1 |
  59353.6 |
  59517.2 |
  59680.8 |
  59844.3 |
  60007.9 |########################################
  60171.5 |
  60335.1 |########################################
  60498.6 |
  60662.2 |########################################
  60825.8 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 46062.9-51778.9 ns)
  46062.9 |########################################
  46348.7 |
  46634.5 |####################
  46920.3 |
  47206.1 |
  47491.9 |
  47777.7 |
  48063.5 |
  48349.3 |
  48635.1 |
  48920.9 |
  49206.7 |####################
  49492.5 |
  49778.3 |
  50064.1 |
  50349.9 |
  50635.7 |
  50921.5 |####################
  51207.3 |
  51493.1 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 25708.3-28118.8 ns)
  25708.3 |####################
  25828.8 |
  25949.3 |
  26069.9 |
  26190.4 |
  26310.9 |
  26431.4 |
  26552.0 |########################################
  26672.5 |
  26793.0 |
  26913.5 |
  27034.0 |
  27154.6 |
  27275.1 |
  27395.6 |
  27516.1 |
  27636.7 |
  27757.2 |
  27877.7 |########################################
  27998.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=200.0% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=199.9% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=366.0% of algo (FFI overhead may distort results)

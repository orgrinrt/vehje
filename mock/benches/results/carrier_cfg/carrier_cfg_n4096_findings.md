# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (3.61 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 1.60 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 78% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (1.60 ms) leads carrier_cfg_threaded (2.85 ms) by 78%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 56% (significant)

carrier_cfg_trace is -2.01 ms (56%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (3.61 ms) is 2.2x the fastest (1.60 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (78% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 78% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 1603298.9 ns median (-55.5% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.25x (fastest 1603298.9 ns, slowest 3605536.8 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 3507936ns | 3505841ns | 3498745ns | 3504044ns | 3518369ns | -2.68% |
| carrier_cfg_switch | 3604651ns | 3609028ns | 3578955ns | 3604035ns | 3618423ns | base |
| carrier_cfg_threaded | 2845342ns | 2855137ns | 2817475ns | 2843585ns | 2861910ns | -21.06% |
| carrier_cfg_trace | 1608255ns | 1606400ns | 1580575ns | 1602918ns | 1630099ns | -55.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 3504451ns | 3495615ns | 3514970ns | -2.69% | 0.001 |
| carrier_cfg_switch | 3601173ns | 3575404ns | 3615084ns | base | 0.001 |
| carrier_cfg_threaded | 2841918ns | 2813777ns | 2858575ns | -21.08% | 0.001 |
| carrier_cfg_trace | 1605312ns | 1577711ns | 1627316ns | -55.42% | 0.003 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cfg_fntable | 21890512 | 51552782 | 0.425 | 0.97× |
| carrier_cfg_switch | 22541222 | 47226611 | 0.477 | 1.00× |
| carrier_cfg_threaded | 17830282 | 34377873 | 0.519 | 0.79× |
| carrier_cfg_trace | 10021023 | 39353694 | 0.255 | 0.44× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.1% |
| carrier_cfg_switch | 0.001 | 43.8% |
| carrier_cfg_threaded | 0.001 | 55.3% |
| carrier_cfg_trace | 0.003 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 3507936ns | 3507936ns | -2.68% |
| carrier_cfg_switch | 3604651ns | 3604651ns | base |
| carrier_cfg_threaded | 2845342ns | 2845342ns | -21.06% |
| carrier_cfg_trace | 1608255ns | 1608255ns | -55.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 3605537ns | base | --- | [3582899, 3615084] | --- | --- | --- | --- |
| carrier_cfg_fntable | 3502092ns | -99249.6ns (-2.8%) | [-108018, -82900]ns | [3496290, 3514970] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_threaded | 2851735ns | -758850.6ns (-21.0%) | [-769784, -749130]ns | [2815445, 2858575] | YES | 0.0313 | 0.0313 | 0 |
| carrier_cfg_trace | 1603299ns | -2010797.1ns (-55.8%) | [-2021204, -1955583]ns | [1585321, 1627316] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 3575404ns | -2.2% | -21.2% | -54.4% |
| 2 | 3607811ns | -3.1% | -20.7% | -56.3% |
| 3 | 3603263ns | -2.8% | -20.9% | -55.8% |
| 4 | 3590395ns | -2.4% | -21.6% | -54.7% |
| 5 | 3614356ns | -2.7% | -21.0% | -55.7% |
| 6 | 3615812ns | -2.9% | -21.1% | -55.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | 0.472 | moderate+ |
| carrier_cfg_switch | -0.106 | ok |
| carrier_cfg_threaded | -0.364 | moderate- |
| carrier_cfg_trace | -0.328 | moderate- |

**Consistency summary:**

- **carrier_cfg_fntable**: won 6/6, lost 0/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 3506065.2ns | 3504450.8ns | 100.0% | HIGH |
| carrier_cfg_switch | 3602296.7ns | 3601173.4ns | 100.0% | HIGH |
| carrier_cfg_threaded | 2843742.8ns | 2841918.5ns | 100.1% | HIGH |
| carrier_cfg_trace | 1603621.4ns | 1605312.1ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 3495615.0-3514970.4 ns)
  3495615.0 |########################################
  3496582.8 |########################################
  3497550.5 |
  3498518.3 |
  3499486.1 |
  3500453.9 |########################################
  3501421.6 |
  3502389.4 |########################################
  3503357.2 |
  3504324.9 |
  3505292.7 |
  3506260.5 |
  3507228.2 |
  3508196.0 |
  3509163.8 |
  3510131.6 |
  3511099.3 |########################################
  3512067.1 |
  3513034.9 |
  3514002.6 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 3575403.8-3615084.2 ns)
  3575403.8 |########################################
  3577387.8 |
  3579371.8 |
  3581355.9 |
  3583339.9 |
  3585323.9 |
  3587307.9 |
  3589291.9 |########################################
  3591275.9 |
  3593260.0 |
  3595244.0 |
  3597228.0 |
  3599212.0 |
  3601196.0 |
  3603180.0 |########################################
  3605164.1 |
  3607148.1 |########################################
  3609132.1 |
  3611116.1 |
  3613100.1 |########################################
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 2813777.1-2858575.4 ns)
  2813777.1 |########################################
  2816017.0 |########################################
  2818256.9 |
  2820496.8 |
  2822736.8 |
  2824976.7 |
  2827216.6 |
  2829456.5 |
  2831696.4 |
  2833936.3 |
  2836176.2 |
  2838416.2 |
  2840656.1 |
  2842896.0 |
  2845135.9 |
  2847375.8 |
  2849615.7 |########################################
  2851855.7 |########################################
  2854095.6 |########################################
  2856335.5 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 1577711.2-1627315.8 ns)
  1577711.2 |########################################
  1580191.4 |
  1582671.7 |
  1585151.9 |
  1587632.1 |
  1590112.3 |
  1592592.6 |########################################
  1595072.8 |
  1597553.0 |
  1600033.3 |########################################
  1602513.5 |########################################
  1604993.7 |
  1607474.0 |
  1609954.2 |
  1612434.4 |
  1614914.6 |
  1617394.9 |
  1619875.1 |
  1622355.3 |
  1624835.6 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=99.8% of algo (FFI overhead may distort results)

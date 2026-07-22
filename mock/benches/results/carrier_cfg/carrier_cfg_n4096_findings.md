# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (3.54 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 1.59 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 76% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (1.59 ms) leads carrier_cfg_threaded (2.80 ms) by 76%, a clear separation rather than a photo finish. CV 0.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 55% (significant)

carrier_cfg_trace is -1.96 ms (55%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.2x slower than the field

carrier_cfg_switch (3.54 ms) is 2.2x the fastest (1.59 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (76% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 76% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 1587602.9 ns median (-55.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.23x (fastest 1587602.9 ns, slowest 3544333.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 3621009ns | 3485801ns | 3439876ns | 3470550ns | 3937265ns | +2.12% |
| carrier_cfg_switch | 3545762ns | 3548042ns | 3527499ns | 3543046ns | 3558966ns | base |
| carrier_cfg_threaded | 3022417ns | 2802942ns | 2778248ns | 2794986ns | 3485649ns | -14.76% |
| carrier_cfg_trace | 1587877ns | 1591530ns | 1572465ns | 1586779ns | 1597230ns | -55.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 3617177ns | 3435890ns | 3933406ns | +2.12% | 0.001 |
| carrier_cfg_switch | 3542141ns | 3524887ns | 3555023ns | base | 0.001 |
| carrier_cfg_threaded | 3018737ns | 2774580ns | 3482009ns | -14.78% | 0.001 |
| carrier_cfg_trace | 1584377ns | 1569610ns | 1593951ns | -55.27% | 0.003 |

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 45.1% |
| carrier_cfg_switch | 0.001 | 44.3% |
| carrier_cfg_threaded | 0.001 | 56.1% |
| carrier_cfg_trace | 0.003 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 3621009ns | 3621009ns | +2.12% |
| carrier_cfg_switch | 3545762ns | 3545762ns | base |
| carrier_cfg_threaded | 3022417ns | 3022417ns | -14.76% |
| carrier_cfg_trace | 1587877ns | 1587877ns | -55.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 3544333ns | base | --- | [3527067, 3555023] | --- | --- | --- | --- |
| carrier_cfg_fntable | 3482050ns | no significant difference | [-101808, +392923]ns | [3436076, 3933406] | no | 0.6875 | 0.6875 | 0 |
| carrier_cfg_threaded | 2799572ns | -744761.6ns (-21.0%) | [-766976, -58474]ns | [2774632, 3482009] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| carrier_cfg_trace | 1587603ns | -1961931.6ns (-55.4%) | [-1972757, -1938604]ns | [1571577, 1593951] | YES (adj: no) | 0.0938 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 3529247ns | -2.6% | -21.4% | -55.0% |
| 2 | 3542146ns | -2.0% | -21.0% | -55.6% |
| 3 | 3556078ns | +18.5% | +16.3% | -55.3% |
| 4 | 3524887ns | +3.7% | -19.8% | -54.9% |
| 5 | 3553968ns | -1.7% | -21.9% | -55.0% |
| 6 | 3546521ns | -3.1% | -21.0% | -55.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.047 | ok |
| carrier_cfg_switch | -0.481 | moderate- |
| carrier_cfg_threaded | -0.204 | moderate- |
| carrier_cfg_trace | -0.303 | moderate- |

**Consistency summary:**

- **carrier_cfg_fntable**: won 4/6, lost 2/6
- **carrier_cfg_threaded**: won 5/6, lost 1/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 3598981.3ns | 3617177.2ns | 99.5% | HIGH |
| carrier_cfg_switch | 3543027.1ns | 3542141.2ns | 100.0% | HIGH |
| carrier_cfg_threaded | 2858543.9ns | 3018737.4ns | 94.7% | HIGH |
| carrier_cfg_trace | 1584141.9ns | 1584376.9ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 3435889.6-3933406.0 ns)
  3435889.6 |########################################
  3460765.4 |####################
  3485641.2 |####################
  3510517.1 |
  3535392.9 |
  3560268.7 |
  3585144.5 |
  3610020.4 |
  3634896.2 |####################
  3659772.0 |
  3684647.8 |
  3709523.6 |
  3734399.5 |
  3759275.3 |
  3784151.1 |
  3809026.9 |
  3833902.8 |
  3858778.6 |
  3883654.4 |
  3908530.2 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 3524887.1-3555023.1 ns)
  3524887.1 |########################################
  3526393.9 |
  3527900.7 |########################################
  3529407.5 |
  3530914.3 |
  3532421.1 |
  3533927.9 |
  3535434.7 |
  3536941.5 |
  3538448.3 |
  3539955.1 |
  3541461.9 |########################################
  3542968.7 |
  3544475.5 |
  3545982.3 |########################################
  3547489.1 |
  3548995.9 |
  3550502.7 |
  3552009.5 |
  3553516.3 |########################################
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 2774580.4-3482008.8 ns)
  2774580.4 |########################################
  2809951.8 |##########
  2845323.2 |
  2880694.7 |
  2916066.1 |
  2951437.5 |
  2986808.9 |
  3022180.3 |
  3057551.7 |
  3092923.2 |
  3128294.6 |
  3163666.0 |
  3199037.4 |
  3234408.8 |
  3269780.2 |
  3305151.7 |
  3340523.1 |
  3375894.5 |
  3411265.9 |
  3446637.3 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 1569610.0-1593951.2 ns)
  1569610.0 |########################################
  1570827.1 |
  1572044.1 |
  1573261.2 |########################################
  1574478.2 |
  1575695.3 |
  1576912.4 |
  1578129.4 |
  1579346.5 |
  1580563.6 |
  1581780.6 |
  1582997.7 |
  1584214.8 |
  1585431.8 |
  1586648.9 |########################################
  1587865.9 |########################################
  1589083.0 |########################################
  1590300.1 |
  1591517.1 |
  1592734.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=100.0% of algo (FFI overhead may distort results)

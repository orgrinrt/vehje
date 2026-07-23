# CFG register-VM dispatch (switch / fntable / threaded / trace)

4 variants, 6 samples per variant.
Baseline: **carrier_cfg_switch**

## Highlights

Baseline for all deltas below: **carrier_cfg_switch**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_cfg_switch) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_cfg_switch has the worst median (225.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_cfg_trace at 100.16 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_cfg_trace dominates: 80% faster than the next best (carrier_cfg_threaded)

carrier_cfg_trace (100.16 us) leads carrier_cfg_threaded (179.86 us) by 80%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_cfg_trace beats baseline by 56% (significant)

carrier_cfg_trace is -125.54 us (56%) faster than baseline carrier_cfg_switch, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_cfg_switch is an outlier: 2.3x slower than the field

carrier_cfg_switch (225.85 us) is 2.3x the fastest (100.16 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_cfg_switch shows alternating (throttle bounce) (autocorr -0.82)

carrier_cfg_switch's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {carrier_cfg_trace} vs {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} (80% apart)

The field splits into a fast tier {carrier_cfg_trace} and a slow tier {carrier_cfg_threaded, carrier_cfg_fntable, carrier_cfg_switch} with a 80% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: carrier_cfg_trace** at 100163.5 ns median (-55.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.25x (fastest 100163.5 ns, slowest 225854.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_cfg_fntable | 223922ns | 221878ns | 216319ns | 220363ns | 233063ns | -2.05% |
| carrier_cfg_switch | 228609ns | 228036ns | 224392ns | 226862ns | 233337ns | base |
| carrier_cfg_threaded | 181004ns | 182068ns | 178218ns | 180813ns | 182685ns | -20.82% |
| carrier_cfg_trace | 102141ns | 102301ns | 100118ns | 101934ns | 103462ns | -55.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_cfg_fntable | 221643ns | 214193ns | 230542ns | -2.10% | 0.001 |
| carrier_cfg_switch | 226402ns | 222259ns | 231056ns | base | 0.001 |
| carrier_cfg_threaded | 178816ns | 176009ns | 180505ns | -21.02% | 0.001 |
| carrier_cfg_trace | 100006ns | 98025ns | 101300ns | -55.83% | 0.003 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_cfg_fntable | 1384557 | 3246008 | 0.427 | 0.98× |
| carrier_cfg_switch | 1419871 | 2974956 | 0.477 | 1.00× |
| carrier_cfg_threaded | 1126498 | 2170273 | 0.519 | 0.79× |
| carrier_cfg_trace | 638277 | 2482908 | 0.257 | 0.45× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.003 Gops/s** (carrier_cfg_trace; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_cfg_fntable | 0.001 | 44.6% |
| carrier_cfg_switch | 0.001 | 43.4% |
| carrier_cfg_threaded | 0.001 | 54.5% |
| carrier_cfg_trace | 0.003 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_cfg_fntable | 223922ns | 223922ns | -2.05% |
| carrier_cfg_switch | 228609ns | 228609ns | base |
| carrier_cfg_threaded | 181004ns | 181004ns | -20.82% |
| carrier_cfg_trace | 102141ns | 102141ns | -55.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_cfg_switch | 225855ns | base | --- | [222296, 231056] | --- | --- | --- | --- |
| carrier_cfg_fntable | 219704ns | no significant difference | [-11655, +3757]ns | [214682, 230542] | no | 0.2188 | 0.2188 | 0 |
| carrier_cfg_threaded | 179860ns | -47031.2ns (-20.8%) | [-53020, -42707]ns | [176082, 180505] | YES | 0.0469 | 0.0313 | 0 |
| carrier_cfg_trace | 100164ns | -125536.7ns (-55.6%) | [-130866, -122785]ns | [98556, 101300] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_cfg_switch | carrier_cfg_fntable | carrier_cfg_threaded | carrier_cfg_trace |
|---|---|---|---|---|
| 1 | 222333ns | -0.3% | -19.2% | -55.9% |
| 2 | 232401ns | -6.3% | -24.3% | -56.8% |
| 3 | 222802ns | -3.9% | -19.2% | -54.9% |
| 4 | 229710ns | -2.5% | -21.6% | -56.5% |
| 5 | 222259ns | -3.2% | -20.7% | -55.4% |
| 6 | 228907ns | +3.6% | -21.0% | -55.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_cfg_fntable | -0.292 | moderate- |
| carrier_cfg_switch | -0.821 | HIGH- (thermal bounce) |
| carrier_cfg_threaded | -0.575 | HIGH- (thermal bounce) |
| carrier_cfg_trace | -0.272 | moderate- |

**Consistency summary:**

- **carrier_cfg_fntable**: won 5/6, lost 1/6
- **carrier_cfg_threaded**: won 6/6, lost 0/6
- **carrier_cfg_trace**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_cfg_fntable | 221491.4ns | 221643.0ns | 99.9% | HIGH |
| carrier_cfg_switch | 226065.9ns | 226402.1ns | 99.9% | HIGH |
| carrier_cfg_threaded | 178918.6ns | 178815.8ns | 100.1% | HIGH |
| carrier_cfg_trace | 99906.2ns | 100006.3ns | 99.9% | HIGH |

## Distribution (algo ns)

```
carrier_cfg_fntable (n=6, range 214192.9-230542.2 ns)
  214192.9 |########################################
  215010.4 |########################################
  215827.8 |
  216645.3 |
  217462.8 |########################################
  218280.2 |
  219097.7 |
  219915.2 |
  220732.6 |
  221550.1 |########################################
  222367.6 |
  223185.0 |
  224002.5 |########################################
  224820.0 |
  225637.4 |
  226454.9 |
  227272.4 |
  228089.8 |
  228907.3 |
  229724.8 |
  (0 below, 1 above range)

carrier_cfg_switch (n=6, range 222259.2-231055.6 ns)
  222259.2 |########################################
  222699.0 |####################
  223138.8 |
  223578.7 |
  224018.5 |
  224458.3 |
  224898.1 |
  225337.9 |
  225777.8 |
  226217.6 |
  226657.4 |
  227097.2 |
  227537.0 |
  227976.9 |
  228416.7 |
  228856.5 |####################
  229296.3 |####################
  229736.1 |
  230176.0 |
  230615.8 |
  (0 below, 1 above range)

carrier_cfg_threaded (n=6, range 176009.2-180505.2 ns)
  176009.2 |########################################
  176234.0 |
  176458.8 |
  176683.6 |
  176908.4 |
  177133.2 |
  177358.0 |
  177582.8 |
  177807.6 |
  178032.4 |
  178257.2 |
  178482.0 |
  178706.8 |
  178931.6 |
  179156.4 |
  179381.2 |
  179606.0 |####################
  179830.8 |####################
  180055.6 |####################
  180280.4 |
  (0 below, 1 above range)

carrier_cfg_trace (n=6, range 98025.4-101299.5 ns)
  98025.4 |####################
  98189.1 |
  98352.8 |
  98516.5 |
  98680.2 |
  98843.9 |
  99007.6 |####################
  99171.4 |
  99335.1 |
  99498.8 |
  99662.5 |
  99826.2 |####################
  99989.9 |
  100153.6 |
  100317.3 |########################################
  100481.0 |
  100644.7 |
  100808.4 |
  100972.1 |
  101135.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_cfg_fntable**: bridge=99.9% of algo (FFI overhead may distort results)
- **carrier_cfg_switch**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_cfg_threaded**: bridge=99.8% of algo (FFI overhead may distort results)
- **carrier_cfg_trace**: bridge=99.7% of algo (FFI overhead may distort results)

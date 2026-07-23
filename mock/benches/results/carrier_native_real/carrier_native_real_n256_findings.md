# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, real profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_real_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_real_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_real_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_real_interp has the worst median (10.82 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_real_direct at 3.74 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_real_direct beats baseline by 66% (significant)

carrier_nat_real_direct is -7.16 us (66%) faster than baseline carrier_nat_real_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_real_interp is an outlier: 2.9x slower than the field

carrier_nat_real_interp (10.82 us) is 2.9x the fastest (3.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_real_direct is fastest but the noisiest (CV 5.3%)

carrier_nat_real_direct wins on median (3.74 us) yet has the highest variance (CV 5.3%), while carrier_nat_real_copypatch is the steadiest (CV 4.7%, 3.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (carrier_nat_real_direct, carrier_nat_real_copypatch) are a dead heat (<1%)

carrier_nat_real_direct (3.74 us) and carrier_nat_real_copypatch (3.75 us) differ by 0.30%, inside the noise, even though the wider field spreads 189.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Speed leader carrier_nat_real_direct vs stability leader carrier_nat_real_copypatch (+0% speed for 1.1x steadier)

carrier_nat_real_direct is fastest (3.74 us, CV 5.3%); carrier_nat_real_copypatch gives up 0.3% median for 1.1x lower variance (CV 4.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_real_direct** at 3741.2 ns median (-65.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.89x (fastest 3741.2 ns, slowest 10815.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 6033ns | 6145ns | 5629ns | 5978ns | 6316ns | -54.04% |
| carrier_nat_real_direct | 6065ns | 6122ns | 5597ns | 5953ns | 6466ns | -53.80% |
| carrier_nat_real_interp | 13126ns | 13257ns | 12222ns | 12938ns | 13860ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_real_copypatch | 3682ns | 3443ns | 3845ns | -65.62% | 0.070 |
| carrier_nat_real_direct | 3688ns | 3431ns | 3886ns | -65.56% | 0.069 |
| carrier_nat_real_interp | 10710ns | 9993ns | 11286ns | base | 0.024 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 268810 | 865942 | 0.310 | 0.88× |
| carrier_nat_real_direct | 258658 | 828534 | 0.312 | 0.85× |
| carrier_nat_real_interp | 304630 | 1374227 | 0.222 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.075 Gops/s** (carrier_nat_real_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_real_copypatch | 0.068 | 91.4% |
| carrier_nat_real_direct | 0.068 | 91.7% |
| carrier_nat_real_interp | 0.024 | 31.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_real_copypatch | 6033ns | 6033ns | -54.04% |
| carrier_nat_real_direct | 6065ns | 6065ns | -53.80% |
| carrier_nat_real_interp | 13126ns | 13126ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_real_interp | 10816ns | base | --- | [10027, 11286] | --- | --- | --- | --- |
| carrier_nat_real_copypatch | 3752ns | -7020.8ns (-64.9%) | [-7508, -6556]ns | [3447, 3845] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_real_direct | 3741ns | -7164.2ns (-66.2%) | [-7407, -6493]ns | [3438, 3886] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_real_interp | carrier_nat_real_copypatch | carrier_nat_real_direct |
|---|---|---|---|
| 1 | 9993ns | -65.5% | -63.7% |
| 2 | 10415ns | -63.1% | -67.1% |
| 3 | 11308ns | -66.0% | -65.5% |
| 4 | 11264ns | -65.9% | -65.7% |
| 5 | 11217ns | -67.3% | -65.5% |
| 6 | 10061ns | -65.8% | -65.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_real_copypatch | 0.087 | ok |
| carrier_nat_real_direct | -0.067 | ok |
| carrier_nat_real_interp | 0.164 | ok |

**Consistency summary:**

- **carrier_nat_real_copypatch**: won 6/6, lost 0/6
- **carrier_nat_real_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_real_copypatch | 85541.0ns | 3681.6ns | 2323.4% | HIGH |
| carrier_nat_real_direct | 83147.7ns | 3688.5ns | 2254.3% | HIGH |
| carrier_nat_real_interp | 90900.5ns | 10709.7ns | 848.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_real_copypatch (n=6, range 3443.3-3845.4 ns)
   3443.3 |########################################
   3463.4 |
   3483.5 |
   3503.6 |
   3523.7 |
   3543.8 |
   3563.9 |
   3584.0 |
   3604.1 |
   3624.2 |
   3644.4 |
   3664.5 |####################
   3684.6 |
   3704.7 |
   3724.8 |
   3744.9 |
   3765.0 |
   3785.1 |
   3805.2 |
   3825.3 |########################################
  (0 below, 1 above range)

carrier_nat_real_direct (n=6, range 3430.8-3886.4 ns)
   3430.8 |########################################
   3453.6 |
   3476.4 |
   3499.1 |
   3521.9 |
   3544.7 |
   3567.5 |
   3590.3 |
   3613.1 |####################
   3635.8 |
   3658.6 |
   3681.4 |
   3704.2 |
   3727.0 |
   3749.8 |
   3772.5 |
   3795.3 |
   3818.1 |
   3840.9 |####################
   3863.7 |####################
  (0 below, 1 above range)

carrier_nat_real_interp (n=6, range 9993.3-11286.0 ns)
   9993.3 |########################################
  10057.9 |########################################
  10122.6 |
  10187.2 |
  10251.8 |
  10316.5 |
  10381.1 |########################################
  10445.7 |
  10510.4 |
  10575.0 |
  10639.6 |
  10704.3 |
  10768.9 |
  10833.6 |
  10898.2 |
  10962.8 |
  11027.5 |
  11092.1 |
  11156.7 |########################################
  11221.4 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_real_copypatch**: bridge=2276.6% of algo (FFI overhead may distort results)
- **carrier_nat_real_direct**: bridge=2173.7% of algo (FFI overhead may distort results)
- **carrier_nat_real_interp**: bridge=836.3% of algo (FFI overhead may distort results)

# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, madd profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_madd_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_madd_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_madd_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_madd_interp has the worst median (12.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_madd_stencil at 7.38 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_madd_stencil beats baseline by 41% (significant)

carrier_nat_madd_stencil is -5.09 us (41%) faster than baseline carrier_nat_madd_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_madd_stencil is fastest but the noisiest (CV 8.2%)

carrier_nat_madd_stencil wins on median (7.38 us) yet has the highest variance (CV 8.2%), while carrier_nat_madd_copypatch is the steadiest (CV 4.8%, 7.54 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### carrier_nat_madd_interp shows alternating (throttle bounce) (autocorr -0.65)

carrier_nat_madd_interp's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader carrier_nat_madd_stencil vs stability leader carrier_nat_madd_copypatch (+2% speed for 1.7x steadier)

carrier_nat_madd_stencil is fastest (7.38 us, CV 8.2%); carrier_nat_madd_copypatch gives up 2.2% median for 1.7x lower variance (CV 4.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_madd_stencil** at 7383.3 ns median (-41.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.69x (fastest 7383.3 ns, slowest 12509.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 9946ns | 10092ns | 8908ns | 10052ns | 10305ns | -32.87% |
| carrier_nat_madd_interp | 14816ns | 15146ns | 13360ns | 14699ns | 15719ns | base |
| carrier_nat_madd_stencil | 10015ns | 9901ns | 8921ns | 9815ns | 10862ns | -32.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_madd_copypatch | 7415ns | 6652ns | 7691ns | -39.43% | 0.035 |
| carrier_nat_madd_interp | 12242ns | 11140ns | 12925ns | base | 0.021 |
| carrier_nat_madd_stencil | 7487ns | 6670ns | 8095ns | -38.84% | 0.034 |

## Performance model

- Peak throughput: **0.038 Gops/s** (carrier_nat_madd_copypatch; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_madd_copypatch | 0.034 | 88.2% |
| carrier_nat_madd_interp | 0.020 | 53.2% |
| carrier_nat_madd_stencil | 0.035 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_madd_copypatch | 9946ns | 9946ns | -32.87% |
| carrier_nat_madd_interp | 14816ns | 14816ns | base |
| carrier_nat_madd_stencil | 10015ns | 10015ns | -32.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_madd_interp | 12509ns | base | --- | [11293, 12925] | --- | --- | --- | --- |
| carrier_nat_madd_copypatch | 7545ns | -4964.3ns (-39.7%) | [-5407, -4110]ns | [7010, 7691] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_madd_stencil | 7383ns | -5093.9ns (-40.7%) | [-5871, -3299]ns | [6984, 8095] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_madd_interp | carrier_nat_madd_copypatch | carrier_nat_madd_stencil |
|---|---|---|---|
| 1 | 11140ns | -40.3% | -22.0% |
| 2 | 13168ns | -41.8% | -49.3% |
| 3 | 11446ns | -32.6% | -36.2% |
| 4 | 12682ns | -41.9% | -41.4% |
| 5 | 12396ns | -39.6% | -40.9% |
| 6 | 12622ns | -39.8% | -40.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_madd_copypatch | -0.156 | ok |
| carrier_nat_madd_interp | -0.646 | HIGH- (thermal bounce) |
| carrier_nat_madd_stencil | -0.374 | moderate- |

**Consistency summary:**

- **carrier_nat_madd_copypatch**: won 6/6, lost 0/6
- **carrier_nat_madd_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_madd_copypatch | 91371.3ns | 7415.1ns | 1232.2% | HIGH |
| carrier_nat_madd_interp | 90897.6ns | 12242.3ns | 742.5% | HIGH |
| carrier_nat_madd_stencil | 93039.2ns | 7487.5ns | 1242.6% | HIGH |

## Distribution (algo ns)

```
carrier_nat_madd_copypatch (n=6, range 6651.7-7691.0 ns)
   6651.7 |########################################
   6703.7 |
   6755.6 |
   6807.6 |
   6859.6 |
   6911.5 |
   6963.5 |
   7015.5 |
   7067.4 |
   7119.4 |
   7171.4 |
   7223.3 |
   7275.3 |
   7327.2 |########################################
   7379.2 |
   7431.2 |
   7483.1 |########################################
   7535.1 |
   7587.1 |########################################
   7639.0 |########################################
  (0 below, 1 above range)

carrier_nat_madd_interp (n=6, range 11139.6-12925.0 ns)
  11139.6 |########################################
  11228.9 |
  11318.1 |
  11407.4 |########################################
  11496.7 |
  11586.0 |
  11675.2 |
  11764.5 |
  11853.8 |
  11943.0 |
  12032.3 |
  12121.6 |
  12210.8 |
  12300.1 |
  12389.4 |########################################
  12478.6 |
  12567.9 |########################################
  12657.2 |########################################
  12746.5 |
  12835.7 |
  (0 below, 1 above range)

carrier_nat_madd_stencil (n=6, range 6670.0-8094.8 ns)
   6670.0 |########################################
   6741.2 |
   6812.5 |
   6883.7 |
   6955.0 |
   7026.2 |
   7097.4 |
   7168.7 |
   7239.9 |########################################
   7311.2 |########################################
   7382.4 |########################################
   7453.6 |########################################
   7524.9 |
   7596.1 |
   7667.4 |
   7738.6 |
   7809.8 |
   7881.1 |
   7952.3 |
   8023.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_madd_copypatch**: bridge=1199.3% of algo (FFI overhead may distort results)
- **carrier_nat_madd_interp**: bridge=715.7% of algo (FFI overhead may distort results)
- **carrier_nat_madd_stencil**: bridge=1233.2% of algo (FFI overhead may distort results)

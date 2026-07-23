# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, wideselect profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_wideselect_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_wideselect_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_wideselect_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_wideselect_interp has the worst median (2.20 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_wideselect_copypatch at 640 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_wideselect_copypatch beats baseline by 73% (significant)

carrier_nat_wideselect_copypatch is -1.60 us (73%) faster than baseline carrier_nat_wideselect_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_wideselect_interp is an outlier: 3.4x slower than the field

carrier_nat_wideselect_interp (2.20 us) is 3.4x the fastest (640 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_wideselect_copypatch, carrier_nat_wideselect_direct) are a dead heat (<1%)

carrier_nat_wideselect_copypatch (640 ns) and carrier_nat_wideselect_direct (646 ns) differ by 0.91%, inside the noise, even though the wider field spreads 243.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Wide spread: slowest is 3.4x the fastest

Fastest carrier_nat_wideselect_copypatch (640 ns) to slowest carrier_nat_wideselect_interp (2.20 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_wideselect_copypatch** at 640.5 ns median (-70.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.43x (fastest 640.5 ns, slowest 2197.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 2979ns | 2950ns | 2740ns | 2901ns | 3214ns | -34.65% |
| carrier_nat_wideselect_direct | 3002ns | 2999ns | 2822ns | 2960ns | 3156ns | -34.13% |
| carrier_nat_wideselect_interp | 4558ns | 4470ns | 4289ns | 4454ns | 4848ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 634ns | 583ns | 672ns | -71.66% | 0.101 |
| carrier_nat_wideselect_direct | 641ns | 588ns | 679ns | -71.35% | 0.100 |
| carrier_nat_wideselect_interp | 2238ns | 2123ns | 2371ns | base | 0.029 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 217148 | 992418 | 0.219 | 0.81× |
| carrier_nat_wideselect_direct | 207678 | 945436 | 0.220 | 0.77× |
| carrier_nat_wideselect_interp | 268442 | 1461658 | 0.184 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.110 Gops/s** (carrier_nat_wideselect_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.100 | 91.1% |
| carrier_nat_wideselect_direct | 0.099 | 90.3% |
| carrier_nat_wideselect_interp | 0.029 | 26.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_wideselect_copypatch | 2979ns | 2979ns | -34.65% |
| carrier_nat_wideselect_direct | 3002ns | 3002ns | -34.13% |
| carrier_nat_wideselect_interp | 4558ns | 4558ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_wideselect_interp | 2198ns | base | --- | [2146, 2371] | --- | --- | --- | --- |
| carrier_nat_wideselect_copypatch | 640ns | -1595.4ns (-72.6%) | [-1699, -1517]ns | [590, 672] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_wideselect_direct | 646ns | -1592.9ns (-72.5%) | [-1695, -1504]ns | [598, 679] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_wideselect_interp | carrier_nat_wideselect_copypatch | carrier_nat_wideselect_direct |
|---|---|---|---|
| 1 | 2123ns | -71.9% | -71.3% |
| 2 | 2193ns | -71.6% | -71.6% |
| 3 | 2203ns | -73.5% | -73.3% |
| 4 | 2168ns | -69.6% | -68.9% |
| 5 | 2345ns | -71.4% | -70.8% |
| 6 | 2398ns | -71.9% | -72.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_wideselect_copypatch | 0.288 | moderate+ |
| carrier_nat_wideselect_direct | 0.308 | moderate+ |
| carrier_nat_wideselect_interp | 0.324 | moderate+ |

**Consistency summary:**

- **carrier_nat_wideselect_copypatch**: won 6/6, lost 0/6
- **carrier_nat_wideselect_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_wideselect_copypatch | 70080.1ns | 634.4ns | 11047.3% | HIGH |
| carrier_nat_wideselect_direct | 67859.2ns | 641.2ns | 10583.2% | HIGH |
| carrier_nat_wideselect_interp | 85976.0ns | 2238.4ns | 3841.0% | HIGH |

## Distribution (algo ns)

```
carrier_nat_wideselect_copypatch (n=6, range 583.3-672.2 ns)
    583.3 |########################################
    587.7 |
    592.2 |
    596.6 |########################################
    601.1 |
    605.5 |
    610.0 |
    614.4 |
    618.9 |########################################
    623.3 |
    627.8 |
    632.2 |
    636.7 |
    641.1 |
    645.6 |
    650.0 |
    654.5 |
    658.9 |########################################
    663.4 |
    667.8 |########################################
  (0 below, 1 above range)

carrier_nat_wideselect_direct (n=6, range 587.5-679.0 ns)
    587.5 |####################
    592.1 |
    596.6 |
    601.2 |
    605.8 |####################
    610.4 |
    615.0 |
    619.5 |####################
    624.1 |
    628.7 |
    633.2 |
    637.8 |
    642.4 |
    647.0 |
    651.5 |
    656.1 |
    660.7 |
    665.3 |
    669.9 |########################################
    674.4 |
  (0 below, 1 above range)

carrier_nat_wideselect_interp (n=6, range 2123.3-2371.4 ns)
   2123.3 |########################################
   2135.7 |
   2148.1 |
   2160.5 |########################################
   2172.9 |
   2185.3 |########################################
   2197.7 |########################################
   2210.2 |
   2222.6 |
   2235.0 |
   2247.4 |
   2259.8 |
   2272.2 |
   2284.6 |
   2297.0 |
   2309.4 |
   2321.8 |
   2334.2 |########################################
   2346.6 |
   2359.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_wideselect_copypatch**: bridge=10717.7% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_direct**: bridge=10286.4% of algo (FFI overhead may distort results)
- **carrier_nat_wideselect_interp**: bridge=3906.4% of algo (FFI overhead may distort results)

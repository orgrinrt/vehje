# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, scatter profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_scatter_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_scatter_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_scatter_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_scatter_interp has the worst median (2.23 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_scatter_copypatch at 553 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_scatter_direct beats baseline by 76% (significant)

carrier_nat_scatter_direct is -1.69 us (76%) faster than baseline carrier_nat_scatter_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_scatter_interp is an outlier: 4.0x slower than the field

carrier_nat_scatter_interp (2.23 us) is 4.0x the fastest (553 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_scatter_copypatch is fastest but the noisiest (CV 7.6%)

carrier_nat_scatter_copypatch wins on median (553 ns) yet has the highest variance (CV 7.6%), while carrier_nat_scatter_interp is the steadiest (CV 5.1%, 2.23 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.0x the fastest

Fastest carrier_nat_scatter_copypatch (553 ns) to slowest carrier_nat_scatter_interp (2.23 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: carrier_nat_scatter_copypatch** at 553.4 ns median (-75.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.04x (fastest 553.4 ns, slowest 2233.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 2903ns | 2760ns | 2709ns | 2754ns | 3222ns | -36.64% |
| carrier_nat_scatter_direct | 2844ns | 2801ns | 2700ns | 2783ns | 3006ns | -37.93% |
| carrier_nat_scatter_interp | 4582ns | 4480ns | 4314ns | 4452ns | 4910ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 578ns | 545ns | 637ns | -74.81% | 0.111 |
| carrier_nat_scatter_direct | 573ns | 543ns | 611ns | -75.03% | 0.112 |
| carrier_nat_scatter_interp | 2296ns | 2190ns | 2455ns | base | 0.028 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 223376 | 1016330 | 0.220 | 0.81× |
| carrier_nat_scatter_direct | 224024 | 1038904 | 0.216 | 0.82× |
| carrier_nat_scatter_interp | 274296 | 1389556 | 0.197 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.118 Gops/s** (carrier_nat_scatter_direct; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_scatter_copypatch | 0.116 | 98.1% |
| carrier_nat_scatter_direct | 0.113 | 96.2% |
| carrier_nat_scatter_interp | 0.029 | 24.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_scatter_copypatch | 2903ns | 2903ns | -36.64% |
| carrier_nat_scatter_direct | 2844ns | 2844ns | -37.93% |
| carrier_nat_scatter_interp | 4582ns | 4582ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_scatter_interp | 2233ns | base | --- | [2200, 2455] | --- | --- | --- | --- |
| carrier_nat_scatter_copypatch | 553ns | -1682.2ns (-75.3%) | [-1858, -1613]ns | [545, 637] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_scatter_direct | 565ns | -1689.2ns (-75.6%) | [-1844, -1636]ns | [544, 611] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_scatter_interp | carrier_nat_scatter_copypatch | carrier_nat_scatter_direct |
|---|---|---|---|
| 1 | 2190ns | -71.5% | -73.4% |
| 2 | 2219ns | -74.9% | -75.5% |
| 3 | 2211ns | -75.2% | -75.3% |
| 4 | 2248ns | -75.8% | -75.7% |
| 5 | 2475ns | -78.0% | -76.4% |
| 6 | 2435ns | -73.3% | -73.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_scatter_copypatch | -0.059 | ok |
| carrier_nat_scatter_direct | 0.250 | moderate+ |
| carrier_nat_scatter_interp | 0.450 | moderate+ |

**Consistency summary:**

- **carrier_nat_scatter_copypatch**: won 6/6, lost 0/6
- **carrier_nat_scatter_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_scatter_copypatch | 72523.8ns | 578.5ns | 12536.9% | HIGH |
| carrier_nat_scatter_direct | 70813.3ns | 573.4ns | 12350.1% | HIGH |
| carrier_nat_scatter_interp | 85948.3ns | 2296.4ns | 3742.8% | HIGH |

## Distribution (algo ns)

```
carrier_nat_scatter_copypatch (n=6, range 545.0-637.1 ns)
    545.0 |########################################
    549.6 |
    554.2 |#############
    558.8 |
    563.4 |
    568.0 |
    572.6 |
    577.2 |
    581.8 |
    586.4 |
    591.0 |
    595.7 |
    600.3 |
    604.9 |
    609.5 |
    614.1 |
    618.7 |
    623.3 |#############
    627.9 |
    632.5 |
  (0 below, 1 above range)

carrier_nat_scatter_direct (n=6, range 542.9-611.5 ns)
    542.9 |########################################
    546.3 |
    549.8 |
    553.2 |
    556.6 |
    560.0 |
    563.5 |
    566.9 |
    570.3 |
    573.7 |
    577.2 |
    580.6 |#############
    584.0 |#############
    587.5 |
    590.9 |
    594.3 |
    597.7 |
    601.2 |
    604.6 |
    608.0 |
  (0 below, 1 above range)

carrier_nat_scatter_interp (n=6, range 2190.0-2455.4 ns)
   2190.0 |########################################
   2203.3 |########################################
   2216.5 |########################################
   2229.8 |
   2243.1 |########################################
   2256.3 |
   2269.6 |
   2282.9 |
   2296.2 |
   2309.4 |
   2322.7 |
   2336.0 |
   2349.2 |
   2362.5 |
   2375.8 |
   2389.1 |
   2402.3 |
   2415.6 |
   2428.9 |########################################
   2442.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_scatter_copypatch**: bridge=12244.6% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_direct**: bridge=11950.6% of algo (FFI overhead may distort results)
- **carrier_nat_scatter_interp**: bridge=3851.2% of algo (FFI overhead may distort results)

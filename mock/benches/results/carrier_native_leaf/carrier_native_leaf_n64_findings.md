# Near-native tier: interp vs direct instruction-selection vs copy-and-patch stencil, leaf profile

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (2.13 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_copypatch at 527 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_copypatch beats baseline by 74% (significant)

carrier_nat_leaf_copypatch is -1.59 us (74%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 4.0x slower than the field

carrier_nat_leaf_interp (2.13 us) is 4.0x the fastest (527 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.0x the fastest

Fastest carrier_nat_leaf_copypatch (527 ns) to slowest carrier_nat_leaf_interp (2.13 us): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader carrier_nat_leaf_copypatch vs stability leader carrier_nat_leaf_direct (+1% speed for 1.2x steadier)

carrier_nat_leaf_copypatch is fastest (527 ns, CV 4.8%); carrier_nat_leaf_direct gives up 1.0% median for 1.2x lower variance (CV 3.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_leaf_copypatch** at 526.9 ns median (-75.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.04x (fastest 526.9 ns, slowest 2128.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 2880ns | 2902ns | 2670ns | 2835ns | 3052ns | -36.14% |
| carrier_nat_leaf_direct | 2944ns | 2931ns | 2770ns | 2890ns | 3112ns | -34.72% |
| carrier_nat_leaf_interp | 4510ns | 4657ns | 4050ns | 4500ns | 4754ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 528ns | 500ns | 558ns | -74.55% | 0.121 |
| carrier_nat_leaf_direct | 532ns | 505ns | 553ns | -74.39% | 0.120 |
| carrier_nat_leaf_interp | 2076ns | 1893ns | 2174ns | base | 0.031 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 231449 | 1013556 | 0.228 | 0.88× |
| carrier_nat_leaf_direct | 239714 | 1055512 | 0.227 | 0.92× |
| carrier_nat_leaf_interp | 261557 | 1485448 | 0.176 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.128 Gops/s** (carrier_nat_leaf_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.121 | 94.9% |
| carrier_nat_leaf_direct | 0.120 | 93.9% |
| carrier_nat_leaf_interp | 0.030 | 23.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 2880ns | 2880ns | -36.14% |
| carrier_nat_leaf_direct | 2944ns | 2944ns | -34.72% |
| carrier_nat_leaf_interp | 4510ns | 4510ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 2129ns | base | --- | [1925, 2174] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 527ns | -1585.4ns (-74.5%) | [-1639, -1419]ns | [500, 558] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_direct | 532ns | -1575.8ns (-74.0%) | [-1641, -1416]ns | [509, 553] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_direct |
|---|---|---|---|
| 1 | 2199ns | -75.4% | -76.6% |
| 2 | 2120ns | -76.4% | -73.8% |
| 3 | 1957ns | -73.8% | -73.8% |
| 4 | 1893ns | -73.6% | -73.3% |
| 5 | 2148ns | -73.8% | -74.4% |
| 6 | 2138ns | -74.1% | -74.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.109 | ok |
| carrier_nat_leaf_direct | -0.172 | ok |
| carrier_nat_leaf_interp | 0.179 | ok |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_direct**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 75962.7ns | 528.2ns | 14380.1% | HIGH |
| carrier_nat_leaf_direct | 80063.9ns | 531.6ns | 15060.9% | HIGH |
| carrier_nat_leaf_interp | 86492.8ns | 2075.8ns | 4166.7% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 500.0-557.9 ns)
    500.0 |########################################
    502.9 |
    505.8 |
    508.7 |
    511.6 |####################
    514.5 |
    517.4 |
    520.3 |
    523.2 |
    526.1 |
    529.0 |
    531.8 |
    534.7 |
    537.6 |
    540.5 |####################
    543.4 |
    546.3 |
    549.2 |
    552.1 |####################
    555.0 |
  (0 below, 1 above range)

carrier_nat_leaf_direct (n=6, range 505.4-553.2 ns)
    505.4 |####################
    507.8 |
    510.2 |
    512.6 |########################################
    515.0 |
    517.3 |
    519.7 |
    522.1 |
    524.5 |
    526.9 |
    529.3 |
    531.7 |
    534.1 |
    536.4 |
    538.8 |
    541.2 |
    543.6 |
    546.0 |
    548.4 |####################
    550.8 |####################
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 1892.9-2173.6 ns)
   1892.9 |########################################
   1906.9 |
   1921.0 |
   1935.0 |
   1949.0 |########################################
   1963.1 |
   1977.1 |
   1991.1 |
   2005.2 |
   2019.2 |
   2033.2 |
   2047.3 |
   2061.3 |
   2075.3 |
   2089.4 |
   2103.4 |
   2117.4 |########################################
   2131.5 |########################################
   2145.5 |########################################
   2159.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=14211.4% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_direct**: bridge=15261.6% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=4063.3% of algo (FFI overhead may distort results)

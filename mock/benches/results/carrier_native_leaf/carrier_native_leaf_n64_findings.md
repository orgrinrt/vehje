# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, leaf profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_leaf_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_leaf_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_leaf_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_leaf_interp has the worst median (1.95 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_leaf_copypatch at 545 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_leaf_stencil beats baseline by 73% (significant)

carrier_nat_leaf_stencil is -1.42 us (73%) faster than baseline carrier_nat_leaf_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_leaf_interp is an outlier: 3.6x slower than the field

carrier_nat_leaf_interp (1.95 us) is 3.6x the fastest (545 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (carrier_nat_leaf_copypatch, carrier_nat_leaf_stencil) are a dead heat (<1%)

carrier_nat_leaf_copypatch (545 ns) and carrier_nat_leaf_stencil (550 ns) differ by 0.91%, inside the noise, even though the wider field spreads 257.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### carrier_nat_leaf_interp shows alternating (throttle bounce) (autocorr -0.57)

carrier_nat_leaf_interp's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.6x the fastest

Fastest carrier_nat_leaf_copypatch (545 ns) to slowest carrier_nat_leaf_interp (1.95 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader carrier_nat_leaf_copypatch vs stability leader carrier_nat_leaf_stencil (+1% speed for 1.0x steadier)

carrier_nat_leaf_copypatch is fastest (545 ns, CV 4.8%); carrier_nat_leaf_stencil gives up 0.9% median for 1.0x lower variance (CV 4.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: carrier_nat_leaf_copypatch** at 544.8 ns median (-72.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.57x (fastest 544.8 ns, slowest 1945.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 3049ns | 3091ns | 2798ns | 3075ns | 3134ns | -30.00% |
| carrier_nat_leaf_interp | 4355ns | 4292ns | 4057ns | 4242ns | 4675ns | base |
| carrier_nat_leaf_stencil | 2988ns | 3059ns | 2793ns | 2976ns | 3105ns | -31.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 538ns | 487ns | 562ns | -73.03% | 0.119 |
| carrier_nat_leaf_interp | 1994ns | 1878ns | 2156ns | base | 0.032 |
| carrier_nat_leaf_stencil | 538ns | 501ns | 562ns | -73.03% | 0.119 |

## Performance model

- Peak throughput: **0.131 Gops/s** (carrier_nat_leaf_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_leaf_copypatch | 0.117 | 89.4% |
| carrier_nat_leaf_interp | 0.033 | 25.0% |
| carrier_nat_leaf_stencil | 0.116 | 88.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_leaf_copypatch | 3049ns | 3049ns | -30.00% |
| carrier_nat_leaf_interp | 4355ns | 4355ns | base |
| carrier_nat_leaf_stencil | 2988ns | 2988ns | -31.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_leaf_interp | 1945ns | base | --- | [1882, 2156] | --- | --- | --- | --- |
| carrier_nat_leaf_copypatch | 545ns | -1405.6ns (-72.3%) | [-1603, -1360]ns | [507, 562] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_leaf_stencil | 550ns | -1422.1ns (-73.1%) | [-1597, -1350]ns | [502, 562] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_leaf_interp | carrier_nat_leaf_copypatch | carrier_nat_leaf_stencil |
|---|---|---|---|
| 1 | 1928ns | -72.7% | -70.9% |
| 2 | 2165ns | -73.7% | -73.9% |
| 3 | 1962ns | -71.9% | -74.5% |
| 4 | 1878ns | -74.1% | -70.9% |
| 5 | 2147ns | -75.0% | -74.2% |
| 6 | 1885ns | -70.5% | -73.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_leaf_copypatch | -0.148 | ok |
| carrier_nat_leaf_interp | -0.570 | HIGH- (thermal bounce) |
| carrier_nat_leaf_stencil | -0.270 | moderate- |

**Consistency summary:**

- **carrier_nat_leaf_copypatch**: won 6/6, lost 0/6
- **carrier_nat_leaf_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_leaf_copypatch | 77247.3ns | 537.9ns | 14360.0% | HIGH |
| carrier_nat_leaf_interp | 86701.2ns | 1994.2ns | 4347.6% | HIGH |
| carrier_nat_leaf_stencil | 74954.3ns | 537.8ns | 13936.3% | HIGH |

## Distribution (algo ns)

```
carrier_nat_leaf_copypatch (n=6, range 487.1-562.1 ns)
    487.1 |########################################
    490.9 |
    494.6 |
    498.4 |
    502.1 |
    505.9 |
    509.6 |
    513.4 |
    517.1 |
    520.9 |
    524.6 |########################################
    528.3 |
    532.1 |
    535.8 |########################################
    539.6 |
    543.3 |
    547.1 |
    550.8 |########################################
    554.6 |########################################
    558.3 |
  (0 below, 1 above range)

carrier_nat_leaf_interp (n=6, range 1878.3-2156.1 ns)
   1878.3 |########################################
   1892.2 |
   1906.1 |
   1920.0 |####################
   1933.8 |
   1947.7 |
   1961.6 |####################
   1975.5 |
   1989.4 |
   2003.3 |
   2017.2 |
   2031.1 |
   2045.0 |
   2058.8 |
   2072.7 |
   2086.6 |
   2100.5 |
   2114.4 |
   2128.3 |
   2142.2 |####################
  (0 below, 1 above range)

carrier_nat_leaf_stencil (n=6, range 501.2-562.1 ns)
    501.2 |########################################
    504.2 |
    507.3 |
    510.3 |
    513.4 |
    516.4 |
    519.5 |
    522.5 |
    525.6 |
    528.6 |
    531.6 |
    534.7 |
    537.7 |
    540.8 |
    543.8 |####################
    546.9 |
    549.9 |
    553.0 |####################
    556.0 |
    559.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_leaf_copypatch**: bridge=14179.4% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_interp**: bridge=4449.7% of algo (FFI overhead may distort results)
- **carrier_nat_leaf_stencil**: bridge=13368.9% of algo (FFI overhead may distort results)

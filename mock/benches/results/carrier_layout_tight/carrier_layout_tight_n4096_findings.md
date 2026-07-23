# Record layout (REC12..REC32) with fixed switch dispatch, tight profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_tight_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_tight_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 1.1% of the fastest

All 5 variants sit between 188.40 us and 190.54 us - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_tight_rec20** at 188395.9 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 188395.9 ns, slowest 190537.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 190219ns | 190747ns | 186940ns | 189967ns | 192236ns | -0.06% |
| carrier_lay_tight_rec16 | 201738ns | 191147ns | 187762ns | 190420ns | 225702ns | +5.99% |
| carrier_lay_tight_rec20 | 190743ns | 190605ns | 187104ns | 190428ns | 193034ns | +0.22% |
| carrier_lay_tight_rec24 | 190333ns | 190861ns | 187258ns | 189875ns | 192557ns | base |
| carrier_lay_tight_rec32 | 192786ns | 192808ns | 191182ns | 192715ns | 193695ns | +1.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_tight_rec12 | 188007ns | 184759ns | 189995ns | -0.05% | 0.022 |
| carrier_lay_tight_rec16 | 199324ns | 185578ns | 222941ns | +5.97% | 0.021 |
| carrier_lay_tight_rec20 | 188517ns | 184950ns | 190733ns | +0.22% | 0.022 |
| carrier_lay_tight_rec24 | 188103ns | 185016ns | 190262ns | base | 0.022 |
| carrier_lay_tight_rec32 | 190547ns | 189007ns | 191440ns | +1.30% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 1176096 | 4800724 | 0.245 | 1.00× |
| carrier_lay_tight_rec16 | 1209589 | 4802430 | 0.252 | 1.03× |
| carrier_lay_tight_rec20 | 1172437 | 4800727 | 0.244 | 1.00× |
| carrier_lay_tight_rec24 | 1176841 | 4800530 | 0.245 | 1.00× |
| carrier_lay_tight_rec32 | 1183711 | 4800618 | 0.247 | 1.01× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_tight_rec12; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_tight_rec12 | 0.022 | 98.0% |
| carrier_lay_tight_rec16 | 0.022 | 97.8% |
| carrier_lay_tight_rec20 | 0.022 | 98.1% |
| carrier_lay_tight_rec24 | 0.022 | 97.9% |
| carrier_lay_tight_rec32 | 0.021 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_tight_rec12 | 190219ns | 190219ns | -0.06% |
| carrier_lay_tight_rec16 | 201738ns | 201738ns | +5.99% |
| carrier_lay_tight_rec20 | 190743ns | 190743ns | +0.22% |
| carrier_lay_tight_rec24 | 190333ns | 190333ns | base |
| carrier_lay_tight_rec32 | 192786ns | 192786ns | +1.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_tight_rec24 | 188671ns | base | --- | [185376, 190262] | --- | --- | --- | --- |
| carrier_lay_tight_rec12 | 188534ns | no significant difference | [-2244, +2390]ns | [185491, 189995] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec16 | 188839ns | no significant difference | [-549, +33155]ns | [186191, 222941] | no | 0.4375 | 0.2188 | 0 |
| carrier_lay_tight_rec20 | 188396ns | no significant difference | [-2248, +3019]ns | [186422, 190733] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_tight_rec32 | 190537ns | +2152.9ns (+1.1%) | [+168, +5011]ns | [189664, 191440] | YES (adj: no) | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_tight_rec24 | carrier_lay_tight_rec12 | carrier_lay_tight_rec16 | carrier_lay_tight_rec20 | carrier_lay_tight_rec32 |
|---|---|---|---|---|---|
| 1 | 190122ns | -1.3% | -0.9% | +0.0% | +0.3% |
| 2 | 190402ns | -0.3% | +34.1% | +0.5% | +0.7% |
| 3 | 189170ns | +0.2% | +0.8% | -0.7% | -0.1% |
| 4 | 185737ns | +2.4% | +0.6% | +1.4% | +2.5% |
| 5 | 185016ns | -0.1% | +0.3% | +1.9% | +2.9% |
| 6 | 188171ns | -1.0% | +0.6% | -1.7% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_tight_rec12 | 0.157 | ok |
| carrier_lay_tight_rec16 | -0.179 | ok |
| carrier_lay_tight_rec20 | 0.130 | ok |
| carrier_lay_tight_rec24 | 0.455 | moderate+ |
| carrier_lay_tight_rec32 | -0.392 | moderate- |

**Consistency summary:**

- **carrier_lay_tight_rec12**: won 4/6, lost 2/6
- **carrier_lay_tight_rec16**: won 1/6, lost 5/6
- **carrier_lay_tight_rec20**: won 2/6, lost 3/6
- **carrier_lay_tight_rec32**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_tight_rec12 | 188472.6ns | 188006.5ns | 100.2% | HIGH |
| carrier_lay_tight_rec16 | 200450.6ns | 199323.8ns | 100.6% | HIGH |
| carrier_lay_tight_rec20 | 188790.6ns | 188517.1ns | 100.1% | HIGH |
| carrier_lay_tight_rec24 | 188921.9ns | 188103.1ns | 100.4% | HIGH |
| carrier_lay_tight_rec32 | 190727.4ns | 190546.9ns | 100.1% | HIGH |

## Distribution (algo ns)

```
carrier_lay_tight_rec12 (n=6, range 184759.2-189994.5 ns)
  184759.2 |########################################
  185021.0 |
  185282.7 |
  185544.5 |
  185806.3 |
  186068.0 |########################################
  186329.8 |
  186591.6 |
  186853.3 |
  187115.1 |
  187376.9 |########################################
  187638.6 |
  187900.4 |
  188162.2 |
  188423.9 |
  188685.7 |
  188947.5 |
  189209.2 |
  189471.0 |########################################
  189732.8 |########################################
  (0 below, 1 above range)

carrier_lay_tight_rec16 (n=6, range 185577.5-222941.0 ns)
  185577.5 |########################################
  187445.7 |########################################
  189313.9 |####################
  191182.0 |
  193050.2 |
  194918.4 |
  196786.6 |
  198654.7 |
  200522.9 |
  202391.1 |
  204259.3 |
  206127.5 |
  207995.6 |
  209863.8 |
  211732.0 |
  213600.2 |
  215468.3 |
  217336.5 |
  219204.7 |
  221072.9 |
  (0 below, 1 above range)

carrier_lay_tight_rec20 (n=6, range 184950.4-190733.1 ns)
  184950.4 |########################################
  185239.5 |
  185528.7 |
  185817.8 |
  186106.9 |
  186396.1 |
  186685.2 |
  186974.3 |
  187263.5 |
  187552.6 |
  187841.8 |########################################
  188130.9 |########################################
  188420.0 |########################################
  188709.2 |
  188998.3 |
  189287.4 |
  189576.6 |
  189865.7 |########################################
  190154.8 |
  190444.0 |
  (0 below, 1 above range)

carrier_lay_tight_rec24 (n=6, range 185016.2-190262.1 ns)
  185016.2 |########################################
  185278.5 |
  185540.8 |########################################
  185803.1 |
  186065.4 |
  186327.7 |
  186590.0 |
  186852.3 |
  187114.6 |
  187376.9 |
  187639.2 |
  187901.4 |
  188163.7 |########################################
  188426.0 |
  188688.3 |
  188950.6 |########################################
  189212.9 |
  189475.2 |
  189737.5 |
  189999.8 |########################################
  (0 below, 1 above range)

carrier_lay_tight_rec32 (n=6, range 189007.1-191439.8 ns)
  189007.1 |########################################
  189128.7 |
  189250.4 |
  189372.0 |
  189493.6 |
  189615.3 |
  189736.9 |
  189858.5 |
  189980.2 |
  190101.8 |
  190223.4 |########################################
  190345.1 |########################################
  190466.7 |
  190588.3 |########################################
  190710.0 |
  190831.6 |
  190953.2 |
  191074.9 |
  191196.5 |########################################
  191318.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_tight_rec12**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec16**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec24**: bridge=100.4% of algo (FFI overhead may distort results)
- **carrier_lay_tight_rec32**: bridge=100.1% of algo (FFI overhead may distort results)

# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 0.3% of the fastest

All 5 variants sit between 770.33 us and 772.73 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_madd_rec20** at 770330.0 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 770330.0 ns, slowest 772726.7 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 774641ns | 774007ns | 773329ns | 773918ns | 776380ns | -0.02% |
| carrier_lay_madd_rec16 | 775114ns | 774627ns | 772863ns | 774241ns | 777549ns | +0.04% |
| carrier_lay_madd_rec20 | 773924ns | 773896ns | 772998ns | 773797ns | 774579ns | -0.11% |
| carrier_lay_madd_rec24 | 774795ns | 773955ns | 772799ns | 773791ns | 777299ns | base |
| carrier_lay_madd_rec32 | 776774ns | 776026ns | 773055ns | 775477ns | 780579ns | +0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 771058ns | 769690ns | 773101ns | -0.04% | 0.021 |
| carrier_lay_madd_rec16 | 771599ns | 769377ns | 773732ns | +0.03% | 0.021 |
| carrier_lay_madd_rec20 | 770391ns | 769818ns | 770866ns | -0.12% | 0.021 |
| carrier_lay_madd_rec24 | 771331ns | 769315ns | 773836ns | base | 0.021 |
| carrier_lay_madd_rec32 | 773468ns | 769479ns | 777624ns | +0.28% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 4859621 | 19148357 | 0.254 | 1.00× |
| carrier_lay_madd_rec16 | 4861875 | 19148771 | 0.254 | 1.00× |
| carrier_lay_madd_rec20 | 4860850 | 19148289 | 0.254 | 1.00× |
| carrier_lay_madd_rec24 | 4860850 | 19148685 | 0.254 | 1.00× |
| carrier_lay_madd_rec32 | 4867030 | 19148529 | 0.254 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.021 Gops/s** (carrier_lay_madd_rec24; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 99.9% |
| carrier_lay_madd_rec16 | 0.021 | 99.7% |
| carrier_lay_madd_rec20 | 0.021 | 99.9% |
| carrier_lay_madd_rec24 | 0.021 | 99.9% |
| carrier_lay_madd_rec32 | 0.021 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 774641ns | 774641ns | -0.02% |
| carrier_lay_madd_rec16 | 775114ns | 775114ns | +0.04% |
| carrier_lay_madd_rec20 | 773924ns | 773924ns | -0.11% |
| carrier_lay_madd_rec24 | 774795ns | 774795ns | base |
| carrier_lay_madd_rec32 | 776774ns | 776774ns | +0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 770440ns | base | --- | [769716, 773836] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 770360ns | no significant difference | [-3712, +3385]ns | [769712, 773101] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec16 | 771276ns | no significant difference | [-3911, +4016]ns | [769789, 773732] | no | 1.0000 | 1.0000 | 0 |
| carrier_lay_madd_rec20 | 770330ns | no significant difference | [-3190, +833]ns | [769977, 770866] | no | 0.9167 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 772727ns | no significant difference | [-3188, +7184]ns | [770053, 777624] | no | 0.8750 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 770118ns | +0.1% | +0.6% | +0.0% | +0.1% |
| 2 | 776362ns | -0.9% | -0.8% | -0.7% | -0.9% |
| 3 | 769315ns | +0.8% | +0.5% | +0.2% | +0.4% |
| 4 | 770339ns | -0.1% | -0.0% | -0.1% | +1.1% |
| 5 | 770541ns | -0.0% | +0.2% | -0.1% | +0.7% |
| 6 | 771310ns | -0.1% | -0.3% | -0.1% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.433 | moderate- |
| carrier_lay_madd_rec16 | -0.438 | moderate- |
| carrier_lay_madd_rec20 | 0.069 | ok |
| carrier_lay_madd_rec24 | -0.414 | moderate- |
| carrier_lay_madd_rec32 | 0.376 | moderate+ |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 1/6, lost 2/6
- **carrier_lay_madd_rec16**: won 2/6, lost 3/6
- **carrier_lay_madd_rec20**: won 2/6, lost 1/6
- **carrier_lay_madd_rec32**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 772922.1ns | 771057.6ns | 100.2% | HIGH |
| carrier_lay_madd_rec16 | 772779.0ns | 771599.3ns | 100.2% | HIGH |
| carrier_lay_madd_rec20 | 771746.8ns | 770390.9ns | 100.2% | HIGH |
| carrier_lay_madd_rec24 | 773207.8ns | 771330.8ns | 100.2% | HIGH |
| carrier_lay_madd_rec32 | 775388.6ns | 773467.8ns | 100.2% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 769689.6-773101.0 ns)
  769689.6 |########################################
  769860.2 |
  770030.7 |####################
  770201.3 |
  770371.9 |
  770542.4 |####################
  770713.0 |
  770883.6 |####################
  771054.2 |
  771224.7 |
  771395.3 |
  771565.9 |
  771736.4 |
  771907.0 |
  772077.6 |
  772248.2 |
  772418.7 |
  772589.3 |
  772759.9 |
  772930.4 |
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 769377.1-773732.5 ns)
  769377.1 |########################################
  769594.9 |
  769812.6 |
  770030.4 |########################################
  770248.2 |
  770465.9 |########################################
  770683.7 |
  770901.5 |
  771119.3 |
  771337.0 |
  771554.8 |
  771772.6 |
  771990.3 |########################################
  772208.1 |
  772425.9 |
  772643.7 |
  772861.4 |########################################
  773079.2 |
  773297.0 |
  773514.7 |
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 769817.5-770865.6 ns)
  769817.5 |########################################
  769869.9 |
  769922.3 |
  769974.7 |
  770027.1 |
  770079.5 |
  770131.9 |########################################
  770184.3 |
  770236.7 |########################################
  770289.1 |
  770341.6 |########################################
  770394.0 |
  770446.4 |
  770498.8 |
  770551.2 |
  770603.6 |
  770656.0 |
  770708.4 |
  770760.8 |
  770813.2 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 769314.6-773836.4 ns)
  769314.6 |########################################
  769540.7 |
  769766.8 |
  769992.9 |########################################
  770219.0 |########################################
  770445.1 |########################################
  770671.2 |
  770897.2 |
  771123.3 |########################################
  771349.4 |
  771575.5 |
  771801.6 |
  772027.7 |
  772253.8 |
  772479.9 |
  772706.0 |
  772932.1 |
  773158.2 |
  773384.3 |
  773610.4 |
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 769479.2-777623.9 ns)
  769479.2 |########################################
  769886.4 |
  770293.7 |########################################
  770700.9 |
  771108.1 |
  771515.4 |
  771922.6 |########################################
  772329.9 |
  772737.1 |
  773144.3 |########################################
  773551.6 |
  773958.8 |
  774366.0 |
  774773.3 |
  775180.5 |
  775587.8 |
  775995.0 |########################################
  776402.2 |
  776809.5 |
  777216.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=100.2% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=100.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=100.3% of algo (FFI overhead may distort results)

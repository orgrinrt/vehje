# Record layout (REC12..REC32) with fixed switch dispatch, madd profile

5 variants, 6 samples per variant.
Baseline: **carrier_lay_madd_rec24**

## Highlights

Baseline for all deltas below: **carrier_lay_madd_rec24**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_lay_madd_rec20 shows alternating (throttle bounce) (autocorr -0.55)

carrier_lay_madd_rec20's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 2.1% of the fastest

All 5 variants sit between 47.74 us and 48.73 us - a 2.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: carrier_lay_madd_rec12** at 47738.8 ns median (-0.2% vs baseline)
- Spread: 1.02x (fastest 47738.8 ns, slowest 48725.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 49924ns | 49984ns | 48572ns | 49692ns | 50949ns | -1.00% |
| carrier_lay_madd_rec16 | 50452ns | 50601ns | 48332ns | 50079ns | 52071ns | +0.05% |
| carrier_lay_madd_rec20 | 50866ns | 50994ns | 49358ns | 50541ns | 52106ns | +0.87% |
| carrier_lay_madd_rec24 | 50429ns | 50213ns | 49069ns | 49935ns | 51849ns | base |
| carrier_lay_madd_rec32 | 50532ns | 50520ns | 49127ns | 50176ns | 51769ns | +0.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_lay_madd_rec12 | 47601ns | 46288ns | 48544ns | -0.98% | 0.022 |
| carrier_lay_madd_rec16 | 48109ns | 46201ns | 49562ns | +0.08% | 0.021 |
| carrier_lay_madd_rec20 | 48556ns | 47075ns | 49682ns | +1.01% | 0.021 |
| carrier_lay_madd_rec24 | 48071ns | 46822ns | 49401ns | base | 0.021 |
| carrier_lay_madd_rec32 | 48094ns | 46872ns | 49152ns | +0.05% | 0.021 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 447731 | 1805495 | 0.248 | 1.00× |
| carrier_lay_madd_rec16 | 447416 | 1805540 | 0.248 | 1.00× |
| carrier_lay_madd_rec20 | 445656 | 1805410 | 0.247 | 0.99× |
| carrier_lay_madd_rec24 | 448128 | 1805538 | 0.248 | 1.00× |
| carrier_lay_madd_rec32 | 447908 | 1805580 | 0.248 | 1.00× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.022 Gops/s** (carrier_lay_madd_rec16; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_lay_madd_rec12 | 0.021 | 96.8% |
| carrier_lay_madd_rec16 | 0.021 | 95.8% |
| carrier_lay_madd_rec20 | 0.021 | 94.8% |
| carrier_lay_madd_rec24 | 0.021 | 96.5% |
| carrier_lay_madd_rec32 | 0.021 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_lay_madd_rec12 | 49924ns | 49924ns | -1.00% |
| carrier_lay_madd_rec16 | 50452ns | 50452ns | +0.05% |
| carrier_lay_madd_rec20 | 50866ns | 50866ns | +0.87% |
| carrier_lay_madd_rec24 | 50429ns | 50429ns | base |
| carrier_lay_madd_rec32 | 50532ns | 50532ns | +0.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_lay_madd_rec24 | 47858ns | base | --- | [46952, 49401] | --- | --- | --- | --- |
| carrier_lay_madd_rec12 | 47739ns | no significant difference | [-1719, +930]ns | [46521, 48544] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec16 | 48233ns | no significant difference | [-558, +511]ns | [46531, 49562] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec20 | 48725ns | no significant difference | [-340, +1507]ns | [47260, 49682] | no | 0.6875 | 0.6875 | 0 |
| carrier_lay_madd_rec32 | 48066ns | no significant difference | [-1003, +961]ns | [47064, 49152] | no | 0.6875 | 0.6875 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_lay_madd_rec24 | carrier_lay_madd_rec12 | carrier_lay_madd_rec16 | carrier_lay_madd_rec20 | carrier_lay_madd_rec32 |
|---|---|---|---|---|---|
| 1 | 48622ns | -4.8% | +1.7% | -0.4% | +0.6% |
| 2 | 49295ns | -1.8% | +0.3% | +0.7% | -3.9% |
| 3 | 46822ns | +0.6% | +0.4% | +0.5% | +0.1% |
| 4 | 47095ns | +3.4% | -1.9% | +5.6% | +3.5% |
| 5 | 49508ns | -2.2% | +0.3% | -1.0% | -0.2% |
| 6 | 47083ns | -0.7% | -0.5% | +0.8% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_lay_madd_rec12 | -0.348 | moderate- |
| carrier_lay_madd_rec16 | -0.200 | ok |
| carrier_lay_madd_rec20 | -0.553 | HIGH- (thermal bounce) |
| carrier_lay_madd_rec24 | -0.334 | moderate- |
| carrier_lay_madd_rec32 | -0.135 | ok |

**Consistency summary:**

- **carrier_lay_madd_rec12**: won 4/6, lost 2/6
- **carrier_lay_madd_rec16**: won 2/6, lost 4/6
- **carrier_lay_madd_rec20**: won 2/6, lost 4/6
- **carrier_lay_madd_rec32**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_lay_madd_rec12 | 95280.6ns | 47601.3ns | 200.2% | HIGH |
| carrier_lay_madd_rec16 | 96373.1ns | 48108.9ns | 200.3% | HIGH |
| carrier_lay_madd_rec20 | 97193.1ns | 48555.8ns | 200.2% | HIGH |
| carrier_lay_madd_rec24 | 96248.8ns | 48070.6ns | 200.2% | HIGH |
| carrier_lay_madd_rec32 | 96353.3ns | 48093.7ns | 200.3% | HIGH |

## Distribution (algo ns)

```
carrier_lay_madd_rec12 (n=6, range 46287.9-48543.9 ns)
  46287.9 |####################
  46400.7 |
  46513.5 |
  46626.3 |
  46739.1 |####################
  46851.9 |
  46964.7 |
  47077.5 |####################
  47190.3 |
  47303.1 |
  47415.9 |
  47528.7 |
  47641.5 |
  47754.3 |
  47867.1 |
  47979.9 |
  48092.7 |
  48205.5 |
  48318.3 |########################################
  48431.1 |
  (0 below, 1 above range)

carrier_lay_madd_rec16 (n=6, range 46201.2-49562.1 ns)
  46201.2 |####################
  46369.2 |
  46537.3 |
  46705.3 |####################
  46873.4 |####################
  47041.4 |
  47209.5 |
  47377.5 |
  47545.6 |
  47713.6 |
  47881.6 |
  48049.7 |
  48217.7 |
  48385.8 |
  48553.8 |
  48721.9 |
  48889.9 |
  49058.0 |
  49226.0 |
  49394.1 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec20 (n=6, range 47074.6-49681.7 ns)
  47074.6 |########################################
  47205.0 |
  47335.3 |########################################
  47465.7 |
  47596.0 |
  47726.4 |
  47856.7 |
  47987.1 |
  48117.4 |
  48247.8 |
  48378.1 |########################################
  48508.5 |
  48638.8 |
  48769.2 |
  48899.5 |########################################
  49029.9 |
  49160.2 |
  49290.6 |
  49420.9 |
  49551.3 |########################################
  (0 below, 1 above range)

carrier_lay_madd_rec24 (n=6, range 46821.7-49401.2 ns)
  46821.7 |####################
  46950.7 |
  47079.7 |########################################
  47208.6 |
  47337.6 |
  47466.6 |
  47595.6 |
  47724.5 |
  47853.5 |
  47982.5 |
  48111.5 |
  48240.5 |
  48369.4 |
  48498.4 |####################
  48627.4 |
  48756.4 |
  48885.3 |
  49014.3 |
  49143.3 |
  49272.3 |####################
  (0 below, 1 above range)

carrier_lay_madd_rec32 (n=6, range 46871.7-49151.5 ns)
  46871.7 |########################################
  46985.7 |
  47099.7 |
  47213.7 |########################################
  47327.7 |########################################
  47441.6 |
  47555.6 |
  47669.6 |
  47783.6 |
  47897.6 |
  48011.6 |
  48125.6 |
  48239.6 |
  48353.6 |
  48467.6 |
  48581.6 |
  48695.5 |########################################
  48809.5 |########################################
  48923.5 |
  49037.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_lay_madd_rec12**: bridge=199.9% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec16**: bridge=200.0% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec20**: bridge=200.1% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec24**: bridge=200.3% of algo (FFI overhead may distort results)
- **carrier_lay_madd_rec32**: bridge=200.5% of algo (FFI overhead may distort results)

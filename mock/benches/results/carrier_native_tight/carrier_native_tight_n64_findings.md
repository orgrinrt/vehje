# Near-native tier: interp vs direct codegen vs copy-and-patch stencil, tight profile (JIT window caps sizes at 1024)

3 variants, 6 samples per variant.
Baseline: **carrier_nat_tight_interp**

## Highlights

Baseline for all deltas below: **carrier_nat_tight_interp**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_nat_tight_interp) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_nat_tight_interp has the worst median (2.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_nat_tight_copypatch at 803 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_nat_tight_stencil beats baseline by 61% (significant)

carrier_nat_tight_stencil is -1.28 us (61%) faster than baseline carrier_nat_tight_interp, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### carrier_nat_tight_interp is an outlier: 2.6x slower than the field

carrier_nat_tight_interp (2.09 us) is 2.6x the fastest (803 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### carrier_nat_tight_copypatch is fastest but the noisiest (CV 17.2%)

carrier_nat_tight_copypatch wins on median (803 ns) yet has the highest variance (CV 17.2%), while carrier_nat_tight_interp is the steadiest (CV 4.1%, 2.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_nat_tight_copypatch** at 802.7 ns median (-61.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2.60x (fastest 802.7 ns, slowest 2087.5 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 3657ns | 3094ns | 2965ns | 3063ns | 4893ns | -18.06% |
| carrier_nat_tight_interp | 4463ns | 4419ns | 4227ns | 4407ns | 4664ns | base |
| carrier_nat_tight_stencil | 3272ns | 3205ns | 3030ns | 3176ns | 3536ns | -26.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_nat_tight_copypatch | 882ns | 762ns | 1066ns | -58.20% | 0.073 |
| carrier_nat_tight_interp | 2109ns | 2021ns | 2207ns | base | 0.030 |
| carrier_nat_tight_stencil | 825ns | 787ns | 863ns | -60.87% | 0.078 |

## Performance model

- Peak throughput: **0.084 Gops/s** (carrier_nat_tight_copypatch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_nat_tight_copypatch | 0.080 | 94.9% |
| carrier_nat_tight_interp | 0.031 | 36.5% |
| carrier_nat_tight_stencil | 0.078 | 92.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_nat_tight_copypatch | 3657ns | 3657ns | -18.06% |
| carrier_nat_tight_interp | 4463ns | 4463ns | base |
| carrier_nat_tight_stencil | 3272ns | 3272ns | -26.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_nat_tight_interp | 2088ns | base | --- | [2034, 2207] | --- | --- | --- | --- |
| carrier_nat_tight_copypatch | 803ns | -1255.8ns (-60.2%) | [-1382, -1045]ns | [777, 1066] | YES | 0.0313 | 0.0313 | 0 |
| carrier_nat_tight_stencil | 820ns | -1277.5ns (-61.2%) | [-1359, -1216]ns | [793, 863] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_nat_tight_interp | carrier_nat_tight_copypatch | carrier_nat_tight_stencil |
|---|---|---|---|
| 1 | 2021ns | -62.3% | -58.6% |
| 2 | 2081ns | -61.1% | -59.9% |
| 3 | 2094ns | -45.5% | -62.4% |
| 4 | 2128ns | -53.5% | -62.2% |
| 5 | 2286ns | -65.3% | -61.0% |
| 6 | 2048ns | -61.2% | -60.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_nat_tight_copypatch | 0.141 | ok |
| carrier_nat_tight_interp | -0.115 | ok |
| carrier_nat_tight_stencil | -0.351 | moderate- |

**Consistency summary:**

- **carrier_nat_tight_copypatch**: won 6/6, lost 0/6
- **carrier_nat_tight_stencil**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_nat_tight_copypatch | 83173.5ns | 881.8ns | 9432.1% | HIGH |
| carrier_nat_tight_interp | 86741.5ns | 2109.4ns | 4112.1% | HIGH |
| carrier_nat_tight_stencil | 76393.8ns | 825.3ns | 9255.9% | HIGH |

## Distribution (algo ns)

```
carrier_nat_tight_copypatch (n=6, range 761.7-1065.7 ns)
    761.7 |####################
    776.9 |
    792.1 |########################################
    807.3 |####################
    822.5 |
    837.7 |
    852.9 |
    868.1 |
    883.3 |
    898.5 |
    913.7 |
    928.9 |
    944.1 |
    959.3 |
    974.5 |####################
    989.7 |
   1004.9 |
   1020.1 |
   1035.3 |
   1050.5 |
  (0 below, 1 above range)

carrier_nat_tight_interp (n=6, range 2020.8-2206.7 ns)
   2020.8 |########################################
   2030.1 |
   2039.4 |########################################
   2048.7 |
   2058.0 |
   2067.3 |
   2076.6 |########################################
   2085.8 |########################################
   2095.1 |
   2104.4 |
   2113.7 |
   2123.0 |########################################
   2132.3 |
   2141.6 |
   2150.9 |
   2160.2 |
   2169.5 |
   2178.8 |
   2188.1 |
   2197.4 |
  (0 below, 1 above range)

carrier_nat_tight_stencil (n=6, range 786.7-863.3 ns)
    786.7 |####################
    790.5 |
    794.4 |
    798.2 |####################
    802.0 |####################
    805.9 |
    809.7 |
    813.5 |
    817.3 |
    821.2 |
    825.0 |
    828.8 |
    832.7 |########################################
    836.5 |
    840.3 |
    844.1 |
    848.0 |
    851.8 |
    855.6 |
    859.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_nat_tight_copypatch**: bridge=9594.0% of algo (FFI overhead may distort results)
- **carrier_nat_tight_interp**: bridge=4145.1% of algo (FFI overhead may distort results)
- **carrier_nat_tight_stencil**: bridge=9449.7% of algo (FFI overhead may distort results)

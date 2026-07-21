# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

2 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (carrier_disp_switch_v4) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline carrier_disp_switch_v4 has the worst median (2.18 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest carrier_disp_fntable_v4 at 1.90 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### carrier_disp_fntable_v4 dominates: 15% faster than the next best (carrier_disp_switch_v4)

carrier_disp_fntable_v4 (1.90 ms) leads carrier_disp_switch_v4 (2.18 ms) by 15%, a clear separation rather than a photo finish. CV 6.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_fntable_v4 is fastest but the noisiest (CV 6.6%)

carrier_disp_fntable_v4 wins on median (1.90 ms) yet has the highest variance (CV 6.6%), while carrier_disp_switch_v4 is the steadiest (CV 4.3%, 2.18 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: carrier_disp_fntable_v4** at 1897569.4 ns median (-13.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.15x (fastest 1897569.4 ns, slowest 2180272.3 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 1955599ns | 1900711ns | 1837425ns | 1887115ns | 2117411ns | -11.64% |
| carrier_disp_switch_v4 | 2213127ns | 2183091ns | 2138247ns | 2170222ns | 2314926ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 1952447ns | 1834963ns | 2113582ns | -11.65% | 0.008 |
| carrier_disp_switch_v4 | 2210012ns | 2135561ns | 2311317ns | base | 0.007 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_disp_fntable_v4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.009 | 96.7% |
| carrier_disp_switch_v4 | 0.008 | 84.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 1955599ns | 1955599ns | -11.64% |
| carrier_disp_switch_v4 | 2213127ns | 2213127ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 2180272ns | base | --- | [2138446, 2311317] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 1897569ns | -263289.0ns (-12.1%) | [-311671, -197735]ns | [1846189, 2113582] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 |
|---|---|---|
| 1 | 2411337ns | -9.0% |
| 2 | 2211298ns | -8.1% |
| 3 | 2180385ns | -12.5% |
| 4 | 2180160ns | -14.8% |
| 5 | 2135561ns | -14.1% |
| 6 | 2141332ns | -11.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | 0.416 | moderate+ |
| carrier_disp_switch_v4 | 0.161 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 3399.7ns | 1952447.0ns | 0.2% |  |
| carrier_disp_switch_v4 | 3030.8ns | 2210011.9ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 1834963.3-2113582.5 ns)
  1834963.3 |########################################
  1848894.3 |########################################
  1862825.2 |
  1876756.2 |########################################
  1890687.1 |
  1904618.1 |########################################
  1918549.0 |
  1932480.0 |
  1946411.0 |
  1960341.9 |
  1974272.9 |
  1988203.8 |
  2002134.8 |
  2016065.7 |
  2029996.7 |########################################
  2043927.7 |
  2057858.6 |
  2071789.6 |
  2085720.5 |
  2099651.5 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 2135561.2-2311317.1 ns)
  2135561.2 |########################################
  2144349.0 |
  2153136.8 |
  2161924.6 |
  2170712.4 |
  2179500.2 |########################################
  2188288.0 |
  2197075.8 |
  2205863.6 |####################
  2214651.4 |
  2223439.2 |
  2232226.9 |
  2241014.7 |
  2249802.5 |
  2258590.3 |
  2267378.1 |
  2276165.9 |
  2284953.7 |
  2293741.5 |
  2302529.3 |
  (0 below, 1 above range)

```

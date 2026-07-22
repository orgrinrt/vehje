# Dispatch shape: switch vs fn-pointer table, op vocab v4 (carrier)

3 variants, 6 samples per variant.
Baseline: **carrier_disp_switch_v4**

## Highlights

Baseline for all deltas below: **carrier_disp_switch_v4**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_disp_fntable_v4 dominates: 16% faster than the next best (carrier_disp_switch_v4)

carrier_disp_fntable_v4 (1.83 ms) leads carrier_disp_switch_v4 (2.12 ms) by 16%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### carrier_disp_switch_v4 shows alternating (throttle bounce) (autocorr -0.75)

carrier_disp_switch_v4's per-pass series has lag-1 autocorrelation -0.75, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: carrier_disp_fntable_v4** at 1831042.0 ns median (-13.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.28x (fastest 1831042.0 ns, slowest 2344009.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 1834882ns | 1834495ns | 1819128ns | 1829582ns | 1850709ns | -13.82% |
| carrier_disp_switch_v4 | 2129128ns | 2128031ns | 2112408ns | 2125102ns | 2143527ns | base |
| carrier_disp_threaded_v4 | 2347739ns | 2346994ns | 2330637ns | 2343538ns | 2362592ns | +10.27% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_disp_fntable_v4 | 1831341ns | 1815484ns | 1847164ns | -13.85% | 0.009 |
| carrier_disp_switch_v4 | 2125821ns | 2108887ns | 2140592ns | base | 0.008 |
| carrier_disp_threaded_v4 | 2344671ns | 2327462ns | 2359647ns | +10.29% | 0.007 |

## Performance model

- Peak throughput: **0.009 Gops/s** (carrier_disp_fntable_v4; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_disp_fntable_v4 | 0.009 | 99.2% |
| carrier_disp_switch_v4 | 0.008 | 85.5% |
| carrier_disp_threaded_v4 | 0.007 | 77.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_disp_fntable_v4 | 1834882ns | 1834882ns | -13.82% |
| carrier_disp_switch_v4 | 2129128ns | 2129128ns | base |
| carrier_disp_threaded_v4 | 2347739ns | 2347739ns | +10.27% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_disp_switch_v4 | 2124362ns | base | --- | [2112510, 2140592] | --- | --- | --- | --- |
| carrier_disp_fntable_v4 | 1831042ns | -301699.1ns (-14.2%) | [-316395, -265346]ns | [1815818, 1847164] | YES | 0.0313 | 0.0313 | 0 |
| carrier_disp_threaded_v4 | 2344009ns | +222229.6ns (+10.5%) | [+194273, +240045]ns | [2330355, 2359647] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_disp_switch_v4 | carrier_disp_fntable_v4 | carrier_disp_threaded_v4 |
|---|---|---|---|
| 1 | 2138365ns | -15.1% | +8.8% |
| 2 | 2108887ns | -12.9% | +11.2% |
| 3 | 2142820ns | -14.5% | +9.3% |
| 4 | 2116132ns | -12.2% | +10.3% |
| 5 | 2122021ns | -14.4% | +11.5% |
| 6 | 2126702ns | -14.0% | +10.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_disp_fntable_v4 | -0.373 | moderate- |
| carrier_disp_switch_v4 | -0.750 | HIGH- (thermal bounce) |
| carrier_disp_threaded_v4 | -0.038 | ok |

**Consistency summary:**

- **carrier_disp_fntable_v4**: won 6/6, lost 0/6
- **carrier_disp_threaded_v4**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_disp_fntable_v4 | 1942.5ns | 1831341.3ns | 0.1% |  |
| carrier_disp_switch_v4 | 2164.1ns | 2125821.2ns | 0.1% |  |
| carrier_disp_threaded_v4 | 2301.5ns | 2344670.5ns | 0.1% |  |

## Distribution (algo ns)

```
carrier_disp_fntable_v4 (n=6, range 1815483.8-1847163.8 ns)
  1815483.8 |########################################
  1817067.8 |
  1818651.8 |
  1820235.8 |
  1821819.8 |
  1823403.8 |
  1824987.8 |
  1826571.8 |
  1828155.8 |
  1829739.8 |####################
  1831323.8 |####################
  1832907.8 |
  1834491.8 |
  1836075.8 |####################
  1837659.8 |
  1839243.8 |
  1840827.8 |
  1842411.8 |
  1843995.8 |
  1845579.8 |
  (0 below, 1 above range)

carrier_disp_switch_v4 (n=6, range 2108886.7-2140592.3 ns)
  2108886.7 |########################################
  2110472.0 |
  2112057.3 |
  2113642.5 |
  2115227.8 |########################################
  2116813.1 |
  2118398.4 |
  2119983.7 |
  2121568.9 |########################################
  2123154.2 |
  2124739.5 |
  2126324.8 |########################################
  2127910.1 |
  2129495.3 |
  2131080.6 |
  2132665.9 |
  2134251.2 |
  2135836.5 |
  2137421.7 |########################################
  2139007.0 |
  (0 below, 1 above range)

carrier_disp_threaded_v4 (n=6, range 2327461.7-2359647.3 ns)
  2327461.7 |########################################
  2329071.0 |
  2330680.3 |
  2332289.5 |########################################
  2333898.8 |
  2335508.1 |
  2337117.4 |
  2338726.7 |
  2340335.9 |
  2341945.2 |########################################
  2343554.5 |
  2345163.8 |########################################
  2346773.1 |
  2348382.3 |
  2349991.6 |
  2351600.9 |
  2353210.2 |########################################
  2354819.5 |
  2356428.7 |
  2358038.0 |
  (0 below, 1 above range)

```

# Per-type strategy (NATIVE tier): all match

5 variants, 6 samples per variant.
Baseline: **an_match_table**

## Highlights

Baseline for all deltas below: **an_match_table**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### an_match_tree is fastest but the noisiest (CV 6.9%)

an_match_tree wins on median (52.33 us) yet has the highest variance (CV 6.9%), while an_match_pred is the steadiest (CV 5.3%, 72.99 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### an_match_seq shows alternating (throttle bounce) (autocorr -0.53)

an_match_seq's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: an_match_tree** at 52330.2 ns median (-5.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.39x (fastest 52330.2 ns, slowest 72991.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| an_match_pred | 73559ns | 75522ns | 67255ns | 73344ns | 77033ns | +27.44% |
| an_match_prof | 60033ns | 61086ns | 54673ns | 59349ns | 63741ns | +4.01% |
| an_match_seq | 57247ns | 56975ns | 52198ns | 56075ns | 61530ns | -0.82% |
| an_match_table | 57718ns | 57801ns | 51087ns | 57150ns | 61886ns | base |
| an_match_tree | 53351ns | 54774ns | 48356ns | 52753ns | 56746ns | -7.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| an_match_pred | 71145ns | 65008ns | 74574ns | +28.46% | 0.058 |
| an_match_prof | 57667ns | 52369ns | 61301ns | +4.12% | 0.071 |
| an_match_seq | 54778ns | 49765ns | 58976ns | -1.10% | 0.075 |
| an_match_table | 55385ns | 48820ns | 59455ns | base | 0.074 |
| an_match_tree | 50966ns | 45957ns | 54482ns | -7.98% | 0.080 |

## Performance model

- Peak throughput: **0.089 Gops/s** (an_match_tree; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| an_match_pred | 0.056 | 63.0% |
| an_match_prof | 0.070 | 78.3% |
| an_match_seq | 0.075 | 84.0% |
| an_match_table | 0.074 | 82.8% |
| an_match_tree | 0.078 | 87.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| an_match_pred | 73559ns | 73559ns | +27.44% |
| an_match_prof | 60033ns | 60033ns | +4.01% |
| an_match_seq | 57247ns | 57247ns | -0.82% |
| an_match_table | 57718ns | 57718ns | base |
| an_match_tree | 53351ns | 53351ns | -7.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| an_match_table | 55479ns | base | --- | [51221, 59455] | --- | --- | --- | --- |
| an_match_pred | 72991ns | +13853.0ns (+25.0%) | [+10391, +23036]ns | [65870, 74574] | YES (adj: no) | 0.1250 | 0.0313 | 0 |
| an_match_prof | 58665ns | no significant difference | [-1949, +8509]ns | [53036, 61301] | no | 0.9167 | 0.6875 | 0 |
| an_match_seq | 54718ns | no significant difference | [-7031, +6518]ns | [50640, 58976] | no | 1.0000 | 1.0000 | 0 |
| an_match_tree | 52330ns | no significant difference | [-12466, +2982]ns | [46085, 54482] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | an_match_table | an_match_pred | an_match_prof | an_match_seq | an_match_tree |
|---|---|---|---|---|---|
| 1 | 53622ns | +38.9% | +0.2% | +2.4% | -0.6% |
| 2 | 58575ns | +27.5% | +0.8% | -15.0% | -12.3% |
| 3 | 48820ns | +51.6% | +26.7% | +24.0% | +12.9% |
| 4 | 54192ns | +20.0% | -3.4% | +0.6% | -0.6% |
| 5 | 60335ns | +19.3% | -3.4% | -4.9% | -23.8% |
| 6 | 56766ns | +17.6% | +7.0% | -9.3% | -18.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| an_match_pred | -0.049 | ok |
| an_match_prof | -0.316 | moderate- |
| an_match_seq | -0.531 | HIGH- (thermal bounce) |
| an_match_table | -0.211 | moderate- |
| an_match_tree | 0.301 | moderate+ |

**Consistency summary:**

- **an_match_pred**: won 0/6, lost 6/6
- **an_match_prof**: won 2/6, lost 4/6
- **an_match_seq**: won 3/6, lost 3/6
- **an_match_tree**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| an_match_pred | 6.0ns | 71144.9ns | 0.0% |  |
| an_match_prof | 6.5ns | 57667.4ns | 0.0% |  |
| an_match_seq | 5.2ns | 54777.9ns | 0.0% |  |
| an_match_table | 4.3ns | 55385.0ns | 0.0% |  |
| an_match_tree | 6.7ns | 50965.7ns | 0.0% |  |

## Distribution (algo ns)

```
an_match_pred (n=6, range 65007.9-74573.5 ns)
  65007.9 |########################################
  65486.2 |
  65964.5 |
  66442.7 |########################################
  66921.0 |
  67399.3 |
  67877.6 |
  68355.9 |
  68834.2 |
  69312.4 |
  69790.7 |
  70269.0 |
  70747.3 |
  71225.6 |
  71703.9 |########################################
  72182.1 |
  72660.4 |
  73138.7 |
  73617.0 |########################################
  74095.3 |########################################
  (0 below, 1 above range)

an_match_prof (n=6, range 52369.2-61301.4 ns)
  52369.2 |########################################
  52815.8 |
  53262.4 |########################################
  53709.0 |
  54155.6 |
  54602.3 |
  55048.9 |
  55495.5 |
  55942.1 |
  56388.7 |
  56835.3 |
  57281.9 |
  57728.5 |
  58175.2 |########################################
  58621.8 |
  59068.4 |########################################
  59515.0 |
  59961.6 |
  60408.2 |########################################
  60854.8 |
  (0 below, 1 above range)

an_match_seq (n=6, range 49765.0-58975.8 ns)
  49765.0 |########################################
  50225.5 |
  50686.1 |
  51146.6 |########################################
  51607.2 |
  52067.7 |
  52528.2 |
  52988.8 |
  53449.3 |
  53909.9 |
  54370.4 |########################################
  54830.9 |########################################
  55291.5 |
  55752.0 |
  56212.6 |
  56673.1 |
  57133.6 |########################################
  57594.2 |
  58054.7 |
  58515.3 |
  (0 below, 1 above range)

an_match_table (n=6, range 48820.0-59455.2 ns)
  48820.0 |########################################
  49351.8 |
  49883.5 |
  50415.3 |
  50947.0 |
  51478.8 |
  52010.6 |
  52542.3 |
  53074.1 |
  53605.8 |########################################
  54137.6 |########################################
  54669.4 |
  55201.1 |
  55732.9 |
  56264.6 |########################################
  56796.4 |
  57328.2 |
  57859.9 |
  58391.7 |########################################
  58923.4 |
  (0 below, 1 above range)

an_match_tree (n=6, range 45956.7-54482.1 ns)
  45956.7 |########################################
  46383.0 |
  46809.2 |
  47235.5 |
  47661.8 |
  48088.0 |
  48514.3 |
  48940.6 |
  49366.8 |
  49793.1 |
  50219.4 |
  50645.6 |
  51071.9 |####################
  51498.2 |
  51924.4 |
  52350.7 |
  52777.0 |
  53203.2 |####################
  53629.5 |####################
  54055.8 |
  (0 below, 1 above range)

```

# Setup cost S (parse / predecode / stackbc compile / optimize / direct-emit / copypatch-emit), real profile

6 variants, 6 samples per variant.
Baseline: **carrier_setup_real_parse**

## Highlights

Baseline for all deltas below: **carrier_setup_real_parse**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### carrier_setup_real_optall shows warm-up / thermal drift (autocorr +0.51)

carrier_setup_real_optall's per-pass series has lag-1 autocorrelation +0.51, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (carrier_setup_real_parse)

The baseline carrier_setup_real_parse is the fastest (0 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_stackcompile, carrier_setup_real_emitdirect, carrier_setup_real_emitcopypatch} vs {carrier_setup_real_optall} (687% apart)

The field splits into a fast tier {carrier_setup_real_parse, carrier_setup_real_predecode, carrier_setup_real_stackcompile, carrier_setup_real_emitdirect, carrier_setup_real_emitcopypatch} and a slow tier {carrier_setup_real_optall} with a 687% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Whole field within 0.0% of the fastest

All 6 variants sit between 0 ns and 40.38 ms - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (carrier_setup_real_parse) is the fastest** at 0.0 ns median
- 5 variants significantly slower than baseline

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 5143456ns | 5134524ns | 5129196ns | 5133961ns | 5164827ns | +232869.22% |
| carrier_setup_real_emitdirect | 2848266ns | 2853596ns | 2818575ns | 2851165ns | 2858763ns | +128910.21% |
| carrier_setup_real_optall | 40439929ns | 40384574ns | 40305266ns | 40366004ns | 40618148ns | +1831598.27% |
| carrier_setup_real_parse | 2208ns | 2211ns | 2098ns | 2180ns | 2304ns | base |
| carrier_setup_real_predecode | 833916ns | 845946ns | 798061ns | 831490ns | 855483ns | +37671.66% |
| carrier_setup_real_stackcompile | 1989780ns | 2000491ns | 1921248ns | 1996367ns | 2014165ns | +90025.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 5139972ns | 5125852ns | 5161061ns | +0.00% | 0.003 |
| carrier_setup_real_emitdirect | 2845363ns | 2815867ns | 2855555ns | +0.00% | 0.006 |
| carrier_setup_real_optall | 40435914ns | 40301358ns | 40614011ns | +0.00% | 0.000 |
| carrier_setup_real_parse | 0ns | 0ns | 0ns | base | 0.000 |
| carrier_setup_real_predecode | 831548ns | 795811ns | 853125ns | +0.00% | 0.020 |
| carrier_setup_real_stackcompile | 1987190ns | 1918975ns | 2011575ns | +0.00% | 0.008 |

## Hardware counters (per call)

| Variant | instructions | cycles | IPC | × base instr |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 32129623 | 104314781 | 0.308 | 1755.30× |
| carrier_setup_real_emitdirect | 17809030 | 48962945 | 0.364 | 972.94× |
| carrier_setup_real_optall | 252446170 | 1099675169 | 0.230 | 13791.61× |
| carrier_setup_real_parse | 18304 | 79554 | 0.230 | 1.00× |
| carrier_setup_real_predecode | 5209942 | 24247017 | 0.215 | 284.63× |
| carrier_setup_real_stackcompile | 12423837 | 40023360 | 0.310 | 678.74× |

Instructions and cycles are the mean over the variant's samples for the measured region. IPC is instructions per cycle. The instruction ratio isolates whether a variant wins by retiring fewer instructions or by executing the same instructions more efficiently.

## Performance model

- Peak throughput: **0.021 Gops/s** (carrier_setup_real_predecode; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| carrier_setup_real_emitcopypatch | 0.003 | 15.5% |
| carrier_setup_real_emitdirect | 0.006 | 27.9% |
| carrier_setup_real_optall | 0.000 | 2.0% |
| carrier_setup_real_predecode | 0.019 | 94.4% |
| carrier_setup_real_stackcompile | 0.008 | 39.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| carrier_setup_real_emitcopypatch | 5143456ns | 5143456ns | +232869.22% |
| carrier_setup_real_emitdirect | 2848266ns | 2848266ns | +128910.21% |
| carrier_setup_real_optall | 40439929ns | 40439929ns | +1831598.27% |
| carrier_setup_real_parse | 2208ns | 2208ns | base |
| carrier_setup_real_predecode | 833916ns | 833916ns | +37671.66% |
| carrier_setup_real_stackcompile | 1989780ns | 1989780ns | +90025.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| carrier_setup_real_parse | 0ns | base | --- | [0, 0] | --- | --- | --- | --- |
| carrier_setup_real_emitcopypatch | 5131182ns | +5131181.9ns (+0.0%) | [+5127674, +5161061]ns | [5127674, 5161061] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_emitdirect | 2850841ns | +2850841.2ns (+0.0%) | [+2829692, +2855555]ns | [2829692, 2855555] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_optall | 40380518ns | +40380518.1ns (+0.0%) | [+40313214, +40614011]ns | [40313214, 40614011] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_predecode | 843465ns | +843465.0ns (+0.0%) | [+798055, +853125]ns | [798055, 853125] | YES | 0.0313 | 0.0313 | 0 |
| carrier_setup_real_stackcompile | 1997625ns | +1997624.6ns (+0.0%) | [+1952369, +2011575]ns | [1952369, 2011575] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | carrier_setup_real_parse | carrier_setup_real_emitcopypatch | carrier_setup_real_emitdirect | carrier_setup_real_optall | carrier_setup_real_predecode | carrier_setup_real_stackcompile |
|---|---|---|---|---|---|---|
| 1 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 2 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 3 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 4 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 5 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |
| 6 | 0ns | +0.0% | +0.0% | +0.0% | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| carrier_setup_real_emitcopypatch | 0.334 | moderate+ |
| carrier_setup_real_emitdirect | -0.405 | moderate- |
| carrier_setup_real_optall | 0.507 | HIGH+ (drift/warm-up) |
| carrier_setup_real_parse | 0.000 | ok |
| carrier_setup_real_predecode | -0.120 | ok |
| carrier_setup_real_stackcompile | -0.170 | ok |

**Consistency summary:**

- **carrier_setup_real_emitcopypatch**: won 0/6, lost 0/6
- **carrier_setup_real_emitdirect**: won 0/6, lost 0/6
- **carrier_setup_real_optall**: won 0/6, lost 0/6
- **carrier_setup_real_predecode**: won 0/6, lost 0/6
- **carrier_setup_real_stackcompile**: won 0/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| carrier_setup_real_emitcopypatch | 5145626.7ns | 5139972.2ns | 100.1% | HIGH |
| carrier_setup_real_emitdirect | 2845545.6ns | 2845362.8ns | 100.0% | HIGH |
| carrier_setup_real_optall | 40457556.3ns | 40435914.4ns | 100.1% | HIGH |
| carrier_setup_real_parse | 2828.8ns | 0.0ns | 0.0% |  |
| carrier_setup_real_predecode | 831774.8ns | 831548.4ns | 100.0% | HIGH |
| carrier_setup_real_stackcompile | 1986840.0ns | 1987189.5ns | 100.0% | HIGH |

## Distribution (algo ns)

```
carrier_setup_real_emitcopypatch (n=6, range 5125852.5-5161060.7 ns)
  5125852.5 |####################
  5127612.9 |
  5129373.3 |########################################
  5131133.7 |####################
  5132894.1 |
  5134654.5 |
  5136414.9 |
  5138175.4 |
  5139935.8 |
  5141696.2 |
  5143456.6 |
  5145217.0 |
  5146977.4 |
  5148737.8 |
  5150498.2 |
  5152258.6 |
  5154019.0 |####################
  5155779.4 |
  5157539.8 |
  5159300.2 |
  (0 below, 1 above range)

carrier_setup_real_emitdirect (n=6, range 2815866.7-2855554.6 ns)
  2815866.7 |####################
  2817851.1 |
  2819835.5 |
  2821819.9 |
  2823804.3 |
  2825788.7 |
  2827773.1 |
  2829757.5 |
  2831741.9 |
  2833726.3 |
  2835710.7 |
  2837695.0 |
  2839679.4 |
  2841663.8 |####################
  2843648.2 |
  2845632.6 |
  2847617.0 |####################
  2849601.4 |
  2851585.8 |
  2853570.2 |########################################
  (0 below, 1 above range)

carrier_setup_real_optall (n=6, range 40301357.5-40614011.5 ns)
  40301357.5 |########################################
  40316990.2 |########################################
  40332622.9 |
  40348255.6 |########################################
  40363888.3 |
  40379521.0 |
  40395153.7 |########################################
  40410786.4 |
  40426419.1 |
  40442051.8 |
  40457684.5 |
  40473317.2 |
  40488949.9 |
  40504582.6 |
  40520215.3 |
  40535848.0 |
  40551480.7 |
  40567113.4 |
  40582746.1 |########################################
  40598378.8 |
  (0 below, 1 above range)

carrier_setup_real_predecode (n=6, range 795810.8-853125.4 ns)
  795810.8 |####################
  798676.5 |####################
  801542.3 |
  804408.0 |
  807273.7 |
  810139.5 |
  813005.2 |
  815870.9 |
  818736.6 |
  821602.4 |
  824468.1 |
  827333.8 |
  830199.6 |
  833065.3 |
  835931.0 |####################
  838796.8 |
  841662.5 |
  844528.2 |
  847393.9 |########################################
  850259.7 |
  (0 below, 1 above range)

carrier_setup_real_stackcompile (n=6, range 1918975.4-2011574.6 ns)
  1918975.4 |########################################
  1923605.4 |
  1928235.3 |
  1932865.3 |
  1937495.2 |
  1942125.2 |
  1946755.2 |
  1951385.1 |
  1956015.1 |
  1960645.0 |
  1965275.0 |
  1969905.0 |
  1974534.9 |
  1979164.9 |
  1983794.8 |########################################
  1988424.8 |
  1993054.8 |########################################
  1997684.7 |########################################
  2002314.7 |########################################
  2006944.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **carrier_setup_real_emitcopypatch**: bridge=100.1% of algo (FFI overhead may distort results)
- **carrier_setup_real_emitdirect**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_optall**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **carrier_setup_real_optall**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_predecode**: bridge=100.0% of algo (FFI overhead may distort results)
- **carrier_setup_real_stackcompile**: bridge=100.0% of algo (FFI overhead may distort results)

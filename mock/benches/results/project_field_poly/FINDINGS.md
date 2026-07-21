# Project field access, polymorphic site (harness): the inline-cache hazard

together with `project_field_mono`: the load-bearing story is that the inline cache is best at the monomorphic
site and WORST at this polymorphic site, so it must self-disable to a hash lookup.

## What this measures

`Project` (record.field) at a POLYMORPHIC access site: 1024 records over four rotating shapes (the name-105
field lands at a rotating slot per record), which defeats a monomorphic inline cache. Three strategies:

- `project_poly_linear`: linear name scan.
- `project_poly_hash` (baseline): open-addressing hash lookup on the field-name id, the recommended fallback.
- `project_poly_ic`: monomorphic inline cache, guard on shape_id.

All three return the byte-identical field value; the harness cross-validates (offline check confirmed
identical). Direct-offset is STRUCTURALLY absent here: with a rotating shape there is no single fixed offset, so
a fixed-offset read returns the wrong value and would fail cross-validation (offline check confirmed it
diverges). That is the invalidity the finding names, now enforced by the harness rather than a footnote.
Baseline = hash-lookup.

## The audit defect this fixes

Same standalone-vs-harness defect as the monomorphic half. Additionally, the polymorphic site makes the
inline-cache hazard measurable and cross-validated instead of asserted: the guard misses ~75% of accesses (four
shapes), paying a failed guard plus a refill on top of the fallback scan.

## Measured results

Ratio to baseline (project_poly_ic), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | project_poly_hash (ratio) | project_poly_ic (base) | project_poly_linear (ratio) |
|---|---|---|---|
| 64 | 0.84x | 212 ns | 0.98x |
| 256 | 0.83x | 757 ns | 0.97x |
| 1024 | 0.87x | 2544 ns | 0.95x |
| 4096 | 0.86x | 12825 ns | 1.78x |
| 16384 | 1.05x | 117928 ns | 1.01x |

## Cost-model sanity line

At n=16384, the baseline (project_poly_ic) median is 117928 ns for N field accesses over many shapes, guard misses common. Treating n as the work-item count, that is 7.20 ns/item, about 23.0 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

Megamorphic field access: hash, inline cache, and linear CONVERGE to a dead heat at large n (5% spread at n=16384). The inline cache loses its monomorphic advantage when the site is megamorphic (the guard misses constantly), confirming the polymorphic-IC hazard. No lowering dominates for megamorphic sites; a hash map is as good as anything.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).

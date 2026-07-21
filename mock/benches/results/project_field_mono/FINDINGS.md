# Project field access, monomorphic site (harness): direct offset vs inline cache vs hash vs linear

monomorphic half of the Project field-access port; the inline-cache hazard is the cross-bench story with
`project_field_poly` (read both together).

## What this measures

`Project` (record.field), the record-heavy templating/config hot path. At a MONOMORPHIC access site (the site
sees one shape repeatedly, e.g. a loop over a homogeneous collection), four strategies over 1024 same-shape
8-field records:

- `project_mono_direct` (baseline): the field name is compile-resolved to a fixed slot offset; runtime is a
  plain load. The static-shape case.
- `project_mono_linear`: linear name scan.
- `project_mono_hash`: open-addressing hash lookup on the field-name id.
- `project_mono_ic`: monomorphic inline cache, guard on shape_id, hit = direct load, miss = scan + refill.

All four return the byte-identical field value; the harness cross-validates (offline check confirmed identical).
Baseline = direct-offset.

## The audit defect this fixes

The standalone `project-field-access/project.zig` ran outside the harness with a hand-timed 50M loop and a
header-only CSV. Fixed here: the record set is built once via `OnceLock` (not const-foldable) outside the timed
region; the record index is folded with the FFI input byte so nothing hoists; the 8-field record layout is
asserted (`size_of::<Rec>() == 104`); and the strategies are cross-validated to identical output.

## Measured results

Ratio to baseline (project_mono_direct), warm median, Apple M1 (harness rev 70fb75b5, wall-clock via CNTVCT_EL0):

| n | project_mono_direct (base) | project_mono_hash (ratio) | project_mono_ic (ratio) | project_mono_linear (ratio) |
|---|---|---|---|---|
| 64 | 59 ns | 3.10x | 1.42x | 3.49x |
| 256 | 162 ns | 3.64x | 1.42x | 4.18x |
| 1024 | 576 ns | 3.78x | 1.40x | 4.64x |
| 4096 | 2866 ns | 3.28x | 1.25x | 3.87x |
| 16384 | 11692 ns | 3.04x | 1.19x | 3.57x |

## Cost-model sanity line

At n=16384, the baseline (project_mono_direct) median is 11692 ns for N field accesses, one offset load each (direct). Treating n as the work-item count, that is 0.71 ns/item, about 2.3 cycles/item at 3.2 GHz, physically plausible (well under the M1's ~8-wide retire). (Coarse throughput proxy: exact per-item op counts vary by variant; the check is that no number implies a physically impossible rate.)

## Cross-validation

pass.

## Verdict

Monomorphic field access: a direct offset wins; an inline cache is 1.2x to 1.4x (near-direct, the guard is cheap when it always hits); hash and linear lookup are 3x to 4.6x. For monomorphic sites, a direct offset or a monomorphic inline cache is the right lowering.

**Strength: measurement** (wall-clock timing, cross-validated per above; no hardware event counters available in M1 userspace, so mechanism attribution rests on designed sweeps, not counters).

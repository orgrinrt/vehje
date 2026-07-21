# Project field access, monomorphic site (harness): direct offset vs inline cache vs hash vs linear

Scaffold. The main agent fills the medians and the cost-model line after the serialized bench run. This is the
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

## Result (fill after run)

Medians, normalised against `project_mono_direct` (mode = subtract):

| n | direct (base) | linear | hash | inline-cache |
|---|---|---|---|---|
| 64 | | | | |
| 256 | | | | |
| 1024 | | | | |
| 4096 | | | | |
| 16384 | | | | |

Expected shape: direct-offset fastest (a single load); inline-cache reaches near-direct speed here (the shape
guard hits every access); hash beats linear. Prior standalone: direct 0.54, ic 0.61, hash 1.27, linear 1.57
ns/access.

## The finding (fill after run)

Cross-validation: [pass/fail]. Where the record shape is statically known, resolve `Project` to a direct offset
([ns], nearly free); push records toward static shapes so most accesses land there. Where the shape is dynamic
but the site stays monomorphic, a monomorphic inline cache reaches near-direct speed ([ns]). See
`project_field_poly` for the hazard that forces the inline cache to self-disable.

## Cost-model sanity line (fill after run)

At n=16384, direct-offset is [ns/access], about [N] cycles at ~3.2 GHz for a single indexed load; the
inline-cache adds [delta] for the shape guard. Time scales [linearly?] with N, confirming the timed work is the
access and not a hoisted constant.

## Boundary

8-field records; larger records widen the hash's lead over the linear scan and grow the direct/IC advantage.
Nested projection (a.b.c) is a chain of single projections, each using this rule at its level.

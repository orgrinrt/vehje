# Project field access, polymorphic site (harness): the inline-cache hazard

Scaffold. The main agent fills the medians and the cost-model line after the serialized bench run. Read
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

## Result (fill after run)

Medians, normalised against `project_poly_hash` (mode = subtract):

| n | linear | hash (base) | inline-cache |
|---|---|---|---|
| 64 | | | |
| 256 | | | |
| 1024 | | | |
| 4096 | | | |
| 16384 | | | |

Expected shape: inline-cache is the WORST here (guard miss + refill on almost every access), slower than both
hash and linear; hash beats linear. Prior standalone at the polymorphic site: ic 1.89, linear 1.57, hash 1.27
ns/access. Compare `project_mono_ic` (near-direct) against `project_poly_ic` (worst) to see the hazard.

## The finding (fill after run)

Cross-validation: [pass/fail]. At a polymorphic (megamorphic) site the inline cache is [the worst] strategy, so
a `Project` inline cache MUST detect polymorphism (after K consecutive guard misses, mark the site polymorphic
and go straight to the hash lookup, the standard JS-engine technique). The composite lowering: static shape ->
direct offset; dynamic monomorphic -> inline cache; dynamic polymorphic -> hash, with the inline cache
self-disabling so the pathological thrash never persists.

## Cost-model sanity line (fill after run)

At n=16384, `project_poly_ic` is [ns/access] vs `project_poly_hash` [ns/access], the [delta] being the failed
shape guard plus the refill scan on top of the fallback. Time scales [linearly?] with N, confirming the timed
work is the access.

## Boundary

Four rotating shapes; a truly megamorphic site (many shapes) makes the inline cache even worse, reinforcing the
self-disable requirement. Field names are interned ids compared as integers, so both the scan and the hash
compare u32 ids, not byte strings.

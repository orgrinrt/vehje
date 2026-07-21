# Project field-access strategy: compile-resolved offset vs dynamic lookup vs inline cache

**Date:** 2026-07-21
**Type:** Zig-native bench, ReleaseFast, 50M accesses, 8-field records, monomorphic and polymorphic access sites,
5-run best. Zig 0.16.0, aarch64. Artifacts: `project.zig`, `access.csv`.
**Settles:** the `Project` core form (field access `record.field`) lowering, the record-heavy hot path for the
templating/config consumers. Sizes the strategy fork by whether the record shape is statically known and, when
dynamic, monomorphic vs polymorphic at the access site.

## Why this probe

`Project` is a core form, and field access is pervasive (`record.field.subfield`) in the record-oriented
consumers. vehje compiles ahead of time (the Rust compile side), so the central question is how much field access
can be resolved to a direct offset at compile time versus needing a runtime name lookup, and for the dynamic case,
which lookup. Four strategies:

- **Direct offset (compile-resolved):** the field name is resolved to a fixed slot offset at compile time; runtime
  is a plain load. Valid only when the record shape is statically known.
- **Linear name scan:** compare the accessed name against the record's field-name ids in order (small records).
- **Hash lookup:** hash the field name to a slot with open-addressing probing (larger dynamic records).
- **Inline cache (monomorphic):** cache the last `(shape_id, offset)` at the access site; guard on the record's
  shape_id; a hit is a direct load, a miss falls back to a scan and refills the cache. The classic dynamic-language
  field-access accelerator.

Two access-site distributions: monomorphic (the site sees one shape repeatedly, e.g. a loop over a homogeneous
collection) and polymorphic (the shape varies at the site, e.g. heterogeneous data).

## Results (ns per access, 8-field records)

| access site | direct-offset | linear-scan | hash-lookup | inline-cache |
|---|---|---|---|---|
| monomorphic | **0.54** | 1.57 | 1.27 | 0.61 |
| polymorphic | 0.54 (invalid) | 1.57 | 1.27 | 1.89 |

Direct-offset at a polymorphic site is marked invalid: when the shape varies, there is no single fixed offset, so
the strategy does not apply (it is shown only as the speed-of-a-load reference).

## The finding: resolve to a direct offset wherever the shape is static; inline-cache the monomorphic dynamic case; fall back to hash, and detect polymorphism to avoid inline-cache thrashing

The strategy rule follows directly:

- **Static shape: direct offset, 0.54 ns.** When the record shape is known at compile time (locally-built records,
  typed records, the majority case given ahead-of-time compilation), the field name resolves to a fixed offset and
  the access is a single load. Nearly free. This is where most accesses should land, and the design should push as
  many records as possible into statically-known shapes so their projections compile to direct loads.
- **Dynamic shape, monomorphic site: inline cache, 0.61 ns.** When the shape is not known at compile time but the
  access site sees the same shape repeatedly (a loop over a homogeneous collection, the common dynamic case), a
  monomorphic inline cache guards on shape_id and hits the direct load, reaching near-static speed (0.61 vs 0.54).
  The cache pays for itself immediately.
- **Dynamic shape, polymorphic site: hash lookup, 1.27 ns, and turn the inline cache OFF.** This is the
  load-bearing hazard: at a polymorphic site the inline cache is the WORST strategy (1.89 ns, slower than both the
  hash 1.27 and the linear scan 1.57), because the shape guard misses almost every access and pays the failed
  guard plus a refill on top of the fallback scan. So an inline cache that does not detect polymorphism actively
  hurts. The design must detect megamorphic sites (after K consecutive guard misses, mark the site polymorphic and
  stop guarding, going straight to the hash lookup), the standard JS-engine technique. For small records the hash
  (1.27) beats the linear scan (1.57); the gap widens with field count.

The composite lowering: `Project` resolves to a direct offset at compile time wherever the shape is static
(most accesses, 0.54 ns); dynamic-shape accesses compile to a site with a monomorphic inline cache (0.61 ns when
the site stays monomorphic, the common dynamic case) that self-disables into a hash lookup (1.27 ns) once the site
is seen to be polymorphic, so the pathological inline-cache-thrash case (1.89 ns) never persists.

## Design impact

- Push records toward statically-known shapes so `Project` compiles to a direct load (0.54 ns). Typed records,
  locally-built records with a known field set, and record literals all have static shapes. This is the design's
  lever: the more shape is known at compile time, the more field access is free.
- Dynamic-shape field access (config data, heterogeneous collections) uses a per-access-site monomorphic inline
  cache with megamorphic self-disable to a hash lookup. Never a bare inline cache (it thrashes on polymorphic
  sites); never a bare linear scan (the hash wins even at 8 fields).
- This composes with the value model: the record's `shape_id` is the inline-cache guard key, and it is the same
  kind/shape discriminant the NaN-box tag and the `Match` jump-table use, so one shape/kind id serves value
  branching, match dispatch, and field-access caching.
- Field names are interned ids (compared as integers, not strings), so both the scan and the hash compare u32 ids,
  not byte strings; interning (already validated in interner-merge-cost) is a prerequisite for cheap dynamic field
  access.

## Boundary

8-field records; larger records widen the hash's lead over the linear scan and make the direct-offset and
inline-cache advantages larger (a linear scan of 32 fields is far slower, while direct/IC stay flat). Nested
projection (`a.b.c`) is a chain of single projections, each using this rule at its level. The polymorphic bench
uses 4 rotating shapes; a truly megamorphic site (many shapes) makes the inline cache even worse, reinforcing the
self-disable requirement. The hash here is a simple open-addressing probe sized to the field count; a perfect hash
computed at record-shape-creation time would lower the dynamic cost further for hot shapes, an available
optimization not benched here.

## Artifacts
- `project.zig` (the four strategies over monomorphic and polymorphic access sites), `access.csv`.

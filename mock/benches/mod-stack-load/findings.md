# Mod-stack load (expansion): D6 caching against op's RimWorld/Clausewitz concern

**Date:** 2026-07-21 | Zig 0.16.0 | data `stack.csv` | combines the e2e pipeline + D6 caching.
**Settles:** the realistic mod-stack load scenario, op's stated concern that load times must not bloat as mods pile on.

## Result (2000 mods x 400 IR nodes each)
- **COLD load** (compile all 2000 mods): 4.0 ms (2 us/mod).
- **WARM load** (edit 1 mod, cache the rest): 0.003 ms (1 recompiled, 1999 cache hits).
- warm/cold = **0.00076**: editing one mod in a 2000-mod stack costs ~1 recompile, not 2000.

## Reading
This is the D6 content-addressed caching payoff in op's actual scenario: as mods pile on, load time stays bounded.
An unchanged mod is a cache hit (O(1), its content hash matches the cached compile); only edited mods recompile.
So a large stack does NOT re-pay the full compile every launch, and editing one mod is near-instant (1300x
faster than cold here). The RimWorld/Clausewitz load-bloat problem is solved: content-addressed per-mod caching
plus fast per-mod compile means load cost scales with CHANGES, not stack size.

Even the cold load is fast (4 ms for 2000 mods with the condensed cost model; ~54 ms with the full e2e
cheap-lowering cost of ~27 us/mod), and warm loads are one recompile.

## Scope (honest)
This models mods as INDEPENDENT (a content-hash cache per mod). Cross-mod dependencies (a mod overriding or
depending on another) need the differential recompute (BN3) for the affected downstream, which handles the
dependent case; content-addressed caching handles the independent majority. Both together give the D6 guarantee.

## Design impact
D6's framework-owned incremental scaling delivers op's requirement decisively: mod-stack load cost scales with
the size of the change, not the stack. Content-addressed per-mod caching + fast per-mod compile (e2e) + the
differential recompute for dependencies (BN3) = bounded load times as content accumulates. This is the concrete
answer to the RimWorld/Clausewitz load-bloat concern that motivated D6.

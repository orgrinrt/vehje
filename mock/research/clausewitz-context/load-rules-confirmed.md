# Confirmed Load Order Rules

Extracted from Workshop pages, community guides, and collection analysis.
These feed into depends_on / loads_after fields in full_index.jsonl.

## Hard Dependencies (depends_on — must be present AND load before)

| Mod | Depends On | Source |
|---|---|---|
| Fleet Formation Mod (2789951717) | Downscaled Ships (2662937312) | Workshop page |
| Ruler Level System (2573906846) | Universal Modifier Patch (1688887083) | Workshop page |
| BPVR submods | BPVR Compatibility Patch (3576949303) | Ecosystem structure |
| All PD submods | Planetary Diversity (819148835) | Ecosystem structure |
| All ESC NEXT submods | ESC NEXT (2648658105) | Ecosystem structure |
| Kek's Origins PD Patch (3504866637) | Kek's Origins + PD | Compat patch structure |
| PD+Guilli's Compat (2857972680) | PD + Guilli's PM | Compat patch structure |
| Sort Those Buildings PD patches | Sort Those Buildings + PD | Compat patch structure |

## Load Order Rules (loads_after — if both present, this loads later)

| Mod | Loads After | Reason | Source |
|---|---|---|---|
| Fleet Formation Mod | Downscaled, Real Space, NSC3, all shipsets | Must override ship_sizes last | Workshop |
| Universal Modifier Patch (1688887083) | Universal Game Rules Patch (2409276081) | Framework ordering | Community consensus (3 collections) |
| Universal Resource Patch (1595876588) | Universal Modifier Patch (1688887083) | Framework ordering | Community consensus (4 collections) |
| ESC NEXT (2648658105) | NSC3 (683230077) | ESC adds to NSC3 classes | Community consensus (3 collections) |
| ESC Overwrites: Component Progression | ESC NEXT | Submod loads after parent | Community consensus |
| ESC Overwrites: Global Ship Designs | ESC Overwrites: Component Progression | Sequential submods | Community consensus |
| ESC Overwrites: Special Weapon Types | ESC Overwrites: Global Ship Designs | Sequential submods | Community consensus |
| PD submods | PD base | Submods after parent | Standard practice |
| Compat patches | Both parent mods | Patches override both | Standard practice |
| NGS (2928840618) | After most content mods | GC overhaul sits above content | Batch 2 research |
| Factional Politics (3322346400) | Before large overhauls | pop_categories overwrite concern | Workshop page |
| At War: Planetary Cannons Submod (3580035003) | Directly after At War: Planetary Cannons | Author requirement | Workshop |

## Coarse Load Groups (framework loads first, megapatch last)

1. **framework** — Ariphaos, Universal patches, Shader Merge, General Fixes, BPU
2. **base_content** — PD, Real Space, Guilli's, NSC3, ESC NEXT (ecosystem parents)
3. **content** — trait mods, event mods, tradition mods, origin mods, civic mods
4. **overhaul** — NGS, FP, GP, DPE, Stellar AI, At War, Federation Overhaul, VF, Theta
5. **ecosystem_submods** — PD submods, ESC submods, UIOD submods, Hydra submods
6. **compat_patches** — all compat/bridge patches between specific mod pairs
7. **visual** — portraits, shipsets, cosmetic, audio, loading screens
8. **late_override** — Fleet Formation, Downscaled Ships (must override ship_sizes last)
9. **megapatch** — our megapatch (second-to-last)
10. **heritage** — our heritage mod (last)

## Known Incompatibilities (we patch in megapatch)

| Mod A | Mod B | Conflict | Resolution |
|---|---|---|---|
| Viable Feudalism | Federation Overhaul | Author says "almost certainly incompatible" | Megapatch merge |
| Factional Politics | BPU | Both override pop_social_classes | Megapatch merge |
| BPU | Many event mods | BPU overrides on_actions | Megapatch merge |
| At War suite | NSC3/ESC | Section/component overlap | Megapatch merge (At War authors refuse patches) |
| 4+ trait mods | Each other | 92 DIFFERENT trait ID collisions | Megapatch merge (pick best version per trait) |

## Additional Rules from Batch 1 (Framework Mods)

| Mod | Rule | Source |
|---|---|---|
| Ariphaos Unofficial Patch (1995601384) | MUST be position #1 (absolute top) | Workshop: "If not first, will clobber many other mods" |
| UIOD (1623423360) | Near bottom of load order | Workshop: "very bottom, as in last" |
| Universal Resource Patch (1595876588) | Absolute bottom (last of Universal patches) | Workshop + community |
| ESC NEXT (2648658105) | Before NSC3 | Workshop: "must be loaded before NSC3" |
| NSC3 (683230077) | Requires UIOD | Workshop: listed as required item |
| Stellar AI (3610149307) | Near bottom | Workshop: author recommends near bottom |
| BPU (2475302050) | Requires Casako's Framework (2466607238) — WE DON'T HAVE THIS | Workshop page |

## Updated Coarse Ordering (top to bottom)

1. Ariphaos Unofficial Patch (MUST BE #1)
2. Stellaris General Fixes
3. Universal Game Rules Patch
4. Universal Modifier Patch
5. (content mods — PD, Guilli's, traits, events, etc.)
6. ESC NEXT (before NSC3)
7. NSC3
8. (overhaul mods — NGS, FP, GP, Theta, etc.)
9. (compat patches)
10. BPU (near bottom — on_actions override)
11. Stellar AI (near bottom)
12. Fleet Formation (after all shipsets)
13. UIOD (near bottom per author)
14. UIOD compat patches (after UIOD)
15. Universal Resource Patch (last of universals)
16. **Megapatch** (second-to-last)
17. **Heritage** (last)

## BPU Dependency Issue

BPU requires Casako's Framework (2466607238) which is NOT in our approved stack.
Options:
- Add Casako's Framework to approved
- Determine if BPU works without it (maybe only optional?)
- Our megapatch's on_actions merge may make Casako's unnecessary
- Investigate when mod files are downloaded

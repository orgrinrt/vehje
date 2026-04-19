# clause

General-purpose scripting language for game modding + embeddable runtimes.

Clause is a **Rust compiler + Zig runtime** with a C ABI between them. Game-specific targets (Clausewitz, Witcher 3, Sims 4, Lua) plug in as extension repos — the core knows about no particular game.

## Substrate

- [`arvo`](https://github.com/orgrinrt/arvo) — numeric primitives + analysis algorithms
- [`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin) — pipeline execution engine (used by Clause runtime)

## Extensions (future)

- `clause-jomini` — Clausewitz grammar + codegen (Stellaris / CK3 / HOI4 / Vic3 / EU4)
- `clause-w3` — Witcher 3 script target
- `clause-ts4` — Sims 4 script target
- `clause-lua` — Lua target

## Status

**Seed.** Extracted from a Python prototype in `stellar-heritage` on 2026-04-19. Design seeds are in `mock/design_rounds/`; see the initial changelist for what's imported and what needs fresh consolidation.

First real session starts with: compiler crate split, runtime split, extension-point contract.

## Reference

The Python prototype remains at `stellar-heritage/tools/clause/` (15k lines, 1500+ tests). It serves as parity oracle during the Rust rewrite — not something to translate line-by-line.

## Contributing

Design conversations happen in `mock/design_rounds/`. Implementation PRs land in `mock/crates/` against the mockspace validation gate.

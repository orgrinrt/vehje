# `clause`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/clause.svg)](https://github.com/orgrinrt/clause/stargazers)
[![Crates.io](https://img.shields.io/crates/v/clause)](https://crates.io/crates/clause)
[![docs.rs](https://img.shields.io/docsrs/clause)](https://docs.rs/clause)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/clause.svg)](https://github.com/orgrinrt/clause/issues)
![License](https://img.shields.io/github/license/orgrinrt/clause?color=%23009689)

> General-purpose scripting language for game modding and embeddable runtimes. Rust compiler, Zig runtime, C ABI between them. Game-agnostic core; targets ship as separate extension repos.

</div>

## What it is

Clause is a scripting language with a two-language implementation. The compiler is Rust: lex, parse, resolve, typecheck, transpile, codegen, pass scheduler. It runs at development time and ships as a single static binary per target platform. The runtime is Zig: small, fast, embeddable, speaks C ABI, pulls no `std` and no Rust runtime. The two are joined by a C ABI that is defined and validated in Rust.

The core is strictly game-agnostic. Anything that names a specific game (Clausewitz, Witcher 3, Sims 4, Lua) lives in a sibling extension repo, not in this one. The extension-point contract is one of the early design rounds; until it lands, inlining game-specific vocabulary, types, or codegen into the core is forbidden because it makes later extraction harder.

Compiler posture is strict. Deny-warnings at crate roots, no `.unwrap()` / `.expect()` outside tests and documented infallible paths, tagged enums plus trait dispatch over `dyn Trait` / `TypeId` / `std::any`. Errors carry span, phase, expected, and actual context rather than a bare string. Procedural macros exist only for Clause language semantics; `macro_rules!` is acceptable internally in the Rust compiler implementation for boilerplate reduction, but never for Clause itself, where macros are proc-only, scheduler-pass-driven, and typechecked.

## Status

**Seed.** Extracted from a Python prototype in `stellar-heritage` on 2026-04-19. Design seeds live in `mock/design_rounds/`; see the initial changelist for what is imported and what still needs fresh consolidation. The first substantive design rounds (R1-R4) decide compiler crate split, Zig runtime C ABI split, extension-point contract, and the validator framework. `mock/crates/` is intentionally empty at seed; crate-level rules and lints arrive once R1-R4 settles the split.

First real session begins with: compiler crate split, runtime split, extension-point contract.

## Contents

Crate layout is provisional (pending R1-R4) and currently resides under `mock/crates/`:

| Crate | Role |
|---|---|
| `clause-lex` | Tokenisation. |
| `clause-syntax` | Parser + AST. |
| `clause-resolve` | Module resolution, use-tree, visibility. |
| `clause-typecheck` | Coherence, bind targets, generics, validator framework. |
| `clause-schedule` | Pass DAG. Consumes `arvo-graph`. |
| `clause-codegen` | Target-agnostic code generation. |
| `clause` | Top-level binary and orchestration. |

Game-specific targets ship as separate repos that extend Clause:

| Extension (future) | Target |
|---|---|
| `clause-jomini` | Clausewitz grammar and codegen (Stellaris / CK3 / HOI4 / Vic3 / EU4). |
| `clause-w3` | Witcher 3 script target. |
| `clause-ts4` | Sims 4 script target. |
| `clause-lua` | Generic Lua target. |

## Two-language strategy

The compiler runs at development time; the runtime embeds in the host program. Splitting them by language keeps each side honest about its dependencies.

**Compiler (Rust).** Everything the user invokes via `clause` on the command line. Cross-compilation is first-class: one static binary per host platform emits artefacts for any supported target. No dynamic runtime dependencies, no plugin shared-object loading at the compiler layer.

**Runtime (Zig).** Everything that executes inside a hosted program: macros, scratch variables, generative pipelines. Built on `hilavitkutin` for morsel-driven execution. The runtime may embed differently per host (static link, shared library, wasm module).

**Build tooling / deployment / orchestration.** Written in Clause itself once the language is usable. Same split as the rustc/cargo separation: the compiler does one thing, tooling runs on top of what the compiler emits.

## Python parity oracle

A Python prototype lives at `stellar-heritage/tools/clause/` (roughly 15k lines, 1500+ tests). It is the parity oracle during the Rust port: the Python test corpus runs against the Rust implementation to verify semantic equivalence. It is not a line-by-line translation source. The Rust shape differs: tagged enums over Protocol duck-typing, monomorphised trait dispatch over `isinstance` checks, `Outcome<T, E>` over exceptions, static composition over runtime registry lookups. Read the Python for semantics; author the Rust from the design docs.

## Single-binary distribution

End users should not need Python, Deno, .NET, JVM, or any runtime alongside Clause. `cargo build --release` produces one static binary per target. The runtime embeds in the host program with whatever linkage the host chooses; the compiler itself has no dynamic runtime dependencies.

## Installation

Clause is not yet published to crates.io. Until the seed round lands crates at version `0.1`, consumers path-dep into `mock/crates/*`:

```toml
[dependencies]
clause = { path = "../clause/mock/crates/clause" }
```

Once R1-R4 graduates the crate split to root-level `crates/`, `cargo add clause` and the usual crates.io flow will apply.

## Positioning

`clause` sits on top of three substrate repos:

- [`notko`](https://github.com/orgrinrt/notko) — foundation primitives: `Just` / `Maybe` / `Outcome` / `MaybeNull`.
- [`arvo`](https://github.com/orgrinrt/arvo) — numeric primitives plus analysis algorithms. The compiler uses `arvo-graph` for the pass DAG and `arvo-bitmask` for artifact read/write sets.
- [`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin) — pipeline execution engine. The Clause runtime uses it for morsel-driven macro and scratch execution.

Reuse across the substrate is strict: the Clause repo does not reimplement DAG topology, bitmask ops, sparse storage, or pipeline scheduling. Those live in `arvo` and `hilavitkutin`. Public APIs in Clause use `Maybe` / `Outcome` in place of `Option` / `Result` and `arvo` primitives in place of bare numerics. Bare `core` primitives appear only in trait method signatures fixed by the language.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/clause/blob/dev/LICENSE)

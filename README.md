# `vehje`

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/vehje.svg)](https://github.com/orgrinrt/vehje/stargazers)
[![Crates.io](https://img.shields.io/crates/v/vehje)](https://crates.io/crates/vehje)
[![docs.rs](https://img.shields.io/docsrs/vehje)](https://docs.rs/vehje)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/vehje.svg)](https://github.com/orgrinrt/vehje/issues)
![License](https://img.shields.io/github/license/orgrinrt/vehje?color=%23009689)

> A general-purpose scripting language. Rust compiler, Zig runtime, C ABI between them. Compiles to an IL that the runtime interprets, with room to lower further as the language matures.

</div>

## What it is

Vehje is a scripting language with a two-language implementation. The compiler is written in Rust: it lexes, parses, resolves names, type-checks, schedules passes, and emits an intermediate representation. The runtime is written in Zig: small, embeddable, speaks C ABI, interprets the IL the compiler emits. The two halves are joined by a C ABI defined and validated on the Rust side.

The IL is the shipping contract between compiler and runtime. Programs compile ahead of time; the runtime reads the resulting artefact and executes it. Later versions can lower the IL further (denser bytecode, ahead-of-time native emission for selected targets) without changing the source language or the runtime's published ABI. Nothing in the language design assumes interpretation in the traditional "source-at-the-ready" sense.

Posture is strict. Deny-warnings at crate roots. No `.unwrap()` or `.expect()` outside tests and documented infallible paths. Tagged enums plus trait dispatch instead of `dyn Trait` or `TypeId` in framework code. Errors carry span, phase, expected, and actual context. Procedural macros exist only for language-level semantics; they are typechecked and scheduler-driven, not ad hoc token shuffling.

## Status

**Seed.** The compiler is scaffolded as a split of small crates (lex, syntax, resolve, typecheck, schedule, codegen, runtime ABI, plus a binary); the runtime is scaffolded as a Zig side that speaks the C ABI. The surface is in place; the substantive bodies land crate by crate as the work progresses.

## Contents

The shipping layout, compiler side:

| Crate | Role |
|---|---|
| `vehje-lex` | Tokenisation. |
| `vehje-syntax` | Parser, AST. |
| `vehje-resolve` | Module resolution, use-tree, visibility, manifest. |
| `vehje-typecheck` | Coherence, generics, validator framework. |
| `vehje-schedule` | Pass DAG. |
| `vehje-codegen` | Target-agnostic code generation into the IL. |
| `vehje-ir` | Intermediate representation shared across compiler crates. |
| `vehje-runtime-abi` | C ABI surface the runtime speaks. |
| `vehje` | Top-level binary. |

The runtime side lives alongside as a Zig project, linked by the C ABI declared in `vehje-runtime-abi`.

## Two halves, one ABI

Splitting the compiler and runtime by language keeps each side honest about its own dependencies and deployment shape. The compiler runs at development time and ships as one static binary per host. The runtime embeds into the program that uses Vehje scripts and has no Rust runtime dependency. Cross-compilation is first-class on the compiler side; linkage on the runtime side is whatever the host program chooses (static, shared library, wasm module).

Build tooling, deployment, and orchestration are written in Vehje itself once the language is usable. The compiler does one thing; tooling runs on top of what the compiler emits.

## Substrate

Vehje depends on three small crates that the language's shape relies on:

- [`notko`](https://github.com/orgrinrt/notko) — foundation primitives: `Just<T>`, `Maybe<T>`, `Outcome<T, E>`, `MaybeNull<T>`. Replaces `Option<T>` and `Result<T, E>` at API boundaries.
- [`arvo`](https://github.com/orgrinrt/arvo) — numeric substrate: fixed-point primitives with strategy markers. Replaces bare integer and float types at API boundaries.
- [`hilavitkutin`](https://github.com/orgrinrt/hilavitkutin) — pipeline execution engine. The runtime uses it for scheduling and dispatch; the compiler uses the same machinery for its internal pass graph.

Public APIs in Vehje use the substrate vocabulary (`Maybe` / `Outcome` / `UFixed` / `IFixed` / interned strings) rather than bare `core` primitives. Bare primitives appear only where a language-level trait signature fixes the choice.

## Installation

Vehje is pre-release. Follow the repository for tagged releases; `cargo add vehje` will become available once the compiler crate publishes.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/vehje/blob/dev/LICENSE)

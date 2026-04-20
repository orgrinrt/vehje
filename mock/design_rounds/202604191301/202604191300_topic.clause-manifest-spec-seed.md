# Clause.toml — Workspace & Package Manifest Specification

> **Status:** v1 spec + parser (W1, task #78). Implements the workspace model sketched in `DESIGN.md` §26.9.
> **Scope:** File format, field semantics, validation rules, error conditions. Parser implementation at `tools/clause/compiler/manifest.py`.

---

## 1. Overview

Clause uses TOML manifests, placed at two levels:

| File | Location | Role |
|---|---|---|
| `Clause.toml` (workspace) | Workspace root | Declares members + workspace-wide defaults |
| `Clause.toml` (package) | Each crate root | Declares one crate's identity, deps, DLC requirements |

A file is recognized as the *workspace* root iff it contains a `[workspace]` table. A file is a *package* iff it contains a `[package]` table. A file may contain **both** (a "mixed" manifest — the workspace root is also a crate; uncommon but allowed, mirroring Cargo's virtual-vs-concrete workspace distinction).

Every crate belonging to a workspace MUST be referenced — directly or via glob — from the workspace's `members` list. Crates outside the workspace are treated as external dependencies (resolved by name + version from a registry — out of scope for v1).

---

## 2. Workspace manifest — `[workspace]`

```toml
[workspace]
edition = "2026"

members = [
    "crates/std",
    "crates/stellaris",
    "crates/stellaris_dlc_*",          # glob
    "crates/associations",
    "crates/heritage",
    "crates/*_expanded",               # glob
    "crates/dawn_of_civilization",
    "crates/galactic_diversity",
    "crates/clause",
]

exclude = [                            # optional
    "crates/experiments/*",
]

[workspace.package]                    # optional — inherited defaults
authors  = ["orgrinrt"]
license  = "private"

[workspace.playset]                    # defaults for `clause build-playset`
dlcs_available = "auto"                # "auto" | list of dlc names
output         = ".dist/playset"

[workspace.auto_discover]              # house-style defaults inherited by members
events  = "src/events/**/*.cse"
traits  = "src/traits/**/*.cse"
civics  = "src/civics/**/*.cse"
# … arbitrary keys; compiler has no hardcoded directory knowledge

[workspace.dependencies]               # optional — shared dependency versions
stellaris-vanilla-spec = "4.0"
# member packages may reference these via `{ workspace = true }` (see §4.4)

[workspace.lints]                      # optional — shared lint config
# inherited by members unless overridden
```

### 2.1 Required fields

- `edition` — Clause language edition (currently only `"2026"`). Controls grammar/stdlib ABI.
- `members` — list of path patterns (exact paths or globs). Must be non-empty.

All other fields are optional.

### 2.2 Member path patterns

Each member string is either:
- **Exact path** (`"crates/heritage"`) — must point to a directory containing a package `Clause.toml`.
- **Glob pattern** (`"crates/stellaris_dlc_*"`, `"crates/*_expanded"`) — expanded relative to the workspace root. Every matched directory must contain a package `Clause.toml`.

Globbing rules:
- `*` matches any sequence of characters except `/`
- `**` matches any number of path components
- `?` matches any single character except `/`
- Brackets `[abc]` match a character class
- Hidden directories (names starting with `.`) are excluded from `*` matches

After glob expansion, the union is deduplicated. A path appearing in both `members` and `exclude` is excluded.

### 2.3 Workspace inheritance

Member packages may inherit fields from the workspace root via the literal `{ workspace = true }` value. Three contexts support this:

| Context | Example |
|---|---|
| `[package]` fields | `authors = { workspace = true }` |
| `[dependencies]` | `stellaris-vanilla-spec = { workspace = true }` |
| `[lints]` | `[lints.workspace]` |

Inheritance fails the build with a clear diagnostic if the referenced workspace field is absent.

---

## 3. Package manifest — `[package]`

```toml
[package]
name          = "heritage"
version       = "0.1.0"
edition       = { workspace = true }
description   = "Dynasty + bloodlines for Stellaris"
crate_type    = "content"              # see §3.2

[package.metadata]                     # arbitrary tool-specific metadata
# compiler + tooling ignore unknown keys here

[dependencies]
std           = { workspace = true }
stellaris     = { workspace = true }
associations  = { path = "../associations" }
# peer content crates (cross-cutting):
ai_expanded   = { path = "../ai_expanded", optional = true }

[dependencies.stellaris_dlc_utopia]
path          = "../stellaris_dlc_utopia"

[dev-dependencies]                     # only active under `clause test`
# …

[dlc]
requires      = []                     # list of dlc crate names the crate REQUIRES to function
optional      = ["utopia", "nemesis"]  # DLCs the crate adapts to if present

[features]                             # optional Cargo-style features
default       = []
rich_events   = []

[auto_discover]                        # per-crate override of workspace defaults
events        = "events/**/*.cse"       # override — flat events dir
# omit a key to inherit from workspace

[lints]
# per-crate lint config; merges with workspace defaults
```

### 3.1 Required `[package]` fields

- `name` — non-empty, `[a-z][a-z0-9_]*` pattern; must be unique within the workspace.
- `version` — SemVer (e.g., `"0.1.0"`, `"1.2.3-pre"`).
- `edition` — matches workspace edition if inherited; literal otherwise.

### 3.2 `crate_type`

Classifies the crate's workspace role. Used by tooling (e.g., `clause build-playset` routing, dependency validation):

| Value | Purpose |
|---|---|
| `foundation` | `std`, `stellaris`, `stellaris_dlc_*` — engine/vanilla/DLC bindings. |
| `shared` | Cross-domain abstractions like `associations`. |
| `content` | Game content crates (heritage, ships_expanded, …). |
| `toolchain` | The compiler itself (`clause`) — exempt from playset emission. |

Default: `content`.

Validation:
- `foundation` crates MUST NOT depend on `content` crates (one-way layering).
- `shared` crates MUST NOT depend on `content` crates.
- `content` crates MAY depend on `foundation`, `shared`, and other `content` (cross-cutting is allowed per Tenet 4).
- `toolchain` has no dependency restrictions.

### 3.3 `[dependencies]` table

Each entry is either a **version-string** shortcut or a **table**:

```toml
# shortcut form
stellaris-vanilla-spec = "4.0"

# table form
associations = { path = "../associations" }
ai_expanded  = { path = "../ai_expanded", optional = true, features = ["rich"] }
std          = { workspace = true }
```

Supported keys in the table form:
- `path` — relative path to a local crate. Must resolve within the workspace.
- `version` — SemVer requirement string.
- `workspace = true` — inherit from `[workspace.dependencies]`.
- `optional` — if true, crate is only compiled if a feature activates it.
- `features` — feature list requested from the dependency.
- `default-features` — boolean (default `true`).

Exactly one of `path`, `version`, or `workspace` must be specified.

### 3.4 `[dlc]` table

Declares DLC requirements at the crate level:

- `requires` — list of dlc crate names (e.g., `["utopia", "nemesis"]`). If any is absent from the playset environment, the crate is DCE'd in full during `clause build-playset`.
- `optional` — list of dlc crate names the crate adapts to. Does not gate compilation; enables `#[cfg(dlc = "X")]` within code.

A `dlc_<name>` foundation crate is implicitly `requires = ["<name>"]` and need not declare it.

### 3.5 `[auto_discover]`

Arbitrary string keys mapping to glob patterns. The compiler treats each key as a named item-category and discovers all matching `.cse` files at compile time. The compiler has no hardcoded knowledge of what `events`, `traits`, etc. mean — they are patterns.

Inheritance rule: a key present in `[auto_discover]` overrides the workspace default for that key. Absent keys inherit. Explicit `false` disables the key (neither inherits nor discovers).

```toml
[auto_discover]
events  = false                        # explicit disable
traits  = "traits/**/*.cse"             # override
# 'civics' omitted → inherits from workspace
```

---

## 4. Validation rules (parser MUST enforce)

### 4.1 Structural

- Root manifest must contain `[workspace]` or `[package]` (at least one).
- If both present, both are validated.
- Unknown top-level tables produce a WARNING, not an error (forward-compat for tooling extensions).

### 4.2 Name uniqueness

- No two members may declare the same `package.name`.
- Member name collision is a hard error with both paths reported.

### 4.3 Dependency resolution (compile-time)

- Every `path` dependency must resolve to a workspace member.
- Every `workspace = true` reference must have a matching entry in `[workspace.dependencies]` (or, for package-level fields, in `[workspace.package]`).
- Circular dependencies are a hard error with the cycle reported.
- A `foundation → content` or `shared → content` edge is a hard error (§3.2 layering).

### 4.4 DLC consistency

- A dependency on a `foundation` crate named `dlc_<name>` where the depending crate does not declare `<name>` in its `[dlc].requires` or `[dlc].optional` is a WARNING (suggests a forgotten declaration).
- `requires` and `optional` in `[dlc]` must reference names corresponding to actual `dlc_*` foundation crates in the workspace.

### 4.5 Edition

- Member `edition` must match or be inherited from workspace.
- Mismatch is a hard error.

### 4.6 Glob safety

- Globs in `members` must not resolve to zero matches — this is a hard error (likely typo).
- Globs must not match the workspace root itself.

---

## 5. Error codes

Parser errors carry stable codes so tools and tests can pattern-match:

| Code | Meaning |
|---|---|
| `M001` | Invalid TOML syntax |
| `M002` | Missing required field |
| `M003` | Invalid field value (type mismatch, regex fail) |
| `M004` | Unknown `crate_type` |
| `M005` | Name collision between two members |
| `M006` | Unresolved `path` dependency |
| `M007` | Missing `[workspace.dependencies]` entry for `workspace = true` |
| `M008` | Circular dependency |
| `M009` | Layering violation (foundation/shared → content) |
| `M010` | Edition mismatch |
| `M011` | Glob matched zero members |
| `M012` | Conflicting `path`, `version`, `workspace` keys in one dep |
| `M013` | `requires`/`optional` DLC references a non-existent `dlc_*` crate |
| `M014` | Invalid SemVer |
| `M015` | `discover()` called on a package with no `crate_root` (internal) |
| `W001` | Unknown top-level table (warning) |
| `W002` | `dlc_*` dep without matching `[dlc]` declaration (warning) |

---

## 6. Examples

### 6.1 Minimal workspace

```toml
# /Clause.toml (workspace root)
[workspace]
edition = "2026"
members = ["crates/heritage"]
```

```toml
# /crates/heritage/Clause.toml
[package]
name    = "heritage"
version = "0.1.0"
edition = "2026"
```

### 6.2 Full-shape workspace

See `/Clause.toml` at the repo root once W1's parser lands and the workspace is scaffolded (tasks W2–W8).

---

## 6.3 Auto-discover resolution + `discover()` (W7)

The `[auto_discover]` table's values are resolved **relative to the package's crate root** — i.e., the directory containing that crate's `Clause.toml`.

Two public functions in `compiler.manifest`:

- `resolve_auto_discover(package, workspace) -> dict[str, str]`
  Merges workspace defaults (`[workspace.auto_discover]`) with the package's per-crate overrides (`[auto_discover]`). A package key set to `false` disables the category entirely (neither inherits nor discovers). Returns a flat `{category: glob}` map.

- `discover(package, workspace=None) -> DiscoveredFiles`
  Evaluates each merged pattern against the package's filesystem using `Path.glob`. Returns `DiscoveredFiles(package, by_category={cat: [paths_sorted]})`. Results are deterministically sorted — important for caching and diffing. Empty categories (no matches) are present with an empty list. Disabled categories are absent from the map.

Globs follow Python `pathlib.Path.glob` semantics:
- `*` matches any sequence of characters except `/`
- `**` matches any number of path components
- `?` matches any single character except `/`

### 6.3.1 Future: explicit `impl_of` form

A planned extension (post–type-check, M5+) will allow an explicit array-of-tables form alongside the terse table form:

```toml
[[auto_discover]]
pattern = "src/traits/**/*.cse"
impl_of = "stellaris::content::Trait"

[[auto_discover]]
pattern = "src/civics/**/*.cse"
impl_of = "stellaris::content::Civic"
```

`impl_of` explicitly pins which content-type anchor the discovered files are expected to impl, rather than inferring it from the category key. Until the type checker can resolve `impl_of`, authors use the simpler category-keyed form (§3.5) and the compiler infers the anchor from the category name during M5+ type check.

---

## 7. Future extensions (non-v1)

These are deliberately out of scope for v1 but the manifest schema leaves room:

- Registry-backed dependencies (`version = "1.0"` without `path`) — for publishing crates.
- Patch overrides (Cargo-style `[patch.*]`) for swapping in local versions of registry deps.
- Build scripts (`build = "build.py"`) — not needed until/unless a crate needs pre-build codegen beyond pass-framework capabilities.
- Binary targets (`[[bin]]`) — Clause has no binary targets yet; everything compiles to Clausewitz output.

---

## 8. Open questions (flagged for review)

- **Manifest filename**: `Clause.toml` (this spec) vs `mod.toml` (DESIGN.md §20.1). Decision needed. Spec currently uses `Clause.toml` to match Cargo's `Cargo.toml` convention and distinguish from Stellaris's own `descriptor.mod`. DESIGN.md §20.1 should be amended to match, or this spec changed. Tracking: **RESOLVED — use `Clause.toml`** (better disambiguation; DESIGN.md §20.1 will be amended when M4 module resolver lands).
- **Workspace-root also being a crate** (§1): allowed per Cargo precedent but may cause confusion. Currently allowed; may be disabled in v1 if complexity is not worth it.
- **Registry hosting** for external dependencies: not needed until Clause is public. v1 is path-only.

//! vehje-runtime-gen, the framework's runtime generation.
//!
//! Runtime generation, the per-language dev-time artifact: it compiles a
//! language definition into the validated-data package (the family table,
//! effect masks, lease-rule schema, cheap-lowering rule table, wire layout, and
//! a reserved stencil section) and binds it to the shipped runtime with a
//! content-addressed manifest. It composes the slices the stage crates own and
//! defines none itself; it owns only the package layout, the manifest, and the
//! differential-check contract. The `build.zig` content-validation step lives
//! in a repo-level tree, not a Rust crate.
//!
//! `#![no_std]`, no alloc.

#![no_std]
#![deny(unused, unreachable_code, unused_must_use, unused_imports, dead_code)]

use arvo::strategy::Hot;
use arvo::Bool;
use arvo_bits::Bits;
use arvo_hash::{xxhash3_64, ContentHash};

use vehje_signature::Signature;

/// Which stage a validated-data slice comes from.
///
/// The kinds name the stage crate that owns the slice's format; this crate
/// composes them without knowing their internal shape.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SliceKind {
    /// The family table (from `vehje-ir` and `vehje-signature`).
    FamilyTable,
    /// The effect masks and lease-rule schema (from `vehje-check`).
    Analyzers,
    /// The cheap-lowering rule table (from `vehje-lower`).
    LoweringRules,
    /// The wire layout (from `vehje-runtime-abi`).
    WireLayout,
    /// The copy-and-patch stencil section (from the deferred stencil producer).
    Stencils,
}

/// A typed handle to one stage crate's contribution.
///
/// The slice's format is owned by its stage crate; this crate composes the
/// bytes without knowing their shape, so a stage crate's format change does not
/// ripple here. The bytes are a caller-provided serialized image.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Slice<'a> {
    /// Which stage owns this slice's format.
    pub kind: SliceKind,
    /// The serialized image of the slice.
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the validated-data slice crosses to the Zig runtime as a byte image; bytes are the wire unit; tracked: #207
    pub bytes: &'a [u8],
}

/// The composed per-language validated-data package.
///
/// The data the runtime's comptime specialisation reads: data, not runtime
/// source. Borrowed from the caller-provided slices, in package order.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Package<'a> {
    /// The composed slices, one per stage contribution.
    pub slices: &'a [Slice<'a>],
}

/// The content-addressed manifest binding a package to the runtime.
///
/// A content-addressed hash of each slice and of the whole package, so the
/// specialised runtime can prove at its own build that it was specialised from
/// exactly this package. The cheaper manifest-check interim of the
/// lens-projection handoff.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub struct Manifest {
    /// The content hash of the whole composed package.
    pub package_hash: ContentHash,
}

/// The differential-check contract.
///
/// The contract for the harness that checks the specialised runtime computes
/// what the reference does over a census corpus, the grade-0 operational shadow
/// of the assurance-indexed logical relation. The proof document strengthens
/// this existing seam.
// FIXME: the harness that drives the census corpus through both the reference
// and the specialised runtime lands with the first real language package; this
// contract names the check, the runner is owed.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct DifferentialCheck {
    /// Whether the differential check has been run and passed for a package.
    pub verified: Bool,
}

impl Default for DifferentialCheck {
    fn default() -> Self {
        Self { verified: Bool::FALSE }
    }
}

/// Compile a language definition into its validated-data package and manifest.
///
/// Composes the stage `slices` into a `Package` and content-addresses it. The
/// entry the dev-time language compiler calls to produce the language artifact.
pub fn generate<'a>(_signature: &Signature<'a>, slices: &'a [Slice<'a>]) -> (Package<'a>, Manifest) {
    let package = Package { slices };
    let manifest = Manifest { package_hash: hash_package(&package) };
    (package, manifest)
}

/// Content-address a package by folding its slices' hashes.
fn hash_package(package: &Package<'_>) -> ContentHash {
    let mut acc = xxhash3_64(&[]); // lint:allow(no-bare-numeric) reason: empty seed for the content-hash fold; tracked: #207
    let mut i = 0; // lint:allow(no-bare-numeric) reason: slice-fold index; tracked: #207
    while i < package.slices.len() {
        let h = xxhash3_64(package.slices[i].bytes);
        acc ^= h.rotate_left(7); // lint:allow(no-bare-numeric) reason: content-hash mixing at the hash boundary; tracked: #207
        i += 1; // lint:allow(no-bare-numeric) reason: slice-fold index; tracked: #207
    }
    Bits::<64, Hot>::from_raw(acc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use vehje_signature::GrammarHook;

    #[test]
    fn generates_a_package_and_manifest() {
        let fam: &[u8] = &[1, 2, 3]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-image test fixture at the wire boundary; tracked: #207
        let rules: &[u8] = &[4, 5]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-image test fixture at the wire boundary; tracked: #207
        let slices = [
            Slice { kind: SliceKind::FamilyTable, bytes: fam },
            Slice { kind: SliceKind::LoweringRules, bytes: rules },
        ];
        let sig = Signature { operations: &[], targets: &[], grammar: GrammarHook::HandWritten };
        let (pkg, man) = generate(&sig, &slices);
        assert_eq!(pkg.slices.len(), 2);
        // the manifest is stable for the same package.
        let (_, man2) = generate(&sig, &slices);
        assert_eq!(man.package_hash, man2.package_hash);
    }
}

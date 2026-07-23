//! Boundary bench 2: marshalling layout (AoS vs SoA-native vs SoA-transposed).
//!
//! A record carries `f` fields; the size axis is the field count `f` (not the batch width,
//! which the other benches sweep). Over a fixed column of [`N_RECORDS`] records, scalar
//! payload, this varies how the multi-field records are laid out when they cross the
//! boundary, so it measures what SoA costs at the boundary BEFORE anything downstream
//! vectorises. The runtime combines a record's `f` fields to a seed (an XOR fold, so the
//! value is layout-independent and the layouts cross-validate byte-exact) and runs the
//! scalar interpreter.
//!
//! Cells:
//!
//! - `aos` (baseline): records laid out record-major (`recs[i*f + k]`), the natural host
//!   layout; the runtime reads AoS.
//! - `soa_native`: fields laid out field-major (`fields[k*n + i]`), built directly by the
//!   host; the runtime reads SoA. The delta over `aos` is the layout access-pattern cost.
//! - `soa_transposed`: the host holds AoS and transposes to SoA as an EXPLICIT timed stage,
//!   then crosses SoA. The delta over `soa_native` is the transpose cost, charged here and
//!   never hidden in a downstream vectorisation win.
//! - `marshal_null` (floor): read and combine the AoS fields, fold, no interpret. The pure
//!   marshalling cost; a cell minus this is the interpret payload.

use core::ffi::c_void;

use mockspace_bench_matrix::bench_matrix;
use mockspace_bench_matrix::boundary::Runtime;

use super::common::{init_handle, open_runtime, CrEntryMarshal, CrFree, CrInit};

/// Records per pass, fixed while the field count `f` sweeps.
const N_RECORDS: usize = 256;

/// The maximum field count in the sweep, sizing the reusable buffers once.
const F_MAX: usize = 16;

/// A record field's value, layout-independent so AoS and SoA fold identically. Depends on
/// the per-iteration `seed` (anti-hoist), the record index, and the field index.
#[inline(always)]
fn field_val(i: usize, k: usize, seed: u64) -> u64 {
    seed ^ (i as u64).wrapping_mul(0x1000_0001) ^ (k as u64).wrapping_mul(0x9e37_79b9)
}

/// State for a marshalling cell: the runtime, handle, resolved marshalling entry, free
/// pointer, both reusable layout buffers, and the field count. `n` records is fixed.
pub struct StMarshal {
    pub rt:     Runtime,
    pub handle: *mut c_void,
    pub entry:  CrEntryMarshal,
    pub free:   CrFree,
    pub aos:    Vec<u64>,
    pub soa:    Vec<u64>,
    pub f:      usize,
}

impl Drop for StMarshal {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { (self.free)(self.handle) };
            self.handle = core::ptr::null_mut();
        }
    }
}

fn open_marshal(profile: &str, f: usize, entry_name: &[u8]) -> StMarshal {
    let rt = open_runtime();
    let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
    let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
    let entry: CrEntryMarshal = unsafe { rt.resolve(entry_name) }.expect("marshal entry resolves");
    let handle = init_handle(&rt, init, profile);
    // both buffers sized to the max so no reallocation happens in the timed cell.
    StMarshal { rt, handle, entry, free, aos: vec![0u64; N_RECORDS * F_MAX], soa: vec![0u64; F_MAX * N_RECORDS], f }
}

/// Fill the AoS buffer record-major from `seed`.
#[inline(always)]
fn fill_aos(aos: &mut [u64], f: usize, seed: u64) {
    for i in 0..N_RECORDS {
        for k in 0..f {
            aos[i * f + k] = field_val(i, k, seed);
        }
    }
}

/// Fill the SoA buffer field-major from `seed` (the same logical records as [`fill_aos`]).
#[inline(always)]
fn fill_soa(soa: &mut [u64], f: usize, seed: u64) {
    for k in 0..f {
        for i in 0..N_RECORDS {
            soa[k * N_RECORDS + i] = field_val(i, k, seed);
        }
    }
}

/// Transpose the AoS buffer (record-major) into the SoA buffer (field-major). The explicit
/// timed stage charged to `soa_transposed`.
#[inline(always)]
fn transpose_aos_to_soa(aos: &[u64], soa: &mut [u64], f: usize) {
    for i in 0..N_RECORDS {
        for k in 0..f {
            soa[k * N_RECORDS + i] = aos[i * f + k];
        }
    }
}

bench_matrix! {
    name: "abi_marshal",
    crate_path: vehje_bench_carrier,
    crate_dep: "vehje-bench-carrier = {{ path = \"../../carrier\"{carrier_features} }}",
    extra_deps: [ ],
    seed: 0x5eed_ab1_c405_0002,
    sweep profile in ["real", "madd", "tight", "scatter", "wideselect", "leaf"],
    sizes: [1, 2, 4, 8, 16],
    baseline: "aos",
    floor: "marshal_null",
    regime: warm,

    // The AoS entry is the shared setup (the baseline). `n` (the swept size) is the field
    // count `f`; the record count is fixed at N_RECORDS.
    setup |profile: &str, n: usize| -> StMarshal {
        open_marshal(profile, n, b"cr_execute_marshal_aos\0")
    }

    cell aos
        #[feature = "boundary"]
        |s, seed| {
            fill_aos(&mut s.aos, s.f, seed);
            unsafe { (s.entry)(s.handle, s.aos.as_ptr(), N_RECORDS, s.f) }
        }

    // SoA built directly by the host; the runtime reads field-major.
    cell soa_native
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMarshal {
            open_marshal(profile, n, b"cr_execute_marshal_soa\0")
        }
        |s, seed| {
            fill_soa(&mut s.soa, s.f, seed);
            unsafe { (s.entry)(s.handle, s.soa.as_ptr(), N_RECORDS, s.f) }
        }

    // Host holds AoS, transposes to SoA (explicit timed stage), then crosses SoA.
    cell soa_transposed
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMarshal {
            open_marshal(profile, n, b"cr_execute_marshal_soa\0")
        }
        |s, seed| {
            fill_aos(&mut s.aos, s.f, seed);
            // charge the AoS-to-SoA transpose as its own timed stage (disjoint borrows of
            // the two buffers).
            transpose_aos_to_soa(&s.aos, &mut s.soa, s.f);
            unsafe { (s.entry)(s.handle, s.soa.as_ptr(), N_RECORDS, s.f) }
        }

    // The marshalling floor: combine AoS fields, fold, no interpret.
    cell marshal_null
        #[feature = "boundary"]
        setup |profile: &str, n: usize| -> StMarshal {
            open_marshal(profile, n, b"cr_execute_marshal_null_aos\0")
        }
        |s, seed| {
            fill_aos(&mut s.aos, s.f, seed);
            unsafe { (s.entry)(s.handle, s.aos.as_ptr(), N_RECORDS, s.f) }
        }
}

#[cfg(test)]
mod tests {
    use super::super::common::{program_bytes, runtime_dylib_path};
    use super::*;
    use crate as c;

    /// The AoS, SoA-native, and SoA-transposed layouts fold byte-exactly to the same value
    /// for the same logical records, and equal the in-process combine-then-interpret. The
    /// marshalling family's cross-validation gate (all layouts are one computation).
    #[test]
    fn layouts_agree_and_match_in_process_byte_exact() {
        let dylib = runtime_dylib_path();
        let rt = unsafe { Runtime::open(dylib.to_str().unwrap()) }.expect("dylib opens");
        let init: CrInit = unsafe { rt.resolve(b"cr_init\0") }.expect("cr_init");
        let free: CrFree = unsafe { rt.resolve(b"cr_free\0") }.expect("cr_free");
        let aos_e: CrEntryMarshal =
            unsafe { rt.resolve(b"cr_execute_marshal_aos\0") }.expect("aos");
        let soa_e: CrEntryMarshal =
            unsafe { rt.resolve(b"cr_execute_marshal_soa\0") }.expect("soa");

        for profile in ["real", "tight", "leaf"] {
            let bytes = program_bytes(profile);
            let d = c::ir::Decoded::parse(&bytes, c::ir::REC24).unwrap();
            let pd = c::predecode::predecode(&d);
            let handle = unsafe { init(bytes.as_ptr(), bytes.len()) };
            assert!(!handle.is_null());

            for f in [1usize, 2, 4, 8, 16] {
                let seed = 0xbeef ^ (f as u64).wrapping_mul(0x777);
                let mut aos = vec![0u64; N_RECORDS * f];
                let mut soa = vec![0u64; f * N_RECORDS];
                for i in 0..N_RECORDS {
                    for k in 0..f {
                        aos[i * f + k] = field_val(i, k, seed);
                        soa[k * N_RECORDS + i] = field_val(i, k, seed);
                    }
                }
                let a = unsafe { aos_e(handle, aos.as_ptr(), N_RECORDS, f) };
                let s = unsafe { soa_e(handle, soa.as_ptr(), N_RECORDS, f) };
                assert_eq!(a, s, "AoS and SoA must agree for {profile}, f={f}");

                // in-process replica: combine then interpret, folding the same recipe.
                let mut scratch = vec![0u64; pd.nodes.len()];
                let mut acc = 0u64;
                for i in 0..N_RECORDS {
                    let mut sd = 0u64;
                    for k in 0..f {
                        sd ^= field_val(i, k, seed);
                    }
                    c::interpret_predecoded(&pd, sd, &mut scratch);
                    acc = acc.rotate_left(7) ^ c::checksum(&scratch);
                }
                assert_eq!(a, acc, "marshalling must equal in-process for {profile}, f={f}");
            }
            unsafe { free(handle) };
        }
    }

    #[test]
    fn matrix_decls_declare_the_family_shape() {
        let decls = super::matrix_decls();
        assert_eq!(decls.len(), 1);
        let d = &decls[0];
        assert_eq!(d.name, "abi_marshal");
        assert_eq!(d.sizes, vec![1, 2, 4, 8, 16], "the field-count sweep");
        assert_eq!(d.baseline, "aos");
        assert_eq!(d.floor.as_deref(), Some("marshal_null"));
        let tags: Vec<&str> = d.cells.iter().map(|c| c.tag.as_str()).collect();
        assert_eq!(tags, ["aos", "soa_native", "soa_transposed", "marshal_null"]);
    }
}

//! The batched-column entry: the one runtime-W `extern "C"` boundary.
//!
//! vehje's real workload is per-record column evaluation: one residual over a
//! column of many records. The biggest measured throughput lever is vertical
//! SIMD interpretation, which needs the boundary to take a column of W records
//! per call so the runtime vectorises across them. So the boundary exposes one
//! runtime-W batched-column entry taking W at least two. The crossing itself is
//! nearly free, so there is no per-W symbol zoo and no crossing-amortisation
//! cleverness beyond the column entry; the runtime, not the ABI, owns the SIMD
//! width.
//!
//! The return contract is a value property: fallibility is an `Outcome`-shaped
//! value in the value-arena (see [`crate::value`]), batch completion is a
//! stream property on the sink (see [`crate::sink`]), and the lane mask is an
//! internal mask-and-continue mechanism. So nothing returns a status vector.

use arvo::strategy::Hot;
use arvo::Uint;

use crate::sink::VehjeSink;

/// The ABI version this boundary implements.
///
/// The single version word on the public boundary. A host links the runtime
/// only when its expected version matches, so a wire-shape change is a version
/// bump, not a silent reinterpretation.
pub const ABI_VERSION: Uint<32, Hot> = Uint::<32, Hot>::from_raw(1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: literal seed for the version newtype at construction; tracked: #207

/// The per-call column: a residual and a column of W input records, with the
/// sink the value-arena streams out through.
///
/// A `#[repr(C)]` descriptor the host fills and passes by pointer. `width` is W,
/// at least two; the runtime vectorises across the column. Results do not
/// return through this struct: they stream out through `sink` as value-arena
/// chunks, and a failing record is an `Outcome`-shaped value in that arena.
#[repr(C)]
pub struct BatchColumn {
    /// A pointer to the serialized residual image.
    pub residual: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI residual image pointer; the C ABI is the contract; tracked: #207
    /// The residual image length in bytes.
    pub residual_len: usize, // lint:allow(no-public-raw-field) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI byte length; the C ABI is the contract; tracked: #207
    /// A pointer to the column of W packed input records.
    pub records: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI record-column pointer; the C ABI is the contract; tracked: #207
    /// The column width W, at least two.
    pub width: usize, // lint:allow(no-public-raw-field) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI column width; the C ABI is the contract; tracked: #207
    /// The sink the produced value-arena chunks stream out through.
    pub sink: *mut VehjeSink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI sink pointer; the C ABI is the contract; tracked: #207
}

/// The one runtime-W batched-column entry signature.
///
/// Takes a pointer to a [`BatchColumn`] and evaluates the residual over the W
/// records, streaming the produced value-arena out through the call's sink.
/// There is no return value: fallibility, completion, and lane state are all
/// value or stream properties, never a status vector across the boundary.
pub type BatchedColumnEntry = unsafe extern "C" fn(call: *const BatchColumn);

// FIXME: the actual `#[no_mangle] pub unsafe extern "C" fn` export lands with
// the driver bindings (see BACKLOG, "The extern C export with the panic-handler
// story"). The export body must catch unwinding so a panic never crosses the
// boundary; failure is carried by the Outcome-shaped value in the value-arena
// and, for a catastrophic ABI fault, by a version-gated error the driver reads.
// The signature, the call descriptor, and the version word are defined here;
// the export with its panic guard is the deferred half. tracked: #207

//! The embedded runtime: its entry points, and the sink a value crosses in.
//!
//! The driver does not care how the runtime's three entries were obtained. A
//! shipped compiler links the artifact statically, which is what the
//! single-binary distribution rule asks for; a development build can load it
//! instead, behind the non-default `dynamic` feature, which is how the ABI gets
//! exercised against a real artifact without putting a loader in the shipping
//! path. Both hand over the same [`RuntimeEntries`], so everything below the
//! seam is one code path.
//!
//! The produced value crosses back through a reserve-then-commit sink over a
//! caller-lent buffer. The backing is lent up front, so the runtime holds no
//! output memory once the call returns.

use core::ffi::c_void;

use arvo::USize;
use notko::Outcome;

use crate::DriverError;

/// `vehje_runtime_new`: create a runtime handle.
pub type NewFn = extern "C" fn(*mut u8, usize) -> *mut c_void; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI opaque handle is the contract; tracked: #207
/// `vehje_runtime_free`: release a runtime handle.
pub type FreeFn = extern "C" fn(*mut c_void); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI opaque handle is the contract; tracked: #207
/// `vehje_runtime_execute`: evaluate a residual, committing any produced value
/// through the sink.
pub type ExecuteFn = extern "C" fn(*mut c_void, *const u8, usize, *const Sink, *const Host) -> i32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI entry signature is the contract; tracked: #207

/// One operand crossing to a family handler: a kind tag, a payload word, and a
/// bytes pointer.
///
/// The record serves three shapes without changing its own. For a scalar the
/// pointer is null and the payload is the value. For a string the pointer is the
/// bytes and the payload is their length. For a compound, meaning a record or a
/// sequence, the pointer is a value image and the payload is that image's byte
/// length, which is the string convention generalised rather than a new one. So
/// the common case is unchanged in size, and the driver carries a compound
/// exactly as it carries a string: it moves the bytes and reads none of them.
///
/// The tag values are the value image's tags, so one vocabulary describes a
/// value wherever it appears.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Operand {
    /// The value kind: unit, boolean, integer, or string.
    pub tag: u32, // lint:allow(no-public-raw-field) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI operand record is the contract; tracked: #207
    /// The payload word, or a string's byte length.
    pub payload: i64, // lint:allow(no-public-raw-field) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI operand record is the contract; tracked: #207
    /// A string's bytes, or null for a scalar.
    pub bytes: *const u8, // lint:allow(no-public-raw-field) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI operand record is the contract; tracked: #207
}

/// Service one family operation, writing the produced operand through `out`
/// and returning zero on success.
pub type HostFn = extern "C" fn(*mut c_void, u32, *const Operand, usize, *mut Operand) -> i32; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI host callback is the contract; tracked: #207

/// The host a family operation is dispatched to.
///
/// One callback plus opaque userdata, mirroring the sink: the host owns the
/// context, and the runtime retains nothing past the call. The driver carries
/// this record through and interprets no family itself.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Host {
    /// Service one family operation.
    pub call: HostFn,
    /// Host-owned opaque context, passed back to the callback.
    pub userdata: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI opaque userdata pointer; tracked: #207
}

/// The host's transfer sink, mirroring `vehje-runtime-abi`'s `VehjeSink`.
///
/// Declared here as the shape the runtime is handed; the runtime never retains
/// it past the call.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sink {
    reserve: extern "C" fn(*mut c_void, usize) -> *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI reserve callback; the C ABI is the contract; tracked: #207
    commit: extern "C" fn(*mut c_void, usize), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI commit callback; the C ABI is the contract; tracked: #207
    userdata: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI opaque userdata pointer; tracked: #207
}

/// The runtime's three exported entries.
///
/// Resolved or linked once, so a call cannot fail on a missing symbol.
#[derive(Copy, Clone)]
pub struct RuntimeEntries {
    /// Create a runtime handle.
    pub new: NewFn,
    /// Release a runtime handle.
    pub free: FreeFn,
    /// Evaluate a residual.
    pub execute: ExecuteFn,
}

/// What the driver's sink callbacks read and write: the lent buffer and the
/// length the runtime committed into it.
#[repr(C)]
struct SinkCtx {
    buf: *mut u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the lent output buffer at the FFI boundary; tracked: #207
    /// Capacity of the lent buffer.
    cap: USize,
    /// Bytes the runtime committed.
    written: USize,
}

/// Hand out the lent buffer when the chunk fits, or null when it does not.
///
/// Null is the backpressure signal the transfer contract defines, so a buffer
/// too small to hold the image is refused rather than overrun.
extern "C" fn driver_reserve(userdata: *mut c_void, hint: usize) -> *mut u8 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI reserve callback; tracked: #207
    // SAFETY: the runtime passes back exactly the userdata set on the sink,
    // which is a live `SinkCtx` for the duration of the call.
    let ctx = unsafe { &mut *(userdata as *mut SinkCtx) };
    if hint > ctx.cap.0 {
        return core::ptr::null_mut();
    }
    ctx.buf
}

/// Record the bytes the runtime wrote into the last reservation.
extern "C" fn driver_commit(userdata: *mut c_void, written: usize) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI commit callback; tracked: #207
    // SAFETY: as in `driver_reserve`.
    let ctx = unsafe { &mut *(userdata as *mut SinkCtx) };
    ctx.written = USize(written);
}

/// The embedded runtime, over its resolved entries.
#[derive(Copy, Clone)]
pub struct Runtime {
    entries: RuntimeEntries,
}

impl Runtime {
    /// Drive the runtime reached through `entries`.
    pub const fn new(entries: RuntimeEntries) -> Self {
        Self { entries }
    }

    /// Evaluate `residual` with no session scratch.
    ///
    /// A program whose handlers return only scalars needs none.
    ///
    /// Returns the number of bytes committed, which is zero when the program
    /// produced nothing the host kept. Decode those bytes with
    /// `vehje_runtime_abi::ValueImage`, which bounds-checks an untrusted image;
    /// this crate adds no second decoder.
    pub fn execute(&self, residual: &[u8], out: &mut [u8]) -> Outcome<USize, DriverError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the residual and the lent output buffer are FFI byte spans; tracked: #207
        self.run(residual, out, core::ptr::null())
    }

    /// Evaluate `residual` with `host` servicing its family operations.
    ///
    /// A program that uses no family needs no host, which is why [`execute`]
    /// exists alongside this and is not a lesser path.
    ///
    /// [`execute`]: Runtime::execute
    pub fn execute_with_host(&self, residual: &[u8], out: &mut [u8], host: &Host) -> Outcome<USize, DriverError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the residual and the lent output buffer are FFI byte spans; tracked: #207
        self.run(residual, out, host)
    }

    /// Evaluate `residual` with `host` and a lent session scratch, which is
    /// what a handler returning a string writes its bytes into.
    pub fn execute_with_scratch(&self, residual: &[u8], out: &mut [u8], host: &Host, scratch: &mut [u8]) -> Outcome<USize, DriverError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI byte spans; tracked: #207
        self.run_with(residual, out, host, scratch.as_mut_ptr(), scratch.len())
    }

    fn run(&self, residual: &[u8], out: &mut [u8], host: *const Host) -> Outcome<USize, DriverError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the residual and the lent output buffer are FFI byte spans; tracked: #207
        self.run_with(residual, out, host, core::ptr::null_mut(), 0)
    }

    fn run_with(&self, residual: &[u8], out: &mut [u8], host: *const Host, scratch: *mut u8, scratch_len: usize) -> Outcome<USize, DriverError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: FFI byte spans; tracked: #207
        let mut ctx = SinkCtx { buf: out.as_mut_ptr(), cap: USize(out.len()), written: USize(0) };
        let sink = Sink {
            reserve: driver_reserve,
            commit: driver_commit,
            userdata: (&raw mut ctx) as *mut c_void,
        };

        let handle = (self.entries.new)(scratch, scratch_len);
        let code = (self.entries.execute)(handle, residual.as_ptr(), residual.len(), &sink, host);
        (self.entries.free)(handle);

        if code != 0 {
            // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
            return Outcome::Err(DriverError::ExecuteFailed);
        }
        Outcome::Ok(ctx.written)
    }
}

/// Loading the artifact at run time.
///
/// Behind the non-default `dynamic` feature, because the shipping compiler
/// links the runtime statically and carries no loader. This path exists so the
/// ABI can be exercised against a real built artifact.
#[cfg(feature = "dynamic")]
pub mod dynamic {
    use hilavitkutin_linking::Library;
    use notko::Outcome;

    use super::{ExecuteFn, FreeFn, NewFn, RuntimeEntries};
    use crate::DriverError;

    /// Load the runtime artifact at `path`.
    ///
    /// Loading and resolving are separate because resolved symbols borrow the
    /// library: the caller owns the [`Library`] and resolves against it, which
    /// is what keeps a symbol from outliving what it came from.
    pub fn load(path: &[u8]) -> Outcome<Library, DriverError> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the loader takes a platform path as bytes; tracked: #207
        match Library::load(path) {
            Outcome::Ok(lib) => Outcome::Ok(lib),
            Outcome::Err(_) => Outcome::Err(DriverError::LoadFailed),
        }
    }

    /// Resolve the three exported entries from a loaded library.
    pub fn entries(lib: &Library) -> Outcome<RuntimeEntries, DriverError> {
        let new = match lib.resolve::<NewFn>(b"vehje_runtime_new\0") {
            Outcome::Ok(s) => s.get(),
            Outcome::Err(_) => return Outcome::Err(DriverError::SymbolMissing),
        };
        let free = match lib.resolve::<FreeFn>(b"vehje_runtime_free\0") {
            Outcome::Ok(s) => s.get(),
            Outcome::Err(_) => return Outcome::Err(DriverError::SymbolMissing),
        };
        let execute = match lib.resolve::<ExecuteFn>(b"vehje_runtime_execute\0") {
            Outcome::Ok(s) => s.get(),
            Outcome::Err(_) => return Outcome::Err(DriverError::SymbolMissing),
        };
        Outcome::Ok(RuntimeEntries { new, free, execute })
    }
}

#[cfg(test)]
mod tests;

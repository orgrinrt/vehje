//! Tests for the embedded runtime and its transfer sink.

use super::*;

/// A stand-in runtime that writes a fixed payload through the sink, so the
/// transfer contract is testable without any built artifact.
mod fake {
    use super::*;

    /// The bytes the stand-in commits.
    pub const PAYLOAD: &[u8] = &[1, 2, 3, 4]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test payload bytes at the FFI boundary; tracked: #207

    pub extern "C" fn new(_scratch: *mut u8, _len: usize) -> *mut c_void { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI handle; tracked: #207
        core::ptr::null_mut()
    }

    pub extern "C" fn free(_rt: *mut c_void) {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI handle; tracked: #207

    /// Reserve, write, commit: the whole-value single-reserve case.
    pub extern "C" fn execute( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI entry; tracked: #207
        _rt: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _input: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        sink: *const Sink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _host: *const Host, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        // SAFETY: the driver passes a live sink for the duration of the call.
        let s = unsafe { &*sink };
        let dst = (s.reserve)(s.userdata, PAYLOAD.len());
        if dst.is_null() {
            return -1; // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
        }
        // SAFETY: reserve returned at least PAYLOAD.len() writable bytes.
        unsafe { core::ptr::copy_nonoverlapping(PAYLOAD.as_ptr(), dst, PAYLOAD.len()) };
        (s.commit)(s.userdata, PAYLOAD.len());
        0 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }

    /// A runtime that always reports failure, without touching the sink.
    pub extern "C" fn execute_fails( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI entry; tracked: #207
        _rt: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _input: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _sink: *const Sink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _host: *const Host, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        -1 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }

    pub fn entries() -> RuntimeEntries {
        RuntimeEntries { new, free, execute }
    }

    pub fn failing_entries() -> RuntimeEntries {
        RuntimeEntries { new, free, execute: execute_fails }
    }
}

#[test]
fn a_committed_value_lands_in_the_lent_buffer() {
    let rt = Runtime::new(fake::entries());
    let mut out = [0u8; 16]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test output buffer bytes; tracked: #207
    let written = match rt.execute(&[], &mut out) {
        Outcome::Ok(n) => n,
        Outcome::Err(_) => panic!("the stand-in commits successfully"),
    };
    assert_eq!(written, USize(fake::PAYLOAD.len()));
    assert_eq!(&out[..written.0], fake::PAYLOAD);
}

#[test]
fn a_buffer_too_small_is_refused_rather_than_overrun() {
    // The sink returns null when the chunk does not fit, which the runtime
    // reads as backpressure. The stand-in reports failure rather than writing
    // past the end, and the lent buffer is untouched.
    let rt = Runtime::new(fake::entries());
    let mut out = [0u8; 2]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: deliberately undersized test buffer; tracked: #207
    assert!(matches!(rt.execute(&[], &mut out), Outcome::Err(DriverError::ExecuteFailed)));
    assert_eq!(out, [0, 0]); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the buffer must be untouched; tracked: #207
}

#[test]
fn a_runtime_failure_is_named_not_swallowed() {
    let rt = Runtime::new(fake::failing_entries());
    let mut out = [0u8; 16]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test output buffer bytes; tracked: #207
    assert!(matches!(rt.execute(&[], &mut out), Outcome::Err(DriverError::ExecuteFailed)));
}

#[test]
fn nothing_committed_reads_as_zero_bytes() {
    // A program the host kept nothing from: the call succeeds and commits
    // nothing, which is a length of zero rather than an error.
    extern "C" fn quiet( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: C ABI entry; tracked: #207
        _rt: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _input: *const u8, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _len: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _sink: *const Sink, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _host: *const Host, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        0 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }
    let rt = Runtime::new(RuntimeEntries { new: fake::new, free: fake::free, execute: quiet });
    let mut out = [0u8; 16]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: test output buffer bytes; tracked: #207
    assert!(matches!(rt.execute(&[], &mut out), Outcome::Ok(n) if n == USize(0)));
}

#[cfg(feature = "dynamic")]
mod dynamic_tests {
    use super::*;

    #[test]
    fn loading_something_that_is_not_a_library_is_named() {
        let bad = b"/nonexistent/not-a-library.dylib\0"; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: platform path bytes at the loader boundary; tracked: #207
        assert!(matches!(super::dynamic::load(bad), Outcome::Err(DriverError::LoadFailed)));
    }
}

#[cfg(feature = "dynamic")]
mod end_to_end {
    use super::*;
    use vehje_runtime_abi::{ValueImage, ValueTag};

    // The tier-0 residual layout, mirrored here so the test states a program
    // rather than depending on the whole compile pipeline to build one.
    pub const WORD: usize = 4; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire word size; tracked: #207
    pub const HEADER_WORDS: usize = 7; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire header width; tracked: #207
    pub const NODE_WORDS: usize = 7; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire node width; tracked: #207

    pub fn put(buf: &mut [u8], at: usize, w: u32) { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire byte encoding; tracked: #207
        buf[at] = (w & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
        buf[at + 1] = ((w >> 8) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
        buf[at + 2] = ((w >> 16) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
        buf[at + 3] = ((w >> 24) & 0xff) as u8; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: LE byte extraction; tracked: #207
    }

    pub fn node_at(i: usize) -> usize { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire offset arithmetic; tracked: #207
        HEADER_WORDS * WORD + i * NODE_WORDS * WORD
    }

    /// `let x = 42 in x`, as a tier-0 residual image.
    fn let_42() -> [u8; HEADER_WORDS * 4 + 3 * NODE_WORDS * 4] { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire image bytes; tracked: #207
        let mut b = [0u8; HEADER_WORDS * 4 + 3 * NODE_WORDS * 4]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire image bytes; tracked: #207
        let x: u32 = 0x0000_0007; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a binder's Sym bits on the wire; tracked: #207
        put(&mut b, 0, 0x3048_4556); // magic VEH0 // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire magic; tracked: #207
        put(&mut b, WORD, 1); // version // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire version; tracked: #207
        put(&mut b, 2 * WORD, 0); // tier = Arena // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire tier; tracked: #207
        put(&mut b, 3 * WORD, 3); // node_count // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count; tracked: #207
        put(&mut b, 6 * WORD, 2); // root = the Let // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire root index; tracked: #207

        let n0 = node_at(0); // Lit Int 42
        put(&mut b, n0, 0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Lit tag; tracked: #207
        put(&mut b, n0 + WORD, 2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Int literal sub-tag; tracked: #207
        put(&mut b, n0 + 2 * WORD, 42); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the literal's low word; tracked: #207

        let n1 = node_at(1); // Var x
        put(&mut b, n1, 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Var tag; tracked: #207
        put(&mut b, n1 + WORD, x);

        let n2 = node_at(2); // Let(x, value = 0, body = 1)
        put(&mut b, n2, 2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Let tag; tracked: #207
        put(&mut b, n2 + 2 * WORD, x);
        put(&mut b, n2 + 3 * WORD, 0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value child index; tracked: #207
        put(&mut b, n2 + 4 * WORD, 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: body child index; tracked: #207
        b
    }

    /// The whole crossing against the real artifact: load it, run a program,
    /// and read the answer back.
    ///
    /// Ignored by default because it needs `zig build` to have produced the
    /// library. Run it with:
    /// `cargo test -p vehje-runtime-driver --features dynamic -- --ignored`
    #[test]
    #[ignore = "requires zig build in mock/runtime-zig"]
    fn a_program_runs_and_its_value_comes_back() {
        let path = b"../../runtime-zig/zig-out/lib/libvehje_runtime.dylib\0"; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: platform path bytes at the loader boundary; tracked: #207
        let lib = match super::dynamic::load(path) {
            Outcome::Ok(l) => l,
            Outcome::Err(_) => panic!("build the runtime first: cd mock/runtime-zig && zig build"),
        };
        let entries = match super::dynamic::entries(&lib) {
            Outcome::Ok(e) => e,
            Outcome::Err(_) => panic!("the artifact should export the three entries"),
        };

        let program = let_42();
        let mut out = [0u8; 128]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: lent output buffer bytes; tracked: #207
        let written = match Runtime::new(entries).execute(&program, &mut out) {
            Outcome::Ok(n) => n,
            Outcome::Err(_) => panic!("the program should evaluate"),
        };

        let img = match ValueImage::parse(&out[..written.0]) {
            notko::Maybe::Is(i) => i,
            notko::Maybe::Isnt => panic!("the committed bytes should parse as a value image"),
        };
        assert!(img.validate().0);
        let root = match img.try_node(img.root()) {
            notko::Maybe::Is(n) => n,
            notko::Maybe::Isnt => panic!("the root should read"),
        };
        assert_eq!(root.tag, ValueTag::Int);
        assert_eq!(img.as_int(root), notko::Maybe::Is(arvo::Int::<64, arvo::strategy::Hot>::from_raw(42)));
    }
}

#[cfg(feature = "dynamic")]
mod end_to_end_host {
    use super::end_to_end::{node_at, put, HEADER_WORDS, NODE_WORDS, WORD};
    use super::*;
    use vehje_runtime_abi::{ValueImage, ValueTag};

    /// The family this test's host implements. The framework knows nothing
    /// about it, which is the property under test.
    const ADD: u32 = 7; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a family id on the wire; tracked: #207

    /// A Rust host that adds the integer operands it is handed.
    extern "C" fn add_host( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI host callback is the contract; tracked: #207
        _userdata: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        family: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        args: *const Operand, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        argc: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        out: *mut Operand, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        if family != ADD {
            return -1; // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
        }
        // SAFETY: the runtime passes `argc` initialised operands and one
        // writable out-slot, both valid for the duration of the call.
        let operands = unsafe { core::slice::from_raw_parts(args, argc) };
        let mut sum: i64 = 0; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI operand payload width; tracked: #207
        for a in operands {
            sum += a.payload;
        }
        // SAFETY: as above.
        unsafe { *out = Operand { tag: 2, payload: sum, bytes: core::ptr::null() } }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the Int scalar tag on the wire; tracked: #207
        0 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }

    /// `add(2, 3)` as a tier-0 residual: two literals and one family node whose
    /// operand list lives in the child pool.
    fn add_2_3() -> [u8; HEADER_WORDS * 4 + 3 * NODE_WORDS * 4 + 2 * 4] { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire image bytes; tracked: #207
        let mut b = [0u8; HEADER_WORDS * 4 + 3 * NODE_WORDS * 4 + 2 * 4]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire image bytes; tracked: #207
        put(&mut b, 0, 0x3048_4556); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire magic; tracked: #207
        put(&mut b, WORD, 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire version; tracked: #207
        put(&mut b, 3 * WORD, 3); // node_count // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count; tracked: #207
        put(&mut b, 4 * WORD, 2); // pool_count // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire count; tracked: #207
        put(&mut b, 6 * WORD, 2); // root = the family node // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire root index; tracked: #207

        let n0 = node_at(0); // Lit Int 2
        put(&mut b, n0, 0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Lit tag; tracked: #207
        put(&mut b, n0 + WORD, 2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Int sub-tag; tracked: #207
        put(&mut b, n0 + 2 * WORD, 2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the literal's low word; tracked: #207

        let n1 = node_at(1); // Lit Int 3
        put(&mut b, n1, 0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Lit tag; tracked: #207
        put(&mut b, n1 + WORD, 2); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Int sub-tag; tracked: #207
        put(&mut b, n1 + 2 * WORD, 3); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the literal's low word; tracked: #207

        let n2 = node_at(2); // Raw(ADD, payload = pool[0..2])
        put(&mut b, n2, 10); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: Raw tag; tracked: #207
        put(&mut b, n2 + WORD, ADD);
        put(&mut b, n2 + 2 * WORD, 0); // payload start // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pool start; tracked: #207
        put(&mut b, n2 + 3 * WORD, 2); // payload len // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: pool length; tracked: #207

        let pool = HEADER_WORDS * WORD + 3 * NODE_WORDS * WORD;
        put(&mut b, pool, 0); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: first operand node index; tracked: #207
        put(&mut b, pool + WORD, 1); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: second operand node index; tracked: #207
        b
    }

    /// The full loop with real computation: a Rust host services a family
    /// operation inside a program the Zig runtime evaluates, and the answer
    /// comes back as a value.
    #[test]
    #[ignore = "requires zig build in mock/runtime-zig"]
    fn a_rust_host_services_a_family_operation() {
        let path = b"../../runtime-zig/zig-out/lib/libvehje_runtime.dylib\0"; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: platform path bytes at the loader boundary; tracked: #207
        let lib = match super::dynamic::load(path) {
            Outcome::Ok(l) => l,
            Outcome::Err(_) => panic!("build the runtime first: cd mock/runtime-zig && zig build"),
        };
        let entries = match super::dynamic::entries(&lib) {
            Outcome::Ok(e) => e,
            Outcome::Err(_) => panic!("the artifact should export the three entries"),
        };

        let host = Host { call: add_host, userdata: core::ptr::null_mut() };
        let program = add_2_3();
        let mut out = [0u8; 128]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: lent output buffer bytes; tracked: #207
        let written = match Runtime::new(entries).execute_with_host(&program, &mut out, &host) {
            Outcome::Ok(n) => n,
            Outcome::Err(_) => panic!("the program should evaluate through the host"),
        };

        let img = match ValueImage::parse(&out[..written.0]) {
            notko::Maybe::Is(i) => i,
            notko::Maybe::Isnt => panic!("the committed bytes should parse"),
        };
        let root = match img.try_node(img.root()) {
            notko::Maybe::Is(n) => n,
            notko::Maybe::Isnt => panic!("the root should read"),
        };
        assert_eq!(root.tag, ValueTag::Int);
        assert_eq!(img.as_int(root), notko::Maybe::Is(arvo::Int::<64, arvo::strategy::Hot>::from_raw(5)));
    }
}

#[cfg(feature = "dynamic")]
mod end_to_end_compound {
    use super::end_to_end::{node_at, put, HEADER_WORDS, NODE_WORDS, WORD};
    use super::*;
    use vehje_runtime_abi::{ValueImage, ValueTag};

    /// The family whose handler produces a record. As with `ADD`, the framework
    /// knows nothing about it.
    const MAKE: u32 = 9; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a family id on the wire; tracked: #207

    const VALUE_MAGIC: u32 = 0x3056_4556; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image magic; tracked: #207
    const VNODE_WORDS: usize = 6; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image node record width; tracked: #207
    const IMG_LEN: usize = HEADER_WORDS * 4 + 5 * VNODE_WORDS * 4 + 4 * 4 + 19; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image byte length; tracked: #207

    /// `{ name: "ok", count: 7 }` as a value image.
    ///
    /// Five nodes emitted children before parents, so the record at index four
    /// names indices that already exist and the reader's one monotone check
    /// proves acyclicity. The record's children alternate the field-name string
    /// with the field's value, which is how a record names its fields when the
    /// node record has no key slot.
    fn record_image() -> [u8; IMG_LEN] { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image bytes; tracked: #207
        let mut b = [0u8; IMG_LEN]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image bytes; tracked: #207
        put(&mut b, 0, VALUE_MAGIC);
        put(&mut b, WORD, 1); // version // lint:allow(no-bare-numeric) reason: wire version; tracked: #207
        put(&mut b, 2 * WORD, 5); // node_count // lint:allow(no-bare-numeric) reason: wire count; tracked: #207
        put(&mut b, 3 * WORD, 4); // pool_count // lint:allow(no-bare-numeric) reason: wire count; tracked: #207
        put(&mut b, 4 * WORD, 0); // region_count // lint:allow(no-bare-numeric) reason: wire count; tracked: #207
        put(&mut b, 5 * WORD, 19); // blob_len // lint:allow(no-bare-numeric) reason: wire count; tracked: #207
        put(&mut b, 6 * WORD, 4); // root // lint:allow(no-bare-numeric) reason: wire root; tracked: #207

        // (tag, region, children.start, children.len, blob.offset, blob.len)
        let nodes: [[u32; 6]; 5] = [ // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image node records; tracked: #207
            [3, 0, 0, 0, 0, 4],   // "name"
            [3, 0, 0, 0, 4, 2],   // "ok"
            [3, 0, 0, 0, 6, 5],   // "count"
            [2, 0, 0, 0, 11, 8],  // 7
            [5, 0, 0, 4, 0, 0],   // the record
        ];
        let base = HEADER_WORDS * WORD;
        for (i, n) in nodes.iter().enumerate() {
            for (w, v) in n.iter().enumerate() {
                put(&mut b, base + (i * VNODE_WORDS + w) * WORD, *v);
            }
        }
        let pool = base + 5 * VNODE_WORDS * WORD;
        for k in 0..4usize {
            put(&mut b, pool + k * WORD, k as u32); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a value-image pool word is a wire-width index; tracked: #207
        }
        let blob = pool + 4 * WORD;
        b[blob..blob + 11].copy_from_slice(b"nameokcount");
        b[blob + 11] = 7; // the integer's eight little-endian bytes // lint:allow(no-bare-numeric) reason: payload byte; tracked: #207
        b
    }

    /// `make()` as a tier-0 residual: one literal operand and one family node.
    fn make_record_call() -> [u8; HEADER_WORDS * 4 + 2 * NODE_WORDS * 4 + 4] { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire image bytes; tracked: #207
        let mut b = [0u8; HEADER_WORDS * 4 + 2 * NODE_WORDS * 4 + 4]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire image bytes; tracked: #207
        put(&mut b, 0, 0x3048_4556); // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: wire magic; tracked: #207
        put(&mut b, WORD, 1); // lint:allow(no-bare-numeric) reason: wire version; tracked: #207
        put(&mut b, 3 * WORD, 2); // node_count // lint:allow(no-bare-numeric) reason: wire count; tracked: #207
        put(&mut b, 4 * WORD, 1); // pool_count // lint:allow(no-bare-numeric) reason: wire count; tracked: #207
        put(&mut b, 6 * WORD, 1); // root = the family node // lint:allow(no-bare-numeric) reason: wire root index; tracked: #207

        let n0 = node_at(0); // Lit Int 0, an operand the handler ignores
        put(&mut b, n0, 0); // lint:allow(no-bare-numeric) reason: Lit tag; tracked: #207
        put(&mut b, n0 + WORD, 2); // lint:allow(no-bare-numeric) reason: Int sub-tag; tracked: #207
        put(&mut b, n0 + 2 * WORD, 0); // lint:allow(no-bare-numeric) reason: the literal's low word; tracked: #207

        let n1 = node_at(1); // Raw(MAKE, payload = pool[0..1])
        put(&mut b, n1, 10); // lint:allow(no-bare-numeric) reason: Raw tag; tracked: #207
        put(&mut b, n1 + WORD, MAKE);
        put(&mut b, n1 + 2 * WORD, 0); // lint:allow(no-bare-numeric) reason: pool start; tracked: #207
        put(&mut b, n1 + 3 * WORD, 1); // lint:allow(no-bare-numeric) reason: pool length; tracked: #207

        let pool = HEADER_WORDS * WORD + 2 * NODE_WORDS * WORD;
        put(&mut b, pool, 0); // lint:allow(no-bare-numeric) reason: the operand's node index; tracked: #207
        b
    }

    /// A Rust host that answers with a record rather than a scalar.
    extern "C" fn record_host( // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI host callback is the contract; tracked: #207
        _userdata: *mut c_void, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        family: u32, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _args: *const Operand, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        _argc: usize, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        out: *mut Operand, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
    ) -> i32 { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI signature is the contract; tracked: #207
        if family != MAKE {
            return -1; // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
        }
        // The image must outlive the call, so it is a static the handler fills.
        // A real host would write into the scratch the runtime lends it.
        static mut IMG: [u8; IMG_LEN] = [0; IMG_LEN]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: value-image bytes; tracked: #207
        // SAFETY: single-threaded test, one call, one writer.
        let p = unsafe {
            IMG = record_image();
            (&raw const IMG).cast::<u8>() // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the C ABI operand bytes pointer; tracked: #207
        };
        // SAFETY: the runtime passes one writable out-slot valid for the call.
        unsafe { *out = Operand { tag: 5, payload: IMG_LEN as i64, bytes: p } }; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: the Record tag on the wire; tracked: #207
        0 // lint:allow(no-bare-numeric) reason: the C ABI result code is the contract; tracked: #207
    }

    #[test]
    #[ignore = "needs the built artifact: cd mock/runtime-zig && zig build"]
    fn a_host_returns_a_record_and_it_comes_back_as_the_value() {
        let path = b"../../runtime-zig/zig-out/lib/libvehje_runtime.dylib\0"; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: platform path bytes at the loader boundary; tracked: #207
        let lib = match super::dynamic::load(path) {
            Outcome::Ok(l) => l,
            Outcome::Err(_) => panic!("build the runtime first: cd mock/runtime-zig && zig build"),
        };
        let entries = match super::dynamic::entries(&lib) {
            Outcome::Ok(e) => e,
            Outcome::Err(_) => panic!("the artifact should export the three entries"),
        };

        let host = Host { call: record_host, userdata: core::ptr::null_mut() };
        let program = make_record_call();
        let mut out = [0u8; 1024]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: lent output buffer bytes; tracked: #207
        let mut scratch = [0u8; 2048]; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: lent session scratch bytes; tracked: #207
        let written = match Runtime::new(entries).execute_with_scratch(&program, &mut out, &host, &mut scratch) {
            Outcome::Ok(n) => n,
            Outcome::Err(_) => panic!("the program should evaluate through the host"),
        };

        let img = match ValueImage::parse(&out[..written.0]) {
            notko::Maybe::Is(i) => i,
            notko::Maybe::Isnt => panic!("the committed bytes should parse"),
        };
        assert!(img.validate().0, "an untrusted image must pass the structural decode");
        let root = match img.try_node(img.root()) {
            notko::Maybe::Is(n) => n,
            notko::Maybe::Isnt => panic!("the root should read"),
        };
        assert_eq!(root.tag, ValueTag::Record);
        assert_eq!(root.children.len, arvo::USize(4));

        // fields alternate key then value, so field one is at indices two and three
        let key = match img.child(root.children, arvo::USize(2)) {
            notko::Maybe::Is(r) => r,
            notko::Maybe::Isnt => panic!("the second field's key should read"),
        };
        let keyn = match img.try_node(key) {
            notko::Maybe::Is(n) => n,
            notko::Maybe::Isnt => panic!("the key node should read"),
        };
        assert_eq!(img.blob(keyn.blob), notko::Maybe::Is(&b"count"[..]));
    }
}

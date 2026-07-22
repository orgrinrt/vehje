//! Fairness-audit disassembly probe. Each `di_*` symbol wraps one interpreter so
//! its lowered code can be disassembled and its dispatch label confirmed at the
//! ISA level. The wrappers are `#[no_mangle] extern "C"` (never inlined), so the
//! interpreter body inlines into the named symbol and objdump can isolate it.
//! Parse/predecode is done inside so the wrapper is self-contained; only the
//! dispatch loop is the code of interest.

use vehje_bench_carrier::access::checksum;
use vehje_bench_carrier::ir::{Decoded, REC24};
use vehje_bench_carrier::predecode::{
    interpret_predecoded, interpret_predecoded_fntable, interpret_predecoded_regcache, predecode,
};
use vehje_bench_carrier::{interpret, interpret_bittree, interpret_fntable, interpret_ifchain};

/// # Safety: caller passes a valid wire-program byte range and an 8-byte output.
unsafe fn parse<'a>(bytes: *const u8, len: usize) -> Decoded<'a> {
    let b = std::slice::from_raw_parts(bytes, len);
    Decoded::parse(b, REC24).expect("probe program parses")
}

macro_rules! wire_probe {
    ($name:ident, $fn:path) => {
        #[no_mangle]
        pub extern "C" fn $name(bytes: *const u8, len: usize, seed: u64, out: *mut u64) {
            let d = unsafe { parse(bytes, len) };
            let mut r = vec![0u64; d.node_count];
            $fn(&d, seed, &mut r);
            unsafe { *out = checksum(&r) };
        }
    };
}

wire_probe!(di_switch, interpret);
wire_probe!(di_fntable, interpret_fntable);
wire_probe!(di_ifchain, interpret_ifchain);
wire_probe!(di_bittree, interpret_bittree);

macro_rules! flat_probe {
    ($name:ident, $fn:path) => {
        #[no_mangle]
        pub extern "C" fn $name(bytes: *const u8, len: usize, seed: u64, out: *mut u64) {
            let d = unsafe { parse(bytes, len) };
            let p = predecode(&d);
            let mut r = vec![0u64; d.node_count];
            $fn(&p, seed, &mut r);
            unsafe { *out = checksum(&r) };
        }
    };
}

flat_probe!(di_pre_switch, interpret_predecoded);
flat_probe!(di_pre_fntable, interpret_predecoded_fntable);
flat_probe!(di_pre_regcache, interpret_predecoded_regcache);

#[no_mangle]
pub extern "C" fn di_threaded(bytes: *const u8, len: usize, seed: u64, out: *mut u64) {
    let d = unsafe { parse(bytes, len) };
    let mut r = vec![0u64; d.node_count];
    vehje_bench_carrier::interpret_threaded(&d, seed, &mut r);
    unsafe { *out = checksum(&r) };
}

#[no_mangle]
pub extern "C" fn di_pre_threaded(bytes: *const u8, len: usize, seed: u64, out: *mut u64) {
    let d = unsafe { parse(bytes, len) };
    let p = predecode(&d);
    let mut r = vec![0u64; d.node_count];
    vehje_bench_carrier::predecode::interpret_predecoded_threaded(&p, seed, &mut r);
    unsafe { *out = checksum(&r) };
}

#[no_mangle]
pub extern "C" fn di_pre_direct(bytes: *const u8, len: usize, seed: u64, out: *mut u64) {
    use vehje_bench_carrier::predecode::threaded_direct::{interpret_predecoded_direct, resolve_handlers, H};
    let d = unsafe { parse(bytes, len) };
    let p = predecode(&d);
    let mut h: Vec<H> = Vec::new();
    resolve_handlers(&p, &mut h);
    let mut r = vec![0u64; d.node_count];
    interpret_predecoded_direct(&p, &h, seed, &mut r);
    unsafe { *out = checksum(&r) };
}

// CFG dispatch cells (control-flow register VM). The three shapes share the
// register-file operand access (`access::rload`/`rstore`), so only the dispatch
// differs; the audit confirms switch=jump-table, fntable=indirect-call, and the
// threaded handlers=spills-0 + tail br, exactly as for the straight-line cells.
#[no_mangle]
pub extern "C" fn di_cfg_switch(seed: u64, outer: u64, inner: u64, out: *mut u64) {
    use vehje_bench_carrier::cfg::{build_nested_loop, interp};
    let blocks = build_nested_loop(outer, inner);
    let (r, _, _) = interp(&blocks, seed, u64::MAX);
    unsafe { *out = r };
}

#[no_mangle]
pub extern "C" fn di_cfg_fntable(seed: u64, outer: u64, inner: u64, out: *mut u64) {
    use vehje_bench_carrier::cfg::{build_nested_loop, interp_fntable};
    let blocks = build_nested_loop(outer, inner);
    let (r, _, _) = interp_fntable(&blocks, seed, u64::MAX);
    unsafe { *out = r };
}

#[no_mangle]
pub extern "C" fn di_cfg_threaded(seed: u64, outer: u64, inner: u64, out: *mut u64) {
    use vehje_bench_carrier::cfg::build_nested_loop; use vehje_bench_carrier::cfg_threaded as threaded;
    let blocks = build_nested_loop(outer, inner);
    let code = threaded::flatten(&blocks);
    let (r, _, _) = threaded::interp_flat(&code, seed, u64::MAX);
    unsafe { *out = r };
}

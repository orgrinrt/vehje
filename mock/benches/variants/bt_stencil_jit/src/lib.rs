//! Branch tier: stencil_jit. Copy-and-patch: emit a native aarch64 loop for the
//! branch kernel into MAP_JIT memory once, then run it. Models the native tier
//! (no interpreter dispatch). The emit is one-time (OnceLock; warmup absorbs it);
//! only execution is timed. Kernel validated byte-exact vs the reference.
use std::ffi::c_void;
use std::sync::OnceLock;
use mockspace_bench_core::{timed, FfiBenchCall};
use mockspace_bench_macro::bench_variant;

extern "C" {
    fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut c_void;
    fn pthread_jit_write_protect_np(enabled: i32);
    fn sys_icache_invalidate(start: *mut c_void, len: usize);
}
const PROT_RWX: i32 = 1 | 2 | 4;
const MAP_FLAGS: i32 = 2 | 0x1000 | 0x800; // PRIVATE | ANON | JIT

fn emit(code: &mut Vec<u32>) {
    code.push(0xD2800002); // mov x2,#0 (acc)
    code.push(0xD2800003); // mov x3,#0 (i)
    let loop_idx = code.len();
    code.push(0xEB01007F); // cmp x3,x1
    let bge_idx = code.len(); code.push(0x5400000A); // b.ge end
    code.push(0x38636804); // ldrb w4,[x0,x3]
    code.push(0x12000085); // and w5,w4,#1
    let cbnz_idx = code.len(); code.push(0x35000005); // cbnz w5, else
    code.push(0x8B040042); // add x2,x2,x4
    code.push(0x8B020442); // add x2,x2,x2,lsl#1  (acc*=3)
    let bcont_idx = code.len(); code.push(0x14000000); // b cont
    let else_idx = code.len();
    code.push(0xD37FF886); // lsl x6,x4,#1
    code.push(0xCA060042); // eor x2,x2,x6
    let cont_idx = code.len();
    code.push(0x91000463); // add x3,x3,#1
    let bloop_idx = code.len();
    let off_loop = (loop_idx as i64) - (bloop_idx as i64);
    code.push(0x14000000 | ((off_loop as u32) & 0x03FFFFFF));
    let end_idx = code.len();
    code.push(0xAA0203E0); // mov x0,x2
    code.push(0xD65F03C0); // ret
    let off = (end_idx as i64) - (bge_idx as i64);
    code[bge_idx] = 0x54000000 | (((off as u32) & 0x7FFFF) << 5) | 0xA;
    let off = (else_idx as i64) - (cbnz_idx as i64);
    code[cbnz_idx] = 0x35000000 | (((off as u32) & 0x7FFFF) << 5) | 5;
    let off = (cont_idx as i64) - (bcont_idx as i64);
    code[bcont_idx] = 0x14000000 | ((off as u32) & 0x03FFFFFF);
}

static JIT_ADDR: OnceLock<usize> = OnceLock::new();
fn jit_fn() -> extern "C" fn(*const u8, u64) -> u64 {
    let addr = *JIT_ADDR.get_or_init(|| unsafe {
        let mut code = Vec::new(); emit(&mut code);
        let bytes = code.len() * 4;
        let mem = mmap(std::ptr::null_mut(), 4096, PROT_RWX, MAP_FLAGS, -1, 0);
        assert!(mem as isize != -1);
        pthread_jit_write_protect_np(0);
        std::ptr::copy_nonoverlapping(code.as_ptr() as *const u8, mem as *mut u8, bytes);
        pthread_jit_write_protect_np(1);
        sys_icache_invalidate(mem, bytes);
        mem as usize
    });
    unsafe { std::mem::transmute(addr) }
}

#[bench_variant("bt_stencil_jit", sizes = [64, 256, 1024, 4096, 16384])]
fn run<const N: usize>(input: &[u8; N], output: &mut [u8; 8]) -> FfiBenchCall {
    let f = jit_fn();
    timed! { run {
        let acc = f(input.as_ptr(), N as u64);
        output.copy_from_slice(&acc.to_le_bytes());
    } }
}

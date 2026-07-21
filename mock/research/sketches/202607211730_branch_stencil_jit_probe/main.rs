// De-risk: JIT an aarch64 loop for the branch kernel and validate byte-exact vs a Rust reference.
// kernel per byte b: cond=(b&1)==0; taken: acc=(acc+b)*3 ; else: acc = acc ^ (b<<1). acc:u64 sequential.
use std::ffi::c_void;

fn reference(input: &[u8]) -> u64 {
    let mut acc: u64 = 0;
    for &b in input {
        if (b & 1) == 0 { acc = acc.wrapping_add(b as u64).wrapping_mul(3); }
        else { acc ^= (b as u64) << 1; }
    }
    acc
}

extern "C" {
    fn mmap(addr: *mut c_void, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut c_void;
    fn pthread_jit_write_protect_np(enabled: i32);
    fn sys_icache_invalidate(start: *mut c_void, len: usize);
}
const PROT_READ: i32 = 1; const PROT_WRITE: i32 = 2; const PROT_EXEC: i32 = 4;
const MAP_PRIVATE: i32 = 2; const MAP_ANON: i32 = 0x1000; const MAP_JIT: i32 = 0x800;

// aarch64 encodings (little-endian u32 each). Registers: x0=input ptr, x1=len, x2=acc, x3=i, w4=b, w5=tmp, x6=tmp
fn emit(code: &mut Vec<u32>) {
    // mov x2, #0
    code.push(0xD2800002);
    // mov x3, #0
    code.push(0xD2800003);
    // loop: (index = code.len())
    let loop_idx = code.len();
    // cmp x3, x1  => subs xzr, x3, x1 : 0xEB01007F
    code.push(0xEB01007F);
    // b.ge end  (placeholder, patch offset later); cond GE = 0b1010 =0xA; B.cond = 0x54000000 | (imm19<<5) | cond
    let bge_idx = code.len(); code.push(0x5400000A); // offset patched later
    // ldrb w4, [x0, x3]  => 0x38636804
    code.push(0x38636804);
    // and w5, w4, #1  => AND (immediate) 32-bit, imm=1 (N=0,immr=0,imms=0): 0x12000085
    code.push(0x12000085);
    // cbnz w5, else (placeholder) ; CBNZ w5 = 0x35000000 | (imm19<<5) | 5
    let cbnz_idx = code.len(); code.push(0x35000005);
    // TAKEN: acc=(acc+b)*3 : add x2,x2,x4 ; then x2 = x2 + x2<<1
    // add x2, x2, x4 (x4 is w4 zero-extended? use add x2,x2,x4): ADD (shifted reg) 64: 0x8B040042
    code.push(0x8B040042);
    // add x2, x2, x2, lsl #1  => ADD x2,x2,x2,LSL#1 : 0x8B420042  (shift=LSL(00), imm6=1 -> imm6<<10)
    code.push(0x8B020442);
    // b cont (placeholder) : B = 0x14000000 | imm26
    let bcont_idx = code.len(); code.push(0x14000000);
    // ELSE: (index) acc ^= (b<<1) : lsl x6, x4, #1 ; eor x2, x2, x6
    let else_idx = code.len();
    // lsl x6, x4, #1 => UBFM / LSL immediate: LSL x6,x4,#1 = 0xD37FF886  (immr=63,imms=62 for <<1)
    code.push(0xD37FF886);
    // eor x2, x2, x6 : EOR (shifted) 64: 0xCA060042
    code.push(0xCA060042);
    // cont: (index) add x3, x3, #1 : 0x91000463
    let cont_idx = code.len();
    code.push(0x91000463);
    // b loop : B with negative offset
    let bloop_idx = code.len();
    let off_loop = (loop_idx as i64) - (bloop_idx as i64);
    code.push(0x14000000 | ((off_loop as u32) & 0x03FFFFFF));
    // end: (index) mov x0, x2 ; ret
    let end_idx = code.len();
    code.push(0xAA0203E0); // mov x0, x2 (orr x0, xzr, x2)
    code.push(0xD65F03C0); // ret
    // patch b.ge end (imm19)
    let off = (end_idx as i64) - (bge_idx as i64);
    code[bge_idx] = 0x54000000 | (((off as u32) & 0x7FFFF) << 5) | 0xA;
    // patch cbnz else (imm19)
    let off = (else_idx as i64) - (cbnz_idx as i64);
    code[cbnz_idx] = 0x35000000 | (((off as u32) & 0x7FFFF) << 5) | 5;
    // patch b cont (imm26)
    let off = (cont_idx as i64) - (bcont_idx as i64);
    code[bcont_idx] = 0x14000000 | ((off as u32) & 0x03FFFFFF);
}

fn main() {
    let mut code = Vec::new();
    emit(&mut code);
    let bytes = code.len() * 4;
    unsafe {
        let mem = mmap(std::ptr::null_mut(), 4096, PROT_READ|PROT_WRITE|PROT_EXEC, MAP_PRIVATE|MAP_ANON|MAP_JIT, -1, 0);
        assert!(mem as isize != -1, "mmap failed");
        pthread_jit_write_protect_np(0);
        std::ptr::copy_nonoverlapping(code.as_ptr() as *const u8, mem as *mut u8, bytes);
        pthread_jit_write_protect_np(1);
        sys_icache_invalidate(mem, bytes);
        let f: extern "C" fn(*const u8, u64) -> u64 = std::mem::transmute(mem);
        // test on random inputs
        let mut s: u64 = 0x1234;
        let mut ok = true;
        for _ in 0..1000 {
            let n = 1 + (s as usize % 300);
            let mut inp = vec![0u8; n];
            for x in inp.iter_mut() { s = s.wrapping_mul(6364136223846793005).wrapping_add(1); *x = (s >> 40) as u8; }
            let jit = f(inp.as_ptr(), n as u64);
            let re = reference(&inp);
            if jit != re { println!("MISMATCH n={} jit={:#x} ref={:#x}", n, jit, re); ok = false; break; }
        }
        println!("{}", if ok { "JIT MATCHES reference on 1000 random inputs => copy-and-patch kernel correct" } else { "JIT WRONG" });
    }
}

// Minimal copy-and-patch feasibility probe on aarch64 macOS (Carmack break 3):
// MAP_JIT mmap -> write an aarch64 stencil (add x0,x0,#HOLE; ret) -> PATCH the
// immediate -> pthread_jit_write_protect toggle -> icache invalidate -> execute.
#include <stdio.h>
#include <stdint.h>
#include <sys/mman.h>
#include <pthread.h>
#include <libkern/OSCacheControl.h>

typedef long (*fn_t)(long);

int main(void) {
    size_t sz = 4096;
    // MAP_JIT executable mapping (macOS hardened-runtime JIT)
    void* mem = mmap(NULL, sz, PROT_READ | PROT_WRITE | PROT_EXEC, MAP_ANON | MAP_PRIVATE | MAP_JIT, -1, 0);
    if (mem == MAP_FAILED) { printf("MAP_JIT FAILED (native tier unavailable on this process)\n"); return 2; }
    // make writable (per-thread W^X toggle)
    pthread_jit_write_protect_np(0);
    uint32_t* code = (uint32_t*)mem;
    // stencil: `add x0, x0, #HOLE` with HOLE=0 placeholder, then `ret`
    uint32_t add_tmpl = 0x91000000; // ADD x0,x0,#0
    uint32_t ret_ins  = 0xD65F03C0; // RET
    // PATCH the immediate hole to 42 (imm12 at bits 10..21)
    uint32_t patched = add_tmpl | (42u << 10);
    code[0] = patched;
    code[1] = ret_ins;
    // make executable + invalidate icache (arm64 needs both)
    pthread_jit_write_protect_np(1);
    sys_icache_invalidate(mem, sz);
    fn_t f = (fn_t)mem;
    long r = f(100);
    printf("copy-and-patch: f(100) = %ld (expect 142: patched stencil add #42)\n", r);
    munmap(mem, sz);
    return r == 142 ? 0 : 1;
}

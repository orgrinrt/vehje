// Copy-and-patch CONTROL-FLOW transfer: stencil A tail-transfers to stencil B via a
// register-indirect branch whose target ADDRESS is patched in (movz/movk x9; br x9).
// This is Cluster B's mechanism that dodges the aarch64 +-128MB branch-range limit.
#include <stdio.h>
#include <stdint.h>
#include <sys/mman.h>
#include <pthread.h>
#include <libkern/OSCacheControl.h>
typedef long (*fn_t)(long);
static uint32_t movz(int rd,uint32_t imm){return 0xD2800000u|((imm&0xFFFF)<<5)|rd;}
static uint32_t movk(int rd,uint32_t imm,int hw){uint32_t b[4]={0xF2800000u,0xF2A00000u,0xF2C00000u,0xF2E00000u};return b[hw]|((imm&0xFFFF)<<5)|rd;}
int main(void){
    size_t sz=4096;
    void* mem=mmap(NULL,sz,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_ANON|MAP_PRIVATE|MAP_JIT,-1,0);
    if(mem==MAP_FAILED){printf("MAP_JIT FAILED\n");return 2;}
    pthread_jit_write_protect_np(0);
    uint32_t* c=(uint32_t*)mem;
    // stencil B placed at word offset 8; compute its address for the patch.
    uint64_t baddr=(uint64_t)(uintptr_t)&c[8];
    int p=0;
    // stencil A: add x0,x0,#5 ; load B's addr into x9 (PATCHED) ; br x9
    c[p++]=0x91000000u|((uint32_t)5<<10);        // add x0,x0,#5
    c[p++]=movz(9,(uint32_t)(baddr));            // movz x9,#addr[0:16]
    c[p++]=movk(9,(uint32_t)(baddr>>16),1);      // movk x9,#addr[16:32],lsl16
    c[p++]=movk(9,(uint32_t)(baddr>>32),2);      // movk x9,#addr[32:48],lsl32
    c[p++]=movk(9,(uint32_t)(baddr>>48),3);      // movk x9,#addr[48:64],lsl48
    c[p++]=0xD61F0120u;                          // br x9  (register-indirect tail transfer)
    // pad to offset 8
    while(p<8)c[p++]=0xD503201Fu;                // nop
    // stencil B at offset 8: add x0,x0,#10 ; ret
    c[p++]=0x91000000u|((uint32_t)10<<10);       // add x0,x0,#10
    c[p++]=0xD65F03C0u;                          // ret
    pthread_jit_write_protect_np(1);
    sys_icache_invalidate(mem,sz);
    fn_t f=(fn_t)mem;
    long r=f(0);
    printf("stencil A --br x9(patched addr)--> stencil B: f(0)=%ld (expect 15: +5 then +10)\n",r);
    munmap(mem,sz);
    return r==15?0:1;
}

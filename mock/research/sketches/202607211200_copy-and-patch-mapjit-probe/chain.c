// Copy-and-patch STENCIL CHAIN: stencils compose by CONCATENATION (the codegen half).
// Emit K single-instruction add-stencils (each `add x0,x0,#imm`, imm PATCHED per node)
// concatenated + a final `ret`, then execute the composed code. Validates that a lowered
// program = a concatenation of patched stencils.
#include <stdio.h>
#include <stdint.h>
#include <sys/mman.h>
#include <pthread.h>
#include <libkern/OSCacheControl.h>
typedef long (*fn_t)(long);
int main(void){
    size_t sz=4096;
    void* mem=mmap(NULL,sz,PROT_READ|PROT_WRITE|PROT_EXEC,MAP_ANON|MAP_PRIVATE|MAP_JIT,-1,0);
    if(mem==MAP_FAILED){printf("MAP_JIT FAILED\n");return 2;}
    pthread_jit_write_protect_np(0);
    uint32_t* code=(uint32_t*)mem;
    // a "program": add 3, add 7, add 11, add 100  (the imms are the patched holes)
    int imms[]={3,7,11,100}; int K=4; long expect=0;
    uint32_t add_tmpl=0x91000000; // add x0,x0,#0
    int pc=0;
    for(int i=0;i<K;i++){ code[pc++]=add_tmpl | ((uint32_t)imms[i]<<10); expect+=imms[i]; } // concat patched stencils
    code[pc++]=0xD65F03C0; // ret
    pthread_jit_write_protect_np(1);
    sys_icache_invalidate(mem,sz);
    fn_t f=(fn_t)mem;
    long r=f(1000);
    printf("stencil chain (%d concatenated patched add-stencils): f(1000)=%ld (expect %ld)\n",K,r,1000+expect);
    munmap(mem,sz);
    return r==1000+expect?0:1;
}

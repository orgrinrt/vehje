/* Runtime-side locus (C host embedding the runtime) links the SAME engine. */
#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
extern uint64_t vehje_engine_reachhash(const uint32_t* b, size_t n, uint64_t seed);
int main(void){ uint32_t binders[6]={0,5,17,42,63,1}; printf("%llu\n",(unsigned long long)vehje_engine_reachhash(binders,6,0xABCD)); return 0; }

#include <stdio.h>
#include <stddef.h>
extern int vehje_decode(const unsigned char* ptr, size_t len, unsigned int* out);
int main(void) {
    unsigned int out = 0;
    unsigned char good[4] = { 0x34, 0x12, 0x00, 0x00 }; // 0x1234, in range
    unsigned char big[4]  = { 0xFF, 0xFF, 0xFF, 0x00 }; // 0xFFFFFF, out of range
    int rc1 = vehje_decode(good, 4, &out);
    printf("C host: decode(good) rc=%d out=0x%X (expect rc=0 out=0x1234)\n", rc1, out);
    int rc2 = vehje_decode(big, 4, &out);
    printf("C host: decode(out-of-range) rc=%d (expect rc=2, NO abort)\n", rc2);
    int rc3 = vehje_decode(good, 2, &out); // truncated
    printf("C host: decode(truncated) rc=%d (expect rc=1, NO abort)\n", rc3);
    printf("C host survived all calls: no panic crossed the C ABI.\n");
    return 0;
}

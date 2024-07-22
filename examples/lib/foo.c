#include <stdio.h>
#include <stdint.h>
#include <string.h>

void foo(const uint8_t* data, size_t length)
{
    int32_t a;
    float b;

    memcpy(&a, data, sizeof(int32_t));
    data += sizeof(int32_t); 
    memcpy(&b, data, sizeof(float));

    printf("Got number: %d %f\n", a, b);
}

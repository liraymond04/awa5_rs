#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

void foo(const uint8_t *data, uint8_t **out, size_t *out_len)
{
    int32_t a;
    float b;
    char str[256];
    char str1[256];
    char c;
    char d;

    memcpy(&a, data, sizeof(int32_t));
    data += sizeof(int32_t); 

    strcpy(str, (char *)data);
    data += strlen(str) + 1;

    strcpy(str1, (char *)data);
    data += strlen(str1) + 1;

    memcpy(&b, data, sizeof(float));
    data += sizeof(float);

    memcpy(&c, data, sizeof(char));
    data += sizeof(char);

    memcpy(&d, data, sizeof(char));
    data += sizeof(char);

    printf("%s\n", str);
    printf("%s\n", str1);

    printf("Got number: %d %f\n", a, b);
    printf("%c %c\n", c, d);

    *out = (uint8_t *)malloc(sizeof(uint8_t));
    (*out)[0] = 1;
    *out_len = sizeof(uint8_t);
}

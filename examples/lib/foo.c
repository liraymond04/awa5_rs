#include <stdio.h>
#include <stdint.h>
#include <string.h>

void foo(const uint8_t* data, size_t length)
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
}

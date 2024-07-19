#include <stdio.h>

void foo(void)
{
    puts("Hello, I am a shared library");
}

void foo_int(int a)
{
    printf("Got number: %d\n", a);
}

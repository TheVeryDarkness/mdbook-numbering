# Chapter 1

```C
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int fib(int n) {
    if (n <= 1) {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

int main() {
    printf("Hello, World!\n");

    for (int i = 0; i < 10; i++) {
        printf("fib(%d) = %d\n", i, fib(i));
    }

    return 0;
}
```

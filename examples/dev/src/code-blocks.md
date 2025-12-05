# Code Blocks

Three implementations of Fibonacci in different programming languages.

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

```rust
fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    println!("Hello, World!");

    for i in 0..10 {
        println!("fib({}) = {}", i, fib(i));
    }
}
```

```python
def fib(n):
    if n <= 1:
        return n
    return fib(n - 1) + fib(n - 2)

def main():
    print("Hello, World!")
    for i in range(10):
        print(f"fib({i}) = {fib(i)}")
```

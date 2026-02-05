import time
import my_rust_lib


def fibonacci_py(n: int) -> int:
    if n < 2:
        return n
    return fibonacci_py(n - 1) + fibonacci_py(n - 2)


n = 30

start = time.perf_counter()
fibonacci_py(n)
python_time = time.perf_counter() - start

start = time.perf_counter()
my_rust_lib.fibonacci(n)
rust_time = time.perf_counter() - start

print(f"Python 실행시간: {python_time:.4f}s")
print(f"Rust 실행시간:   {rust_time:.4f}s")
print("-------------------------")
print(f"=> Rust가 {python_time / rust_time:.1f}배 더 빠르네요.")

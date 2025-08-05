import time

iterations = 1_000_000
max_fib = 100

start = time.perf_counter()

for _ in range(iterations):
    a = 0
    b = 1

    for j in range(max_fib):
        a, b = b, a + b

end = time.perf_counter()

print(f"Elapsed time: {end - start:.6f} seconds")
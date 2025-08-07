import time

start = time.perf_counter()

iterations = 1_000_000
max_fib = 100

for _ in range(iterations):
    a = 0
    b = 1

    for _ in range(max_fib):
        temp = a + b
        a = b
        b = temp


end = time.perf_counter()

print(f"Elapsed time: {end - start:.6f} seconds")
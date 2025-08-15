

import time

n = 30
iterations = 1_000_000

# Benchmark start
start_time = time.perf_counter()

for _ in range(iterations):
    a, b = 0, 1
    for i in range(n):
        temp = a + b
        a = b
        b = temp


end_time = time.perf_counter()

execution_time = (end_time - start_time)

print(f"Total time: {execution_time:.3f}s")

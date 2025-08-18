

import time
start_time = time.perf_counter()
n = 100_000_007

for i in range(2, n):
    if n % i == 0:
        print("no")
        break

end_time = time.perf_counter()

execution_time = (end_time - start_time)

print(f"Total time: {execution_time:.3f}s")

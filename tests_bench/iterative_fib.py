import time

iterations = 1_000_000
max_fib = 30

start = time.perf_counter()


for i in range(iterations):
    a, b = 0, 1
    for j in range(max_fib):
        a, b = b, a + b

end = time.perf_counter()
print((end - start) * 1000)  

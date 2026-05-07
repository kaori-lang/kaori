import time

iterations = 1_000_000
max_fib = 30

for i in range(iterations):
    a, b = 0, 1
    for j in range(max_fib):
        a, b = b, a + b


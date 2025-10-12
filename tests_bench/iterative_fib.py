

iterations = 1_000_000
max_fib = 40


for i in range(iterations):
    a = 0
    b = 1

    for j in range(max_fib):
        temp = a + b
        a = b
        b = temp


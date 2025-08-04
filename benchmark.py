""" import time


start_time = time.time()


iterations = 100_000

for i in range(iterations):
    a = 0
    b = 1
    max_fib = 100

    for j in range(100):
        temp = a + b
        a = b

        if temp < b:
            b = temp * 1.0001 + (a % 3)
        elif temp == b:
            b = temp / 1.0001 - (a % 5)
        else:  # temp > b
            b = temp * temp - (a * b) / (temp + 1)

        c = (a * 3.1415) + (b / 2.718)
        d = (c * c) - (b * a) + (temp % 7)

        e = 0
        for k in range(10):
            e += (d % (k + 1)) + (c / (k + 2))

        if e > 1000:
            a += 1
        else:
            b -= 1


end_time = time.time()
elapsed = end_time - start_time
print(f"Time taken: {elapsed:.6f} seconds")
 """


print(bin(5))

print(5&1)
import time

start = time.perf_counter()

number = 1.0



j = 0.0

while j < 100000000:
    number += j
    j += 1.0

print(number)


end = time.perf_counter()

print(end - start)
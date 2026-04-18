import time

start = time.perf_counter()


cat = {"name": "Whiskers", "age": 5, "color": "black"}


sum_ = 0
N = 100_000_000

for _ in range(N):
    sum_ += cat["age"]

end = time.perf_counter()

print("sum:", sum_)
print("time:", end - start)


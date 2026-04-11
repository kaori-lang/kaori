import time

start = time.perf_counter()
class Cat:
    def __init__(self, name, age, color):
        self.name = name
        self.age = age
        self.color = color


cat = Cat("meow", 5, "black")

sum_ = 0
N = 100_000_000

for _ in range(N):
    sum_ += cat.age

end = time.perf_counter()

print("sum:", sum_)
print("time:", end - start)


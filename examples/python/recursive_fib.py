import time

def fib(n: int) -> int:
    if n < 2:
        return n
    return fib(n - 1) + fib(n - 2)


start = time.perf_counter()
fib(30)

end = time.perf_counter()
print((end - start) * 1000)  

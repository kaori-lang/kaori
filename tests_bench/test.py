import time
import sys
import io

N = 100_0000

def bench(func, name):
    start = time.perf_counter()
    func()
    end = time.perf_counter()


    print(f"{name}: {end - start:.4f}s")


# --- tests ---

def print_string():
    for _ in range(N):
        "hello world"

def print_boolean_true():
    for _ in range(N):
        True

def print_boolean_false():
    for _ in range(N):
        False


# --- run ---

bench(print_string, "print string")
bench(print_boolean_true, "print True")
bench(print_boolean_false, "print False")
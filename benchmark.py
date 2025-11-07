import time
import subprocess
import platform
import statistics
from pathlib import Path

folder = Path("tests_bench")
kaori_folder = Path("test_suite")
rust_exe = Path("target/release/kaori")

if platform.system() == "Windows":
    rust_exe = rust_exe.with_suffix(".exe")

scripts = [
    ("Kaori Iterative", kaori_folder / "iterative_fib.kr", [str(rust_exe)]),
    ("Kaori Recursive", kaori_folder / "recursive_fib.kr", [str(rust_exe)]),
    ("Lua Iterative", folder / "iterative_fib.lua", ["lua"]),
    ("Lua Recursive", folder / "recursive_fib.lua", ["lua"]),
    ("Python Iterative", folder / "iterative_fib.py", ["python"]),
    ("Python Recursive", folder / "recursive_fib.py", ["python"]),
    ("Pypy Iterative", folder / "iterative_fib.py", ["pypy"]),
    ("Pypy Recursive", folder / "recursive_fib.py", ["pypy"]),
]


def run_script(path, cmd_list, runs=20, warmups=5):
    """
    Run a single benchmark script multiple times and compute mean and relative stddev.
    - `runs`: number of measured executions.
    - `warmups`: number of ignored warm-up executions.
    """
    full_cmd = cmd_list + [str(path)]

    # Warm-up phase (ignore results)
    for _ in range(warmups):
        subprocess.run(full_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    times = []
    for _ in range(runs):
        output = subprocess.check_output(full_cmd, stderr=subprocess.DEVNULL)
        ms = float(output.strip())  # script prints milliseconds
        times.append(ms)

    mean = statistics.mean(times)
    stdev = statistics.stdev(times)
    pct = (stdev / mean * 100) if mean != 0 else 0.0
    return mean, pct


if __name__ == "__main__":
    print(f"{'Fibonacci':<25} {'Mean (ms)':<12} {'± %':<8}")
    print("-" * 48)

    for name, path, cmd_list in scripts:
        try:
            mean, pct = run_script(path, cmd_list)
            print(f"{name:<25} {mean:>9.3f} {pct:>7.2f}%")
        except subprocess.CalledProcessError:
            print(f"{name:<25} ERROR")

import subprocess
import statistics
import time
from pathlib import Path

folder = Path("examples")

scripts = [
    ("Kaori",   "1.0.0",  folder / "kaori/iterative_fib.kr",  folder / "kaori/recursive_fib.kr",  ["kaori"],   ["kaori"]),
    ("Lua",     "5.5.0",  folder / "lua/iterative_fib.lua",   folder / "lua/recursive_fib.lua",   ["lua"],     ["lua"]),
    ("Python",  "3.14.4", folder / "python/iterative_fib.py", folder / "python/recursive_fib.py", ["python"],  ["python"]),
    ("PyPy",    "7.3.20", folder / "python/iterative_fib.py", folder / "python/recursive_fib.py", ["pypy"],    ["pypy"]),
]


def run_script(path, cmd_list, runs=20, warmups=5):
    full_cmd = cmd_list + [str(path)]

    for _ in range(warmups):
        subprocess.run(full_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    times = []
    for _ in range(runs):
        start = time.perf_counter()
        subprocess.run(full_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        end = time.perf_counter()
        times.append((end - start) * 1000)

    mean = statistics.mean(times)
    stdev = statistics.stdev(times)
    pct = (stdev / mean * 100) if mean != 0 else 0.0
    return mean, pct


def fmt_result(mean, pct):
    ms = f"{mean:.3f} ms"
    pc = f"±{pct:.2f}%"
    return f"{ms:>16}  {pc:<8}"


def fmt_error():
    return f"{'ERROR':>16}  {'':8}"


if __name__ == "__main__":
    print(f"\n{'Language':<16} {'Version':<10} {'Iterative':>24}    {'Recursive':>24}")
    print("─" * 88)

    for lang, version, iter_path, rec_path, iter_cmd, rec_cmd in scripts:
        try:
            iter_mean, iter_pct = run_script(iter_path, iter_cmd)
            iter_str = fmt_result(iter_mean, iter_pct)
        except (subprocess.CalledProcessError, FileNotFoundError, ValueError):
            iter_str = fmt_error()

        try:
            rec_mean, rec_pct = run_script(rec_path, rec_cmd)
            rec_str = fmt_result(rec_mean, rec_pct)
        except (subprocess.CalledProcessError, FileNotFoundError, ValueError):
            rec_str = fmt_error()

        print(f"{lang:<16} {version:<10} {iter_str}    {rec_str}")

    print()
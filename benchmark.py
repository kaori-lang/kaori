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
    ("Kaori",  "1.0.0",  kaori_folder / "iterative_fib.kr", kaori_folder / "recursive_fib.kr",  [str(rust_exe)], [str(rust_exe)]),
    ("Lua",    "5.5.0",  folder / "iterative_fib.lua",       folder / "recursive_fib.lua",       ["lua"],         ["lua"]),
    ("Python", "3.14.4",    folder / "iterative_fib.py",        folder / "recursive_fib.py",        ["python"],      ["python"]),
    ("PyPy",   "7.3.20",  folder / "iterative_fib.py",        folder / "recursive_fib.py",        ["pypy"],        ["pypy"]),
]


def run_script(path, cmd_list, runs=20, warmups=5):
    """
    Run a single benchmark script multiple times.
    Returns (mean_ms, stdev_pct) or raises on error.
    """
    full_cmd = cmd_list + [str(path)]

    for _ in range(warmups):
        subprocess.run(full_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)

    times = []
    for _ in range(runs):
        output = subprocess.check_output(full_cmd, stderr=subprocess.DEVNULL)
        ms = float(output.strip())
        times.append(ms)

    mean = statistics.mean(times)
    stdev = statistics.stdev(times)
    pct = (stdev / mean * 100) if mean != 0 else 0.0
    return mean, pct


def fmt_result(mean, pct):
    return f"{mean:>10.3f} ms  ±{pct:>5.2f}%"


def fmt_error():
    return f"{'ERROR':>10}{'':12}"


if __name__ == "__main__":
    col = 10
    print(f"\n{'Language':<10} {'Version':<10} {'Iterative':>16}        {'Recursive':>16}")
    print("─" * 72)

    for lang, version, iter_path, rec_path, iter_cmd, rec_cmd in scripts:
        # iterative
        try:
            iter_mean, iter_pct = run_script(iter_path, iter_cmd)
            iter_str = fmt_result(iter_mean, iter_pct)
        except (subprocess.CalledProcessError, FileNotFoundError, ValueError):
            iter_str = fmt_error()

        # recursive
        try:
            rec_mean, rec_pct = run_script(rec_path, rec_cmd)
            rec_str = fmt_result(rec_mean, rec_pct)
        except (subprocess.CalledProcessError, FileNotFoundError, ValueError):
            rec_str = fmt_error()

        print(f"{lang:<10} {version:<10} {iter_str}    {rec_str}")

    print()
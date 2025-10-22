import subprocess
import time
import platform
from pathlib import Path

# Folders
folder = Path("tests_bench")       # Lua and Python scripts
kaori_folder = Path("test_suite")  # Kaori .kr scripts
rust_exe = Path("target/release/kaori")

# On Windows, add .exe
if platform.system() == "Windows":
    rust_exe = rust_exe.with_suffix(".exe")

# Scripts to benchmark
scripts = [
    ("Kaori Iterative", kaori_folder / "iterative_fib.kr", [str(rust_exe)]),
    ("Kaori Recursive", kaori_folder / "recursive_fib.kr", [str(rust_exe)]),
    ("Lua Iterative", folder / "iterative_fib.lua", ["lua"]),
    ("Lua Recursive", folder / "recursive_fib.lua", ["lua"]),
    ("Python Iterative", folder / "iterative_fib.py", ["python"]),
    ("Python Recursive", folder / "recursive_fib.py", ["python"]),
]

def run_script(path, cmd_list):
    """
    Run a script with the given command list, measure elapsed time.
    """
    # Append the script path as argument
    full_cmd = cmd_list + [str(path)]
    start = time.perf_counter()
    subprocess.run(full_cmd, check=True)
    elapsed = time.perf_counter() - start
    return elapsed

if __name__ == "__main__":
    print(f"{'Fibonacci':<25} {'Elapsed (s)':<12}")
    print("-" * 40)

    for name, path, cmd_list in scripts:
        elapsed = run_script(path, cmd_list)
        print(f"{name:<25} {elapsed:.6f}")



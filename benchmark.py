import subprocess
import json
import os
import tempfile
from pathlib import Path

folder = Path("examples")
scripts = [
    ("Kaori",  "1.0.0",  folder / "kaori/iterative_fib.kr",  folder / "kaori/recursive_fib.kr",  ["kaori"],  ["kaori"]),
    ("Lua",    "5.5.0",  folder / "lua/iterative_fib.lua",   folder / "lua/recursive_fib.lua",   ["lua"],    ["lua"]),
    ("Python", "3.14.4", folder / "python/iterative_fib.py", folder / "python/recursive_fib.py", ["python"], ["python"]),
    ("PyPy",   "7.3.20", folder / "python/iterative_fib.py", folder / "python/recursive_fib.py", ["pypy"],   ["pypy"]),
]

def run_hyperfine(path, cmd_list, runs=20, warmups=5):
    full_cmd = cmd_list + [str(path)]
    command_str = " ".join(f'"{p}"' if " " in p else p for p in full_cmd)

    with tempfile.NamedTemporaryFile(suffix=".json", delete=False) as f:
        tmp = f.name

    try:
        result = subprocess.run(
            [
                "hyperfine",
                "--export-json", tmp,
                "--runs", str(runs),
                "--warmup", str(warmups),
                command_str,
            ],
            capture_output=True,
            text=True,
        )

        if result.returncode != 0:
            raise RuntimeError(f"hyperfine failed:\n{result.stderr}")

        with open(tmp) as f:
            data = json.load(f)

    finally:
        os.unlink(tmp)

    r = data["results"][0]
    mean_ms = r["mean"] * 1000
    stddev_ms = r["stddev"] * 1000
    pct = (stddev_ms / mean_ms * 100) if mean_ms != 0 else 0.0
    return mean_ms, pct

def fmt_result(mean, pct):
    ms = f"{mean:.3f} ms"
    pc = f"±{pct:.2f}%"
    return f"{ms:>16}  {pc:<8}"

def fmt_error(e):
    msg = str(e)[:12]
    return f"{'ERROR':>16}  {msg:<8}"

if __name__ == "__main__":
    print(f"\n{'Language':<16} {'Version':<10} {'Iterative':>24}    {'Recursive':>24}")
    print("─" * 88)

    for lang, version, iter_path, rec_path, iter_cmd, rec_cmd in scripts:
        try:
            iter_mean, iter_pct = run_hyperfine(iter_path, iter_cmd)
            iter_str = fmt_result(iter_mean, iter_pct)
        except Exception as e:
            print(f"  [iter error for {lang}]: {e}")
            iter_str = fmt_error(e)

        try:
            rec_mean, rec_pct = run_hyperfine(rec_path, rec_cmd)
            rec_str = fmt_result(rec_mean, rec_pct)
        except Exception as e:
            print(f"  [rec error for {lang}]: {e}")
            rec_str = fmt_error(e)

        print(f"{lang:<16} {version:<10} {iter_str}    {rec_str}")

    print()


# hyperfine --warmup 10 --min-runs 40 'kaori main.kr'
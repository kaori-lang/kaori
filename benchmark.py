import subprocess
from pathlib import Path

folder = Path("examples")

scripts = [
    ("Kaori",  ["kaori"],      "kr",  "kaori"),
    ("Lua",    ["lua"],        "lua", "lua"),
    ("Python", ["python"],     "py",  "python"),
    ("PyPy",   ["pypy"],       "py",  "python"),
]

benchmarks = [
    ("Mandelbrot",    "mandelbrot"),
    ("Recursive Fib", "recursive_fib"),
]

for bench_name, bench_slug in benchmarks:
    print(f"\n── {bench_name} ──\n")
    commands = []
    for lang, cmd, ext, lang_folder in scripts:
        path = folder / lang_folder / f"{bench_slug}.{ext}"
        full_cmd = " ".join(cmd + [str(path)])
        commands.append(full_cmd)

    subprocess.run(
    [
        "hyperfine",
        "--warmup", "5",
        "--runs", "10",
        *commands,
    ]
    )
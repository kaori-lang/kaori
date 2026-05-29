import subprocess
from pathlib import Path

folder = Path("examples")

scripts = [
    ("Kaori",   ["kaori"],  "kr",  "kaori"),
    ("Lua",     ["lua"],    "lua", "lua"),
    ("LuaJIT",  ["luajit"], "lua", "lua"),
    ("PyPy",    ["pypy"],   "py",  "python"),
    ("Node.js", ["node"],   "js",  "javascript"),
    ("Python",  ["python"], "py",  "python"),
]

benchmarks = [
    #("Mandelbrot",    "mandelbrot"),
    ("Fibonnaci",    "recursive_fib"),
]

for bench_name, bench_slug in benchmarks:
    print(f"\n── {bench_name} ──\n")

    commands = []

    for lang, cmd, ext, lang_folder in scripts:
        path = folder / lang_folder / f"{bench_slug}.{ext}"
        full_cmd = " ".join(cmd + [str(path)])
        commands.append(full_cmd)

    subprocess.run([
        "hyperfine",
        "--warmup", "5",
        "--runs", "10",
        *commands,
    ])
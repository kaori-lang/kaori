import subprocess
import sys

file = sys.argv[1] if len(sys.argv) > 1 else "main.kr"

build = subprocess.run(
    ["cargo", "build", "--profile", "profiling"]
)

if build.returncode != 0:
    sys.exit(build.returncode)

subprocess.run([
    "samply",
    "record",
    r".\target\profiling\kaori.exe",
    file
])
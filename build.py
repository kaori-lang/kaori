
import subprocess
import sys

def run(cmd):
    result = subprocess.run(cmd)
    if result.returncode != 0:
        print(f"Error: {' '.join(cmd)} failed.")
        sys.exit(1)

run(["cargo", "build", "--release"])
run(["cargo", "install", "--path", "."])
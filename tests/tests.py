import subprocess
import sys
import os
from tests_data import tests

KAORI_DIR = r"C:\programming-projects\kaori"
KAORI = os.path.join(KAORI_DIR, "target", "release", "kaori.exe")


def normalize(val):
    try:
        f = float(val)
        return str(int(f)) if f == int(f) else str(f)
    except:
        return val


def run_kaori(code):
    path = os.path.join(KAORI_DIR, "_test_tmp.kr")
    with open(path, 'w') as f:
        f.write(code.strip())
    try:
        result = subprocess.run(
            [KAORI, path],
            capture_output=True,
            text=True,
            cwd=KAORI_DIR
        )
        return result.stdout.strip(), result.stderr
    finally:
        os.remove(path)


passed = 0
failed = 0

print(f"\n{'TEST':<40} {'EXPECTED':>12} {'GOT':>12} {'STATUS':>8}")
print("-" * 76)

for test in tests:
    name = test["name"]
    kaori_code = test["kaori"]
    fn = test["fn"]

    expected = normalize(str(fn()))
    got, stderr = run_kaori(kaori_code)
    got = normalize(got)

    status = "PASS" if expected == got else "FAIL"

    if expected == got:
        passed += 1
    else:
        failed += 1

    print(f"{name:<40} {expected:>12} {got:>12} {status:>8}")

    if stderr and expected != got:
        for line in stderr.strip().splitlines():
            print(f"  > {line}")

print("-" * 76)
print(f"\n  {passed} passed  {failed} failed\n")
"""
    run_test_suite.py\n
    Test runs the entire interpreter on all sample Kaori programs in `test_suite`.\n
    Added by: DrkWithT\n
    v0.0.1: Added numbers & fibonacci tests.\n
"""

from subprocess import run
from pathlib import Path

ALL_TEST_SOURCES = [
    'primitives.kr',
    'fib_recur.kr',
]

def try_kaori_tests(test_sources: list[str]) -> bool:
    test_suite_dir: Path = Path('test_suite/')
    had_failed_test = False

    for test_file in test_sources:
        next_test_path = test_suite_dir.joinpath(test_file)

        print(f'Testing source: \'{next_test_path}\'\n')

        test_result = run(['cargo', 'run', f'{next_test_path}'], capture_output=False)

        if test_result.returncode != 0:
            print(f'\x1b[1;31mTest failed for source at {next_test_path}\x1b[0m')
            had_failed_test = True
        else:
            print(f'\x1b[1;32mTest passed for source at {next_test_path}\x1b[0m')

    return not had_failed_test

if __name__ == '__main__':
    if try_kaori_tests(ALL_TEST_SOURCES):
        print(f'\x1b[1;32mAll tests passed :)\x1b[0m')
        exit(0)
    else:
        print(f'\x1b[1;31mSome tests failed, please check above logs.\x1b[0m')
        exit(1)



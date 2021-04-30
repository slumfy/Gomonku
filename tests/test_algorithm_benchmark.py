import subprocess

process = subprocess.Popen("rust_compilation.sh", shell=True, stdout=subprocess.PIPE)
process.wait()
print(process.returncode)
import gomoku_rust


def test_algorithm_benchmark():
    gomoku_rust.gomoku_tests.pytest_algorithm_benchmark()

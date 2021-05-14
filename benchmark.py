import subprocess

# Trying to import the gomoku_rust lib, if not compiled, execute the script to compile it.
try:
    import gomoku_rust
except ImportError:
    # Build rust lib
    process = subprocess.Popen("rust_compilation_release.sh", shell=True, stdout=subprocess.PIPE)
    process.wait()
    import gomoku_rust


def benchmark_test():
    print("Starting benchmark...")
    gomoku_rust.gomoku_tests.pytest_algorithm_benchmark()


def main():
    benchmark_test()


if __name__ == "__main__":
    main()
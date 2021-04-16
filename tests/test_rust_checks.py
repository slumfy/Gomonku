import subprocess

# Trying to import the gomoku_rust lib, if not compiled, execute the script to compile it.
try:
    import gomoku_rust
except ImportError:
    # Build rust lib
    process = subprocess.Popen("rust_compilation.sh", shell=True, stdout=subprocess.PIPE)
    process.wait()
    print(process.returncode)
    import gomoku_rust


def test_check_rust():
    gomoku_rust.gomoku_tests.test_check_is_unblockable_five()
    gomoku_rust.gomoku_tests.test_check_free_development()
    gomoku_rust.gomoku_tests.test_is_on_axe()

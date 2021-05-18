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
    gomoku_rust.gomoku_tests.pytest_is_on_axe()
    gomoku_rust.gomoku_tests.pytest_print_pos_in_human_format()


def test_check_is_unblockable_five():
    gomoku_rust.gomoku_tests.pytest_check_is_unblockable_five()


def test_pattern_axes_finder():
    gomoku_rust.gomoku_tests.pytest_pattern_axes_finder()


def test_check_potential_winning_alignment():
    gomoku_rust.gomoku_tests.pytest_check_potential_winning_alignment()


def test_check_is_capturable():
    gomoku_rust.gomoku_tests.pytest_check_is_capturable()

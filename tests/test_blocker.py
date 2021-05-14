import subprocess

# Trying to import the gomoku_rust lib, if not compiled, execute the script to compile it.
try:
    import gomoku_rust
except ImportError:
    # Build rust lib
    process = subprocess.Popen("rust_compilation.sh", shell=True, stdout=subprocess.PIPE)
    process.wait()
    import gomoku_rust

# TEST DOUBLE BLOCKER
def test_blocker_doubles_border():
    gomoku_rust.gomoku_tests.pytest_test_blocker_doubles_border()


def test_blocker_doubles_1_blocker_left():
    gomoku_rust.gomoku_tests.pytest_test_blocker_doubles_1_blocker_left()


def test_blocker_doubles_1_blocker_right():
    gomoku_rust.gomoku_tests.pytest_test_blocker_doubles_1_blocker_right()


def test_blocker_doubles_2_blocker_left():
    gomoku_rust.gomoku_tests.pytest_test_blocker_doubles_2_blocker_left()


def test_blocker_doubles_2_blocker_right():
    gomoku_rust.gomoku_tests.pytest_test_blocker_doubles_2_blocker_right()


# TEST TRIPLE BLOCKER
def test_blocker_triple_border():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_border()


def test_blocker_triple_1_blocker_left():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_1_blocker_left()


def test_blocker_triple_1_blocker_right():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_1_blocker_right()


def test_blocker_triple_2_blocker_right():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_2_blocker_right()


def test_blocker_triple_2_blocker_left():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_2_blocker_left()


def test_blocker_triple_2_with_hole_blocker_left():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_2_with_hole_blocker_left()


def test_blocker_triple_2_with_hole_blocker_right():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_2_with_hole_blocker_right()


def test_blocker_triple_3_blocker_right():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_3_blocker_right()


def test_blocker_triple_3_blocker_middle():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_3_blocker_middle()


def test_blocker_triple_3_blocker_left():
    gomoku_rust.gomoku_tests.pytest_test_blocker_triple_3_blocker_left()


def test_blocker_split_triple_rev_1_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_triple_rev_1_blocker()


def test_blocker_split_triple_rev_2_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_triple_rev_2_blocker()


def test_blocker_split_triple_rev_2_blocker_wrong():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_triple_rev_2_blocker_wrong()


def test_blocker_split_triple_1_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_triple_1_blocker()


def test_blocker_split_triple_2_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_triple_2_blocker()


def test_blocker_split_triple_2_blocker_wrong():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_triple_2_blocker_wrong()


# TEST FOUR
def test_test_blocker_four_1_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_four_1_blocker()


def test_blocker_four_2_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_four_2_blocker()


def test_blocker_split_four2_1_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four2_1_blocker()


def test_blocker_split_four2_2_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four2_2_blocker()


def test_blocker_split_four1_1_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four1_1_blocker()


def test_blocker_split_four1_1_blocker_wrong():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four1_1_blocker_wrong()


def test_blocker_split_four1_2_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four1_2_blocker()


def test_blocker_split_four1_2_blocker_wrong():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four1_2_blocker_wrong()


def test_blocker_split_four3_1_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four3_1_blocker()


def test_blocker_split_four3_1_blocker_wrong():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four3_1_blocker_wrong()


def test_blocker_split_four3_2_blocker():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four3_2_blocker()


def test_blocker_split_four3_2_blocker_wrong():
    gomoku_rust.gomoku_tests.pytest_test_blocker_split_four3_2_blocker_wrong()

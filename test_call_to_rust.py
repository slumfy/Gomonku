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

board = []
m = 19
n = 19
board = [[0] * m for i in range(n)]
dic = {
    "map": board,
    "player": 0,
    "x": 0,
    "y": 0,
}

gomoku_rust.gomoku_tests.test_getting_dict_from_python(dic)
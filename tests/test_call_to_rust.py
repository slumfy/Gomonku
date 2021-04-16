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


def test_call_returning_dict_to_python():
    map = []
    m = 19
    n = 19
    map = [[0] * m for i in range(n)]
    dic = {
        "map": map,
        "player": 1,
        "x": 0,
        "y": 0,
    }
    output_dic = gomoku_rust.gomoku_tests.pytest_returning_dict_to_python(
        dic["map"], dic["player"], dic["x"], dic["y"]
    )
    assert output_dic["eated_piece"] == 10
    assert output_dic["board"] == map


def test_call_updating_map_from_other_function():
    map = []
    m = 19
    n = 19
    map = [[0] * m for i in range(n)]
    dic = {
        "map": map,
        "player": 1,
        "x": 0,
        "y": 0,
    }
    output_dic = gomoku_rust.gomoku_tests.pytest_updating_from_other_function(
        dic["map"], dic["player"], dic["x"], dic["y"]
    )
    map[0][0] = dic["player"]
    map[0][1] = 2
    assert output_dic["eated_piece"] == 10
    assert output_dic["board"] == map


def test_call_sending_dict_to_python():
    # Cannot send different type for values...
    dic = {
        "map": 0,
        "player": 1,
        "x": 3,
        "y": 0,
    }
    gomoku_rust.gomoku_tests.pytest_get_pydict(dic)

from GoRules import GoRules
from PyGameGo import PyGameGo
import gomoku_lib


if __name__ == "__main__":
    print(gomoku_lib.sum_as_string("AABBCC", "DDDD"))
    go = GoRules()
    game_inst = PyGameGo()
    game_inst.menu(go)

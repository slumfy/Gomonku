from GoRules import GoRules
from PyGameGo import PyGameGo
import string_sum


if __name__ == "__main__":
    print(string_sum.sum_as_string(2,2))
    go = GoRules()
    game_inst = PyGameGo()
    game_inst.menu(go)

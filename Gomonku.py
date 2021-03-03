from GoRules import GoRules
from PyGameGo import PyGameGo

if __name__ == "__main__":
    go = GoRules()
    game_inst = PyGameGo()
    game_inst.menu(go)

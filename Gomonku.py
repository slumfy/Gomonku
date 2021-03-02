from GoRules import GoRules
from PygameGo import PyGameGo

if __name__ == "__main__":
    go = GoRules()
    go.print_table()
    game_inst = PyGameGo()
    game_inst.menu(go)

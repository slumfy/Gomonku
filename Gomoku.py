from go_rules import GoRules
from py_game_go import PyGameGo


if __name__ == "__main__":
    go_rules = GoRules()
    game = PyGameGo()
    game.menu(go_rules=go_rules)

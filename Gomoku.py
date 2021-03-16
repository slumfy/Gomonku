from go_rules import GoRules
from PyGameGo import PyGameGo


if __name__ == "__main__":
    go_rules = GoRules()
    game = PyGameGo()
    game.menu(go_rules=go_rules)

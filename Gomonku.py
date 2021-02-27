from Go_Rules import Go_Rules
from Pygame_Go import Game

if __name__ == "__main__":
    go = Go_Rules()
    go.Print_Table()
    game_inst = Game()
    game_inst.Menu(go)

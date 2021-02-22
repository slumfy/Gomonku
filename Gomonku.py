from pygame_go import Game
from go_rules import Go

if __name__ == "__main__":
	go = Go()
	go.print_table()
	game_inst = Game()
	game_inst.playing(go)

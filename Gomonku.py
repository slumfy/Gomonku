from go_rules import Go
from pygame_go import Game

if __name__ == "__main__":
	go = Go()
	go.print_table()
	game_inst = Game()
	game_inst.menu(go)
	# game_inst.playing(go)

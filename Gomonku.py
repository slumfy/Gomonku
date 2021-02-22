import sys, pygame
pygame.init()

size = width, height = 720, 720
board_size = width, height = 720, 720
stone_size = width, height = 32, 32
player = 1

screen = pygame.display.set_mode(size)

goboard = pygame.image.load("ressources/images/go_board_hd.png")
goboard_resize = pygame.transform.scale(goboard, size)
startpoint = [0,0]
white_stone = pygame.image.load("ressources/images/whitecircle.png")
white_stone_resize = pygame.transform.scale(white_stone, stone_size)
black_stone = pygame.image.load("ressources/images/blackcircle.png")
black_stone_resize = pygame.transform.scale(black_stone, stone_size)

screen.blit(goboard_resize, startpoint)
pygame.display.flip()
while 1:
	for event in pygame.event.get():
		if event.type == pygame.QUIT: sys.exit()
		if event.type == pygame.MOUSEBUTTONDOWN:
			if player == 1:
				stone = white_stone_resize
				player = 2
			else:
				stone = black_stone_resize
				player = 1
			print(event.pos)
			screen.blit(stone, (event.pos[0] - stone_size[0]/2,event.pos[1] - stone_size[1]/2))
			pygame.display.update()




# if __name__ == "__main__":

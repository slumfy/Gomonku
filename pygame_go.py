import sys, pygame

class Game:
	
	def __init__(self):
		pygame.init()
		self.size = width, height = 720, 720
		self.board_size = width, height = 720, 720
		self.stone_size = width, height = 24, 24
		self.player = 1

		self.screen = pygame.display.set_mode(self.size)

		self.goboard = pygame.image.load("ressources/images/goboard.png")
		# goboard = pygame.image.load("ressources/images/go_board_hd.png")
		self.goboard_resize = pygame.transform.scale(self.goboard, self.size)
		self.startpoint = [0,0]
		self.white_stone = pygame.image.load("ressources/images/whitecircle.png")
		self.white_stone_resize = pygame.transform.scale(self.white_stone, self.stone_size)
		self.black_stone = pygame.image.load("ressources/images/blackcircle.png")
		self.black_stone_resize = pygame.transform.scale(self.black_stone, self.stone_size)
		self.screen.blit(self.goboard_resize, self.startpoint)
		pygame.display.flip()

	def playing(self, go):
		offset = 62
		space = 33
		while 1:
			for event in pygame.event.get():
				if event.type == pygame.QUIT: sys.exit()
				if event.type == pygame.MOUSEBUTTONDOWN:
					self.screen.blit(self.goboard_resize, self.startpoint)
					#img position for mouse position
					print(event.pos)
					x = int((event.pos[1] - offset)/space)
					modx = int((event.pos[1] - offset)%space)
					if modx > space/2 and x < 18:
						x += 1
					y = int((event.pos[0] - offset)/space)
					mody = int((event.pos[0] - offset)%space)
					if mody > space/2 and y < 18:
						y += 1
					print(x,modx,y, mody)
					#end
					stonestatus = go.place_stone(self.player, x , y)
					print("stonestatus: ", stonestatus)
					if stonestatus == -1:
						print("illegal move")
						self.print_font('Illegal move', 64, 32)
					elif stonestatus == 0:
						if self.player == 1:
							self.player = 2
						elif self.player == 2:
							self.player = 1
					else:
						if stonestatus > 0:
							winstr = "player " + str(stonestatus) + " win"
							print(winstr)
							self.print_font(winstr, 64, 32)
					for L in range(len(go.table)):
						for l in range(len(go.table[L])):
							if go.table[L][l] == 1:
								self.screen.blit(self.white_stone_resize, (l*space + offset - self.stone_size[0]/2,L*space + offset - self.stone_size[1]/2))
							elif go.table[L][l] == 2:
								self.screen.blit(self.black_stone_resize, (l*space + offset - self.stone_size[0]/2,L*space + offset - self.stone_size[1]/2))
					pygame.display.flip()

	def print_font(self,msg,x,y):
		BLACK = (0, 0, 0)
		sysfont = pygame.font.get_default_font()
		font = pygame.font.SysFont(None,32)
		render = font.render(msg,True,BLACK)
		self.screen.blit(render,(x,y))
		# pygame.display.update()
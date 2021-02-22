class Go:
	table = []

	def __init__(self):
		m = 19
		n = 19
		self.table = [[0] * m for i in range(n)]

	def check_win_position(self,player,x,y):
		#updown solution
		it = 1
		for n in range(1,18):
			if x + n < 18 and self.table[x + n][y] == player:
				it += 1
			else:
				break
		for n in range(1,18):
			if x - n >= 0 and self.table[x - n][y] == player:
				it += 1
			else:
				break
		self.win(player,it)
		#leftright solution
		it = 1
		for n in range(1,18):
			if y + n < 18 and self.table[x][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,18):
			if y - n >= 0 and self.table[x][y - n] == player:
				it += 1
			else:
				break
		self.win(player,it)
		#diagdownleft solution
		it = 1
		for n in range(1,18):
			if x - n >= 0 and y + n < 18 and self.table[x - n][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,18):
			if x + n >= 0 and y - n < 18 and self.table[x + n][y - n] == player:
				it += 1
			else:
				break
		self.win(player,it)
		#diagupleft solution
		it = 1
		for n in range(1,18):
			if x + n >= 0 and y + n < 18 and self.table[x + n][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,18):
			if x - n >= 0 and y - n < 18 and self.table[x - n][y - n] == player:
				it += 1
			else:
				break
		self.win(player,it)
		
	def win(self,player,it):
		if it == 5:
			print("player ", player, " win")

	def check_eat_position(self,player,x,y):
		# backward take
		if y + 1 < 18 and self.table[x][y + 1] != player and self.table[x][y + 1] != 0:
			if self.table[x][y + 2] == player:
				self.table[x][y + 1] = 0
			elif y + 2 < 18 and self.table[x][y + 2] != 0:
				if self.table[x][y + 3] == player:
					self.table[x][y + 1] = 0
					self.table[x][y + 2] = 0
		# front take
		if y - 1 >= 0 and self.table[x][y - 1] != player and self.table[x][y - 1] != 0:
			if self.table[x][y - 2] == player:
				self.table[x][y - 1] = 0
			elif y - 2 >= 0 and self.table[x][y - 2] != 0:
				if self.table[x][y - 3] == player:
					self.table[x][y - 1] = 0
					self.table[x][y - 2] = 0
		# up take
		if x + 1 < 18 and self.table[x + 1][y] != player and self.table[x + 1][y] != 0:
			if self.table[x + 2][y] == player:
				self.table[x + 1][y] = 0
			elif x + 2 < 18 and self.table[x + 2][y] != 0:
				if self.table[x + 3][y] == player:
					self.table[x + 1][y] = 0
					self.table[x + 2][y] = 0
		# down take
		if x - 1 >= 0 and self.table[x - 1][y] != player and self.table[x - 1][y] != 0:
			if self.table[x - 2][y] == player:
				self.table[x - 1][y] = 0
			elif x - 2 >= 0 and self.table[x - 2][y] != 0:
				if self.table[x - 3][y] == player:
					self.table[x - 1][y] = 0
					self.table[x - 2][y] = 0
		# diag upleft take
		if x + 1 < 18 and y + 1 < 18 and self.table[x + 1][y + 1] != player and self.table[x + 1][y + 1] != 0:
			if self.table[x + 2][y + 2] == player:
				self.table[x + 1][y + 1] = 0
			elif x + 2 < 18 and y + 2 < 18 and self.table[x + 2][y + 2] != 0:
				if self.table[x + 3][y + 3] == player:
					self.table[x + 1][y + 1] = 0
					self.table[x + 2][y + 2] = 0
		# diag upright take
		if x + 2 < 18 and y - 1 >= 0 and self.table[x + 1][y - 1] != player and self.table[x + 1][y - 1] != 0:
			if self.table[x + 2][y - 2] == player:
				self.table[x + 1][y - 1] = 0
			elif x + 2 < 18 and y - 2 >= 0 and self.table[x + 2][y - 2] != 0:
				if self.table[x + 3][y - 3] == player:
					self.table[x + 1][y - 1] = 0
					self.table[x + 2][y - 2] = 0
		# diag downleft take
		if x - 1 >= 0 and y + 1 < 18 and self.table[x - 1][y + 1] != player and self.table[x - 1][y + 1] != 0:
			if self.table[x - 2][y + 2] == player:
				self.table[x - 1][y + 1] = 0
			elif x - 2 >= 0 and y + 2 < 18 and self.table[x - 2][y + 2] != 0:
				if self.table[x - 3][y + 3] == player:
					self.table[x - 1][y + 1] = 0
					self.table[x - 2][y + 2] = 0
		# diag downright take
		if x - 1 >= 0 and y - 1 >= 0 and self.table[x - 1][y - 1] != player and self.table[x - 1][y - 1] != 0:
			if self.table[x - 2][y - 2] == player:
				self.table[x - 1][y - 1] = 0
			elif x - 2 >= 0 and y - 2 >= 0 and self.table[x - 2][y - 2] != 0:
				if self.table[x - 3][y - 3] == player:
					self.table[x - 1][y - 1] = 0
					self.table[x - 2][y - 2] = 0

	def place_stone(self,player, x,y):
		if self.table[x][y] != 0:
			print("illegal move")
		else:
			self.table[x][y] = player
			self.check_eat_position(player,x,y)
			self.check_win_position(player,x,y)
			# self.print_table()

	def print_table(self):
		for line in self.table:
			print(line)
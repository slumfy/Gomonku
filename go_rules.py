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
		if self.win(player,it) == 1:
			return(1)
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
		if self.win(player,it) == 1:
			return(1)
		#diagdownleft solution
		it = 1
		for n in range(1,18):
			if x - n >= 0 and y + n < 18 and self.table[x - n][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,18):
			if x + n < 18 and y - n >= 0 and self.table[x + n][y - n] == player:
				it += 1
			else:
				break
		if self.win(player,it) == 1:
			return(1)
		#diagupleft solution
		it = 1
		for n in range(1,18):
			if x + n < 18 and y + n < 18 and self.table[x + n][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,18):
			if x - n >= 0 and y - n >= 0 and self.table[x - n][y - n] == player:
				it += 1
			else:
				break
		if self.win(player,it) == 1:
			return(1)
		return(0)
		
	def win(self,player,it):
		print("player: ",player, "iter: ",it)
		if it == 5:
			print("player ", player, " win")
			return(1)
		return(0)

	def check_wrong_position(self,player,x,y):
		if x > 18 or x < 0 or y > 18 or y < 0 or self.table[x][y] != 0:
			return(1)
		#updown solution
		it = 1
		uptrap = 0
		downtrap = 0
		for n in range(1,3):
			if x + n < 18 and self.table[x + n][y] != player and self.table[x + n][y] != 0:
				uptrap = 1
			if x + n < 18 and self.table[x + n][y] == player:
				it += 1
			else:
				break
		for n in range(1,3):
			if x - n >= 0 and self.table[x - n][y] != player and self.table[x - n][y] != 0:
				downtrap = 1
			if x - n >= 0 and self.table[x - n][y] == player:
				it += 1
			else:
				break
		if it <= 2 and uptrap and downtrap:
			return(1)
		#leftright solution
		it = 1
		uptrap = 0
		downtrap = 0
		for n in range(1,3):
			if y + n < 18 and self.table[x][y + n] != player and self.table[x][y + n] != 0:
				uptrap = 1
			if y + n < 18 and self.table[x][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,3):
			if y - n >= 0 and self.table[x][y - n] != player and self.table[x][y - n] != 0:
				downtrap = 1
			if y - n >= 0 and self.table[x][y - n] == player:
				it += 1
			else:
				break
		if it <= 2 and uptrap and downtrap:
			return(1)
		#diagdownleft solution
		it = 1
		uptrap = 0
		downtrap = 0
		for n in range(1,3):
			if x - n >= 0 and y + n < 18 and self.table[x - n][y + n] != player and self.table[x - n][y + n] != 0:
				uptrap = 1
			if x - n >= 0 and y + n < 18 and self.table[x - n][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,3):
			if x + n < 18 and y - n >= 0 and self.table[x + n][y - n] != player and self.table[x + n][y - n] != 0:
				downtrap = 1
			if x + n < 18 and y - n >= 0 and self.table[x + n][y - n] == player:
				it += 1
			else:
				break
		if it <= 2 and uptrap and downtrap:
			return(1)
		#diagupleft solution
		it = 1
		uptrap = 0
		downtrap = 0
		for n in range(1,3):
			if x + n < 18 and y + n < 18 and self.table[x + n][y + n] != player and self.table[x + n][y + n] != 0:
				uptrap = 1
			if x + n < 18 and y + n < 18 and self.table[x + n][y + n] == player:
				it += 1
			else:
				break
		for n in range(1,3):
			if x - n >= 0 and y - n >= 0 and self.table[x - n][y - n] != player and self.table[x - n][y - n] != 0:
				downtrap = 1
			if x - n >= 0 and y - n >= 0 and self.table[x - n][y - n] == player:
				it += 1
			else:
				break
		if it <= 2 and uptrap and downtrap:
			return(1)
		return(0)


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
		if self.check_wrong_position(player,x,y) == 1:
			return(-1)
		else:
			self.table[x][y] = player
			self.check_eat_position(player,x,y)
			if self.check_win_position(player,x,y) != 0:
				return(player)
			return(0)

	def print_table(self):
		for line in self.table:
			print(line)
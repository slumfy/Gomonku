from player import Player


class Go:
    table = []
    player_list = []

    def __init__(self):
        m = 19
        n = 19
        self.table = [[0] * m for i in range(n)]
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))
        print(self.player_list[0].nb, self.player_list[1].nb)

    def check_win_position(self, player, x, y):
        # updown solution
        it = 1
        for n in range(1, 18):
            if x + n < 18 and self.table[x + n][y] == player:
                it += 1
            else:
                break
        for n in range(1, 18):
            if x - n >= 0 and self.table[x - n][y] == player:
                it += 1
            else:
                break
        if self.win(player, it) == 1:
            return 1
        # leftright solution
        it = 1
        for n in range(1, 18):
            if y + n < 18 and self.table[x][y + n] == player:
                it += 1
            else:
                break
        for n in range(1, 18):
            if y - n >= 0 and self.table[x][y - n] == player:
                it += 1
            else:
                break
        if self.win(player, it) == 1:
            return 1
        # diagdownleft solution
        it = 1
        for n in range(1, 18):
            if x - n >= 0 and y + n < 18 and self.table[x - n][y + n] == player:
                it += 1
            else:
                break
        for n in range(1, 18):
            if x + n < 18 and y - n >= 0 and self.table[x + n][y - n] == player:
                it += 1
            else:
                break
        if self.win(player, it) == 1:
            return 1
        # diagupleft solution
        it = 1
        for n in range(1, 18):
            if x + n < 18 and y + n < 18 and self.table[x + n][y + n] == player:
                it += 1
            else:
                break
        for n in range(1, 18):
            if x - n >= 0 and y - n >= 0 and self.table[x - n][y - n] == player:
                it += 1
            else:
                break
        if self.win(player, it) == 1:
            return 1
        return 0

    def win(self, player, it):
        print("player: ", player, "iter: ", it)
        if it == 5:
            print("player ", player, " win")
            return 1
        return 0

    def check_wrong_position(self, player, x, y):
        if x > 18 or x < 0 or y > 18 or y < 0 or self.table[x][y] != 0:
            return 1
        # updown solution
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 3):
            if x + n < 18 and self.table[x + n][y] != player and self.table[x + n][y] != 0:
                uptrap = 1
            if x + n < 18 and self.table[x + n][y] == player:
                it += 1
            else:
                break
        for n in range(1, 3):
            if x - n >= 0 and self.table[x - n][y] != player and self.table[x - n][y] != 0:
                downtrap = 1
            if x - n >= 0 and self.table[x - n][y] == player:
                it += 1
            else:
                break
        if it <= 2 and uptrap and downtrap:
            return 1
        # leftright solution
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 3):
            if y + n < 18 and self.table[x][y + n] != player and self.table[x][y + n] != 0:
                uptrap = 1
            if y + n < 18 and self.table[x][y + n] == player:
                it += 1
            else:
                break
        for n in range(1, 3):
            if y - n >= 0 and self.table[x][y - n] != player and self.table[x][y - n] != 0:
                downtrap = 1
            if y - n >= 0 and self.table[x][y - n] == player:
                it += 1
            else:
                break
        if it <= 2 and uptrap and downtrap:
            return 1
        # diagdownleft solution
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 3):
            if (
                x - n >= 0
                and y + n < 18
                and self.table[x - n][y + n] != player
                and self.table[x - n][y + n] != 0
            ):
                uptrap = 1
            if x - n >= 0 and y + n < 18 and self.table[x - n][y + n] == player:
                it += 1
            else:
                break
        for n in range(1, 3):
            if (
                x + n < 18
                and y - n >= 0
                and self.table[x + n][y - n] != player
                and self.table[x + n][y - n] != 0
            ):
                downtrap = 1
            if x + n < 18 and y - n >= 0 and self.table[x + n][y - n] == player:
                it += 1
            else:
                break
        if it <= 2 and uptrap and downtrap:
            return 1
        # diagupleft solution
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 3):
            if (
                x + n < 18
                and y + n < 18
                and self.table[x + n][y + n] != player
                and self.table[x + n][y + n] != 0
            ):
                uptrap = 1
            if x + n < 18 and y + n < 18 and self.table[x + n][y + n] == player:
                it += 1
            else:
                break
        for n in range(1, 3):
            if (
                x - n >= 0
                and y - n >= 0
                and self.table[x - n][y - n] != player
                and self.table[x - n][y - n] != 0
            ):
                downtrap = 1
            if x - n >= 0 and y - n >= 0 and self.table[x - n][y - n] == player:
                it += 1
            else:
                break
        if it <= 2 and uptrap and downtrap:
            return 1
        return 0

    def check_eat_in_table(self, player, x, y, xsign, ysign, offset):
        if (
            x + offset * xsign > 18
            or x + offset * xsign < 0
            or y + offset * ysign > 18
            or y + offset * ysign < 0
        ):
            return 1
        return 0

    def eat_position_routine(self, player, x, y, xsign, ysign):
        if (
            self.check_eat_in_table(player, x, y, xsign, ysign, 1) == 0
            and self.table[x + 1 * xsign][y + 1 * ysign] != player.nb
            and self.table[x + 1 * xsign][y + 1 * ysign] != 0
        ):
            if (
                self.check_eat_in_table(player, x, y, xsign, ysign, 2) == 0
                and self.table[x + 2 * xsign][y + 2 * ysign] == player.nb
            ):
                self.eat_stone(player, [(x + 1 * xsign, y + 1 * ysign)])
            elif (
                self.check_eat_in_table(player, x, y, xsign, ysign, 2) == 0
                and self.table[x + 2 * xsign][y + 2 * ysign] != 0
            ):
                if (
                    self.check_eat_in_table(player, x, y, xsign, ysign, 3) == 0
                    and self.table[x + 3 * xsign][y + 3 * ysign] == player.nb
                ):
                    self.eat_stone(
                        player, [(x + 1 * xsign, y + 1 * ysign), (x + 2 * xsign, y + 2 * ysign)]
                    )

    def check_eat_position(self, player, x, y):
        # backward take
        self.eat_position_routine(player, x, y, 0, 1)
        # front take
        self.eat_position_routine(player, x, y, 0, -1)
        # up take
        self.eat_position_routine(player, x, y, 1, 0)
        # down take
        self.eat_position_routine(player, x, y, -1, 0)
        # diag upleft take
        self.eat_position_routine(player, x, y, 1, 1)
        # diag upright take
        self.eat_position_routine(player, x, y, 1, -1)
        # diag downleft take
        self.eat_position_routine(player, x, y, -1, 1)
        # diag downright take
        self.eat_position_routine(player, x, y, -1, -1)

    def eat_stone(self, player, poslist):
        for x, y in poslist:
            self.table[x][y] = 0
            player.eat_piece += 1

    def place_stone(self, player, x, y):
        if self.check_wrong_position(player.nb, x, y) == 1:
            return -1
        else:
            self.table[x][y] = player.nb
            self.check_eat_position(player, x, y)
            if player.eat_piece >= 10:
                return player.nb
            if self.check_win_position(player.nb, x, y) != 0:
                return player.nb
            return 0

    def clear_board(self):
        for L in range(len(self.table)):
            for l in range((len(self.table[L]))):
                self.table[L][l] = 0
        self.print_table()

    def print_table(self):
        for line in self.table:
            print(line)

from Player import Player


class Go_Rules:
    table = []
    player_list = []

    def __init__(self):
        m = 19
        n = 19
        self.table = [[0] * m for i in range(n)]
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))
        print(self.player_list[0].nb, self.player_list[1].nb)

    def Check_Three_Position(self, player, x, y):
        three_count = 0
        # updown solution
        if self.Check_Three_Routine(player, x, y, 1, 0) == 1:
            three_count += 1
        # leftright solution
        if self.Check_Three_Routine(player, x, y, 0, 1) == 1:
            three_count += 1
        # diagdownleft solution
        if self.Check_Three_Routine(player, x, y, -1, 1) == 1:
            three_count += 1
        # diagupleft solution
        if self.Check_Three_Routine(player, x, y, 1, 1) == 1:
            three_count += 1
        # print("threecount", three_count)
        if three_count < 2:
            return 0
        return 1

    def Check_Win_Position(self, player, x, y):
        # updown solution
        if self.Check_Win_Routine(player, x, y, 1, 0) == 1:
            return 1
        # leftright solution
        if self.Check_Win_Routine(player, x, y, 0, 1) == 1:
            return 1
        # diagdownleft solution
        if self.Check_Win_Routine(player, x, y, -1, 1) == 1:
            return 1
        # diagupleft solution
        if self.Check_Win_Routine(player, x, y, 1, 1) == 1:
            return 1
        return 0

    def Check_Wrong_Position(self, player, x, y):
        if self.Check_Is_In_Table(x, y, 0, 0, 0) or self.table[x][y] != 0:
            return 1
        if self.Check_Three_Position(player, x, y) != 0:
            return 1
        # updown solution
        elif self.Check_Wrong_Routine(player, x, y, 1, 0) == 1:
            return 1
        # leftright solution
        elif self.Check_Wrong_Routine(player, x, y, 0, 1) == 1:
            return 1
        # diagdownleft solution
        elif self.Check_Wrong_Routine(player, x, y, -1, 1) == 1:
            return 1
        # diagupleft solution
        elif self.Check_Wrong_Routine(player, x, y, 1, 1) == 1:
            return 1
        return 0

    def Check_Eat_Position(self, player, x, y):
        # backward take
        self.Eat_Position_Routine(player, x, y, 0, 1)
        # front take
        self.Eat_Position_Routine(player, x, y, 0, -1)
        # up take
        self.Eat_Position_Routine(player, x, y, 1, 0)
        # down take
        self.Eat_Position_Routine(player, x, y, -1, 0)
        # diag upleft take
        self.Eat_Position_Routine(player, x, y, 1, 1)
        # diag upright take
        self.Eat_Position_Routine(player, x, y, 1, -1)
        # diag downleft take
        self.Eat_Position_Routine(player, x, y, -1, 1)
        # diag downright take
        self.Eat_Position_Routine(player, x, y, -1, -1)

    def Eat_Stone(self, player, poslist):
        for x, y in poslist:
            self.table[x][y] = 0
            player.eat_piece += 1

    def Place_Stone(self, player, x, y):
        if self.Check_Wrong_Position(player.nb, x, y) == 1:
            return -1
        else:
            self.table[x][y] = player.nb
            self.Check_Eat_Position(player, x, y)
            if player.eat_piece >= 10:
                return player.nb
            if self.Check_Win_Position(player.nb, x, y) != 0:
                player.wining_position.append([x, y])
                return 0
            return 0

    def Check_Is_In_Table(self, x, y, xsign, ysign, offset):
        if (
            x + offset * xsign > 18
            or x + offset * xsign < 0
            or y + offset * ysign > 18
            or y + offset * ysign < 0
        ):
            return 1
        return 0

    def Win(self, player, it):
        if it == 5:
            print("player ", player, " win")
            return 1
        return 0

    def Clear_Board(self):
        for L in range(len(self.table)):
            for l in range((len(self.table[L]))):
                self.table[L][l] = 0
        self.print_table()

    def Print_Table(self):
        for line in self.table:
            print(line)

    def Eat_Position_Routine(self, player, x, y, xsign, ysign):
        if (
            self.Check_Is_In_Table(x, y, xsign, ysign, 1) == 0
            and self.table[x + 1 * xsign][y + 1 * ysign] != player.nb
            and self.table[x + 1 * xsign][y + 1 * ysign] != 0
        ):
            if (
                self.Check_Is_In_Table(x, y, xsign, ysign, 2) == 0
                and self.table[x + 2 * xsign][y + 2 * ysign] == player.nb
            ):
                self.Eat_Stone(player, [(x + 1 * xsign, y + 1 * ysign)])
            elif (
                self.Check_Is_In_Table(x, y, xsign, ysign, 2) == 0
                and self.table[x + 2 * xsign][y + 2 * ysign] != 0
            ):
                if (
                    self.Check_Is_In_Table(x, y, xsign, ysign, 3) == 0
                    and self.table[x + 3 * xsign][y + 3 * ysign] == player.nb
                ):
                    self.Eat_Stone(
                        player, [(x + 1 * xsign, y + 1 * ysign), (x + 2 * xsign, y + 2 * ysign)]
                    )

    def Check_Wrong_Routine(self, player, x, y, xsign, ysign):
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 3):
            if (
                self.Check_Is_In_Table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] != player
                and self.table[x + n * xsign][y + n * ysign] != 0
            ):
                uptrap = 1
            elif (
                self.Check_Is_In_Table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == player
            ):
                it += 1
            else:
                break
        for n in range(1, 3):
            if (
                self.Check_Is_In_Table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] != player
                and self.table[x - n * xsign][y - n * ysign] != 0
            ):
                downtrap = 1
            elif (
                self.Check_Is_In_Table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == player
            ):
                it += 1
            else:
                break
        if it <= 2 and uptrap == 1 and downtrap == 1:
            return 1
        return 0

    def Check_Win_Routine(self, player, x, y, xsign, ysign):
        it = 1
        for n in range(1, 18):
            if (
                self.Check_Is_In_Table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == player
            ):
                it += 1
            else:
                break
        for n in range(1, 18):
            if (
                self.Check_Is_In_Table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == player
            ):
                it += 1
            else:
                break
        if self.Win(player, it) == 1:
            return 1
        return 0

    def Check_Three_Routine(self, player, x, y, xsign, ysign):
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 4):
            if (
                self.Check_Is_In_Table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == 0
            ):
                uptrap = 1
            elif (
                self.Check_Is_In_Table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == player
            ):
                it += 1
            else:
                break
        for n in range(1, 4):
            if (
                self.Check_Is_In_Table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == 0
            ):
                downtrap = 1
            elif (
                self.Check_Is_In_Table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == player
            ):
                it += 1
            else:
                break
        # print("it:", it, "trap:", uptrap, downtrap)
        if it >= 3 and uptrap == 1 and downtrap == 1:
            return 1
        return 0
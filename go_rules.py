import subprocess


# Trying to import the gomoku_rust lib, if not compiled, execute the script to compile it.
try:
    import gomoku_rust
except ImportError:
    # Build rust lib
    process = subprocess.Popen("rust_compilation.sh", shell=True, stdout=subprocess.PIPE)
    process.wait()
    print(process.returncode)
    import gomoku_rust

from player import Player


class GoRules:
    table = []
    player_list = []

    def __init__(self):
        m = 19
        n = 19
        self.table = [[0] * m for i in range(n)]
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))
        print(self.player_list[0].nb, self.player_list[1].nb)

    def check_three_position(self, player, x, y):
        three_count = 0
        # updown solution
        if self.check_three_routine(player, x, y, 1, 0) == 1:
            three_count += 1
        # leftright solution
        if self.check_three_routine(player, x, y, 0, 1) == 1:
            three_count += 1
        # diagdownleft solution
        if self.check_three_routine(player, x, y, -1, 1) == 1:
            three_count += 1
        # diagupleft solution
        if self.check_three_routine(player, x, y, 1, 1) == 1:
            three_count += 1
        # print("threecount", three_count)
        if three_count < 2:
            return 0
        return 1

    def check_win_position(self, player, x, y):
        it = 0
        tmp = 0
        if self.table[x][y] != player:
            return 0
        # updown solution
        tmp = self.check_win_routine(player, x, y, 1, 0)
        if tmp > it:
            it = tmp
        # leftright solution
        tmp = self.check_win_routine(player, x, y, 0, 1)
        if tmp > it:
            it = tmp
        # diagdownleft solution
        tmp = self.check_win_routine(player, x, y, -1, 1)
        if tmp > it:
            it = tmp
        # diagupleft solution
        tmp = self.check_win_routine(player, x, y, 1, 1)
        if tmp > it:
            it = tmp
        return it

    def check_wrong_position(self, player, x, y):
        if self.check_is_in_table(x, y, 0, 0, 0) or self.table[x][y] != 0:
            return 1
        if self.check_three_position(player, x, y) != 0:
            return 1
        # updown solution
        elif self.check_wrong_routine(player, x, y, 1, 0) == 1:
            return 1
        # leftright solution
        elif self.check_wrong_routine(player, x, y, 0, 1) == 1:
            return 1
        # diagdownleft solution
        elif self.check_wrong_routine(player, x, y, -1, 1) == 1:
            return 1
        # diagupleft solution
        elif self.check_wrong_routine(player, x, y, 1, 1) == 1:
            return 1
        return 0

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
            print("python ", self.check_win_position(player.nb, x, y))
            print("RUST ", gomoku_rust.rust_calculating_move(self.table, player.nb, x, y))
            if self.check_win_position(player.nb, x, y) == 5:
                for p in self.player_list:
                    if p.nb == player.nb:
                        p.wining_position.append([x, y])
                return 0
            return 0

    def check_is_in_table(self, x, y, xsign, ysign, offset):
        if (
            x + offset * xsign > 18
            or x + offset * xsign < 0
            or y + offset * ysign > 18
            or y + offset * ysign < 0
        ):
            return 1
        return 0

    def print_game_status(self):
        for player in self.player_list:
            print("player: ", player.color, "")

    def reset_players(self):
        self.player_list.clear()
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))
        print("ici printing player list")
        print(self.player_list)

    def reset_game(self):
        self.reset_players()
        self.clear_board()

    def clear_board(self):
        for L in range(len(self.table)):
            for l in range((len(self.table[L]))):
                self.table[L][l] = 0
        self.print_table()

    def print_table(self):
        for line in self.table:
            print(line)

    def eat_position_routine(self, player, x, y, xsign, ysign):
        if (
            self.check_is_in_table(x, y, xsign, ysign, 1) == 0
            and self.table[x + 1 * xsign][y + 1 * ysign] != player.nb
            and self.table[x + 1 * xsign][y + 1 * ysign] != 0
        ):
            if (
                self.check_is_in_table(x, y, xsign, ysign, 2) == 0
                and self.table[x + 2 * xsign][y + 2 * ysign] == player.nb
            ):
                self.eat_stone(player, [(x + 1 * xsign, y + 1 * ysign)])
            elif (
                self.check_is_in_table(x, y, xsign, ysign, 2) == 0
                and self.table[x + 2 * xsign][y + 2 * ysign] != 0
            ):
                if (
                    self.check_is_in_table(x, y, xsign, ysign, 3) == 0
                    and self.table[x + 3 * xsign][y + 3 * ysign] == player.nb
                ):
                    self.eat_stone(
                        player, [(x + 1 * xsign, y + 1 * ysign), (x + 2 * xsign, y + 2 * ysign)]
                    )

    def check_wrong_routine(self, player, x, y, xsign, ysign):
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 3):
            if (
                self.check_is_in_table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] != player
                and self.table[x + n * xsign][y + n * ysign] != 0
            ):
                uptrap = 1
            elif (
                self.check_is_in_table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == player
            ):
                it += 1
            else:
                break
        for n in range(1, 3):
            if (
                self.check_is_in_table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] != player
                and self.table[x - n * xsign][y - n * ysign] != 0
            ):
                downtrap = 1
            elif (
                self.check_is_in_table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == player
            ):
                it += 1
            else:
                break
        if it <= 2 and uptrap == 1 and downtrap == 1:
            return 1
        return 0

    def check_win_routine(self, player, x, y, xsign, ysign):
        it = 1
        for n in range(1, 18):
            if (
                self.check_is_in_table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == player
            ):
                it += 1
            else:
                break
        for n in range(1, 18):
            if (
                self.check_is_in_table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == player
            ):
                it += 1
            else:
                break
        return it

    def check_three_routine(self, player, x, y, xsign, ysign):
        it = 1
        uptrap = 0
        downtrap = 0
        for n in range(1, 4):
            if (
                self.check_is_in_table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == 0
            ):
                uptrap = 1
            elif (
                self.check_is_in_table(x, y, xsign, ysign, n) == 0
                and self.table[x + n * xsign][y + n * ysign] == player
            ):
                it += 1
            else:
                break
        for n in range(1, 4):
            if (
                self.check_is_in_table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == 0
            ):
                downtrap = 1
            elif (
                self.check_is_in_table(x, y, -xsign, -ysign, n) == 0
                and self.table[x - n * xsign][y - n * ysign] == player
            ):
                it += 1
            else:
                break
        # print("it:", it, "trap:", uptrap, downtrap)
        if it >= 3 and uptrap == 1 and downtrap == 1:
            return 1
        return 0
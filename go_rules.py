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
    board = []
    player_list = []

    def __init__(self):
        m = 19
        n = 19
        self.board = [[0] * m for i in range(n)]
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))

    def place_stone(self, player, x, y):
        Rust_res = gomoku_rust.place_stone(self.board, player.nb, x, y)
        gomoku_rust.show_state(Rust_res["board"], player.nb, x, y)
        if Rust_res["game_status"] == -1:
            return -1
        else:
            self.board = Rust_res["board"]
            player.eat_piece += Rust_res["eated_piece"]
            if player.eat_piece >= 10:
                return player.nb
            if "wining_position" in Rust_res.keys():
                for p in self.player_list:
                    if p.nb == player.nb:
                        p.wining_position.append([x, y])
            for pl in self.player_list:
                if pl != player:
                    if pl.wining_position:
                        for position in pl.wining_position:
                            if (
                                gomoku_rust.check_win(self.board, pl.nb, position[0], position[1])
                                >= 5
                            ):
                                return pl.nb
                            else:
                                pl.wining_position.remove(position)
            return 0

    def print_game_status(self):
        for player in self.player_list:
            print("player: ", player.color, "")

    def reset_players(self):
        self.player_list.clear()
        self.player_list.append(Player(1, 0, "White"))
        self.player_list.append(Player(2, 0, "Black"))

    def reset_game(self):
        self.reset_players()
        self.clear_board()

    def clear_board(self):
        for L in range(len(self.board)):
            for l in range((len(self.board[L]))):
                self.board[L][l] = 0
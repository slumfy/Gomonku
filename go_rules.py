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
from global_var import PLAYER_BLACK_NB, PLAYER_WHITE_NB, PlayerType


class GoRules:
    board = []
    player_list = []
    ai_helper: bool = False
    ai_versus = 0
    wining_position = []

    def __init__(self, ai_helper: bool = False):
        self.ai_helper = ai_helper
        m = 19
        n = 19
        self.board = [[0] * m for i in range(n)]
        self.player_list.append(Player(PLAYER_WHITE_NB, PlayerType.HUMAN.value, "White"))
        self.player_list.append(Player(PLAYER_BLACK_NB, PlayerType.HUMAN.value, "Black"))

    def place_stone(self, player, x, y):
        Rust_res = gomoku_rust.place_stone(self.board, player.nb, x, y, self.wining_position)
        # print(Rust_res)
        if Rust_res["game_status"] != 0:
            return -2
        else:
            self.board = Rust_res["board"]
            player.eat_piece += Rust_res["stone_captured"]
            # gomoku_rust.show_state(Rust_res["board"], player.nb, x, y)
            # if self.ai_helper:
            #     gomoku_rust.negamax(Rust_res["board"], player.nb, x, y)
            if player.eat_piece >= 10:
                return player.nb
            if "wining_position" in Rust_res.keys():
                for p in self.player_list:
                    if p.nb == player.nb:
                        p.wining_position.append([x, y])
                self.wining_position.append(((x, y),player.nb))
            for pl in self.player_list:
                if pl != player:
                    if pl.wining_position:
                        for position in pl.wining_position:
                            if (
                                gomoku_rust.check_move_is_a_fiverow(self.board, pl.nb, position[0], position[1], self.wining_position) == True
                            ):
                                return pl.nb
                            else:
                                pl.wining_position.remove(position)
            return 0

    def AI_move(self, player, x, y, turn):
        print(player, x, y)
        opponant = -player.nb
        move = gomoku_rust.ai_move(self.board, opponant, x, y, turn, self.wining_position)
        print("AI: ", move)
        return move

    def print_game_status(self):
        for player in self.player_list:
            print("player: ", player.color, "")

    def reset_players(self):
        self.player_list.clear()
        self.player_list.append(Player(PLAYER_WHITE_NB, 0, "White"))
        self.player_list.append(Player(PLAYER_BLACK_NB, 0, "Black"))

    def reset_game(self):
        self.reset_players()
        self.clear_board()

    def clear_board(self):
        for L in range(len(self.board)):
            for l in range((len(self.board[L]))):
                self.board[L][l] = 0
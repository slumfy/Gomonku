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
from global_var import PLAYER_BLACK_NB, PLAYER_WHITE_NB,BOARD_NOTATION , PlayerType

class Move:
    board = []
    move = None
    player = 0
    eated_stone = 0
    def __init__(self, board, move, player, eated_stone):
            self.board = board
            self.move = move
            self.player = player
            self.eated_stone = eated_stone


class GoRules:
    board = []
    player_list = []
    ai_versus = 0
    move_list = []

    def __init__(self):
        m = 19
        n = 19
        self.board = [[0] * m for i in range(n)]
        self.player_list.append(Player(PLAYER_WHITE_NB, PlayerType.HUMAN.value, "White"))
        self.player_list.append(Player(PLAYER_BLACK_NB, PlayerType.HUMAN.value, "Black"))

    def place_stone(self, player, x, y):
        opponant = -player.nb
        winpos = (0, 0)
        for p in self.player_list:
            if p.nb == opponant:
                if p.wining_position[1] != 0:
                    winpos = p.wining_position
        Rust_res = gomoku_rust.place_stone(self.board, player.nb, x, y)
        # print(Rust_res)
        if Rust_res["game_status"] != 0:
            return -2
        else:
            self.move_list.append( Move(Rust_res["board"], (x*19+y), player, Rust_res["stone_captured"]) )
            self.print_move_list()
            self.board = Rust_res["board"]
            player.capture_piece += Rust_res["stone_captured"]
            player.nb_move_to_win = Rust_res["nb_move_to_win"]
            # gomoku_rust.show_state(Rust_res["board"], player.nb, x, y)
            if player.capture_piece >= 10:
                return player.nb
            if "wining_position" in Rust_res.keys():
                for p in self.player_list:
                    if p.nb == player.nb:
                        p.wining_position = Rust_res["wining_position"]
            for pl in self.player_list:
                if pl != player:
                    if pl.wining_position[1] != 0:
                        if (
                            gomoku_rust.check_move_is_still_winning(self.board, pl.wining_position)
                            == True
                        ):
                            print("true")
                            return pl.nb
                        else:
                            pl.wining_position = (0, 0)
            return 0

    def AI_move(self, player, x, y, turn, display_ai_time: bool, search_algorithm: str, depth):
        print("player nb = ", player.nb, "x = ", x, "y = ", y)
        winpos = (0, 0)
        for p in self.player_list:
            if p.wining_position[1] != 0:
                winpos = p.wining_position
                print("AI WINPOS", winpos)
        move = gomoku_rust.ai_move(
            self.board,
            player.nb,
            x,
            y,
            turn,
            winpos,
            player.nb_move_to_win,
            display_ai_time,
            search_algorithm,
            depth,
        )
        print("AI: ", move)

        return move

    def print_search_box(self, player, x, y, turn):
        box = gomoku_rust.get_rust_box(self.board)
        return box

    def print_game_status(self):
        for player in self.player_list:
            print("player: ", player.color, "")

    def reset_players(self):
        player_type = PlayerType.HUMAN.value
        if self.player_list[1].player_type == PlayerType.AI.value:
            player_type = PlayerType.AI.value
        self.player_list.clear()
        self.player_list.append(Player(PLAYER_WHITE_NB, PlayerType.HUMAN.value, "White"))
        if player_type == PlayerType.AI.value:
            self.player_list.append(Player(PLAYER_BLACK_NB, PlayerType.AI.value, "Black"))
        else:
            self.player_list.append(Player(PLAYER_BLACK_NB, PlayerType.HUMAN.value, "Black"))

    def reset_game(self):
        self.move_list = []
        self.reset_players()
        self.clear_board()
        gomoku_rust.reset_game()

    def clear_board(self):
        for L in range(len(self.board)):
            for l in range((len(self.board[L]))):
                self.board[L][l] = 0

    def previous_move(self):
        move_list_len = len(self.move_list)
        if move_list_len != 0:
            move_to_remove = self.move_list[move_list_len - 1]
            offset = 2
            for player in self.player_list:
                if player.player_type == 1:
                    offset = 3
                if player == move_to_remove.player:
                    player.capture_piece -= move_to_remove.eated_stone
            if move_list_len == 0 or move_list_len == 1 or (offset == 3 and move_list_len <= 2):
                self.move_list = []
                self.reset_game()
                return 0, 0
            move = self.move_list[move_list_len - offset]
            self.board = move.board
            if offset == 2:
                self.move_list.pop(-1)
            else:
                self.move_list.pop(-1)
                self.move_list.pop(-1)
            # self.print_move_list()
            return move.move // 19, move.move % 19
        return 0, 0

    def print_move_list(self):
        print("list of move(" + str(len(self.move_list)) +"): ", end="")
        for move in self.move_list:
            x = move.move // 19
            y = move.move % 19
            print(BOARD_NOTATION[0][x] + BOARD_NOTATION[1][y] + ", ", end="")
        print()
class BoardState:
    board = []
    available_move = []
    player_to_play = []
    win_state = 0
    heuristic = 0

    def __init__(self, go, player):
        self.board = go.table
        self.player = player
        self.available_move = (go, player)

    @property
    def available_move(self):
        return self._available_move

    @available_move.setter
    def available_move(self, arglist):
        self._available_move = []
        try:
            go, player = arglist
        except ValueError:
            raise ValueError("Pass an iterable with two items")
        for L in range(len(self.board)):
            for l in range(len(self.board[L])):
                if go.check_wrong_position(player, L, l) == 0:
                    self._available_move.append([L, l])

    def heuristic(self, go):
        # self.heuristic += diff nb piece player - adversaire
        # self.heuristic += alignement max
        # self.heuristic += player eat piece
        pass
class Player:
    nb = 0
    player_type = 0
    capture_piece = 0
    color = None

    def __init__(self, nb, player_type, color):
        self.nb = nb
        self.player_type = player_type
        self.capture_piece = 0
        self.nb_move_to_win = 5
        self.color = color
        self.wining_position = (0,0)

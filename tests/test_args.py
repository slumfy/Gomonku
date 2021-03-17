from go_rules import GoRules
from py_game_go import PyGameGo
from player import Player

def test_tests():
    go_rules = GoRules()
    game = PyGameGo(sound_status=False, test_mode=True)
    game.player = go_rules.player_list[0]
    state = go_rules.place_stone(game.player, 0, 0)
    assert state == 0
    game.player = go_rules.player_list[1]
    # Placing a piece in a good pos but already taken.
    state = go_rules.place_stone(game.player, 0, 0)
    assert state == -1
import pytest

from go_rules import GoRules
from py_game_go import PyGameGo
from player import Player


@pytest.fixture
def white_winning_state() -> (GoRules, PyGameGo):
    go_rules = GoRules()
    game = PyGameGo(sound_status=False, test_mode=True)
    for i in range(4):
        game.player = go_rules.player_list[0]
        go_rules.place_stone(game.player, 10, i)
        game.player = go_rules.player_list[1]
        go_rules.place_stone(game.player, 11, i)
    game.player = go_rules.player_list[0]
    go_rules.place_stone(game.player, 10, i + 1)

    return go_rules, game


@pytest.fixture
def black_winning_state() -> (GoRules, PyGameGo):
    go_rules = GoRules()
    game = PyGameGo(sound_status=False, test_mode=True)
    for i in range(4):
        game.player = go_rules.player_list[0]
        go_rules.place_stone(game.player, 10, i)
        game.player = go_rules.player_list[1]
        go_rules.place_stone(game.player, 11, i)
    game.player = go_rules.player_list[0]
    go_rules.place_stone(game.player, 9, 0)
    game.player = go_rules.player_list[1]
    go_rules.place_stone(game.player, 11, 4)

    return go_rules, game


def test_white_win(white_winning_state):
    go_rules, game = white_winning_state
    game.player = go_rules.player_list[1]
    state = go_rules.place_stone(game.player, 11, 4)
    assert state == go_rules.player_list[0].nb


def test_black_win(black_winning_state):
    go_rules, game = black_winning_state
    game.player = go_rules.player_list[0]
    state = go_rules.place_stone(game.player, 10, 4)
    assert state == go_rules.player_list[1].nb


@pytest.mark.parametrize("x", [0, 1, 2, 3, 4])
def test_black_prevent_white_win(white_winning_state, x):
    go_rules, game = white_winning_state
    game.player = go_rules.player_list[1]
    state = go_rules.place_stone(game.player, 9, x)
    assert state == 0


@pytest.mark.parametrize("x", [0, 1, 2, 3, 4])
def test_white_prevent_black_win(black_winning_state, x):
    go_rules, game = black_winning_state
    game.player = go_rules.player_list[0]
    state = go_rules.place_stone(game.player, 12, x)
    assert state == 0

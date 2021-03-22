import pytest
import time
from hypothesis import strategies as st
from hypothesis import given

from go_rules import GoRules
from py_game_go import PyGameGo
from player import Player
from tests.gui_from_board import GuiFromBoard


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


@pytest.mark.parametrize("x", [7, 8, 9, 10, 11, 12, 13])
@pytest.mark.parametrize("y", [8, 9, 10, 11, 12])
def test_white_prevent_black_win_diagonal(x, y):
    gui_from_board = GuiFromBoard()

    go_rules = GoRules()
    go_rules.player_list[1].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
    go_rules.board[3] = [2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0]
    go_rules.player_list[0].wining_position.append([2, y])

    game.player = go_rules.player_list[1]
    state = go_rules.place_stone(game.player, 1, x)

    # # Print the gui moves
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.5)

    if x == 7 or x == 9 or x == 11 or x == 13:
        assert state == 1
    else:
        assert state == 0
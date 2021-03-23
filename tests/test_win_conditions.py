import pytest
import time

from go_rules import GoRules
from py_game_go import PyGameGo
from player import Player
from tests.gui_from_board import GuiFromBoard
from global_var import PLAYER_BLACK_NB, PLAYER_WHITE_NB


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
    assert state == PLAYER_WHITE_NB


def test_black_win(black_winning_state):
    go_rules, game = black_winning_state
    game.player = go_rules.player_list[0]
    state = go_rules.place_stone(game.player, 10, 4)
    assert state == PLAYER_BLACK_NB


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


@pytest.mark.parametrize("y", [7, 8, 9, 10, 11, 12, 13])
@pytest.mark.parametrize("winning_pos_y", [8, 9, 10, 11, 12])
def test_white_prevent_black_win_diagonal(y, winning_pos_y):

    go_rules = GoRules()
    go_rules.player_list[1].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
    go_rules.board[3] = [-1, -1, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0]
    go_rules.player_list[0].wining_position.append([2, winning_pos_y])

    game.player = go_rules.player_list[1]
    state = go_rules.place_stone(game.player, 1, y)

    # # Print the gui moves
    # gui_from_board = GuiFromBoard()
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.5)

    if y == 7 or y == 9 or y == 11 or y == 13:
        assert state == 1
    else:
        assert state == 0


@pytest.mark.parametrize("y", [7, 8, 9, 10, 11, 12, 13])
@pytest.mark.parametrize("winning_pos_y", [8, 9, 10, 11, 12])
def test_black_prevent_white_win_diagonal(y, winning_pos_y):

    go_rules = GoRules()
    go_rules.player_list[0].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [0, 0, 0, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0]
    go_rules.board[3] = [1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1]
    go_rules.player_list[1].wining_position.append([2, winning_pos_y])

    game.player = go_rules.player_list[0]
    state = go_rules.place_stone(game.player, 1, y)

    # # Print the gui moves
    # gui_from_board = GuiFromBoard()
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.5)

    if y == 7 or y == 9 or y == 11 or y == 13:
        assert state == PLAYER_BLACK_NB
    else:
        assert state == 0


def test_white_eat_win():

    go_rules = GoRules()
    go_rules.player_list[1].eat_piece = 0
    go_rules.player_list[0].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1]
    go_rules.board[3] = [-1, -1, -1, -1, 0, -1, -1, -1, -1, 0, -1, -1, -1, -1, 0, -1, -1, -1, -1]

    state = 0
    for l in range(4):
        if state == PLAYER_WHITE_NB:
            break
        for i in range(4):
            game.player = go_rules.player_list[0]
            state = go_rules.place_stone(game.player, 4, i + l * 5)
            if state == PLAYER_WHITE_NB:
                assert l == 2
                assert i == 0
                assert go_rules.player_list[0].eat_piece == 10
                break
            game.player = go_rules.player_list[1]
            go_rules.place_stone(game.player, 10, i + l * 5)
            # # Print the gui moves
            # gui_from_board = GuiFromBoard()
            # gui_from_board.display_current_state(go_rules.board)
            # time.sleep(0.4)
    # # Print the gui final moves
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.4)


def test_black_eat_win():

    go_rules = GoRules()
    go_rules.player_list[1].eat_piece = 0
    go_rules.player_list[0].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 2, 2, 2, 2]
    go_rules.board[3] = [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1]

    state = 0
    for l in range(4):
        if state == PLAYER_BLACK_NB:
            break
        for i in range(4):
            game.player = go_rules.player_list[0]
            go_rules.place_stone(game.player, 10, i + l * 5)
            game.player = go_rules.player_list[1]
            state = go_rules.place_stone(game.player, 4, i + l * 5)
            if state == PLAYER_BLACK_NB:
                assert l == 2
                assert i == 0
                assert go_rules.player_list[1].eat_piece == 10
                break
            # # Print the gui moves
            # gui_from_board = GuiFromBoard()
            # gui_from_board.display_current_state(go_rules.board)
            # time.sleep(0.4)
    # # Print the gui final moves
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.4)


def test_white_eat_more_than_10_win():

    go_rules = GoRules()
    go_rules.player_list[1].eat_piece = 0
    go_rules.player_list[0].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1]
    go_rules.board[3] = [2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 2, 2, 2]

    state = 0
    for l in range(4):
        if state == PLAYER_WHITE_NB:
            break
        for i in range(4):
            game.player = go_rules.player_list[0]
            state = go_rules.place_stone(game.player, 4, i + l * 5)
            if state == PLAYER_WHITE_NB:
                assert l == 3
                assert i == 0
                assert go_rules.player_list[0].eat_piece == 11
                break
            game.player = go_rules.player_list[1]
            go_rules.place_stone(game.player, 10, i + l * 5)
            # # Print the gui moves
            # gui_from_board = GuiFromBoard()
            # gui_from_board.display_current_state(go_rules.board)
            # time.sleep(0.4)
    # # Print the gui final moves
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.4)


def test_black_eat_more_than_10_win():

    go_rules = GoRules()
    go_rules.player_list[1].eat_piece = 0
    go_rules.player_list[0].eat_piece = 0
    game = PyGameGo(sound_status=False, test_mode=True)
    go_rules.board[2] = [2, 2, 2, 2, 0, 2, 2, 2, 2, 0, 2, 0, 0, 0, 0, 2, 2, 2, 2]
    go_rules.board[3] = [1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1]

    state = 0
    for l in range(4):
        if state == PLAYER_BLACK_NB:
            break
        for i in range(4):
            game.player = go_rules.player_list[0]
            go_rules.place_stone(game.player, 10, i + l * 5)
            game.player = go_rules.player_list[1]
            state = go_rules.place_stone(game.player, 4, i + l * 5)
            if state == PLAYER_BLACK_NB:
                assert l == 3
                assert i == 0
                assert go_rules.player_list[1].eat_piece == 11
                break
    #         # Print the gui moves
    #         gui_from_board = GuiFromBoard()
    #         gui_from_board.display_current_state(go_rules.board)
    #         time.sleep(0.4)
    # # Print the gui final moves
    # gui_from_board.display_current_state(go_rules.board)
    # time.sleep(0.4)
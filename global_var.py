from enum import Enum


# DISPLAY
BOARD_NOTATION = (
    (
        "19",
        "18",
        "17",
        "16",
        "15",
        "14",
        "13",
        "12",
        "11",
        "10",
        "9",
        "8",
        "7",
        "6",
        "5",
        "4",
        "3",
        "2",
        "1",
    ),
    (
        "A",
        "B",
        "C",
        "D",
        "E",
        "F",
        "G",
        "H",
        "I",
        "J",
        "K",
        "L",
        "M",
        "N",
        "O",
        "P",
        "Q",
        "R",
        "S",
    ),
)

MAIN_WINDOW_SIZE = (720, 720)
BOARD_SIZE = (720, 720)
STONE_SIZE = (32, 32)

# PLAYER GLOBAL VAR
PLAYER_WHITE_NB = 1
PLAYER_BLACK_NB = -1


class PlayerType(Enum):
    HUMAN = 0
    AI = 1
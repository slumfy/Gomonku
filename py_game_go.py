import sys
import pygame
import math

from go_rules import GoRules
from player import Player
from utils import logger_factory
from global_var import (
    BOARD_NOTATION,
    MAIN_WINDOW_SIZE,
    STONE_SIZE,
    PLAYER_WHITE_NB,
    PLAYER_BLACK_NB,
    PlayerType,
)


class PyGameGo:
    def __init__(
        self,
        sound_status: bool = False,
        test_mode: bool = False,
        search_box_status: bool = False,
        display_ai_time: bool = False,
        search_algorithm: str = "negamax",
    ):
        self.test_mode = test_mode
        self.display_ai_time = display_ai_time
        self.search_algorithm = search_algorithm

        self.logger = logger_factory("PyGameGo")
        self.moves_count = 0
        # Creating GUI and sound
        if not self.test_mode:
            pygame.init()
            pygame.mixer.init()
            pygame.mixer.music.load("ressources/sound/bensound-thejazzpiano.mp3")
            self.sound_status = sound_status
            self.search_box_status = search_box_status
            self.placing_stone_sound = pygame.mixer.Sound("ressources/sound/MOVE.wav")
            self.sound_icon_size = width, height = 32, 32
            self.reset_icon_size = width, height = 32, 32
            self.return_icon_size = width, height = 32, 32
            self.screen = pygame.display.set_mode(size=MAIN_WINDOW_SIZE)
            self.theme_number = 1
            self.go_board = pygame.image.load("ressources/images/goboard-1.png")
            self.go_menu = pygame.image.load("ressources/images/gomenu-1.png")
            self.go_settings = pygame.image.load("ressources/images/gosettings-1.png")
            self.go_sound_off = pygame.transform.scale(
                pygame.image.load("ressources/images/sound-icon/sound_off.png"),
                self.sound_icon_size,
            )
            self.go_sound_on = pygame.transform.scale(
                pygame.image.load("ressources/images/sound-icon/sound_on.png"), self.sound_icon_size
            )
            self.reset_on = pygame.transform.scale(
                pygame.image.load("ressources/images/reset-icon.png"),
                self.reset_icon_size,
            )
            self.return_on = pygame.transform.scale(
                pygame.image.load("ressources/images/return-icon.png"),
                self.return_icon_size,
            )
            self.go_board_resize = pygame.transform.scale(self.go_board, MAIN_WINDOW_SIZE)
            self.start_point = [0, 0]
            self.white_stone = pygame.image.load("ressources/images/whitecircle.png")
            self.white_stone_resize = pygame.transform.scale(self.white_stone, STONE_SIZE)
            self.black_stone = pygame.image.load("ressources/images/blackcircle.png")
            self.black_stone_resize = pygame.transform.scale(self.black_stone, STONE_SIZE)
            self.grey_stone = pygame.image.load("ressources/images/greycircle.png")
            self.grey_stone_resize = pygame.transform.scale(self.grey_stone, STONE_SIZE)
        self.player: Player = None

    def update_sound_status(self, sound_status: bool, background_page):
        self.sound_status = sound_status
        if self.sound_status is True:
            pygame.mixer.music.set_volume(0.05)
            pygame.mixer.music.play(-1)
        else:
            pygame.mixer.music.stop()

    def display_page(self, background_page, return_button=False, sound_status=False):
        self.screen.blit(background_page, self.start_point)

        if return_button:
            self.screen.blit(self.return_on, (0, 0))
        if sound_status:
            if self.sound_status is True:
                self.screen.blit(
                    self.go_sound_on, (MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0], 0)
                )
            else:
                self.screen.blit(
                    self.go_sound_off,
                    (MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0], 0),
                )
        pygame.display.flip()

    def display_setting_page(self):
        self.display_page(background_page=self.go_settings, return_button=True, sound_status=True)
        self.print_font(
            64,
            "NEXT THEME",
            200,
            150,
            "Black",
        )
        pygame.display.flip()

    def settings(self):
        self.display_setting_page()

        while 1:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    # Return to menu button
                    if (
                        event.pos[1] >= 0
                        and event.pos[1] <= self.return_icon_size[0]
                        and event.pos[0] >= 0
                        and event.pos[0] <= self.return_icon_size[1]
                    ):
                        return 0
                    # Click on NEXT THEME button
                    elif event.pos[1] >= 125 and event.pos[1] < 205:
                        self.theme_number += 1
                        if self.theme_number > 2:
                            self.theme_number = 1
                        self.go_board = pygame.image.load(
                            "ressources/images/goboard-" + str(self.theme_number) + ".png"
                        )
                        self.go_menu = pygame.transform.scale(
                            pygame.image.load(
                                "ressources/images/gomenu-" + str(self.theme_number) + ".png"
                            ),
                            MAIN_WINDOW_SIZE,
                        )
                        self.go_settings = pygame.transform.scale(
                            pygame.image.load(
                                "ressources/images/gosettings-" + str(self.theme_number) + ".png"
                            ),
                            MAIN_WINDOW_SIZE,
                        )
                        self.go_board_resize = pygame.transform.scale(
                            self.go_board, MAIN_WINDOW_SIZE
                        )
                        self.display_setting_page()
                    # Click on sound icon
                    elif (
                        event.pos[1] <= self.sound_icon_size[1]
                        and event.pos[1] >= 0
                        and event.pos[0] >= MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0]
                        and event.pos[0] <= MAIN_WINDOW_SIZE[0]
                    ):
                        self.update_sound_status(not self.sound_status, self.go_settings)
                        self.display_setting_page()

    def menu(self, go_rules: GoRules):
        self.screen.blit(self.go_menu, self.start_point)
        self.update_sound_status(self.sound_status, self.go_menu)
        pygame.display.flip()
        self.display_page(background_page=self.go_menu, return_button=False, sound_status=True)
        self.player = go_rules.player_list[0]
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    # Click on playing AI vs AI button
                    if event.pos[1] <= 505 and event.pos[1] >= 430:
                        print("AI vs AI clicked.")
                        go_rules.player_list[0].player_type = 1
                        go_rules.player_list[1].player_type = 1
                        go_rules.ai_versus = 1
                        self.playing(go_rules=go_rules)
                    # Click on playing human vs AI button
                    if event.pos[1] <= 585 and event.pos[1] > 505:
                        print("Human vs AI clicked.")
                        go_rules.player_list[1].player_type = PlayerType.AI.value
                        self.playing(go_rules=go_rules)
                    # Click on playing human vs human button
                    if event.pos[1] <= 665 and event.pos[1] > 585:
                        print("Human vs Human clicked.")
                        go_rules.player_list[1].player_type = PlayerType.HUMAN.value
                        self.playing(go_rules=go_rules)
                    # Click on settings
                    if event.pos[1] <= 765 and event.pos[1] > 665:
                        print("Settings clicked.")
                        go_rules.player_list[1].player_type = PlayerType.HUMAN.value
                        self.settings()
                    self.display_page(
                        background_page=self.go_menu, return_button=False, sound_status=True
                    )

                    # Click on sound icon
                    if (
                        event.pos[1] <= self.sound_icon_size[1]
                        and event.pos[1] >= 0
                        and event.pos[0] >= MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0]
                        and event.pos[0] <= MAIN_WINDOW_SIZE[0]
                    ):
                        self.screen.blit(self.go_menu, self.start_point)
                        self.update_sound_status(not self.sound_status, self.go_menu)
                        self.display_page(
                            background_page=self.go_menu, return_button=False, sound_status=True
                        )

    def win(self, go_rules: GoRules):
        self.moves_count = 0
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    go_rules.reset_game()
                    self.menu(go_rules)

    def print_illegal_move(self):
        self.print_font(
            32,
            "Turn :  "
            + str(math.floor((self.moves_count + 1) / 2))
            + "  "
            + self.player.color
            + "  previous move: "
            + "Illegal move",
            64,
            680,
            "Red",
        )

    def print_player_move(self, x: int, y: int):
        self.print_font(
            32,
            "Turn :  "
            + str(math.floor((self.moves_count + 1) / 2))
            + "  "
            + self.player.color
            + "  previous move: "
            + BOARD_NOTATION[0][x]
            + BOARD_NOTATION[1][y],
            64,
            680,
            "Black",
        )

    def print_capture_count(self, white_capture_count: int, black_capture_count: int):
        self.print_font(
            32,
            "White capture count :  "
            + str(white_capture_count)
            + "  "
            + "Black capture count :   "
            + str(black_capture_count),
            64,
            660,
            "Black",
        )

    def playing(self, go_rules: GoRules):
        self.screen.blit(self.go_board_resize, self.start_point)
        self.screen.blit(self.reset_on, (MAIN_WINDOW_SIZE[0] - self.reset_icon_size[0], 0))
        self.screen.blit(self.return_on, (0, 0))
        pygame.display.flip()

        x, y = 0, 0
        self.print_capture_count(
            white_capture_count=go_rules.player_list[0].capture_piece,
            black_capture_count=go_rules.player_list[1].capture_piece,
        )
        self.board_screen_blit(go_rules, 33, 62)

        while 1:
            self.screen.blit(self.reset_on, (MAIN_WINDOW_SIZE[0] - self.reset_icon_size[0], 0))
            self.screen.blit(self.return_on, (0, 0))
            pygame.display.flip()
            win_status = 0
            if self.player.player_type == PlayerType.AI.value:
                self.screen.blit(self.go_board_resize, self.start_point)
                AI_move = go_rules.AI_move(
                    self.player, x, y, self.moves_count, self.display_ai_time, self.search_algorithm
                )
                x, y = AI_move[0]
                stone_status = go_rules.place_stone(self.player, x, y)
                self.play_piece(go_rules, stone_status, win_status, x, y)
            else:
                for event in pygame.event.get():
                    if event.type == pygame.QUIT:
                        sys.exit()
                    if event.type == pygame.MOUSEBUTTONDOWN:
                        # Reset game button
                        if (
                            event.pos[1] <= self.sound_icon_size[1]
                            and event.pos[1] >= 0
                            and event.pos[0] >= MAIN_WINDOW_SIZE[0] - self.reset_icon_size[0]
                            and event.pos[0] <= MAIN_WINDOW_SIZE[0]
                        ):
                            x, y = self.reset_button(go_rules)
                        # Return to menu button
                        elif (
                            event.pos[1] >= 0
                            and event.pos[1] <= self.return_icon_size[0]
                            and event.pos[0] >= 0
                            and event.pos[0] <= self.return_icon_size[1]
                        ):
                            return 0
                        else:
                            self.screen.blit(self.go_board_resize, self.start_point)
                            x = self.mouse_pos_to_piece_pos(event.pos[1], 33, 62)
                            y = self.mouse_pos_to_piece_pos(event.pos[0], 33, 62)
                            stone_status = go_rules.place_stone(self.player, x, y)
                            self.play_piece(go_rules, stone_status, win_status, x, y)

    def play_piece(self, go_rules, stone_status, win_status, x, y):
        if stone_status == -2:
            self.print_illegal_move()
            self.print_capture_count(
                white_capture_count=go_rules.player_list[0].capture_piece,
                black_capture_count=go_rules.player_list[1].capture_piece,
            )
        elif stone_status == 0:
            if self.sound_status:
                self.placing_stone_sound.play()
            if self.player.nb == PLAYER_WHITE_NB:
                self.player = go_rules.player_list[1]
            elif self.player.nb == PLAYER_BLACK_NB:
                self.player = go_rules.player_list[0]
            self.moves_count += 1
            self.print_player_move(x=x, y=y)
            self.print_capture_count(
                white_capture_count=go_rules.player_list[0].capture_piece,
                black_capture_count=go_rules.player_list[1].capture_piece,
            )
        else:
            win_status = stone_status
            self.print_player_move(x=x, y=y)
            self.print_capture_count(
                white_capture_count=go_rules.player_list[0].capture_piece,
                black_capture_count=go_rules.player_list[1].capture_piece,
            )
        self.board_screen_blit(go_rules, 33, 62)
        if self.search_box_status == True:
            self.print_box(go_rules, self.player, x, y, self.moves_count)
        if win_status != 0:
            for pl in go_rules.player_list:
                if pl.nb == win_status:
                    color = pl.color
                    self.print_font(132, "player " + str(stone_status) + " win", 100, 300, color)
        pygame.display.flip()
        if win_status != 0:
            self.win(go_rules=go_rules)

    def mouse_pos_to_piece_pos(self, pos, space, offset):
        var = int((pos - offset) / space)
        if var >= 19:
            return 18
        elif var < 0:
            return 0

        mod = int((pos - offset) % space)
        if mod > space / 2 and pos > 65 and var < 18:
            var += 1
        return var

    def board_screen_blit(self, go_rules: GoRules, space, offset):
        for L in range(len(go_rules.board)):
            for l in range(len(go_rules.board[L])):
                if go_rules.board[L][l] == 1:
                    self.screen.blit(
                        self.white_stone_resize,
                        (
                            l * space + offset - STONE_SIZE[0] / 2,
                            L * space + offset - STONE_SIZE[1] / 2,
                        ),
                    )
                elif go_rules.board[L][l] == -1:
                    self.screen.blit(
                        self.black_stone_resize,
                        (
                            l * space + offset - STONE_SIZE[0] / 2,
                            L * space + offset - STONE_SIZE[1] / 2,
                        ),
                    )

    def print_font(self, size, msg, x, y, color):
        BLACK = (0, 0, 0)
        WHITE = (255, 255, 255)
        RED = (218, 56, 21)
        sysfont = pygame.font.get_default_font()
        font = pygame.font.SysFont(None, size)
        if color == "Black":
            render = font.render(msg, True, BLACK)
        elif color == "Red":
            render = font.render(msg, True, RED)
        else:
            render = font.render(msg, True, WHITE)
        self.screen.blit(render, (x, y))

    def reset_button(self, go_rules):
        go_rules.reset_game()
        self.moves_count = 0
        self.player = go_rules.player_list[0]
        self.screen.blit(self.go_board_resize, self.start_point)
        self.screen.blit(self.reset_on, (MAIN_WINDOW_SIZE[0] - self.reset_icon_size[0], 0))
        pygame.display.flip()
        return 0, 0

    def print_box(self, go_rules, player, x, y, turn):
        space, offset = 33, 62
        box = go_rules.print_search_box(self.player, x, y, self.moves_count)
        for pos in box:
            if go_rules.board[pos[0]][pos[1]] == 0:
                self.screen.blit(
                    self.grey_stone_resize,
                    (
                        pos[1] * space + offset - STONE_SIZE[0] / 2,
                        pos[0] * space + offset - STONE_SIZE[1] / 2,
                    ),
                )
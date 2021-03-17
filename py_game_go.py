import sys
import pygame

from go_rules import GoRules
from board_state import BoardState
from player import Player
from utils import logger_factory
from global_var import BOARD_NOTATION, MAIN_WINDOW_SIZE, STONE_SIZE


# rust_response = {
# 	"board" = [],
# 	"sugest_move" = (0,0),
# 	"eated_piece" = 0,
# 	"game_status" = 0,
# 	"player_to_play" = 0,
# 	"wining_player" = 0
# 	"wining_position" = 0
# }

class PyGameGo:
    def __init__(self, sound_status=bool):
        pygame.init()
        pygame.mixer.init()
        pygame.mixer.music.load("ressources/sound/bensound-thejazzpiano.mp3")

        self.logger = logger_factory("PyGameGo")

        self.sound_status = sound_status
        self.placing_stone_sound = pygame.mixer.Sound("ressources/sound/MOVE.wav")

        self.sound_icon_size = width, height = 32, 32
        self.player: Player = None

        self.screen = pygame.display.set_mode(size=MAIN_WINDOW_SIZE)

        self.go_board = pygame.image.load("ressources/images/goboard.png")
        self.go_menu = pygame.image.load("ressources/images/gomenu.png")
        self.go_sound_off = pygame.transform.scale(
            pygame.image.load("ressources/images/sound-icon/sound_off.png"), self.sound_icon_size
        )
        self.go_sound_on = pygame.transform.scale(
            pygame.image.load("ressources/images/sound-icon/sound_on.png"), self.sound_icon_size
        )
        self.go_board_resize = pygame.transform.scale(self.go_board, MAIN_WINDOW_SIZE)
        self.start_point = [0, 0]
        self.white_stone = pygame.image.load("ressources/images/whitecircle.png")
        self.white_stone_resize = pygame.transform.scale(self.white_stone, STONE_SIZE)
        self.black_stone = pygame.image.load("ressources/images/blackcircle.png")
        self.black_stone_resize = pygame.transform.scale(self.black_stone, STONE_SIZE)

    def update_sound_status(self, sound_status: bool):
        self.sound_status = sound_status
        if self.sound_status is True:
            pygame.mixer.music.set_volume(0.05)
            pygame.mixer.music.play(-1)
            self.screen.blit(self.go_sound_on, (MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0], 0))
            self.logger.info("Sound is on.")
        else:
            self.screen.blit(self.go_sound_off, (MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0], 0))
            pygame.mixer.music.stop()
            self.logger.info("Sound is off.")
        pygame.display.flip()

    def menu(self, go_rules: GoRules):
        self.screen.blit(self.go_menu, self.start_point)
        self.update_sound_status(self.sound_status)
        pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    # click on playing button
                    if event.pos[1] <= 585 and event.pos[1] >= 505:
                        self.playing(go_rules=go_rules)
                    # click on sound icon
                    if (
                        event.pos[1] <= self.sound_icon_size[1]
                        and event.pos[1] >= 0
                        and event.pos[0] >= MAIN_WINDOW_SIZE[0] - self.sound_icon_size[0]
                        and event.pos[0] <= MAIN_WINDOW_SIZE[0]
                    ):
                        self.screen.blit(self.go_menu, self.start_point)
                        self.update_sound_status(not self.sound_status)

    def win(self, go_rules: GoRules):
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
            "Player Turn: " + self.player.color + "  previous move: " + "Illegal move",
            64,
            32,
            "Black",
        )

    def print_player_move(self, x: int, y: int):
        self.print_font(
            32,
            "Player Turn: "
            + self.player.color
            + "  previous move: "
            + BOARD_NOTATION[0][x]
            + BOARD_NOTATION[1][y],
            64,
            32,
            "Black",
        )

    def playing(self, go_rules: GoRules):
        self.screen.blit(self.go_board_resize, self.start_point)
        pygame.display.flip()
        self.player = go_rules.player_list[0]
        while 1:
            win_status = 0
            for pos in self.player.wining_position:
                print("player: ", self.player.color, "pos: ", self.player.wining_position)
                if go_rules.check_win_position(self.player.nb, pos[0], pos[1]) == 5:
                    win_status = 1
                else:
                    self.player.wining_position.remove(pos)
            if win_status == 5:
                self.print_font(
                    132, "player " + str(self.player.nb) + " win", 100, 300, self.player.color
                )
                pygame.display.flip()
                self.win(go_rules=go_rules)
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.screen.blit(self.go_board_resize, self.start_point)
                    x = self.mouse_pos_to_piece_pos(event.pos[1], 33, 62)
                    y = self.mouse_pos_to_piece_pos(event.pos[0], 33, 62)
                    stone_status = go_rules.place_stone(self.player, x, y)
                    # test state
                    state = BoardState(go_rules, self.player)
                    print("STATEMOVE: ")
                    print(state.available_move)
                    # end test
                    if stone_status == -1:
                        self.print_illegal_move()
                    elif stone_status == 0:
                        if self.sound_status:
                            self.placing_stone_sound.play()
                        if self.player.nb == 1:
                            self.player = go_rules.player_list[1]
                        elif self.player.nb == 2:
                            self.player = go_rules.player_list[0]
                        self.print_player_move(x=x, y=y)
                    else:
                        win_status = 1
                    self.board_screen_blit(go_rules, 33, 62)
            if win_status != 0:
                self.print_font(
                    132, "player " + str(self.player.nb) + " win", 100, 300, self.player.color
                )
            pygame.display.flip()
            if win_status != 0:
                self.win(go_rules=go_rules)

    def mouse_pos_to_piece_pos(self, pos, space, offset):
        var = int((pos - offset) / space)
        mod = int((pos - offset) % space)
        if mod > space / 2 and var < 18:
            var += 1
        return var

    def board_screen_blit(self, go_rules: GoRules, space, offset):
        for L in range(len(go_rules.table)):
            for l in range(len(go_rules.table[L])):
                if go_rules.table[L][l] == 1:
                    self.screen.blit(
                        self.white_stone_resize,
                        (
                            l * space + offset - STONE_SIZE[0] / 2,
                            L * space + offset - STONE_SIZE[1] / 2,
                        ),
                    )
                elif go_rules.table[L][l] == 2:
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
        sysfont = pygame.font.get_default_font()
        font = pygame.font.SysFont(None, size)
        if color == "Black":
            render = font.render(msg, True, BLACK)
        else:
            render = font.render(msg, True, WHITE)
        self.screen.blit(render, (x, y))

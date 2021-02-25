import sys

import pygame


class Game:
    map_notation = [
        [
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
        ],
        [
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
        ],
    ]

    def __init__(self):
        pygame.init()
        self.size = width, height = 720, 720
        self.board_size = width, height = 720, 720
        self.stone_size = width, height = 24, 24
        self.player = []

        self.screen = pygame.display.set_mode(self.size)

        self.goboard = pygame.image.load("ressources/images/goboard.png")
        self.gomenu = pygame.image.load("ressources/images/gomenu.png")
        self.goboard_resize = pygame.transform.scale(self.goboard, self.size)
        self.startpoint = [0, 0]
        self.white_stone = pygame.image.load("ressources/images/whitecircle.png")
        self.white_stone_resize = pygame.transform.scale(self.white_stone, self.stone_size)
        self.black_stone = pygame.image.load("ressources/images/blackcircle.png")
        self.black_stone_resize = pygame.transform.scale(self.black_stone, self.stone_size)

    def menu(self, go):
        self.screen.blit(self.gomenu, self.startpoint)
        pygame.display.flip()
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    print(event.pos)
                    if event.pos[1] <= 585 and event.pos[1] >= 505:
                        self.playing(go)

    def win(self, go):
        while 1:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    go.clear_board()
                    for player in go.player_list:
                        player.eat_piece = 0
                    self.player = []
                    self.menu(go)

    def playing(self, go):
        offset = 62
        space = 33
        self.screen.blit(self.goboard_resize, self.startpoint)
        pygame.display.flip()
        self.player = go.player_list[0]
        win_status = 0
        while 1:
            for pos in self.player.wining_position:
                if go.check_win_position(self.player.nb, pos[0], pos[1]) != 0:
                    print("tatata")
                    win_status = 1
            if win_status != 0:
                self.print_font(132, "player " + str(self.player.nb) + " win", 100, 300)
                pygame.display.flip()
                self.win(go)
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    sys.exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    self.screen.blit(self.goboard_resize, self.startpoint)
                    # img position for mouse position
                    # print(event.pos)
                    x = int((event.pos[1] - offset) / space)
                    modx = int((event.pos[1] - offset) % space)
                    if modx > space / 2 and x < 18:
                        x += 1
                    y = int((event.pos[0] - offset) / space)
                    mody = int((event.pos[0] - offset) % space)
                    if mody > space / 2 and y < 18:
                        y += 1
                    # print(x, modx, y, mody)
                    # end
                    stonestatus = go.place_stone(self.player, x, y)
                    # print("stonestatus: ", stonestatus)
                    if stonestatus == -1:
                        self.print_font(
                            32,
                            "Player Turn: "
                            + self.player.color
                            + "  previous move: "
                            + "Illegal move",
                            64,
                            32,
                        )
                    elif stonestatus == 0:
                        if self.player.nb == 1:
                            self.player = go.player_list[1]
                        elif self.player.nb == 2:
                            self.player = go.player_list[0]
                        self.print_font(
                            32,
                            "Player Turn: "
                            + self.player.color
                            + "  previous move: "
                            + self.map_notation[0][x]
                            + self.map_notation[1][y],
                            64,
                            32,
                        )
                    for L in range(len(go.table)):
                        for l in range(len(go.table[L])):
                            if go.table[L][l] == 1:
                                self.screen.blit(
                                    self.white_stone_resize,
                                    (
                                        l * space + offset - self.stone_size[0] / 2,
                                        L * space + offset - self.stone_size[1] / 2,
                                    ),
                                )
                            elif go.table[L][l] == 2:
                                self.screen.blit(
                                    self.black_stone_resize,
                                    (
                                        l * space + offset - self.stone_size[0] / 2,
                                        L * space + offset - self.stone_size[1] / 2,
                                    ),
                                )
            pygame.display.flip()

    def print_font(self, size, msg, x, y):
        BLACK = (0, 0, 0)
        sysfont = pygame.font.get_default_font()
        font = pygame.font.SysFont(None, size)
        render = font.render(msg, True, BLACK)
        self.screen.blit(render, (x, y))

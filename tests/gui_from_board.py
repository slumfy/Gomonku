import pygame

from global_var import BOARD_NOTATION, MAIN_WINDOW_SIZE, STONE_SIZE


class GuiFromBoard:
    def __init__(self):
        pygame.init()
        self.screen = pygame.display.set_mode(size=MAIN_WINDOW_SIZE)
        self.go_board = pygame.image.load("ressources/images/goboard.png")
        self.go_board_resize = pygame.transform.scale(self.go_board, MAIN_WINDOW_SIZE)
        self.start_point = [0, 0]
        self.white_stone = pygame.image.load("ressources/images/whitecircle.png")
        self.white_stone_resize = pygame.transform.scale(self.white_stone, STONE_SIZE)
        self.black_stone = pygame.image.load("ressources/images/blackcircle.png")
        self.black_stone_resize = pygame.transform.scale(self.black_stone, STONE_SIZE)

    def display_current_state(self, board):
        self.screen.blit(self.go_board_resize, self.start_point)
        pygame.display.flip()
        self.board_screen_blit(board, 33, 62)
        pygame.display.flip()

    def board_screen_blit(self, board, space, offset):
        for L in range(len(board)):
            for l in range(len(board[L])):
                if board[L][l] == 1:
                    self.screen.blit(
                        self.white_stone_resize,
                        (
                            l * space + offset - STONE_SIZE[0] / 2,
                            L * space + offset - STONE_SIZE[1] / 2,
                        ),
                    )
                elif board[L][l] == 2:
                    self.screen.blit(
                        self.black_stone_resize,
                        (
                            l * space + offset - STONE_SIZE[0] / 2,
                            L * space + offset - STONE_SIZE[1] / 2,
                        ),
                    )

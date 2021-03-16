import argparse

from go_rules import GoRules
from py_game_go import PyGameGo


def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--no_sound",
        help="Disable or enable sound.",
        action="store_true",
    )
    args = parser.parse_args(argv)
    go_rules = GoRules()
    print("args.no_sound = ", args.no_sound)
    game = PyGameGo(sound_status=not args.no_sound)
    game.menu(go_rules=go_rules)


if __name__ == "__main__":
    main()

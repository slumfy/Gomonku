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
    parser.add_argument(
        "--no_ai_helper",
        help="Disable or enable AI helper.",
        action="store_false",
    )
    args = parser.parse_args(argv)
    go_rules = GoRules(ai_helper=args.no_ai_helper)
    game = PyGameGo(sound_status=not args.no_sound)
    game.menu(go_rules=go_rules)


if __name__ == "__main__":
    main()

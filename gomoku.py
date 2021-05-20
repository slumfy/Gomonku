import argparse
import sys

sys.path.append("./python_GUI/")
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
        "--ai_helper",
        help="Disable or enable AI helper.",
        action="store_true",
    )
    parser.add_argument(
        "--display_ai_time",
        help="Disable or enable the time it takes to the AI to do a move.",
        action="store_true",
    )
    parser.add_argument(
        "--with_search_box",
        help="Disable or enable Search box.",
        action="store_true",
    )
    parser.add_argument(
        "--depth",
        help="Set the depth of algorithm search (from 1 to 10).",
        choices=range(1, 11),
        type=int,
        default=5,
    )
    parser.add_argument(
        "--search_algorithm",
        choices=["negamax", "negascout"],
        help="""
    - negamax
    - negascout""",
        default="negamax",
    )
    args = parser.parse_args(argv)
    print("search_algorithm = ", args.search_algorithm)
    go_rules = GoRules()
    game = PyGameGo(
        sound_status=not args.no_sound,
        ai_helper=args.ai_helper,
        search_box_status=args.with_search_box,
        display_ai_time=args.display_ai_time,
        depth=args.depth,
        search_algorithm=args.search_algorithm,
    )
    game.menu(go_rules=go_rules)


if __name__ == "__main__":
    main()

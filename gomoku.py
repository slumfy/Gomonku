import argparse
import sys
sys.path.append('./python_GUI/')
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
        "--search_algorithm",
        help="""
    - negamax
    - negamax_with_transpotable
    - negascout
    - negascout_with_transpotable""",
        default="negamax",
    )
    args = parser.parse_args(argv)
    go_rules = GoRules(ai_helper=args.no_ai_helper)
    game = PyGameGo(
        sound_status=not args.no_sound,
        search_box_status=args.with_search_box,
        display_ai_time=args.display_ai_time,
        search_algorithm=args.search_algorithm,
    )
    game.menu(go_rules=go_rules)


if __name__ == "__main__":
    main()

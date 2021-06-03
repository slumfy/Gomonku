import sys
import json

sys.path.append("./python_GUI/")
from go_rules import GoRules
from py_game_go import PyGameGo


class Settings:
    sound = None
    ai_depth = None
    second_ai_depth = None
    ai_helper = None
    ai_helper_depth = None
    display_ai_time = None
    search_box = None
    theme = None

    def __init__(self, data):
        self.sound = data["sound"]
        self.ai_depth = data["ai_depth"]
        self.second_ai_depth = data["second_ai_depth"]
        self.ai_helper = data["ai_helper"]
        self.ai_helper_depth = data["ai_helper_depth"]
        self.display_ai_time = data["display_ai_time"]
        self.search_box = data["search_box"]
        self.theme = data["theme"]


def main(argv=None):
    try:
        with open("config.json") as config_file:
            try:
                data = json.load(config_file)
            except ValueError:
                print("Bad formatting of json file.")
                exit()
        settings = Settings(data)
    except FileNotFoundError:
        print("No config.json found.")
        exit()
    except KeyError as e:
        print("Missing Key: ", e)
    except Exception as e:
        print("An error occured: ", e)
    print("Selected depth : ", settings.ai_depth)
    go_rules = GoRules()
    game = PyGameGo(
        sound_status=settings.sound,
        ai_helper=settings.ai_helper,
        search_box_status=settings.search_box,
        display_ai_time=settings.display_ai_time,
        depth=settings.ai_depth,
        ai_helper_depth=settings.ai_helper_depth,
        ai_black_depth=settings.second_ai_depth,
        theme=settings.theme,
    )
    game.menu(go_rules=go_rules)


if __name__ == "__main__":
    main()

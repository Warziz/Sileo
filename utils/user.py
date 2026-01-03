import random
import string


def generate_username() -> str:
    length = 10
    username = "".join(random.choices(string.ascii_letters + string.digits, k=length))
    return username


def color_text(text, color="red"):
    colors = {
        "red": "\033[91m",
        "green": "\033[92m",
        "yellow": "\033[93m",
        "blue": "\033[94m",
        "magenta": "\033[95m",
        "cyan": "\033[96m",
        "white": "\033[97m",
        "reset": "\033[0m",
    }
    return f"{colors.get(color, colors['reset'])}{text}{colors['reset']}"

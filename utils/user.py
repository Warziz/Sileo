import random
import string

def generate_username() -> str:
    length = 10
    username = ''.join(random.choices(string.ascii_letters + string.digits, k=length))
    return username
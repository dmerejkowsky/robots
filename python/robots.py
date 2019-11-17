import string
import random


def generate_random_name():
    letters = string.ascii_uppercase
    res = ""
    res += "".join([random.choice(letters) for _ in range(0, 2)])
    res += f"{random.randint(0, 1000):03}"
    return res



class Robot:
    def __init__ (self):
        self._name = None

    def name(self):
        return self._name

    def start(self):
        if self._name is None:
            self._name = generate_random_name()
            print(self._name)

    def stop(self):
        pass

    def reset(self):
        self._name = None

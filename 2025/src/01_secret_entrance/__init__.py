from pathlib import Path
from ..utils import import_txt_as_list

def phase_1(data=None):
    current = 50
    acc = 0

    for line in data:
        next = line.strip()
        direction = next[0]
        value = int(next[1:])

        if direction == "L":
            current = (current - value) % 100

        elif direction == "R":
            current = (current + value) % 100

        if current == 0:
            acc += 1 

    print(current, acc)

def phase_2(data=None):
    print(data)

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "input.txt"
    data = import_txt_as_list(file_path)

    phase_1(data)
    #phase_2(data)

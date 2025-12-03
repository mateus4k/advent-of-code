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
            if current - value < 0:
                current = current - value + 100
            else:
                current = current - value

        elif direction == "R":
            if current - value > 100:
                current = current + value - 100
            else:
                current = current + value

        if current == 0 or current == 100:
            acc += 1 

        if current == 100:
            current = 0

    print(acc)

def phase_2(data=None):
    current = 50
    acc = 0

    for line in data:
        next = line.strip()
        direction = next[0]
        value = int(next[1:])

        if direction == "L":
            if current - value < 0:
                current = current - value + 100
            else:
                current = current - value

        elif direction == "R":
            if current - value > 100:
                current = current + value - 100
            else:
                current = current + value

        if current == 0 or current == 100:
            acc += 1 

        if current == 100:
            current = 0

    print(acc)

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "test.txt"
    data = import_txt_as_list(file_path)

    phase_1(data)
    phase_2(data)

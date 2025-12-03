from pathlib import Path
from ..utils import import_txt_as_list

def phase_1(data=None):
    current = 50
    acc = 0

    for line in data:
        line = line.strip()
        direction = line[0]
        value = int(line[1:])

        if direction == "L":
            current = (current - value) % 100

        elif direction == "R":
            current = (current + value) % 100

        if current == 0:
            acc += 1 

    print(acc)

def phase_2(data=None):
    current = 50
    acc = 0

    for line in data:
        line = line.strip()
        direction = line[0]
        value = int(line[1:])
        start = current
        
        full_loops = value // 100
        rest = value % 100

        passes = full_loops

        if direction == "R":
            if start + rest >= 100:
                passes += 1
            current = (start + value) % 100

        elif direction == "L":
            if start > 0 and start - rest <= 0:
                passes += 1
            current = (start - value) % 100

        acc += passes

    print(acc)

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "input.txt"
    data = import_txt_as_list(file_path)

    phase_1(data)
    phase_2(data)

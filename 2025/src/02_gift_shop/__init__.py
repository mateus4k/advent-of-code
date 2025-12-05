from pathlib import Path
from ..utils import import_txt_as_list

def phase_1(data=None):
    sum = 0

    for line in data:
        [start, end] = line.split("-")
        start = int(start)
        end = int(end)

        for i in range(start, end + 1):
            middle = len(str(i)) // 2
            part_1 = str(i)[:middle]
            part_2 = str(i)[middle:]
            if part_1 == part_2:
                sum += i

    print(sum)

def is_repeated_pattern(n: int) -> bool:
    s = str(n)
    L = len(s)

    for size in range(1, L//2 + 1):
        if L % size != 0:
            continue

        part = s[:size]
        if part * (L // size) == s:
            return True

    return False

def phase_2(data=None):
    sum = 0

    for line in data:
        start, end = map(int, line.split("-"))
        for i in range(start, end + 1):
            if is_repeated_pattern(i):
                sum += i

    print(sum)

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "input.txt"
    data = import_txt_as_list(file_path)[0]
    data = data.split(",")

    phase_1(data)
    phase_2(data)

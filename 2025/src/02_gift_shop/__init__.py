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

def phase_2(data=None):
    print(data)

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "input.txt"
    data = import_txt_as_list(file_path)[0]
    data = data.split(",")

    phase_1(data)
    # phase_2(data)

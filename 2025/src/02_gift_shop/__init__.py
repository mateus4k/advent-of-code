from pathlib import Path
from ..utils import import_txt_as_list

def phase_1(data=None):
    print(data)

def phase_2(data=None):
    print(data)

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "input.txt"
    data = import_txt_as_list(file_path)

    phase_1(data)
    phase_2(data)

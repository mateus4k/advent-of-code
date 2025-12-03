from pathlib import Path
from ..utils import import_txt_as_list

def main():
    base = Path(__file__).resolve().parent
    file_path = base / "test.txt"
    data = import_txt_as_list(file_path)
    print(data)
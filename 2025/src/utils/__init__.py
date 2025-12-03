def import_txt_as_list(filepath):
    try:
        with open(filepath, 'r') as file:
            lines = file.readlines()
        return lines
    except FileNotFoundError:
        print(f"Error: The file '{filepath}' was not found.")
        return []
    except Exception as e:
        print(f"An error occurred while reading the file: {e}")
        return []

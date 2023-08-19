import os

def remove_prefix(text, prefix):
    if text.startswith(prefix):
        return text[len(prefix):]
    return text

def main():
    cwd = os.getcwd()
    files = os.listdir(cwd)
    
    for filename in files:
        new_filename = remove_prefix(filename, "alpha_numerics_")
        if new_filename != filename:
            os.rename(filename, new_filename)
            print(f"Renamed '{filename}' to '{new_filename}'")

if __name__ == "__main__":
    main()

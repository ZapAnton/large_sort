import string
import random


if __name__ == '__main__':
    letters = string.ascii_letters

    lines_count = 500000

    line_width = 500

    with open('big_file.txt', 'w') as file:
        for _ in range(lines_count):
            char_list = [random.choice(letters) for _ in range(line_width)]

            file_line = ''.join(char_list)

            file.write(file_line)

            file.write('\n')

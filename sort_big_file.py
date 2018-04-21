import os


def split_by_many_files(file):
    small_file_line_count = 50

    small_file_name = 'small_files/file_{}.txt'

    if not os.path.exists('small_files'):
        os.makedirs('small_files')
    else:
        for temp_file in os.scandir('small_files'):
            os.unlink(temp_file.path)

    line = file.readline()

    line_count = 0

    file_count = 1

    line = file.readline()

    while line:
        if line_count <= small_file_line_count:
            with open(small_file_name.format(file_count), 'a') as small_file:
                small_file.write(line)

                line_count += 1
        else:
            line_count = 0

            file_count += 1

            with open(small_file_name.format(file_count), 'a') as small_file:
                small_file.write(line)

                line_count += 1

        line = file.readline()

    return file_count


def sort_small_files(file_count: int):
    for file_number in range(1, file_count):
        sorted_lines = None

        with open('small_files/file_{}.txt'.format(file_number), 'r') as file:
            lines = file.readlines()

            sorted_lines = sorted(lines, key=lambda line: line[:51])

        with open('small_files/file_{}.txt'.format(file_number), 'w') as file:
            file.write(''.join(sorted_lines))


def number_of_reached_eof(first_lines: list):
    eof_count = 0

    for line in first_lines:
        if not line:
            eof_count += 1

    return eof_count


def merge_sorted_files(file_count: int):
    sorted_files = [open(sorted_file, 'r')
                    for sorted_file in
                    list(map(
                        lambda file_number: 'small_files/file_{}.txt'
                        .format(file_number),
                        range(1, file_count)))]

    with open('big_file_sorted.txt', 'w') as big_file_sorted:
        first_lines = [sorted_file.readline() for sorted_file in sorted_files]

        eof_count = number_of_reached_eof(first_lines)

        while eof_count < file_count - 1:
            first_lines_filtered = [line for line in first_lines if line]

            smallest_line =\
                min(first_lines_filtered, key=lambda line: line[:51])

            smallest_line_index = first_lines.index(smallest_line)

            big_file_sorted.write(smallest_line)

            first_lines[smallest_line_index] =\
                sorted_files[smallest_line_index].readline()

            eof_count = number_of_reached_eof(first_lines)

    for sorted_file in sorted_files:
        sorted_file.close()


if __name__ == '__main__':
    file_count = 1

    with open('big_file.txt', 'r') as file:
        file_count = split_by_many_files(file)

    sort_small_files(file_count)

    merge_sorted_files(file_count)

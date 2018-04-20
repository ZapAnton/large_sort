def split_by_many_files(file):
    small_file_line_count = 50

    small_file_name = 'small_files/file_{}.txt'

    line = file.readline()

    line_count = 0

    file_count = 1

    line = file.readline()

    while line:
        if line_count <= small_file_line_count:
            with open(small_file_name.format(file_count), 'a') as small_file:
                small_file.write(line)

                small_file.write('\n')

                line_count += 1
        else:
            print('Stoped wirintg to file', file_count)

            line_count = 0

            file_count += 1

            with open(small_file_name.format(file_count), 'a') as small_file:
                small_file.write(line)

                small_file.write('\n')

                line_count += 1

        line = file.readline()

    return file_count


def sort_small_files(file_count: int):
    pass


if __name__ == '__main__':
    file_count = 1

    with open('big_file.txt', 'r') as file:
        file_count = split_by_many_files(file)

    sort_small_files(file_count)

use std::{fs, fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}, path::Path};

fn split_by_small_files(big_file_name: &str) -> i32 {
    let temp_dir_name = "small_files";

    let temp_dir = Path::new(temp_dir_name);

    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir).expect("Could not delete temp dir!");
    }

    fs::create_dir(temp_dir).expect("Error creating temp dir!");

    let temp_file_lines_count = 50;

    let mut temp_files_count = 1;

    let mut lines_count = 0;

    let big_file = File::open(big_file_name).unwrap();

    let mut temp_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("small_files/file_{}.txt", temp_files_count))
        .unwrap();

    for line in BufReader::new(big_file).lines() {
        if lines_count == temp_file_lines_count {
            lines_count = 0;

            temp_files_count += 1;

            temp_file = OpenOptions::new()
                .append(true)
                .create(true)
                .open(format!("small_files/file_{}.txt", temp_files_count))
                .unwrap();
        }

        temp_file.write_all(line.unwrap().as_bytes()).unwrap();

        temp_file.write("\n".as_bytes()).unwrap();

        lines_count += 1;
    }

    temp_files_count
}

fn sort_temp_files(temp_files_count: i32) {
    for file_number in 1..(temp_files_count + 1) {
        let temp_file_name = format!("small_files/file_{}.txt", file_number);

        let temp_file = File::open(&temp_file_name).expect(&format!(
            "Cannot read unsorted temp file {}!",
            &temp_file_name
        ));

        let mut temp_file_lines = BufReader::new(temp_file)
            .lines()
            .map(|x| x.expect("Error unwraping string line!"))
            .collect::<Vec<String>>();

        temp_file_lines.sort_by(|line1, line2| line1.cmp(&line2));

        let mut temp_file = File::create(&temp_file_name).expect(&format!(
            "Cannot write sorted lines to file {}!",
            &temp_file_name
        ));

        for line in temp_file_lines {
            temp_file.write(line.as_bytes()).unwrap();

            temp_file.write("\n".as_bytes()).unwrap();
        }
    }
}

fn merge_temp_files(temp_files_count: i32) {
    let mut temp_file_lines = Vec::with_capacity(temp_files_count as usize);

    for temp_file_num in 1..(temp_files_count + 1) {
        let temp_file = File::open(format!("small_files/file_{}.txt", temp_file_num)).expect(
            &format!("Could not read small_files/file_{}.txt", temp_file_num),
        );

        temp_file_lines.push(BufReader::new(temp_file).lines());
    }

    let mut first_lines: Vec<String> = Vec::with_capacity(temp_files_count as usize);
}

fn main() {
    let temp_files_count = split_by_small_files("big_file.txt");

    sort_temp_files(temp_files_count);

    merge_temp_files(temp_files_count);
}

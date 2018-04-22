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

fn main() {
    let _temp_files_count = split_by_small_files("big_file.txt");
}

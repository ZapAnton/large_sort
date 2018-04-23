use std::{fs, fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}, path::Path};

fn split_by_small_files(big_file_name: &str) -> i32 {
    let temp_dir_name = "small_files";

    let temp_dir = Path::new(temp_dir_name);

    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir).expect("Could not delete temp dir!");
    }

    fs::create_dir(temp_dir).expect("Error creating temp dir!");

    let temp_file_lines_count = 500;

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

fn get_eof_count(first_lines: &Vec<String>) -> i32 {
    let mut eof_count = 0;

    for line in first_lines {
        if line.is_empty() {
            eof_count += 1;
        }
    }

    eof_count
}

fn merge_temp_files(temp_files_count: i32) {
    let mut temp_files = Vec::<BufReader<File>>::with_capacity(temp_files_count as usize);

    for file_num in 1..(temp_files_count + 1) {
        temp_files.push(BufReader::new(
            File::open(format!("small_files/file_{}.txt", file_num))
                .expect(&format!("Could not open small_files/file_{}.txt", file_num)),
        ));
    }

    //TODO Try to bypass reader mutability error
    /*let mut first_lines = (&mut temp_files)
        .iter()
        .map(|reader| {
            let mut line = String::new();
            reader.get_mut().read_line(&mut line).unwrap();
            line
        })
        .collect::<Vec<String>>();*/

    let mut first_lines = Vec::<String>::new();

    for temp_file in &mut temp_files {
        let mut line = String::new();

        temp_file.read_line(&mut line).unwrap();

        first_lines.push(line);
    }

    let mut eof_count = get_eof_count(&first_lines);

    let mut big_file_sorted =
        File::create("big_file_sorted.txt").expect("Cannot write to file big_file_sorted.txt!");

    while eof_count < temp_files_count {
        let smallest_line = first_lines
            .iter()
            .filter(|line| !line.is_empty())
            .min_by(|line1, line2| line1[..51].cmp(&line2[0..51]))
            .unwrap()
            .clone();

        let smallest_line_index = first_lines
            .iter()
            .position(|line| line == &smallest_line)
            .unwrap();

        big_file_sorted.write(smallest_line.as_bytes()).unwrap();

        first_lines.remove(smallest_line_index);

        let mut new_line = String::new();

        temp_files
            .get_mut(smallest_line_index)
            .unwrap()
            .read_line(&mut new_line)
            .unwrap();

        first_lines.insert(smallest_line_index, new_line);

        eof_count = get_eof_count(&first_lines);
    }
}

fn main() {
    let temp_files_count = split_by_small_files("big_file.txt");

    sort_temp_files(temp_files_count);

    merge_temp_files(temp_files_count);
}

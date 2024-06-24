use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    // --snip--
    // define file_path
    let file_path = "src/large_file50m.csv";
    println!("In file {file_path}");

    let file_read = File::open(file_path);
    if file_read.is_err(){
        panic!("File not found");
    }
    let file = file_read.unwrap();
    let reader = io::BufReader::new(file);

    let mut highest_line = String::new();
    let mut highest_value: f32 = -1.0;
    for line in reader.lines().skip(1) {
        if line.is_err(){
            continue;

        }

        let line = line.unwrap();
        let line1: &str = line.as_str();
        let values: Vec<&str> = line1.split(",").collect();
        let high = values[2].parse::<f32>().unwrap();
        if high > highest_value{
            highest_value = high;
            highest_line = String::from(line1);
        }
    }
    println!("{}", highest_line.clone());
}


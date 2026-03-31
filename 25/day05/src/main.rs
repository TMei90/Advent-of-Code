use std::{env, fs, path};

const EXAMPLE: &str = "example_input.txt";
const REAL: &str = "input.txt";
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let real_input = &args[1];
    let input: &str;
    if real_input == "REAL" {
        input = REAL;
    } else {
        input = EXAMPLE;
    }
    let input = fs::read_to_string(path::Path::new(input)).expect("Failed to read file");
    let lines = input.lines().collect::<Vec<&str>>();

    let split: Vec<&[&str]> = lines.split(|line| line.is_empty()).collect();
    let ranges = split[0].to_owned();
    let test = split[1].to_owned();
    let mut fresh_counter = 0;

    for current in test {
        let current = current.parse::<u64>().unwrap();
        for range in &ranges {
            let (first, second) = range.split_once("-").unwrap();
            let first = first.parse::<u64>().unwrap();
            let second = second.parse::<u64>().unwrap();
            print!("Range: {:?} Current: {:?}\n", range, current);
            if first <= current && current <= second {
                fresh_counter += 1;
                break;
            }
        }
        print!("Fresh: {:?}\n", fresh_counter);
    }
}

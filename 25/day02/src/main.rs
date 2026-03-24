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
    let input = input.trim().to_string();
    let list: Vec<&str> = input.split(',').collect();
    let mut invalid: Vec<u64> = Vec::new();

    for range in &list {
        let (lower, upper) = range.split_once('-').unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();
        for current in lower..=upper {
            let current = current.to_string();
            let current_length = current.len();
            let (current_low, current_high) = current.split_at(current_length / 2);
            if current_low == current_high {
                print!("{:?} ", current);
                invalid.push(current.parse::<u64>().unwrap());
            }
        }
    }
    let product: u64 = invalid.into_iter().sum();
    println!("\nProduct: {}", product);
}

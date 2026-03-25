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
        // Part 2
        for current in lower..=upper {
            let current = current.to_string();
            let current_length = current.len();
            for i in 0..current_length / 2 {
                let temp = current.clone();
                let temp = temp.chars().nth(i).unwrap();
                let mut test: String = temp.to_string();
                test.push(temp);
                if current.contains(&test) {
                    current.rem
                    print!("{:?} ", current);
                    invalid.push(current.parse::<u64>().unwrap());
                    break;
                }
            }
        }
    }
    let product: u64 = invalid.into_iter().sum();
    println!("\nProduct: {}", product);
    //   Part1
    //         let (current_low, current_high) = current.split_at(current_length / 2);
    //         if current_low == current_high {
    //             print!("{:?} ", current);
    //             invalid.push(current.parse::<u64>().unwrap());
    //         }
}

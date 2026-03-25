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

    let mut row_best: Vec<i32> = Vec::new();
    let mut current_best = 0;
    for line in &lines {
        let chars = line.chars().collect::<Vec<char>>();
        for i in 0..chars.len() - 1 {
            for j in i + 1..chars.len() {
                let mut current: String = chars[i].to_string();
                current.push(chars[j]);
                let current = current.parse::<i32>().expect("Failed to parse");
                if current > current_best {
                    current_best = current;
                }
            }
        }
        row_best.push(current_best);
        current_best = 0;
    }
    print!("Row best: {:?} ", row_best.into_iter().sum::<i32>());
}

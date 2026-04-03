use std::{env, fs, path, result};

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

    let mut rows: Vec<Vec<u64>> = Vec::new();

    for line in 0..lines.len() - 1 {
        let line = lines[line].split_ascii_whitespace().collect::<Vec<&str>>();
        let mut row: Vec<u64> = Vec::new();
        for stings in &line {
            let temp = stings
                .parse::<u64>()
                .expect("Failed to parse string to u64");
            row.push(temp);
        }
        rows.push(row);
    }

    let operants = lines[lines.len() - 1]
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();

    let mut result_vec: Vec<u64> = Vec::new();
    let mut temp_vec: Vec<u64> = Vec::new();
    let mut j = 0;
    loop {
        for i in 0..rows.len() {
            temp_vec.push(rows[i][j]);
        }
        if operants[j] == "+" {
            result_vec.push(temp_vec.iter().sum());
        } else if operants[j] == "*" {
            result_vec.push(temp_vec.iter().product());
        }
        temp_vec.clear();
        j += 1;
        if j == operants.len() {
            break;
        }
    }
    println!("{:?}", result_vec);
    print!("Sum: {}", result_vec.iter().sum::<u64>());
}

#![allow(unused)]
use std::{collections::HashSet, env, fs, path, result};

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
    print!("Lines: {:?}\n", lines);
    let coordinates = lines
        .iter()
        .map(|line| {
            let mut parts = line.split(",");
            let x = parts.next().unwrap().parse::<u64>().unwrap();
            let y = parts.next().unwrap().parse::<u64>().unwrap();
            (x, y)
        })
        .collect::<Vec<(u64, u64)>>();
    let mut current_best = 0;
    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            let (x1, y1) = coordinates[i];
            let (x2, y2) = coordinates[j];
            let x_distance = x1.abs_diff(x2) + 1;
            let y_distance = y1.abs_diff(y2) + 1;
            let area = x_distance * y_distance;
            if area > current_best {
                current_best = area;
            }
        }
    }
    println!("Best area: {}", current_best);
}

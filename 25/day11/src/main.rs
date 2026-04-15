#![allow(unused)]
use std::collections::{HashMap, HashSet};
use std::os::windows::process;
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
    let map: HashMap<String, String> = lines
        .iter()
        .map(|line| {
            let line = line.split(":").collect::<Vec<&str>>();
            (line[0].trim().to_string(), line[1].trim().to_string())
        })
        .collect();

    let mut current_vec: Vec<&str> = map.get("you").unwrap().split_whitespace().collect();
    println!("{:?}", current_vec);
    let mut path = 0;
    while current_vec.len() > 0 {
        let curren_str = current_vec.remove(0);
        if curren_str == "out" {
            path += 1;
        } else {
            current_vec.extend(map.get(curren_str).unwrap().split_whitespace());
        }
    }
    println!("Path: {}", path);
}

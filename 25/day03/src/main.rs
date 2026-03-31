use std::{char, env, fs, path};

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

    let mut row_best: Vec<u64> = Vec::new();
    let mut column_best: String = "".to_string();
    for line in &lines {
        let chars = line.chars().collect::<Vec<char>>();
        let mut current_best = 0;
        let mut position_best = 0;
        for i in 0..12 {
            let temp1 = chars[i].to_string();
            let temp2 = temp1.parse::<u64>().expect("Failed to parse");
            current_best = temp2;
            position_best = i;
            for position in position_best + 1..chars.len() - 11 + i {
                let temp3 = chars[position].to_string();
                let temp4 = temp3.parse::<u64>().expect("Failed to parse");
                if temp4 > current_best {
                    current_best = temp4;
                    position_best = position;
                }
            }
            column_best.push(chars[position_best]);
        }
        print!("Column best: {:?} ", column_best);
        row_best.push(column_best.parse::<u64>().expect("Failed to parse"));
        column_best = "".to_string();
        // row_best.push(current_best.parse::<u64>().expect("Failed to parse"));

        // part 1
        // for i in 0..chars.len() - 1 {
        //     for j in i + 1..chars.len() {
        //         let mut current: String = chars[i].to_string();
        //         current.push(chars[j]);
        //         let current = current.parse::<i32>().expect("Failed to parse");
        //         if current > current_best {
        //             current_best = current;
        //         }
        //     }
        // }
        // row_best.push(current_best);
        // current_best = 0;
    }
    print!("Row best: {:?} ", row_best.into_iter().sum::<u64>());
}

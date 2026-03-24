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
    let list: Vec<&str> = input.lines().collect();

    let mut current: i32 = 50;
    let mut zeros: i32 = 0;
    for directions in &list {
        let direction = directions.chars().next().unwrap().to_string();
        let steps = directions[1..].parse::<i32>().unwrap();
        print!("\n{:?}, {:?}, {:?}, ", current, direction, steps);

        if direction == "L" {
            for _ in 0..steps {
                current = current - 1;
                if current == 0 {
                    zeros += 1;
                }
                if current == -1 {
                    current = 99;
                }
            }
        } else {
            for _ in 0..steps {
                current = current + 1;
                if current == 100 {
                    current = 0;
                }
                if current == 0 {
                    zeros += 1;
                }
            }
        }

        print!("{:?}, {:?} ", current, zeros);
    }
    print!("\n{:?}", zeros);
}

// part 1
// let mut current: i32 = 50;
// let mut zeros: u32 = 0;
// print!("{:?} ", list);
// for directions in &list {
//     let direction = directions.chars().next().unwrap().to_string();
//     let steps = directions[1..].parse::<i32>().unwrap() % 100;
//     if direction == "L" {
//         current = current - steps;
//         if current < 0 {
//             current += 100;
//         }
//     } else if direction == "R" {
//         current = current + steps;
//         if current >= 100 {
//             current -= 100;
//         }
//     }
//     if current == 0 {
//         zeros = zeros + 1;
//     }

//     print!("{:?} ", current);
// }
// print!("\n{:?}", zeros);

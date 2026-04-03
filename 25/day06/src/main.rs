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
    let mut lines = input.lines().collect::<Vec<&str>>();

    let operants = lines
        .pop()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>();
    let mut forth_line = "";
    if real_input == "REAL" {
        forth_line = lines.pop().unwrap();
    }

    let mut third_line = lines.pop().unwrap();
    let mut second_line = lines.pop().unwrap();
    let mut first_line = lines.pop().unwrap();

    let mut group: Vec<u64> = Vec::new();
    let mut groups: Vec<Vec<u64>> = Vec::new();
    while first_line.len() > 0 {
        if real_input == "REAL" {
            let (temp1, first_line_inner) = first_line.split_at(1);
            let (temp2, second_line_inner) = second_line.split_at(1);
            let (temp3, third_line_inner) = third_line.split_at(1);
            let (temp4, forth_line_inner) = forth_line.split_at(1);
            first_line = first_line_inner;
            second_line = second_line_inner;
            third_line = third_line_inner;
            forth_line = forth_line_inner;

            if temp1 == temp2 && temp1 == temp3 && temp1 == temp4 && temp1 == " " {
                groups.push(group);
                group = Vec::new();
                continue;
            }
            let mut temp: String = String::new();
            temp.push_str(temp1);
            temp.push_str(temp2);
            temp.push_str(temp3);
            temp.push_str(temp4);
            print!("temp: {:?}\n", temp);
            let temp = temp.trim().parse::<u64>().unwrap();

            group.push(temp);
        } else {
            let (temp2, second_line_inner) = second_line.split_at(1);
            let (temp3, third_line_inner) = third_line.split_at(1);
            let (temp1, first_line_inner) = first_line.split_at(1);
            first_line = first_line_inner;
            second_line = second_line_inner;
            third_line = third_line_inner;

            if temp1 == temp2 && temp1 == temp3 && temp1 == " " {
                groups.push(group);
                group = Vec::new();
                continue;
            }
            let mut temp: String = String::new();
            temp.push_str(temp1);
            temp.push_str(temp2);
            temp.push_str(temp3);
            let temp = temp.trim().parse::<u64>().unwrap();

            group.push(temp);
        }
    }
    groups.push(group);

    print!("operants: {:?}\n", operants);
    print!("groups: {:?}\n", groups);

    let mut sum: u64 = 0;
    let mut index = 0;
    loop {
        let current_group = &groups[index];
        if operants[index] == "+" {
            sum += current_group.iter().sum::<u64>();
        } else if operants[index] == "*" {
            sum += current_group.iter().product::<u64>();
        }
        index += 1;
        if index == operants.len() {
            break;
        }
    }
    println!("Sum: {}", sum);
}
// print!("first line: {:?}\n", first_line);
// print!("second line: {:?}\n", second_line);
// print!("third line: {:?}\n", third_line);
// print!("operants: {:?}\n", operants);

// Part 1
// let mut rows: Vec<Vec<u64>> = Vec::new();
// for line in 0..lines.len() - 1 {
//     let line = lines[line].split_ascii_whitespace().collect::<Vec<&str>>();
//     let mut row: Vec<u64> = Vec::new();
//     for stings in &line {
//         let temp = stings
//             .parse::<u64>()
//             .expect("Failed to parse string to u64");
//         row.push(temp);
//     }
//     rows.push(row);
// }

// let operants = lines[lines.len() - 1]
//     .split_ascii_whitespace()
//     .collect::<Vec<&str>>();

// let mut result_vec: Vec<u64> = Vec::new();
// let mut temp_vec: Vec<u64> = Vec::new();
// let mut j = 0;
// loop {
//     for i in 0..rows.len() {
//         temp_vec.push(rows[i][j]);
//     }
//     if operants[j] == "+" {
//         result_vec.push(temp_vec.iter().sum());
//     } else if operants[j] == "*" {
//         result_vec.push(temp_vec.iter().product());
//     }
//     temp_vec.clear();
//     j += 1;
//     if j == operants.len() {
//         break;
//     }
// }
// println!("{:?}", result_vec);
// print!("Sum: {}", result_vec.iter().sum::<u64>());

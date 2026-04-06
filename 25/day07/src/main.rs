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

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        map.push(chars);
    }

    // for i in 0..map.len() {
    //     println!("{:?}", map[i]);
    // }
    // println!();

    let mut timelines: Vec<Vec<Vec<char>>> = Vec::new();
    timelines.push(map);
    let mut splits = 0;
    let mut timeline_splits = 1;
    while timelines.len() > 0 {
        map = timelines.pop().unwrap();
        'outer: for row in 0..map.len() {
            for col in 0..map[0].len() {
                if map[row][col] == 'S' {
                    map[row + 1][col] = '|';
                } else if map[row][col] == '^' && map[row - 1][col] == '|' {
                    map[row][col] = '*';
                    splits += 1;
                    timeline_splits += 1;
                    if col > 0 {
                        let temp = map[row][col - 1];
                        map[row][col - 1] = '|';
                        timelines.push(map.clone());
                        map[row][col - 1] = temp;
                    }
                    if col < map[0].len() - 1 {
                        map[row][col + 1] = '|';
                        timelines.push(map.clone());
                    }
                    break 'outer;
                } else if row > 0 && map[row - 1][col] == '|' && map[row][col] == '.' {
                    map[row][col] = '|';
                }
            }
            // for i in 0..map.len() {
            //     println!("{:?}", map[i]);
            // }
            // println!()
        }
    }
    print!("Splits: {}\n", splits);
    print!("Timeline splits: {}", timeline_splits);
}

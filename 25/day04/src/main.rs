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

    let mut map: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars = line.chars().collect::<Vec<char>>();
        map.push(chars);
    }

    let mut pickable = 0;
    let mut removed = true;
    while removed {
        removed = false;
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y][x] == '@' && is_pickable(&map, x, y) {
                    pickable += 1;
                    removed = true;
                    map[y][x] = '.';
                }
            }
        }
    }
    println!("{} trees are visible", pickable);
}

fn is_pickable(map: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let mut count = 0;
    if (x == 0 && y == 0)
        || (x == 0 && y == map.len() - 1)
        || (x == map.len() - 1 && y == 0)
        || (x == map.len() - 1 && y == map.len() - 1)
    {
        return true;
    } else if x == 0 {
        for i in y - 1..=y + 1 {
            for j in x..=x + 1 {
                if map[i][j] == '@' {
                    count += 1;
                }
            }
        }
    } else if x == map.len() - 1 {
        for i in y - 1..=y + 1 {
            for j in x - 1..=x {
                if map[i][j] == '@' {
                    count += 1;
                }
            }
        }
    } else if y == 0 {
        for i in y..=y + 1 {
            for j in x - 1..=x + 1 {
                if map[i][j] == '@' {
                    count += 1;
                }
            }
        }
    } else if y == map.len() - 1 {
        for i in y - 1..=y {
            for j in x - 1..=x + 1 {
                if map[i][j] == '@' {
                    count += 1;
                }
            }
        }
    } else {
        for i in y - 1..=y + 1 {
            for j in x - 1..=x + 1 {
                if map[i][j] == '@' {
                    count += 1;
                }
            }
        }
    }
    count -= 1;
    if count < 4 {
        return true;
    }
    return false;
}

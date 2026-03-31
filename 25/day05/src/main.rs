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

    let split: Vec<&[&str]> = lines.split(|line| line.is_empty()).collect();
    let ranges = split[0].to_owned();
    // let test = split[1].to_owned();

    print!("Ranges: {:?}\n", ranges.len());
    let mut fresh_ranges: Vec<Vec<u64>> = Vec::new();
    fresh_ranges.push(vec![0, 0]);
    let mut counter = 0;

    for range in &ranges {
        let (lower, upper) = range.split_once("-").unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();
        let mut temp_fresh: Vec<Vec<u64>> = Vec::new();
        let mut pushed = false;
        while fresh_ranges.len() > 0 {
            let fresh = fresh_ranges.pop().unwrap();
            let merge = can_combine(fresh.to_owned(), vec![lower, upper]);
            if merge.is_ok() {
                temp_fresh.push(merge.unwrap());
                temp_fresh.append(&mut fresh_ranges);
                pushed = true;
                break;
            } else {
                temp_fresh.push(fresh);
            }
        }
        print!("Temp Fresh: {:?}\n", temp_fresh);
        if pushed == false {
            temp_fresh.push(vec![lower, upper]);
        }
        fresh_ranges = temp_fresh;
        if pushed {
            let mut temp_fresh: Vec<Vec<u64>> = Vec::new();
            let mut current = fresh_ranges.pop().unwrap();

            while fresh_ranges.len() > 0 {
                let next = fresh_ranges.pop().unwrap();
                let merge = can_combine(current.to_owned(), next.to_owned());
                if merge.is_ok() {
                    temp_fresh.push(merge.unwrap());
                } else {
                    temp_fresh.push(current);
                    current = next;
                }
            }
            fresh_ranges = temp_fresh;
            fresh_ranges.retain(|v| v != &[0, 0]);
            print!("Fresh Ranges: {:?}\n", fresh_ranges);
        }
    }
    print!("Fresh Ranges: {:?}\n", fresh_ranges);
    for range in &fresh_ranges {
        let lower = range[0];
        let upper = range[1];
        counter += upper + 1 - lower;
    }
    print!("Counter: {:?}\n", counter);

    // Part 1
    // let mut fresh_counter = 0;
    // for current in test {
    //     let current = current.parse::<u64>().unwrap();
    //     for range in &ranges {
    //         let (first, second) = range.split_once("-").unwrap();
    //         let first = first.parse::<u64>().unwrap();
    //         let second = second.parse::<u64>().unwrap();
    //         print!("Range: {:?} Current: {:?}\n", range, current);
    //         if first <= current && current <= second {
    //             fresh_counter += 1;
    //             break;
    //         }
    //     }
    //     print!("Fresh: {:?}\n", fresh_counter);
    // }
}

fn can_combine(range_one: Vec<u64>, range_two: Vec<u64>) -> Result<Vec<u64>, String> {
    let range_one_lower = range_one[0];
    let range_one_upper = range_one[1];
    let range_two_lower = range_two[0];
    let range_two_upper = range_two[1];
    let mut combined_lower: u64 = 0;
    let mut combined_upper: u64 = 0;
    let mut can_combine: bool = false;
    if range_one_lower <= range_two_lower && range_two_upper <= range_one_upper {
        return Ok(range_one);
    } else if range_two_lower <= range_one_lower && range_one_upper <= range_two_upper {
        return Ok(range_two);
    } else if range_one_lower <= range_two_lower
        && range_two_lower < range_one_upper
        && range_one_upper <= range_two_upper
    {
        combined_lower = range_one_lower;
        combined_upper = range_two_upper;
        can_combine = true;
    } else if range_two_lower <= range_one_lower
        && range_one_lower < range_two_upper
        && range_two_upper <= range_one_upper
    {
        combined_lower = range_two_lower;
        combined_upper = range_one_upper;
        can_combine = true;
    }
    if can_combine {
        return Ok(vec![combined_lower, combined_upper]);
    }
    Err("not combinable".to_string())
}

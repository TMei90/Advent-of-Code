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
    let mut ranges_int: Vec<(u64, u64)> = Vec::new();
    for rang in &ranges {
        let (lower, upper) = rang.split_once("-").unwrap();
        let lower = lower.parse::<u64>().unwrap();
        let upper = upper.parse::<u64>().unwrap();
        ranges_int.push((lower, upper));
    }

    print!("Ranges: {:?}\n", ranges.len());
    print!("Ranges Int: {:?}\n", ranges_int.len());

    ranges_int.sort_by_key(|&(a, _)| a);

    // let mut current = ranges_int.pop().unwrap();
    // let mut looked_at: Vec<(u64, u64)> = Vec::new();
    // let mut count = ranges_int.len() as u64;
    // let mut finished_combining: Vec<(u64, u64)> = Vec::new();

    loop {
        let mut combined = false;
        let mut i = 0;

        while i < ranges_int.len() {
            let mut j = i + 1;
            while j < ranges_int.len() {
                if let Ok(merged) = can_combine_int(ranges_int[i], ranges_int[j]) {
                    ranges_int[i] = merged;
                    ranges_int.remove(j);
                    combined = true;
                } else {
                    j += 1;
                }
            }
            i += 1;
        }

        if !combined {
            break;
        }
    }

    let mut counter = 0;
    for range in &ranges_int {
        let lower = range.0;
        let upper = range.1;
        counter += upper + 1 - lower;
    }
    print!("Counter: {:?}\n", counter);

    // let mut fresh_ranges: Vec<Vec<u64>> = Vec::new();
    // fresh_ranges.push(vec![0, 0]);
    // let mut counter = 0;

    // for range in &ranges {
    //     let (lower, upper) = range.split_once("-").unwrap();
    //     let lower = lower.parse::<u64>().unwrap();
    //     let upper = upper.parse::<u64>().unwrap();
    //     let mut temp_fresh: Vec<Vec<u64>> = Vec::new();
    //     let mut pushed = false;
    //     print!("Range: {:?}\n", range);

    // while fresh_ranges.len() > 0 {
    //     let fresh = fresh_ranges.pop().unwrap();
    //     let merge = can_combine(fresh.to_owned(), vec![lower, upper]);
    //     if merge.is_ok() {
    //         temp_fresh.push(merge.unwrap());
    //         temp_fresh.append(&mut fresh_ranges);
    //         pushed = true;
    //         break;
    //     } else {
    //         temp_fresh.push(fresh);
    //     }
    // }
    // print!("Temp Fresh: {:?}\n", temp_fresh);
    // if pushed == false {
    //     temp_fresh.push(vec![lower, upper]);
    // }
    // fresh_ranges = temp_fresh;
    // if pushed {
    //     let mut temp_fresh: Vec<Vec<u64>> = Vec::new();
    //     let mut current = fresh_ranges.pop().unwrap();

    //     while fresh_ranges.len() > 0 {
    //         let next = fresh_ranges.pop().unwrap();
    //         let merge = can_combine(current.to_owned(), next.to_owned());
    //         if merge.is_ok() {
    //             temp_fresh.push(merge.unwrap());
    //         } else {
    //             temp_fresh.push(current);
    //             current = next;
    //         }
    //     }
    //     fresh_ranges = temp_fresh;
    //     fresh_ranges.retain(|v| v != &[0, 0]);
    //     print!("Fresh Ranges: {:?}\n", fresh_ranges);
    // }
    // }
    // print!("Fresh Ranges: {:?}\n", fresh_ranges);
    // for range in &fresh_ranges {
    //     let lower = range[0];
    //     let upper = range[1];
    //     counter += upper + 1 - lower;
    // }
    // print!("Counter: {:?}\n", counter);

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

// claud code found  an error for combining intervals that are adjacent, but not overlapping
fn can_combine_int(range_one: (u64, u64), range_two: (u64, u64)) -> Result<(u64, u64), String> {
    let range_one_lower = range_one.0;
    let range_one_upper = range_one.1;
    let range_two_lower = range_two.0;
    let range_two_upper = range_two.1;
    let mut combined_lower: u64 = 0;
    let mut combined_upper: u64 = 0;
    let mut can_combine: bool = false;
    if range_one_lower <= range_two_lower && range_two_upper <= range_one_upper {
        return Ok(range_one);
    } else if range_two_lower <= range_one_lower && range_one_upper <= range_two_upper {
        return Ok(range_two);
    } else if range_one_lower <= range_two_lower
        && range_two_lower <= range_one_upper + 1
        && range_one_upper <= range_two_upper
    {
        combined_lower = range_one_lower;
        combined_upper = range_two_upper;
        can_combine = true;
    } else if range_two_lower <= range_one_lower
        && range_one_lower <= range_two_upper + 1
        && range_two_upper <= range_one_upper
    {
        combined_lower = range_two_lower;
        combined_upper = range_one_upper;
        can_combine = true;
    }
    if can_combine {
        return Ok((combined_lower, combined_upper));
    }
    Err("not combinable".to_string())
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

use std::path;
use std::fs;

fn main() {
    let input = fs::read_to_string(path::Path::new("example_input.txt")).expect("Failed to read file");
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];
    let mut diff: u32 = 0;

    for lines in input.lines() {
        let line: Vec<&str> = lines.split_whitespace().collect();
        let left = line[0].parse::<u32>().unwrap();
        let right = line[1].parse::<u32>().unwrap();
        list1.push(left);
        list2.push(right);
        }
        list1.sort();
        list2.sort();

        //part 1
        // for _i in 0..list1.len() {
        //     diff = diff + u32::abs_diff(list1.pop().unwrap(), list2.pop().unwrap());
        // }

        //part 2
        for current in list1 {
            let mut amount: u32 = 0;
            for similar in &list2 {
                if current == *similar {
                    amount = amount + 1;
                }
            }
            diff = diff + current * amount;
        }

    println!("{:?}", diff);
}

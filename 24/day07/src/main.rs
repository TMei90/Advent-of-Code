

use std::fs;
use rand::Rng;

fn main() {
    let input = fs::read_to_string("example_input.txt").unwrap();

    let targets: Vec<u32> = input
        .lines()
        .map(|s| {
            let line = s.split(':').collect::<Vec<&str>>();
            line[0].parse().unwrap()
        })
        .collect();
    let numbers: Vec<Vec<u32>> = input
        .lines()
        .map(|s| {
            let line = s.split(':').collect::<Vec<&str>>();
            let numbers = line[1].trim().split(' ').collect::<Vec<_>>();
            let numbers = numbers
                .iter()
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>();
            numbers
        })
        .collect();

    // println!("{:?}", targets);
    // println!("{:?}", numbers);
    let puzzle = targets.iter().zip(numbers.iter()).collect::<Vec<_>>();
    let mut good_lines = 0;
    
}



fn generate_operators (line: Vec<u32>) -> Vec<Vec<char>>{
    let operators = ['+', '*'];
    let mut operator_lists = Vec::new();
    let mut inner = Vec::new();
    for length in 0..line.len() {
        for operator in 0..operators.len() {
            inner.push(operators[operator]);
        }
    }
    operator_lists
}
// a function that generates all possible combinations of operators
fn generator_recursive (length: u32, operators: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let operators = ['+', '*'];
    if  length == 0{
        return ;
        
    }else {
        for operator in 0..operators.len() {
            generator_recursive(length-1, operator.push('+'));
            generator_recursive(length-1, operator.push('*'));
        }
    }
}

    // let mut rng = rand::thread_rng();
    // for line in puzzle {
    //     let target = line.0;
    //     let mut numbers = line.1.clone();
    //     let mut operators_used: Vec<char> = Vec::new();
    //     for _i in 0..numbers.len()-1{
    //         operators_used.push(operators[rng.gen_range(0..operators.len())]);
    //     }
    //     while target != &numbers[0] {
    //         println!("{} != {}", target, numbers[0]);
    //         println!("{:?}", operators_used);
    //         while numbers.len() > 1 {
    //             let first = numbers.remove(0);
    //             print!("{:?} ", first);
    //             let second = numbers.remove(0);
    //             print!("{:?} ", second);
    //             let operator = operators_used.remove(0);
    //             match operator {
    //                 '+' => {
    //                     let sum = first + second;
    //                     numbers.insert(0, sum);
    //                 }
    //                 '*' => {
    //                     let product = first * second;
    //                     numbers.insert(0, product);
    //                 }
    //                 _ => {}
    //             }
    //         }
    //         if target == &numbers[0] {
    //             good_lines += target;
    //             break;

    //         }
    //         numbers = line.1.clone();
    //         operators_used = Vec::new();
    //         for _i in 0..numbers.len()-1{
    //             operators_used.push(operators[rng.gen_range(0..operators.len())]);
    //         }
    //         // print!("= {:?}\n", numbers[0]);
    //     }
    // }
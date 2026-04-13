#![allow(unused)]
use std::{collections::HashSet, env, fs, path, result};

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
    let coordinates: Vec<(u32, u32, u32)> = lines
        .iter()
        .map(|line| {
            let parts = line
                .split(',')
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (parts[0], parts[1], parts[2])
        })
        .collect();

    let distances: Vec<Vec<f64>> = coordinates
        .iter()
        .map(|&coord| {
            coordinates
                .iter()
                .map(|&other_coord| euclidean_distance(coord, other_coord))
                .collect::<Vec<f64>>()
        })
        .collect();

    println!("calculated distances, now looking for closest ");
    let mut current_distance = 0.0;
    let mut cuircets: Vec<HashSet<usize>> = Vec::new();

    for _ in 0..1000 {
        let (i, j, dist) = find_next_closest(current_distance, &distances);
        current_distance = dist;
        let value = HashSet::from([i, j]);
        cuircets.push(value);
        // println!("Closest pair: ({}, {}) with distance {}", i, j, dist);
        // println!("Current cuircets: {:?}", cuircets);
    }
    println!("found closest 10000 pairs, now merging ");
    // let mut changes = true;
    // while changes {
    //     changes = false;
    //     let mut current = cuircets.pop().unwrap();
    //     let mut looked_at: Vec<HashSet<usize>> = Vec::new();
    //     while cuircets.len() > 0 {
    //         let next = cuircets.pop().unwrap();
    //         if next.iter().any(|x| current.contains(x)) {
    //             current = current.union(&next).cloned().collect::<HashSet<usize>>();
    //             changes = true;
    //         } else {
    //             looked_at.push(next);
    //         }
    //     }
    //     looked_at.insert(0, current);
    //     cuircets = looked_at;
    // }

    let mut changes = true;
    while changes {
        changes = false;
        'outer: loop {
            let mut i = 0;
            while i < cuircets.len() {
                let mut j = i + 1;
                while j < cuircets.len() {
                    if cuircets[i].iter().any(|x| cuircets[j].contains(x)) {
                        let merged = cuircets.remove(j);
                        cuircets[i] = cuircets[i].union(&merged).cloned().collect();
                        changes = true;
                        continue 'outer; // restart since cuircets[i] changed
                    }
                    j += 1;
                }
                i += 1;
            }
            break;
        }
    }
    println!("Cuircets: {:?} with length {}", cuircets, cuircets.len());

    let (mut largest, mut second, mut third) = (0, 0, 0);
    for cuircet in cuircets {
        let size = cuircet.len();
        if size > largest {
            third = second;
            second = largest;
            largest = size;
        } else if size > second {
            third = second;
            second = size;
        } else if size > third {
            third = size;
        }
    }
    println!(": {}", largest * second * third);
}

fn euclidean_distance(x: (u32, u32, u32), y: (u32, u32, u32)) -> f64 {
    let x_diff = (x.0 as f64 - y.0 as f64).powi(2);
    let y_diff = (x.1 as f64 - y.1 as f64).powi(2);
    let z_diff = (x.2 as f64 - y.2 as f64).powi(2);
    (x_diff + y_diff + z_diff).sqrt()
}
fn find_next_closest(current: f64, distances: &Vec<Vec<f64>>) -> (usize, usize, f64) {
    let mut current_closest = 1000000000000000.0;
    let mut current_best = (0, 0, current_closest);
    for i in 0..distances.len() {
        for j in 0..distances.len() {
            if distances[i][j] < current_closest && distances[i][j] > current {
                current_closest = distances[i][j];
                current_best = (i, j, distances[i][j]);
            }
        }
    }
    current_best
}

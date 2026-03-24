use std::{fs};
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut heading: char = 'u';
    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard_position: (usize, usize) = (0, 0);

    for right in 0..map[0].len() {
        for down in 0..map.len() {
            if map[down][right] == '^' {
                guard_position = (down, right);
            }
        }
    }
    let start_position = guard_position;

    // part 1
    // loop {
    //     match heading {
    //         'u' => {
    //             if guard_position.0 > 0 {
    //                 if !(map[guard_position.0-1][guard_position.1] == '#') {
    //                     map[guard_position.0][guard_position.1] = 'X';
    //                     guard_position = (guard_position.0-1, guard_position.1);
    //                 }else {
    //                     heading = 'r';
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //         'r' => {
    //             if guard_position.1 < map[0].len() - 1 {
    //                 if !(map[guard_position.0][guard_position.1+1] == '#') {
    //                     map[guard_position.0][guard_position.1] = 'X';
    //                     guard_position = (guard_position.0, guard_position.1+1);
    //                 }else {
    //                     heading = 'd';
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //         'd' => {
    //             if  guard_position.0 < map.len() - 1 {
    //                 if !(map[guard_position.0+1][guard_position.1] == '#') {
    //                     map[guard_position.0][guard_position.1] = 'X';
    //                     guard_position = (guard_position.0+1, guard_position.1);
    //                 }else {
    //                     heading = 'l';
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //         'l' => {
    //             if guard_position.1 > 0 {
    //                 if !(map[guard_position.0][guard_position.1-1] == '#') {
    //                     map[guard_position.0][guard_position.1] = 'X';
    //                     guard_position = (guard_position.0, guard_position.1-1);
    //                 }else {
    //                     heading = 'u';
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }
    //         _ => break
    //     }
    //     for line in map.iter() {
    //         println!("{:?}",line);
    //     }
    //     println!();
    // }
    // for line in map.iter() {
    //     println!("{:?}",line);
    // }
    // let mut count = 1;

    // for down in 0..map.len() {
    //     for right in 0..map[0].len() {
    //         if map[down][right] == 'X' {
    //             count += 1;
    //         }
    //     }
    // }
    // println!("{}", count);
    
    // part 2
    let prep_map: Vec<Vec<Vec<char>>> = map.clone().into_iter().map(|inner_vec| {inner_vec.into_iter().map(|ch| vec![ch]).collect::<Vec<_>>()}).collect();
    let mut count = 0;
    
    for down in 0..map.len() {
        for right in 0..map[0].len() {
            let mut current_map = map.clone();
            if current_map[down][right] == '.' && (down, right) != start_position {
                current_map[down][right] = '#';
            } else {
                continue;
            }
            guard_position = start_position;
            let mut loop_map = prep_map.clone();
            heading = 'u';
            loop {
                match heading {
                    'u' => {
                        if guard_position.0 > 0 {
                            if !(current_map[guard_position.0 - 1][guard_position.1] == '#') {
                                // println!("{:?}",loop_map[guard_position.0][guard_position.1]);
                                if loop_map[guard_position.0][guard_position.1].iter().any(|e| e == &'u') {
                                    count += 1;
                                    break;
                                }
                                // loop_map[guard_position.0][guard_position.1].push('u');
                                guard_position = (guard_position.0 - 1, guard_position.1);
                            } else {
                                heading = 'r';
                            }
                        } else {
                            break;
                        }
                    }
                    'r' => {
                        if guard_position.1 < map[0].len() - 1 {
                            if !(current_map[guard_position.0][guard_position.1 + 1] == '#') {
                                if loop_map[guard_position.0][guard_position.1].contains(&'r') {
                                    count += 1;
                                    break;
                                }
                                loop_map[guard_position.0][guard_position.1].push('r');
                                guard_position = (guard_position.0, guard_position.1 + 1);
                            } else {
                                heading = 'd';
                            }
                        } else {
                            break;
                        }
                    }
                    'd' => {
                        if guard_position.0 < map.len() - 1 {
                            if !(current_map[guard_position.0 + 1][guard_position.1] == '#') {
                                if loop_map[guard_position.0][guard_position.1].contains(&'d') {
                                count += 1;
                                break;
                                }
                                loop_map[guard_position.0][guard_position.1].push('d');
                                guard_position = (guard_position.0 + 1, guard_position.1);
                            } else {
                                heading = 'l';
                            }
                        } else {
                            break;
                        }
                    }
                    'l' => {
                        if guard_position.1 > 0 {
                            if !(current_map[guard_position.0][guard_position.1 - 1] == '#') {
                                if loop_map[guard_position.0][guard_position.1].contains(&'l') {
                                    count += 1;
                                    break;
                                }
                                loop_map[guard_position.0][guard_position.1].push('l');
                                guard_position = (guard_position.0, guard_position.1 - 1);
                            } else {
                                heading = 'u';
                            }
                        } else {
                            break;
                        }
                    }
                    _ => break,
                }
            }
        }
    }

    
    println!("{}", count);
}

use std::fs;

fn main() {
    let input = fs::read_to_string("example_input.txt").unwrap();
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut hotspot_map: Vec<Vec<char>> = map.clone();
    let mut frequencies = get_frequencies(&map);

    while frequencies.len() > 0 {
        let current = frequencies.pop().unwrap();
        let mut frequency_location_list: Vec<(usize, usize)> = Vec::new();
        frequency_location_list.push(find_next(&current, (0, 0), &map).unwrap());
        while let Ok(location) = find_next(&current, frequency_location_list[frequency_location_list.len() - 1], &map) {
            frequency_location_list.push(location);
        }
        while frequency_location_list.len() > 1 {
            let current = frequency_location_list.remove(0);
            for location in frequency_location_list.clone() {
                finde_harmonics(current, location, &mut hotspot_map);
            }
        }
        
        }
        let mut count = 0;
        for down in 0..map.len() {
            for right in 0..map[0].len()-1 {
                if hotspot_map[down][right] == '#' {
                    count += 1;
                }
            }
        }
        for line in hotspot_map {
            println!("{:?}", line);
        }
        println!("{}", count);
    }


fn find_next(
    frequency: &char,
    starting_location: (usize, usize),
    map: &Vec<Vec<char>>,
) -> Result<(usize, usize), ()> {
    for down in starting_location.0..map.len() {
        for right in 0..map[0].len()-1 {
            if map[down][right] == *frequency && (down, right) != starting_location {
                return Ok((down, right));
            }
        }
    }
    Err(())
}
// part 1
// fn finde_hotspot(
//     first_location: (usize, usize),
//     second_location: (usize, usize),
//     hotspot_map: &mut Vec<Vec<char>>,
// ) {
//     let offset_down = second_location.0 as i32 - first_location.0 as i32;
//     let offset_right = second_location.1 as i32 - first_location.1 as i32;
//     let is_left: bool = offset_right < 0;
//     if is_left {
//         if !(first_location.0 as i32 - offset_down < 0)
//             && !(first_location.1 as i32 + offset_right.abs() >= hotspot_map[0].len() as i32)
//         {
//             hotspot_map[first_location.0 as usize - offset_down as usize]
//                 [first_location.1 as usize + offset_right.abs() as usize] = '#';
//         }
//         if !(second_location.0 as i32 + offset_down >= hotspot_map.len() as i32)
//             && !(second_location.1 as i32 - offset_right.abs() < 0)
//         {
//             hotspot_map[second_location.0 as usize + offset_down as usize]
//                 [second_location.1 as usize - offset_right.abs() as usize] = '#';
//         }
//     } else {
//         if !(first_location.0 as i32 - offset_down < 0) && !(first_location.1 as i32 - offset_right < 0)
//         {
//             hotspot_map[first_location.0 as usize - offset_down as usize]
//                 [first_location.1 as usize - offset_right as usize] = '#';
//         }
//         if !(second_location.0 as i32 + offset_down >= hotspot_map.len() as i32)
//             && !(second_location.1 as i32 + offset_right >= hotspot_map[0].len() as i32)
//         {
//             hotspot_map[second_location.0 as usize + offset_down as usize]
//                 [second_location.1 as usize + offset_right as usize] = '#';
//         }
//     }
//     // for line in hotspot_map {
//     //     println!("{:?}", line);
//     // }
// }

// part 2
fn finde_harmonics(
    mut first_location: (usize, usize),
    mut second_location: (usize, usize),
    hotspot_map: &mut Vec<Vec<char>>,
) {
    let offset_down = second_location.0 as i32 - first_location.0 as i32;
    let offset_right = second_location.1 as i32 - first_location.1 as i32;
    let is_left: bool = offset_right < 0;
    if is_left {
        while !(first_location.0 as i32 - offset_down < 0)
            && !(first_location.1 as i32 + offset_right.abs() >= hotspot_map[0].len() as i32)
        {
            hotspot_map[first_location.0 as usize - offset_down as usize]
                [first_location.1 as usize + offset_right.abs() as usize] = '#';
                first_location.0 = first_location.0 - offset_down as usize;
                first_location.1 = first_location.1 + offset_right.abs() as usize;
        }
        while !(first_location.0 as i32 + offset_down >= hotspot_map.len() as i32)
            && !(first_location.1 as i32 + offset_right.abs() >= hotspot_map[0].len() as i32)
        {
            hotspot_map[first_location.0 as usize + offset_down as usize]
                [first_location.1 as usize + offset_right.abs() as usize] = '#';
                first_location.0 = first_location.0 + offset_down as usize;
                first_location.1 = first_location.1 + offset_right.abs() as usize;
        }
        while !(second_location.0 as i32 + offset_down >= hotspot_map.len() as i32)
            && !(second_location.1 as i32 - offset_right.abs() < 0)
        {
            hotspot_map[second_location.0 as usize + offset_down as usize]
                [second_location.1 as usize - offset_right.abs() as usize] = '#';
                second_location.0 = second_location.0 + offset_down as usize;
                second_location.1 = second_location.1 - offset_right.abs() as usize;
        }
    } else {
        while !(first_location.0 as i32 - offset_down < 0) && !(first_location.1 as i32 - offset_right < 0)
        {
            hotspot_map[first_location.0 as usize - offset_down as usize]
                [first_location.1 as usize - offset_right as usize] = '#';
                first_location.0 = first_location.0 - offset_down as usize;
                first_location.1 = first_location.1 - offset_right as usize;
        }
        while !(first_location.0 as i32 + offset_down >= hotspot_map.len() as i32)
            && !(first_location.1 as i32 - offset_right.abs() < 0)
        {
            hotspot_map[first_location.0 as usize + offset_down as usize]
                [first_location.1 as usize - offset_right.abs() as usize] = '#';
                first_location.0 = first_location.0 + offset_down as usize;
                first_location.1 = first_location.1 - offset_right.abs() as usize;
            
        }
        while !(second_location.0 as i32 + offset_down >= hotspot_map.len() as i32)
            && !(second_location.1 as i32 + offset_right >= hotspot_map[0].len() as i32)
        {
            hotspot_map[second_location.0 as usize + offset_down as usize]
                [second_location.1 as usize + offset_right as usize] = '#';
                second_location.0 = second_location.0 + offset_down as usize;
                second_location.1 = second_location.1 + offset_right as usize;
        }
    }
}
fn get_frequencies(map: &Vec<Vec<char>>) -> Vec<char> {
    let mut frequencies: Vec<char> = Vec::new();
    for line in map {
        for char in line {
            if char != &'.' && !frequencies.contains(&char) {
                frequencies.push(char.to_owned());
            }
        }
    }
    frequencies
}

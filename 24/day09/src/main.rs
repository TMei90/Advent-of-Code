use std::{fs, ops::MulAssign};

fn main() {
    let mut input = fs::read_to_string("example_input.txt").unwrap();
    let mut intermediate: Vec<u32> = Vec::new();

    let mut id_number = 0;
    while !input.is_empty() {
        if input.len() >= 2 {
            let block_length = input.remove(0).to_string().parse::<i32>().unwrap();
            let free_space = input.remove(0).to_string().parse::<i32>().unwrap();
            for _ in 0..block_length {
                intermediate.push(id_number);
            }
            for _ in 0..free_space {
                intermediate.push(u32::MAX);
            }
            id_number += 1;
        } else {
            let block_length = input.remove(0).to_string().parse::<i32>().unwrap();
            for _ in 0..block_length {
                intermediate.push(id_number);
            }
        }
    }

    // part 1
    // while let Ok(next_free) = finde_next_free(&intermediate) {
    //     let to_place = intermediate.pop().unwrap();
    //     intermediate.remove(next_free);
    //     intermediate.insert(next_free, to_place);
    // }

    // part 2

    // TODO need to find space for the next part from end of intermediate
    //  not find next fitting chunk for remaining space in intermediate
    let mut unused_end: Vec<u32> = Vec::new();
    let mut temp = Vec::new();
    let mut already_saved_beginning:Vec<u32> = Vec::new();
    while intermediate.len() != 0 {
        if let Ok((next_free, length)) = finde_next_free_chunk(&intermediate) {
            already_saved_beginning.append(&mut intermediate.drain(0..next_free).collect::<Vec<u32>>());
            let available_space = length;
            temp = return_last_chunk(&mut intermediate);
            while temp.len() > available_space || temp[0] == u32::MAX {
                temp.extend_from_slice(&unused_end);
                unused_end = temp;
                temp = return_last_chunk(&mut intermediate);
            }
            if !temp.is_empty() {
                already_saved_beginning.extend_from_slice(&temp);
                for _ in 0..temp.len() {
                    unused_end.push(u32::MAX);
                    intermediate.remove(0);
                }
            }

            println!("beginning {:?}", already_saved_beginning);
            println!(" to be done {:?}", intermediate);
            println!("end {:?}", unused_end);
            already_saved_beginning.append(&mut intermediate);
            already_saved_beginning.append(&mut unused_end);
            println!("{:?}", already_saved_beginning);
            intermediate = already_saved_beginning;
            already_saved_beginning = Vec::new();
            
        }
    }

    let mut sum: u64 = 0;
    for i in 0..intermediate.len() {
        if intermediate[i] != u32::MAX {
            sum += i as u64 * intermediate[i] as u64;
        }
    }
    println!("{}", sum);
}

fn finde_next_free(intermediate: &Vec<u32>) -> Result<usize, ()> {
    let next_free = intermediate.iter().take_while(|x| **x != u32::MAX).count();
    if next_free == intermediate.len() {
        return Err(());
    } else {
        return Ok(next_free);
    }
}

fn finde_next_free_chunk(intermediate: &Vec<u32>) -> Result<(usize, usize), ()> {
    let mut count = intermediate.iter().take_while(|x| **x != u32::MAX).count();
    let next_free = count;
    while count != intermediate.len() && intermediate[count] == u32::MAX {
        count += 1;
    }
    if count == intermediate.len() {
        return Err(());
    } else {
        return Ok((next_free, count - next_free));
    }
}

fn return_last_chunk(intermediate: &mut Vec<u32>) -> Vec<u32> {
    if intermediate.len() == 0 {
        return Vec::new();
    }
    let current = intermediate.pop().unwrap();
    let mut last_chunk: Vec<u32> = vec![current];
    let mut count = 1;

    while intermediate.len() >= count && intermediate[intermediate.len() - count] == current {
        count += 1;
    }
    for _ in 0..count - 1 {
        last_chunk.push(intermediate.pop().unwrap());
    }
    return last_chunk;
}

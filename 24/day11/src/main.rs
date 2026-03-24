use std::{fs, sync::WaitTimeoutResult, thread};

fn main() {
    //let input = fs::read_to_string("input.txt").unwrap();
    let input = "70949 6183 4 3825336 613971 0 15 182".to_string();
    let mut working: Vec<u64> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", working);
    
        let div_by_8 = working.len() / 8;
        let ( working1, remainder) = working.split_at(div_by_8);
        let mut working1 = working1.to_vec();
        let (working2, remainder) = remainder.split_at(div_by_8);
        let mut working2 = working2.to_vec();
        let (working3, remainder) = remainder.split_at(div_by_8);
        let mut working3 = working3.to_vec();
        let (working4, remainder) = remainder.split_at(div_by_8);
        let mut working4 = working4.to_vec();
        let (working5, remainder) = remainder.split_at(div_by_8);
        let mut working5 = working5.to_vec();
        let (working6, remainder) = remainder.split_at(div_by_8);
        let mut working6 = working6.to_vec();
        let (working7, remainder) = remainder.split_at(div_by_8);
        let mut working7 = working7.to_vec();
        let working8 = remainder;
        let mut working8 = working8.to_vec();

        let thread1 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working1 = blinked(&mut working1).to_owned();
            println!("thead 1 blink : {}, Size: {:?}",blink, working1.len());
            }
            working1
        });
        let thread2 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working2 = blinked(&mut working2).to_owned();
            println!("thead 2 blink : {}, Size: {:?}",blink, working2.len());
            }
            working2
        });
        let thread3 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working3 = blinked(&mut working3).to_owned();
            println!("thead 3 blink : {}, Size: {:?}",blink, working3.len());
            }
            working3
        });
        let thread4 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working4 = blinked(&mut working4).to_owned();
            println!("thead 4 blink : {}, Size: {:?}",blink, working4.len());
            }
            working4
        });
        let thread5 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working5 = blinked(&mut working5).to_owned();
            println!("thead 5 blink : {}, Size: {:?}",blink, working5.len());
            }
            working5
        });
        let thread6 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working6 = blinked(&mut working6).to_owned();
            println!("thead 6 blink : {}, Size: {:?}",blink, working6.len());
            }
            working6
        });
        let thread7 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working7 = blinked(&mut working7).to_owned();
            println!("thead 7 blink : {}, Size: {:?}",blink, working7.len());
            }
            working7
        });
        let thread8 = thread::spawn(move || -> Vec<u64> {
            for blink in 0..75 {
            let working8 = blinked(&mut working8).to_owned();
            println!("thead 8 blink : {}, Size: {:?}",blink, working8.len());
            }
            working8
        });
        let mut res1 = thread1.join().unwrap();
        let mut res2 = thread2.join().unwrap();
        let mut res3 = thread3.join().unwrap();
        let mut res4 = thread4.join().unwrap();
        let mut res5 = thread5.join().unwrap();
        let mut res6 = thread6.join().unwrap();
        let mut res7 = thread7.join().unwrap();
        let mut res8 = thread8.join().unwrap();
        let mut working_new = vec![];
        working_new.append(&mut res1);
        working_new.append(&mut res2);
        working_new.append(&mut res3);
        working_new.append(&mut res4);
        working_new.append(&mut res5);
        working_new.append(&mut res6);
        working_new.append(&mut res7);
        working_new.append(&mut res8);
        working = working_new;

        
        // print!("Blink: {}, Size: ", blink);
        println!("{:?}", working.len());
}

fn blinked(working: &mut Vec<u64>) -> &mut Vec<u64> {
    let mut i = 0;
    while i < working.len() {
        if working[i] == 0 {
            working[i] = 1;
            i += 1;
        } else if working[i].to_string().len() % 2 == 0 {
            let temp = working[i].to_string();
            let (rock1, rock2) = temp.split_at(working[i].to_string().len() / 2);
            working[i] = rock1.parse().unwrap();
            working.insert(i + 1, rock2.parse().unwrap());
            i += 2;
        } else {
            working[i] = working[i] * 2024;
            i += 1;
        }
        // println!("{:?}", working);
    }

    working
}

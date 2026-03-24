use std::path;
use std::fs;

fn main() {
    let input = fs::read_to_string(path::Path::new("input.txt")).expect("Failed to read file");
    let mut save: u32 = 0;

    for lines in input.lines() {
        let mut line: Vec<i32> = lines.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let mut increasing: bool;
        if line[0] < line[1] {
            increasing = true;
        } else {
            increasing = false;             
        }

        //part 1
        // let mut line_save: bool = true;
        // for i in 0..line.len() - 1 {
        //     if increasing {
        //         if !(line[i] < line[i+1]) || !(line[i+1] - line[i] <= 3) {
        //             line_save = false;
        //         }
        //     } else {
        //         if !(line[i] > line[i+1]) || !(line[i] - line[i+1] <= 3) {
        //             line_save = false;
        //         }
        //     }
        // }

        //part 2
        let (mut line_save, index_of_error) = l_save(line.clone(), increasing);
        if !line_save {
            println!("{:?},{}",line, index_of_error);
            let mut line2 = line.clone();
            if index_of_error != 0 {
                line2.remove(index_of_error-1);
            }
            if line2[0] < line2[1] {
                increasing = true;
            } else {
                increasing = false;             
            }
            (line_save, _) = l_save(line2.clone(), increasing);
            if !line_save {
            line2.remove(index_of_error);
            if line2[0] < line2[1] {
                increasing = true;
            } else {
                increasing = false;             
            }
            (line_save, _) = l_save(line2.clone(), increasing);
        }
            if !line_save {
                println!("  {:?},{}",line2, index_of_error);
                line.remove(index_of_error+1);
                line_save = l_save(line.clone(), increasing).0;
                if !line_save {
                    println!("      {:?},{}",line, index_of_error);
                }
            }
        }
        if line_save {
            save += 1;
        }
    }

    println!("{}", save);
}

fn l_save (line: Vec<i32>, increasing: bool) -> (bool,usize){
    
    let mut line_save: bool = true;
    for i in 0..line.len() - 1 {
        if increasing {
            if !(line[i] < line[i+1]) || !(line[i+1] - line[i] <= 3) || line[i+1] - line[i] == 0 {
                line_save = false;
                return (line_save,i);
            }
        } else {
            if !(line[i] > line[i+1]) || !(line[i] - line[i+1] <= 3) || line[i] - line[i+1] == 0 {
                line_save = false;
                return (line_save,i);
            }
        }
    }
    (line_save,0)
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let width = input.lines().next().unwrap().len();
    let hight = input.lines().count();
    let data: Vec<Vec<char>> = input.lines()
                                    .map(|line| line.chars().collect())
                                    .collect();
    
    let mut count = 0;
    //         // part 1
    // for down in 0.. hight {
    //     for right in 0.. width {
            
            // if data[down][right] == 'X' {
            //     count += look_straight(&data, down, right, width, hight);
            //     count += look_45(&data, down, right, width, hight);
            // }

            // part 2
    for down in 0.. hight-2 {
        for right in 0.. width-2 {
    
            if 
            (data[down][right] == 'M' && data[down+1][right+1] == 'A' && data[down+2][right] == 'S' && data[down+2][right+2] == 'S' && data[down][right+2] == 'M') || 
            (data[down][right] == 'M' && data[down+1][right+1] == 'A' && data[down+2][right] == 'M' && data[down+2][right+2] == 'S' && data[down][right+2] == 'S') ||
            (data[down][right] == 'S' && data[down+1][right+1] == 'A' && data[down+2][right] == 'S' && data[down+2][right+2] == 'M' && data[down][right+2] == 'M') ||
            (data[down][right] == 'S' && data[down+1][right+1] == 'A' && data[down+2][right] == 'M' && data[down+2][right+2] == 'M' && data[down][right+2] == 'S'){
                count += 1;
        }
    }
    println!("{}", count);
    }
}


fn look_straight(data: &Vec<Vec<char>>, down: usize, right: usize, width: usize, hight: usize) -> i32 {
    let mut count = 0;
    if down < hight-3 {
        if data[down+1][right] == 'M' && data[down+2][right] == 'A' && data[down+3][right] == 'S' {
            count += 1;
            println!("{} {} {} down", down, right, count);
        }         
    }
    if right < width-3 {
        if data[down][right+1] == 'M' && data[down][right+2] == 'A' && data[down][right+3] == 'S' {
            count += 1;
            println!("{} {} {} right", down, right, count);
        }
    }
    if down > 2 {
        if data[down-1][right] == 'M' && data[down-2][right] == 'A' && data[down-3][right] == 'S' {
            count += 1;
            println!("{} {} {} up", down, right, count);            
        }
    }
    if right > 2 {
        if data[down][right-1] == 'M' && data[down][right-2] == 'A' && data[down][right-3] == 'S' {
            count += 1;
            println!("{} {} {} left", down, right, count);
        }
    }
    count
}

fn look_45 (data: &Vec<Vec<char>>, down: usize, right: usize, width: usize, hight: usize) -> i32 {
    let mut count = 0;
    if down < hight-3 && right < width-3 {
        if data[down+1][right+1] == 'M' && data[down+2][right+2] == 'A' && data[down+3][right+3] == 'S' {
            count += 1;
            println!("{} {} {} down right", down, right, count);
        }
    }
    if down < hight-3 && right > 2 {
        if data[down+1][right-1] == 'M' && data[down+2][right-2] == 'A' && data[down+3][right-3] == 'S' {
            count += 1;
            println!("{} {} {} down left", down, right, count);
        }
    }
    if down > 2 && right < width-3 {
        if data[down-1][right+1] == 'M' && data[down-2][right+2] == 'A' && data[down-3][right+3] == 'S' {
            count += 1;
            println!("{} {} {} up right", down, right, count);
        }
    }
    if down > 2  && right > 2 {
        if data[down-1][right-1] == 'M' && data[down-2][right-2] == 'A' && data[down-3][right-3] == 'S' {
            count += 1;
            println!("{} {} {} up left", down, right, count);
        }
    }
    count
}

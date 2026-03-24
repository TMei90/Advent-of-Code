use regex::Regex;

fn main() {
    let reg_clean_up = Regex::new(r"do\(\)[^d]*").unwrap();
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;

    // part 1
    // for line in input.lines() {
    //     let mults: Vec<(&str, &str)> = reg.captures_iter(line).map(|captur|{
    //         let (_, [a, b]) = captur.extract();
    //         sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    //         (a, b)
    //     })
    //     .collect();
    //     println!("{:?}", mults);
    // }

    // part 2
    let found: Vec<_> = reg_clean_up.find_iter(&input).map(|m| m.as_str()).collect();
    let new_input = found.join("");
    for _line in input.lines() {
        // println!("{}", new_input);
        let _mults: Vec<(&str, &str)> = reg.captures_iter(&new_input).map(|captur|{
                    let (_, [a, b]) = captur.extract();
                    sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
                    (a, b)
                })
                .collect();
                
        // println!("{:?}", mults);
    }

    println!("{}", sum/6);

    // correct 90772405
}

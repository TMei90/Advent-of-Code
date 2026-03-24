use std::vec;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut split = input.split('*');
    let rules_string = split.next().unwrap().to_string();
    let mut updates_string = split.next().unwrap().to_string();
    _ = updates_string.drain(0..2);
    let rules: Vec<(i32, i32)> = rules_string
        .lines()
        .map(|line| {
            let mut numbers = line.split('|');
            let first = numbers.next().unwrap().parse().unwrap();
            let second = numbers.next().unwrap().parse().unwrap();
            (first, second)
        })
        .collect();
    let updates: Vec<Vec<i32>> = updates_string
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let mut good_updates: Vec<Vec<i32>> = vec![];
    let mut bad_updates: Vec<Vec<i32>> = vec![];
    for update in updates {
        if check_rule(&update, &rules) {
            good_updates.push(update);
        } else {
            bad_updates.push(update);
        }
    }
    let mut score = 0;

    // part 1
    // for update in good_updates {
    //     score += get_mid(&update);
    // }

    // part 2
    for update in bad_updates {
        let mut new_update = correct_update(&update, &rules);
        while !check_rule(&new_update, &rules) {
            new_update = correct_update(&new_update, &rules);
        }
        score += get_mid(&new_update);
    }
    println!("{}", score);
}

fn check_rule(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    let mut checks_out = 0;
    for rule in rules {
        if !(update.contains(&rule.0) && update.contains(&rule.1)) {
            checks_out += 1;
        } else {
            let first = update.iter().position(|&x| x == rule.0).unwrap();
            let second = update.iter().position(|&x| x == rule.1).unwrap();
            if first < second {
                checks_out += 1;
            }
        }
    }
    checks_out == rules.len()
}

fn get_mid(update: &Vec<i32>) -> i32 {
    update.get(update.len() / 2).unwrap().to_owned()
}

fn correct_update(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    for rule in rules {
        if !(update.contains(&rule.0) && update.contains(&rule.1)) {
            continue;
        } else {
            let first = update.iter().position(|&x| x == rule.0).unwrap();
            let second = update.iter().position(|&x| x == rule.1).unwrap();
            if first < second {
                continue;
            } else {
                let mut new_update = update.clone();
                new_update.remove(second);
                new_update.insert(second, rule.0);
                new_update.remove(first);
                new_update.insert(first, rule.1);
                return new_update;
            }
        }
    }
    update.clone()
}

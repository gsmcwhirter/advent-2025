use std::process::exit;

#[path = "./util.rs"]
mod util;

const DIAL_SIZE: i32 = 100;
const INITIAL_VALUE: i32 = 50;
const TARGET_VALUE: i32 = 0;

pub fn a(inf: &String) {
    // let mut list1: Vec<i32> = vec![];
    // let mut list2: Vec<i32> = vec![];

    let mut current = INITIAL_VALUE;
    let mut target_count = 0;

    if let Ok(lines) = util::read_lines(inf) {
        for mut line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            println!("current: {}, line: {}", current, line);

            let size = String::split_off(&mut line, 1);
            let factor: i32;
            match line.as_str() {
                "L" => factor = -1,
                "R" => factor = 1,
                _ => {
                    println!("ERROR: did not recognize '{}' as direction", line);
                exit(1);
                }
            }

            if let Ok(v) = size.parse::<i32>() {
                current += factor * v;
                current %= DIAL_SIZE;
            } else {
                println!("ERROR: could not parse '{}' as i32", size);
                exit(1);
            }

            if current == TARGET_VALUE {
                target_count += 1;
            }
            
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    println!("Password: {}", target_count)
}

pub fn b(inf: &String) {
    // let mut list1: Vec<i32> = vec![];
    // let mut list2: Vec<i32> = vec![];

    let mut current = INITIAL_VALUE;
    let mut target_count = 0;

    if let Ok(lines) = util::read_lines(inf) {
        for mut line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            print!("current: {}, line: {}, ", current, line);

            let size = String::split_off(&mut line, 1);
            let factor: i32;
            match line.as_str() {
                "L" => factor = -1,
                "R" => factor = 1,
                _ => {
                    println!("ERROR: did not recognize '{}' as direction", line);
                exit(1);
                }
            }

            if let Ok(v) = size.parse::<i32>() {
                // current += factor * v;

                for _ in 0..v {
                    current += factor;
                    current %= DIAL_SIZE;
                    if current == TARGET_VALUE {
                        target_count += 1;
                    }
                }

                // if current == 0 && v > 0 {
                //     target_count += 1;
                // }

                // if current < 0 && prev == 0 {
                //     target_count -= 1;
                // }

                // if current >= DIAL_SIZE && prev == 0 {
                //     target_count -= 1;
                // }
                
                // while current < 0 {
                //     current += DIAL_SIZE;
                //     target_count += 1;
                // }

                // while current >= DIAL_SIZE {
                //     current -= DIAL_SIZE;
                //     target_count += 1;
                // }
            } else {
                println!("ERROR: could not parse '{}' as i32", size);
                exit(1);
            }

            println!("new: {}, target_count: {}", current, target_count);
            
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    println!("Password: {}", target_count)
}
use std::{process::exit};

#[path = "./util.rs"]
mod util;


pub fn a(inf: &String) {
    if let Ok(lines) = util::read_lines(inf) {
        let mut total_max: i64 = 0;
        for bank in lines.flatten() {
            if bank.trim() == "" {
                continue
            }

            let bank_vec: Vec<i64> = bank.chars()
                .map(|x| -> i64 {
                    match x.to_string().parse::<i64>() {
                        Ok(v) => { v }
                        Err(_) => {
                            println!("Bank entry is not a number: {}", x);
                            exit(1);
                        }
                    }
                })
                .collect();

            let (tens, idx) = max_and_index(bank_vec[..(bank_vec.len()-1)].as_ref());
            let (ones, _) = max_and_index(bank_vec[(idx+1)..].as_ref());

            total_max += 10*tens + ones;
        }

        println!("Bank sum: {}", total_max)
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}

fn max_and_index(bank: &[i64]) -> (i64, usize) {
    let mut max: i64 = -1;
    let mut max_idx: usize = 0;
    for i in 0..bank.len() {
        if bank[i] > max {
            max = bank[i];
            max_idx = i;
        }
    }
    return (max, max_idx)
}

pub fn b(inf: &String) {
    if let Ok(lines) = util::read_lines(inf) {
        let mut total_max: i64 = 0;
        for bank in lines.flatten() {
            if bank.trim() == "" {
                continue
            }

            let bank_vec: Vec<i64> = bank.chars()
                .map(|x| -> i64 {
                    match x.to_string().parse::<i64>() {
                        Ok(v) => { v }
                        Err(_) => {
                            println!("Bank entry is not a number: {}", x);
                            exit(1);
                        }
                    }
                })
                .collect();

            println!("bank: {:?}", bank_vec.as_slice());

            let mut start_idx = 0;
            let mut idx_mod = 1;
            for i in 0..12 {
                println!("considering: {:?}", bank_vec[start_idx+1-idx_mod..bank_vec.len()-(11-i)].as_ref());
                let (next_max, idx) = max_and_index(bank_vec[start_idx+1-idx_mod..bank_vec.len()-(11-i)].as_ref());
                println!("found: {} at {}", next_max, idx);
                start_idx = start_idx+1-idx_mod + idx;
                idx_mod = 0;
                total_max += i64::pow(10,(11-i).try_into().unwrap()) * next_max;
            }
        }

        println!("Bank sum: {}", total_max)
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}
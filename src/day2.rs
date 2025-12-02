use std::process::exit;

#[path = "./util.rs"]
mod util;


pub fn a(inf: &String) {

    let mut invalid_sum = 0;

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let ranges = line.split(",");
            for range in ranges {
                let parts: Vec<&str> = range.split("-").collect();
                if parts.len() != 2 {
                    println!("Malformed range: {}", range);
                    exit(1);
                }

                let range_start: i64;
                let range_end: i64;

                match parts[0].parse::<i64>() {
                    Ok(v) => {
                        range_start = v;
                    }
                    Err(_) => {
                        println!("Range start is not a number {}", parts[0]);
                        exit(1);
                    }
                }
                
                match parts[1].parse::<i64>() {
                    Ok(v) => {
                        range_end = v;
                    }
                    Err(_) => {
                        println!("Range end is not a number {}", parts[0]);
                        exit(1);
                    }
                }

                for i in range_start..=range_end {
                    // reject any odd-length strings
                    if i64::ilog10(i) % 2 != 1 {
                        continue
                    }

                    let istr = i.to_string();
                    if is_double_pattern(istr) {
                        invalid_sum += i
                    }
                }
            }
        }


    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    println!("Invalid sum: {}", invalid_sum)
}

fn is_double_pattern(istr: String) -> bool {
    let halflen = istr.len()/2;
    for j in 0..halflen {
        if istr.chars().nth(j) != istr.chars().nth(halflen+j) {
            return false
        }
    }
    return true
}

pub fn b(inf: &String) {
    let mut invalid_sum = 0;

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            let ranges = line.split(",");
            for range in ranges {
                let parts: Vec<&str> = range.split("-").collect();
                if parts.len() != 2 {
                    println!("Malformed range: {}", range);
                    exit(1);
                }

                let range_start: i64;
                let range_end: i64;

                match parts[0].parse::<i64>() {
                    Ok(v) => {
                        range_start = v;
                    }
                    Err(_) => {
                        println!("Range start is not a number {}", parts[0]);
                        exit(1);
                    }
                }
                
                match parts[1].parse::<i64>() {
                    Ok(v) => {
                        range_end = v;
                    }
                    Err(_) => {
                        println!("Range end is not a number {}", parts[0]);
                        exit(1);
                    }
                }

                for i in range_start..=range_end {
                    let istr = i.to_string();
                    if is_repeating_pattern(istr) {
                        invalid_sum += i
                    }
                }
            }
        }


    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    println!("Invalid sum: {}", invalid_sum)
}

fn is_repeating_pattern(istr: String) -> bool {
    let halflen = istr.len()/2;

    for patlen in 1..=halflen {
        if istr.len() % patlen != 0 {
            continue
        }

        if has_repeating_pattern_of_len(&istr, patlen) {
            return true
        }
    }
    
    return false
}

fn has_repeating_pattern_of_len(istr: &String, patlen: usize) -> bool {
    let repeats = istr.len() / patlen;

    for j in 0..patlen {
        for repeat in 1..repeats {
            if istr.chars().nth(repeat*patlen+j) != istr.chars().nth((repeat-1)*patlen+j) {
                return false
            }
        }  
    }

    return true
}
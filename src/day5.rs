use std::{process::exit};

#[path = "./util.rs"]
mod util;

#[derive(Clone, Copy, Debug)]
struct Range {
    start: i64,
    end: i64,
}

pub fn a(inf: &String) {
    let mut ranges: Vec<Range> = vec![];
    let mut ranges_stop = false;

    let mut fresh_ct = 0;

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                ranges_stop = true;
                continue
            }

            if !ranges_stop {
                match line.split_once("-") {
                    Some((start_str, end_str)) => {
                        let mut range = Range{start: 0, end: 0};
                        match start_str.parse::<i64>() {
                            Ok(start) => { range.start = start; }
                            Err(_) => {
                                println!("Could not parse range line start '{}'", start_str);
                                exit(1);
                            }
                        }
                        match end_str.parse::<i64>() {
                            Ok(end) => { range.end = end; }
                            Err(_) => {
                                println!("Could not parse range line end '{}'", end_str);
                                exit(1);
                            }
                        }
                        ranges.push(range);
                        post_insert_sort(&mut ranges);
                    },
                    None => {
                        println!("Could not parse range line '{}'", line);
                        exit(1);
                    }
                }
                
                continue
            }


            match line.parse::<i64>() {
                Ok(target) => {
                    for range in &ranges {
                        if range.start <= target && range.end >= target {
                            fresh_ct += 1;
                            break;
                        }
                        if range.start > target {
                            break;
                        }
                    }
                    // match ranges.binary_search_by(|range| -> Ordering {
                    //     if range.start <= target {
                    //         if range.end >= target { // found it!
                    //             return Ordering::Equal
                    //         } else { // there might be a wider range to the left, so this is greater
                    //             return Ordering::Greater
                    //         }
                    //     } else {
                    //         if 
                    //     }
                    //     if range.start <= target && range.end >= target {
                    //         return Ordering::Equal
                    //     } else if range.start > target {

                    //     } else if range.end < target {
                    //         return Ordering::Greater
                    //     } else {

                    //     }
                    // }) {
                    //     Some((_, _)) => {}
                    //     None => {}
                    // }
                }
                Err(_) => {
                    println!("Could not parse target line '{}'", line);
                    exit(1);
                }
            }
        }

        println!("Fresh count: {}", fresh_ct)
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}

fn post_insert_sort(ranges: &mut Vec<Range>) {
    if ranges.len() == 0 {
        return
    }

    let to_move = ranges[ranges.len()-1];
    let mut inserted = false;
    for i in (0..ranges.len()-1).rev() {
        // println!("Comparing [i]={:?} to to_move={:?}", ranges[i], to_move);
        if ranges[i].start > to_move.start {
            ranges[i+1] = ranges[i];
            // println!("shift back");
            continue
        }

        if ranges[i].start == to_move.start && ranges[i].end < to_move.end {
            ranges[i+1] = ranges[i];
            // println!("push back 2");
            continue
        }

        // println!("insert");
        ranges[i+1] = to_move;
        inserted = true;
        break;
    }

    if !inserted {
        ranges[0] = to_move;
    }

    // println!("Sorted ranges: {:?}", ranges)
}

pub fn b(inf: &String) {
    let mut ranges: Vec<Range> = vec![];
    let mut ranges_stop = false;

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                ranges_stop = true;
                continue
            }

            if !ranges_stop {
                match line.split_once("-") {
                    Some((start_str, end_str)) => {
                        let mut range = Range{start: 0, end: 0};
                        match start_str.parse::<i64>() {
                            Ok(start) => { range.start = start; }
                            Err(_) => {
                                println!("Could not parse range line start '{}'", start_str);
                                exit(1);
                            }
                        }
                        match end_str.parse::<i64>() {
                            Ok(end) => { range.end = end; }
                            Err(_) => {
                                println!("Could not parse range line end '{}'", end_str);
                                exit(1);
                            }
                        }
                        ranges.push(range);
                        post_insert_sort(&mut ranges);
                    },
                    None => {
                        println!("Could not parse range line '{}'", line);
                        exit(1);
                    }
                }
                
                continue
            }

            break
        }

        let mut fresh_ids_ct = 0;
        let mut calc_until = -1;
        for range in &ranges {
            if range.start > calc_until {
                fresh_ids_ct += range.end - range.start + 1;
                calc_until = range.end;
            } else if range.end > calc_until {
                fresh_ids_ct += range.end - calc_until;
                calc_until = range.end;
            }
        }

        println!("Fresh ids count: {}", fresh_ids_ct);
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}
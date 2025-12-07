use std::{collections::HashMap, collections::HashSet, process::exit};

#[path = "./util.rs"]
mod util;

pub fn a(inf: &String) {
    let mut splitters: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut streams: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut split_ct = 0;

    if let Ok(lines) = util::read_lines(inf) {
        for (lineno, line) in lines.flatten().enumerate() {
            if line.trim() == "" {
                continue
            }

            if !streams.contains_key(&lineno) {
                _ = streams.insert(lineno, HashSet::new());
            }

            if !splitters.contains_key(&lineno) {
                _ = splitters.insert(lineno, HashSet::new());
            }

            for (col, c) in line.chars().enumerate() {
                if c == 'S' {
                    match streams.get_mut(&lineno) {
                        Some(s) => {
                            _ = s.insert(col);
                        },
                        None => {
                            let mut s = HashSet::new();
                            _ = s.insert(col);
                            _ = streams.insert(lineno, s);
                        }
                    }
                } else if c == '^' {
                    match splitters.get_mut(&lineno) {
                        Some(s) => {
                            _ = s.insert(col);
                        },
                        None => {
                            let mut s = HashSet::new();
                            _ = s.insert(col);
                            _ = splitters.insert(lineno, s);
                        }
                    }
                }
            }

            if lineno == 0 {
                println!("{}", line);
                continue
            }

            let prev_streams; 
            let empty_prev = HashSet::new();
            match streams.get(&(lineno-1)) {
                Some(s) => {
                    prev_streams = s.clone();
                }
                None => {
                    prev_streams = empty_prev;
                }
            }

            let curr_streams;
            match streams.get_mut(&lineno) {
                Some(s) => {
                    curr_streams = s;
                },
                None => {
                    _ = streams.insert(lineno, HashSet::new());
                    match streams.get_mut(&lineno) {
                        Some(s) => {
                            curr_streams = s;
                        },
                        None => {
                            println!("How did we get here?");
                            exit(1)
                        }
                    }
                }
            }

            for stream_col in prev_streams.iter() {
                match splitters.get(&lineno) {
                    Some(s) => {
                        if s.contains(stream_col) {
                            split_ct += 1;
                            if *stream_col > 0 {
                                _ = curr_streams.insert(stream_col - 1);
                            }
                            _ = curr_streams.insert(stream_col + 1);
                        } else {
                            curr_streams.insert(*stream_col);
                        }
                    }
                    None => {
                        curr_streams.insert(*stream_col);
                    }
                }
            }

            for (i, c) in line.chars().enumerate() {
                match c {
                    '^' => {
                        print!("{}", c);
                    },
                    'S' => {
                        print!("{}", c);
                    }
                    _ => {
                        if curr_streams.contains(&i) {
                            print!("{}", "|")
                        } else {
                            print!("{}", ".")
                        }
                    }
                }
            }
            println!();
        }

        println!("Splits: {}", split_ct);
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}

pub fn b(inf: &String) {
    let mut splitters: HashMap<usize, HashSet<usize>> = HashMap::new();
    // let mut worlds: HashMap<usize, HashSet<Vec<usize>>> = HashMap::new();
    let mut streams: HashMap<usize, HashMap<usize, i64>> = HashMap::new();

    if let Ok(lines) = util::read_lines(inf) {
        for (lineno, line) in lines.flatten().enumerate() {
            if line.trim() == "" {
                continue
            }

            if line.trim() == "" {
                continue
            }

            if !streams.contains_key(&lineno) {
                _ = streams.insert(lineno, HashMap::new());
            }

            if !splitters.contains_key(&lineno) {
                _ = splitters.insert(lineno, HashSet::new());
            }

            for (col, c) in line.chars().enumerate() {
                if c == 'S' {
                    match streams.get_mut(&lineno) {
                        Some(s) => {
                            _ = s.insert(col, 1);
                        },
                        None => {
                            let mut s = HashMap::new();
                            _ = s.insert(col, 1);
                            _ = streams.insert(lineno, s);
                        }
                    }
                } else if c == '^' {
                    match splitters.get_mut(&lineno) {
                        Some(s) => {
                            _ = s.insert(col);
                        },
                        None => {
                            let mut s = HashSet::new();
                            _ = s.insert(col);
                            _ = splitters.insert(lineno, s);
                        }
                    }
                }
            }

            if lineno == 0 {
                continue
            }

            let prev_streams; 
            let empty_prev = HashMap::new();
            match streams.get(&(lineno-1)) {
                Some(s) => {
                    prev_streams = s.clone();
                }
                None => {
                    prev_streams = empty_prev;
                }
            }

            let curr_streams;
            match streams.get_mut(&lineno) {
                Some(s) => {
                    curr_streams = s;
                },
                None => {
                    _ = streams.insert(lineno, HashMap::new());
                    match streams.get_mut(&lineno) {
                        Some(s) => {
                            curr_streams = s;
                        },
                        None => {
                            println!("How did we get here?");
                            exit(1)
                        }
                    }
                }
            }

            for (stream_col, ct) in prev_streams.iter() {
                match splitters.get(&lineno) {
                    Some(s) => {
                        if s.contains(stream_col) {
                            if *stream_col > 0 {
                                let curr_val;
                                match curr_streams.get(&(stream_col - 1)) {
                                    Some(v) => {
                                        curr_val = *v;
                                    }
                                    None => {
                                        curr_val = 0;
                                    }
                                }
                                _ = curr_streams.insert(stream_col-1, curr_val + ct);
                            }

                            let curr_val;
                            match curr_streams.get(&(stream_col + 1)) {
                                Some(v) => {
                                    curr_val = *v;
                                }
                                None => {
                                    curr_val = 0;
                                }
                            }
                            _ = curr_streams.insert(stream_col+1, curr_val + ct);
                            
                        } else {
                            let curr_val;
                            match curr_streams.get(stream_col) {
                                Some(v) => {
                                    curr_val = *v;
                                }
                                None => {
                                    curr_val = 0;
                                }
                            }
                            _ = curr_streams.insert(*stream_col, curr_val + ct);
                        }
                    }
                    None => {
                        curr_streams.insert(*stream_col, *ct);
                    }
                }
            }

            let mut sum = 0;
            for (_stream_col, ct) in curr_streams.iter() {
                sum += *ct;
            }
            println!("Worlds: {}", sum);
            
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}
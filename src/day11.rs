use std::{collections::{HashMap, HashSet}, process::exit};

#[path = "./util.rs"]
mod util;

pub fn a(inf: &String) {

    let start: String = String::from("you");
    let target: String = String::from("out");

    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }
            
            match line.split_once(":") {
                Some((from, to_list)) => {
                    let out_list = to_list.split(" ");
                    let mut out_vec: Vec<String> = vec![];
                    for out in out_list {
                        if out == "" {
                            continue
                        }
                        out_vec.push(String::from(out));
                    }
                    graph.insert(String::from(from), out_vec);
                }
                None => {
                    println!("Error malformed line: {}", line);
                    exit(1);
                }
            }
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    let blocked: HashSet<String> = HashSet::new();
    let mut memo: HashMap<String, i64> = HashMap::new();

    match count_paths(&graph, &start, &target, &blocked, &mut memo) {
        Ok(ct) => {
            println!("Count: {}", ct);
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }
}

fn count_paths(graph: &HashMap<String, Vec<String>>, from: &String, to: &String, blocked: &HashSet<String>, memo: &mut HashMap<String, i64>) -> Result<i64, String> {
    println!("at {}", from);
    if blocked.contains(from) {
        println!("\t blocked");
        return Ok(0);
    }

    if *from == *to {
        println!("\ttarget");
        return Ok(1);
    }
    match graph.get(from) {
        Some(outs) => {
            let mut ct = 0;
            for out in outs.iter() {
                match memo.get(out) {
                    Some(incr) => {
                        ct += incr;
                        continue
                    }
                    None => {}
                }

                match count_paths(graph, out, to, blocked, memo) {
                    Ok(incr) => {
                        memo.insert(out.clone(), incr);
                        ct += incr
                    }
                    Err(e) => {
                        return Err(e)
                    }
                }
            }
            return Ok(ct);
        }
        None => {
            println!("from {}", from);
            return Err("missig out paths".to_string())
        }
    }
}

pub fn b(inf: &String) {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    
    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }
            
            match line.split_once(":") {
                Some((from, to_list)) => {
                    let out_list = to_list.split(" ");
                    let mut out_vec: Vec<String> = vec![];
                    for out in out_list {
                        if out == "" {
                            continue
                        }
                        out_vec.push(String::from(out));
                    }
                    graph.insert(String::from(from), out_vec);
                }
                None => {
                    println!("Error malformed line: {}", line);
                    exit(1);
                }
            }
        }
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }

    let svr: String = String::from("svr");
    let out: String = String::from("out");
    let dac: String = String::from("dac");
    let fft: String = String::from("fft");

    let blocked_out: HashSet<String> = HashSet::from([out.clone()]);
    let blocked_dac: HashSet<String> = HashSet::from([dac.clone()]);
    let blocked_fft: HashSet<String> = HashSet::from([fft.clone()]);
    let blocked_dac_out: HashSet<String> = HashSet::from([dac.clone(), out.clone()]);
    let blocked_fft_out: HashSet<String> = HashSet::from([fft.clone(), out.clone()]);

    let mut dac_first = 1;
    let mut fft_first = 1;

    let mut memo: HashMap<String, i64> = HashMap::new();

    match count_paths(&graph, &svr, &dac , &blocked_fft_out, &mut memo) {
        Ok(ct) => {
            println!("svr->dac (no fft) = {}\n", ct);
            dac_first *= ct;
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    memo.clear();
    match count_paths(&graph, &svr, &fft , &blocked_dac_out, &mut memo) {
        Ok(ct) => {
            println!("svr->fft (no dac) = {}\n", ct);
            fft_first *= ct;
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    memo.clear();
    match count_paths(&graph, &dac, &fft , &blocked_out, &mut memo) {
        Ok(ct) => {
            println!("dac->fft (no out) = {}\n", ct);
            dac_first *= ct;
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    memo.clear();
    match count_paths(&graph, &fft, &dac , &blocked_out, &mut memo) {
        Ok(ct) => {
            println!("fft->dac (no out) = {}\n", ct);
            fft_first *= ct;
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    memo.clear();
    match count_paths(&graph, &dac, &out , &blocked_fft, &mut memo) {
        Ok(ct) => {
            println!("dac->out (no fft) = {}\n", ct);
            fft_first *= ct;
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    memo.clear();
    match count_paths(&graph, &fft, &out , &blocked_dac, &mut memo) {
        Ok(ct) => {
            println!("fft->out (no dac) = {}\n", ct);
            dac_first *= ct;
        }
        Err(e) => {
            println!("{}", e);
            exit(1);
        }
    }

    println!("total count: {}", dac_first + fft_first);
}
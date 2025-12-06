use std::{process::exit};

#[path = "./util.rs"]
mod util;

#[derive(Clone, Debug)]
struct Problem {
    operands: Vec<i64>,
    operator: String
}

pub fn a(inf: &String) {
    let mut problems: Vec<Problem> = vec![];

    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten() {
            if line.trim() == "" {
                continue
            }

            // println!("next line");
            let mut field = 0;
            let mut tmp = String::new();
            for c in line.chars() {
                if c == ' ' {
                    if tmp.len() > 0 {
                        if field+1 > problems.len()  {
                            problems.push(Problem { operands: vec![], operator: String::from("") })
                        }

                        match tmp.parse::<i64>() {
                            Ok(v) => {
                                problems[field].operands.push(v);
                            }
                            Err(_) => {
                                if tmp == "+" || tmp == "*" {
                                    problems[field].operator = tmp.clone();
                                } else {
                                    println!("Could not parse field '{:?}'.", tmp);
                                    exit(1);
                                }
                            }
                        }
                        field += 1;
                        tmp.truncate(0);
                    }
                    continue
                }

                tmp.push(c);
            }

            if tmp.len() > 0 {
                if field+1 > problems.len()  {
                    problems.push(Problem { operands: vec![], operator: String::from("") })
                }

                match tmp.parse::<i64>() {
                    Ok(v) => {
                        problems[field].operands.push(v);
                    }
                    Err(_) => {
                        if tmp == "+" || tmp == "*" {
                            problems[field].operator = tmp.clone();
                        } else {
                            println!("Could not parse field '{:?}'.", tmp);
                            exit(1);
                        }
                    }
                }
                // field += 1;
                tmp.truncate(0);
            }
        }

        let mut sol_total = 0;
        for problem in problems {
            match problem.operator.as_str() {
                "+" => {
                    sol_total += problem.operands.iter().fold(0, |acc, e| acc + e ); 
                }
                "*" => {
                    sol_total += problem.operands.iter().fold(1, |acc, e| acc * e);
                }
                _ => {
                    println!("Unrecognized operator '{}'.", problem.operator);
                    exit(1);
                }
            }
        }

        println!("Solution Total: {}", sol_total);
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}

#[derive(Clone, Debug)]
struct ProblemB {
    operands: Vec<String>,
    operator: String,
    start: usize
}

pub fn b(inf: &String) {
    println!("MAKE SURE TO RUN THIS ON THE `tac` OF THE INPUT");
    let mut problems: Vec<ProblemB> = vec![];

    let mut lineno = 0;
    if let Ok(lines) = util::read_lines(inf) {
        for line in lines.flatten(){
            lineno += 1;
            if line.trim() == "" {
                continue
            }

            if lineno == 2 {
                println!("Operators: {:?}", problems);
            }

            // println!("next line");
            let mut field = 0;
            let mut tmp = String::new();
            let mut tmp_start = 0;
            let mut ci = 0;
            for c in line.chars() {
                ci += 1;
                // push characters to tmp
                if c != ' ' {
                    if tmp.len() == 0 {
                        tmp_start = ci - 1;
                    }
                    tmp.push(c);
                    continue
                } 

                // got a space, but there wasn't a previous field to process, so skip
                if tmp.len() == 0 {
                    continue
                }
                
                // we have a tmp to process, so
                // make sure we have a problem to fill
                // this will be done on the operator line
                if field+1 > problems.len()  {
                    problems.push(ProblemB { operands: vec![], operator: String::new(), start: tmp_start })
                }

                // handle operator lines
                if tmp == "+" || tmp == "*" {
                    problems[field].operator = tmp.clone();
                } else { // handle number input lines
                    let pads = tmp_start - problems[field].start;

                    let mut i = 0;
                    for c in tmp.chars() {
                        while pads+i+1 > problems[field].operands.len() {
                            problems[field].operands.push(String::new())
                        }

                        problems[field].operands[pads+i].push(c);
                        i += 1;
                    }
                }

                field += 1;
                tmp.truncate(0);
            }

            // handle the last entry if the tmp value runs up to the end of the line
            if tmp.len() > 0 {
                if field+1 > problems.len()  {
                    problems.push(ProblemB { operands: vec![], operator: String::new(), start: tmp_start })
                }

                // handle operator lines
                if tmp == "+" || tmp == "*" {
                    problems[field].operator = tmp.clone();
                } else { // handle number input lines
                    let pads = tmp_start - problems[field].start;

                    let mut i = 0;
                    for c in tmp.chars() {
                        while pads+i+1 > problems[field].operands.len() {
                            problems[field].operands.push(String::new())
                        }

                        problems[field].operands[pads+i].push(c);
                        i += 1;
                    }
                }

                tmp.truncate(0);
            }
        }

        let mut sol_total = 0;
        for problem in problems {
            println!("Problem: {:?}", problem);
            match problem.operator.as_str() {
                "+" => {
                    sol_total += problem.operands.iter()
                        .map(|e| match e.chars().rev().collect::<String>().parse::<i64>() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("could not parse number '{}'.", e);
                                exit(1);
                            }
                        }).fold(0, |acc, e| acc + e ); 
                }
                "*" => {
                    sol_total += problem.operands.iter()
                        .map(|e| match e.chars().rev().collect::<String>().parse::<i64>() {
                            Ok(v) => v,
                            Err(_) => {
                                println!("could not parse number '{}'.", e);
                                exit(1);
                            }
                        }).fold(1, |acc, e| acc * e ); 
                }
                _ => {
                    println!("Unrecognized operator '{}'.", problem.operator);
                    exit(1);
                }
            }
        }

        println!("Solution Total: {}", sol_total);
    } else {
        println!("Could not read input file '{}'.", inf);
        exit(1)
    }
}
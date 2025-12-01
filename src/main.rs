use std::env;

mod day1;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Expected <task> <inputfile>")
    }

    let task = &args[1];
    let inf = &args[2];

    match task.as_str() {
        "day1a" => day1::a(inf),
        "day1b" => day1::b(inf),
        _ => {
            println!("Unrecognized task '{}'", task)
        }
    }
}
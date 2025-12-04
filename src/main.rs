use std::env;

mod day1;
mod day2;
mod day3;

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
        "day2a" => day2::a(inf),
        "day2b" => day2::b(inf),
        "day3a" => day3::a(inf),
        "day3b" => day3::b(inf),
        _ => {
            println!("Unrecognized task '{}'", task)
        }
    }
}
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;

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
        "day4a" => day4::a(inf),
        "day4b" => day4::b(inf),
        "day5a" => day5::a(inf),
        "day5b" => day5::b(inf),
        "day6a" => day6::a(inf),
        "day6b" => day6::b(inf),
        "day7a" => day7::a(inf),
        "day7b" => day7::b(inf),
        "day8a" => day8::a(inf),
        "day8b" => day8::b(inf),
        "day9a" => day9::a(inf),
        "day9b" => day9::b(inf),
        "day10a" => day10::a(inf),
        "day10b" => day10::b(inf),
        _ => {
            println!("Unrecognized task '{}'", task)
        }
    }
}
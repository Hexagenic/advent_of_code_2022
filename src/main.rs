use std::env;

use error::Error;
use solutions::Solution;

mod error;
mod solutions;

fn print_solution(day: u8, part: char, solution: &Option<Solution>) {
    match solution {
        Some(Solution::Integer(i)) => println!("{}{}: {}", day, part, i),
        Some(Solution::String(string)) => println!("{}{}: {}", day, part, string),
        None => println!("{}{}: Incomplete", day, part),
    }
}

fn run() -> Result<(), Error> {
    let (times, puzzles) = get_args()?;

    for _ in 0..times {
        println!("Solving");

        for day in &puzzles {
            let (solution1, solution2) = solutions::get_solution(*day);
            let input_file = format!("day{:0>2}.txt", day);
            let path = env::current_dir().unwrap().join("input").join(&input_file);

            let file: String = std::fs::read_to_string(&path)?;

            print_solution(*day, 'a', &solution1.map(|s| s(&file)));
            print_solution(*day, 'b', &solution2.map(|s| s(&file)));
        }

        println!("Done");
    }

    Ok(())
}

fn get_args() -> Result<(u32, Vec<u8>), Error> {
    let mut args = env::args().skip(1);
    let times: Option<Result<u32, _>> = args.next().map(|s| s.parse());

    if times.is_none() {
        return Ok((1, (1..=solutions::MAX_SOLVED_DAY).collect()));
    }

    let times = times.unwrap()?;

    let puzzles = args.map(|s| s.parse()).collect::<Result<Vec<u8>, _>>()?;

    if puzzles.is_empty() {
        return Ok((times, (1..=solutions::MAX_SOLVED_DAY).collect()));
    }

    Ok((times, puzzles))
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:?}", e);
    }
}

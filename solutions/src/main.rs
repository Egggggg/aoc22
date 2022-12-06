use std::io::BufReader;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub trait Solver {
    type Input;

    fn parse() -> Self::Input;
    fn part1(input: Self::Input) -> String;
    fn part2(input: Self::Input) -> String;
}

fn main() {
    let days = vec![
        day1::Day,
        day2::Day,
        day3::Day,
        day4::Day,
        day5::Day,
        day6::Day,
    ];

    let args: Vec<String> = std::env::args().collect();

    // first arg is binary path
    let day = &args[1];

    // default to both parts
    let part = &args.get(2).or(Some(&"12".to_owned()));
}

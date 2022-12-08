mod days;

use std::default;

use days::*;

const NUM_DAYS: usize = 7;

fn main() {
    // skip the binary path
    let mut args = std::env::args().skip(1);

    // take the next arg as a comma separated list of days to run
    let days_arg = args.next().unwrap();
    let run_days = days_arg.split(',');

    let mut solvers: [Box<dyn Day>; NUM_DAYS] = [
        Box::new(day1::Solver {
            input: default::Default::default(),
        }),
        Box::new(day2::Solver {
            input: default::Default::default(),
        }),
        Box::new(day3::Solver {
            input: default::Default::default(),
        }),
        Box::new(day4::Solver {
            input: default::Default::default(),
        }),
        Box::new(day5::Solver {
            input: default::Default::default(),
        }),
        Box::new(day6::Solver {
            input: default::Default::default(),
        }),
        Box::new(day7::Solver {
            input: default::Default::default(),
        }),
    ];

    for day in run_days {
        let index: std::ops::RangeInclusive<usize> = if day.contains('-') {
            let mut day = day.split('-');
            let start = day.next().unwrap().parse().unwrap();
            let end = day.next().unwrap().parse().unwrap();

            start..=end
        } else {
            let day = day.parse().unwrap();

            day..=day
        };

        for i in index {
            let start = std::time::Instant::now();
            solvers[i].parse_input();
            let us = start.elapsed().as_nanos() as f64 / 1000.;

            println!("Parsed in {us}us");

            let start = std::time::Instant::now();
            let part1 = solvers[i].part1();
            let us = start.elapsed().as_nanos() as f64 / 1000.;

            println!("Part 1: {part1}\nIn {us}us");

            let start = std::time::Instant::now();
            let part2 = solvers[i].part2();
            let us = start.elapsed().as_nanos() as f64 / 1000.;

            println!("Part 2: {part2}\nIn {us}us");
        }
    }
}

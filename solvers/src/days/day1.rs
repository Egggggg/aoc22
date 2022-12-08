use crate::Day;

pub struct Solver {
    pub input: Vec<String>,
}

impl Day for Solver {
    fn parse_input(&mut self) {
        let file = std::fs::read("day1.txt").unwrap();
        let input = String::from_utf8(file).unwrap();
        self.input = input.split("\n\n").map(|e| e.to_owned()).collect();
    }

    fn part1(&self) -> String {
        let mut max = 0;

        for elf in &self.input {
            let food = elf.split("\n");
            let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

            if food > max {
                max = food
            }
        }

        max.to_string()
    }

    fn part2(&self) -> String {
        let mut max = 0;

        for elf in &self.input {
            let food = elf.split("\n");
            let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

            if food > max {
                max = food
            }
        }

        max.to_string()
    }
}

use crate::Day;

pub struct Solver {
    pub input: String,
}

impl Day for Solver {
    fn parse_input(&mut self) {
        let file = std::fs::read("input.txt").unwrap();
        self.input = String::from_utf8(file).unwrap();
    }

    fn part1(&self) -> String {
        todo!()
    }

    fn part2(&self) -> String {
        todo!()
    }
}

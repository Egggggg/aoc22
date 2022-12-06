use crate::Solver;

pub struct Day;

impl Solver for Day {
    type Input = Vec<&'static str>;

    fn parse() -> Self::Input {
        let file = std::fs::read("input.txt").unwrap();
        let input = String::from_utf8(file).unwrap();

        input.split("\n\n").collect()
    }

    fn part1(elves: Self::Input) -> String {
        let mut max = 0;

        for elf in elves {
            let food = elf.split("\n");
            let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

            if food > max {
                max = food
            }
        }

        max.to_string()
    }

    fn part2(elves: Self::Input) -> String {
        let mut max = vec![0, 0, 0];

        for elf in elves {
            let food = elf.split("\n");
            let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

            for i in 0..3 {
                if food > max[i] {
                    max.insert(i, food);

                    break;
                }
            }
        }

        let total = max[0] + max[1] + max[2];

        total.to_string()
    }
}

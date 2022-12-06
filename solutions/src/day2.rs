use std::collections::HashMap;

use crate::Solver;

pub struct Day;

impl Solver for Day {
    type Input = Vec<u8>;

    fn parse(&self) -> Self::Input {
        std::fs::read("input.txt").unwrap()
    }

    fn part1(&self, input: Self::Input) -> String {
        let a = b'A' as u16;
        let b = b'B' as u16;
        let c = b'C' as u16;

        // [points, win, draw]
        let key = HashMap::from([(b'X', [1, c, a]), (b'Y', [2, a, b]), (b'Z', [3, b, c])]);

        let mut total: u16 = 0;

        for (i, lhs) in input.iter().step_by(4).enumerate() {
            let lhs = *lhs as u16;
            let rhs_index = (i * 4 + 2) as usize;
            let rhs = input[rhs_index];

            let combo = key.get(&rhs).unwrap();
            total += combo[0];

            if combo[1] == lhs {
                total += 6;
            } else if combo[2] == lhs {
                total += 3;
            }
        }

        total.to_string()
    }

    fn part2(&self, input: Self::Input) -> String {
        let rock = 1;
        let paper = 2;
        let scissors = 3;

        // [lose, draw, win]
        // [x, y, z]
        let key = HashMap::from([
            (b'A', [scissors, rock, paper]),
            (b'B', [rock, paper, scissors]),
            (b'C', [paper, scissors, rock]),
        ]);

        let mut total: u16 = 0;

        for (i, lhs) in input.iter().step_by(4).enumerate() {
            let lhs = lhs;
            let rhs_index = (i * 4 + 2) as usize;
            let rhs = input[rhs_index];
            let index = rhs as u16 - 88;

            // 0 for losing, 3 for drawing, 6 for winning
            total += index * 3;

            let combo = key.get(lhs).unwrap();

            total += combo[index as usize];
        }

        total.to_string()
    }
}

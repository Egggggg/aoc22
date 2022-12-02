use std::{collections::HashMap, io::Read};

fn main() {
    let file = std::fs::read("input.txt").unwrap();

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

    for (i, lhs) in file.iter().step_by(4).enumerate() {
        let lhs = lhs;
        let rhs_index = (i * 4 + 2) as usize;
        let rhs = file[rhs_index];
        let index = rhs as u16 - 88;

        // 0 for losing, 3 for drawing, 6 for winning
        total += index * 3;

        let combo = key.get(lhs).unwrap();

        total += combo[index as usize];
    }

    println!("Total: {total}");
}

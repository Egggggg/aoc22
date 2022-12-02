use std::{collections::HashMap, io::Read};

fn main() {
    let file = std::fs::read("input.txt").unwrap();

    let a = b'A' as u16;
    let b = b'B' as u16;
    let c = b'C' as u16;

    // [points, win, draw]
    let key = HashMap::from([(b'X', [1, c, a]), (b'Y', [2, a, b]), (b'Z', [3, b, c])]);

    let mut total: u16 = 0;

    for (i, lhs) in file.iter().step_by(4).enumerate() {
        let lhs = *lhs as u16;
        let rhs_index = (i * 4 + 2) as usize;
        let rhs = file[rhs_index];

        let combo = key.get(&rhs).unwrap();
        total += combo[0];

        if combo[1] == lhs {
            total += 6;
        } else if combo[2] == lhs {
            total += 3;
        }
    }

    println!("Total: {total}");
}

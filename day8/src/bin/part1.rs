use std::collections::HashMap;

fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let trees: Vec<&[u8]> = file.split(|c| *c == b'\n').collect();
    let height = trees.len();
    let width = trees[0].len();

    // 0 - right
    // 1 - down
    // 2 - left
    // 3 - up
    // (these are the directions they face *to*, not where they face *from*)
    // u16 to fit all 10 heights
    let lines: HashMap<u8, Vec<u16>> = HashMap::from([
        (0, vec![0; width]),
        (1, vec![0; height]),
        (2, vec![0; width]),
        (3, vec![0; height]),
    ]);

    let start_time = std::time::Instant::now();

    for y in 0..height {
        for x in 0..width {
            // subtract b'1' to map tree height to one-indexed value
            // `as u16` to match with `lines`
            let current = (trees[y][x] - b'1') as u16;

            // 0x1 if 0, 0x10 if 1, 0x100 if 2, etc
            let current_bit = 1 << current;

            let right = lines.get_mut(&0).unwrap()[x];

            // if this tree is taller than the highest, add it to the list
            // a byte with one `1` bit is a higher value than one with all less significant bits combined
            if current > right {
                // OR them together, to get the new height in there
                right |= current;
            }

            // same thing as with right, we are iterating left to right top to bottom
            // remember, the directions are where we're facing, not where we are
            let down = lines.get_mut(&1).unwrap()[y];

            if current > down {
                down |= current;
            }

            let left = lines.get_mut(&2).unwrap()[x];

            // OR them together to count the tree if it's there
            left |= current;

            // NOR 1 less than current to set all the previously seeable lower trees to be unseeable
            left = !(left | current);

            let up = lines.get_mut(&3).unwrap()[y];

            // same thing as left, these ones go in reverse
            up |= current;
            up = !(up | current);
        }
    }

    let total = lines.into_values();

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Total: {total}\nIn {us}us");
}

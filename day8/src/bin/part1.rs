const LEN: usize = 99;

fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let trees: Vec<&[u8]> = file.split(|c| *c == b'\n').collect();

    // 0 - right
    // 1 - down
    // 2 - left
    // 3 - up
    // (these are the directions they face *to*, not where they face *from*)
    // u16 to fit all 10 heights
    // u8 is the position of the visible tree at that height
    // index is the position of the line (x for up and down, y for left and right)
    let mut lines: [[(u16, [i8; 10]); LEN]; 4] = [[(0, [-1; 10]); LEN]; 4];

    let start_time = std::time::Instant::now();

    for y in 0..LEN {
        for x in 0..LEN {
            // subtract b'0' to map tree height to offset
            // `as u16` to match with `lines`
            let current = (trees[y][x] - b'0') as u16;
            let current_idx = current as usize;

            // 0x1 if 0, 0x10 if 1, 0x100 if 2, etc
            // subtract one to get back to zero indexed
            let current_bit = 1 << current;

            let right = lines.get_mut(0).unwrap();

            // if this tree is taller than the highest, add it to the list
            // a byte with one `1` bit is a higher value than one with all less significant bits combined
            if current_bit > right[y].0 {
                let right = right.get_mut(y).unwrap();

                // OR them together, to get the new height in there
                right.0 |= current_bit;
                right.1[current_idx] = x as i8;
            }

            // same thing as with right, we are iterating left to right top to bottom
            let down = lines.get_mut(1).unwrap();

            if current_bit > down[x].0 {
                let down = down.get_mut(x).unwrap();

                down.0 |= current_bit;
                down.1[current_idx] = y as i8;
            }

            let left = lines.get_mut(2).unwrap().get_mut(y).unwrap();

            // make sure the newest tree is marked as seen
            left.1[current_idx] = x as i8;

            // remove smaller trees blocked by the new smallest
            for i in 0..current_idx {
                left.1[i] = -1;
            }

            let up = lines.get_mut(3).unwrap().get_mut(x).unwrap();

            // same thing as left, these two are similar because they are discovered in reverse
            up.1[current_idx] = y as i8;

            for i in 0..current_idx {
                up.1[i] = -1;
            }
        }
    }

    // for each direction, check each position and see if any of their counted trees are the same as another counted tree
    let mut counted: Vec<Vec<bool>> = vec![vec![false; LEN]; LEN];

    for (index, line) in lines.iter().enumerate() {
        for (pos, (_, trees)) in line.iter().enumerate() {
            for tree in trees {
                if *tree == -1 {
                    continue;
                }

                match index {
                    // right or left
                    // pos is y, tree is x
                    0 | 2 => counted[*tree as usize][pos as usize] = true,
                    // up or down
                    // pos is x, tree is y
                    _ => counted[pos as usize][*tree as usize] = true,
                }
            }
        }
    }

    let total: usize = counted
        .iter()
        .map(|row| row.iter().filter(|pos| **pos).count())
        .sum();

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Total: {total}\nIn {us}us");
}

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
    // u8 is the position of the visible tree at that height
    // index is the position of the line (x for up and down, y for left and right)
    let mut lines: [u16; 4] = [0; 4];
    let mut trees: Vec<Vec<u16

    let start_time = std::time::Instant::now();

    for y in 0..height {
        for x in 0..width {
            // subtract b'0' to map tree height to offset
            // `as u16` to match with `lines`
            let current = (trees[y][x] - b'0') as u16;

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
                // println!("right: ({x}, {y})\n");
                right.1[current as usize] = x as u8;
            }

            // same thing as with right, we are iterating left to right top to bottom
            let down = lines.get_mut(1).unwrap();

            if current_bit > down[x].0 {
                let down = down.get_mut(x).unwrap();

                down.0 |= current_bit;
                // println!("down:  ({x}, {y})");
                down.1[current as usize] = y as u8;
            }

            let left = lines.get_mut(2).unwrap().get_mut(y).unwrap();
            let left_old = left.0;

            // OR them together to count the tree
            left.0 |= current_bit;

            // shift `left` right by `current` positions, then back left by the same amount
            // this will get rid of all zeroes to the right of the new shortest visible tree
            left.0 = (left.0 >> current) << current;

            if left.0 != left_old {
                // println!("left:  ({x}, {y})");
            }

            left.1[current as usize] = x as u8;

            let up = lines.get_mut(3).unwrap().get_mut(x).unwrap();
            let up_old = up.0;

            // println!("up:  {:#018b}\nbit: {:#018b}", up.0, current_bit);

            // same thing as left, these ones go in reverse
            up.0 |= current_bit;
            up.0 = (up.0 >> current) << current;

            if up.0 != up_old {
                println!("up:    ({x}, {y})");
            }

            up.1[current as usize] = y as u8;
        }
    }

    dbg!(&lines[1]);

    // for each direction, check each position and see if any of their counted trees are the same as another counted tree
    let mut counted: Vec<Vec<bool>> = vec![vec![false; height]; width];

    for (index, line) in lines.iter().enumerate() {
        for (pos, (_, trees)) in line.iter().enumerate() {
            for tree in trees {
                match index {
                    // right or left
                    // pos is y, tree is x
                    0 | 2 => counted[pos as usize][*tree as usize] = true,
                    // up or down
                    // pos is x, tree is y
                    _ => counted[*tree as usize][pos as usize] = true,
                }
            }
        }
    }

    dbg!(&counted);

    let total: usize = counted
        .iter()
        .map(|row| row.iter().filter(|pos| **pos).count())
        .sum();

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Total: {total}\nIn {us}us");
}

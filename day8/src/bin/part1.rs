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
    let mut lines: [Vec<u16>; 4] = [
        vec![0; height],
        vec![0; width],
        vec![0; height],
        vec![0; width],
    ];

    let start_time = std::time::Instant::now();

    print!("{}", String::from_utf8(trees[50].to_vec()).unwrap());

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
            if current_bit > right[y] {
                let right = right.get_mut(y).unwrap();

                // OR them together, to get the new height in there
                *right |= current_bit;
            }

            // same thing as with right, we are iterating left to right top to bottom
            let down = lines.get_mut(1).unwrap();

            if current_bit > down[x] {
                let down = down.get_mut(x).unwrap();

                *down |= current_bit;
            }

            let left = lines.get_mut(2).unwrap().get_mut(y).unwrap();
            let left_bak = *left;

            // OR them together to count the tree
            *left |= current_bit;

            // shift `left` right by `current` positions, then back left by the same amount
            // this will get rid of all zeroes to the right of the new shortest visible tree
            *left = (*left >> current) << current;

            if y == 50 && *left != left_bak {
                println!(
                    "\ntree: {}\nx: {x}\ncurrent: {current}\ncurrent_bit: {:#018b}\nleft:        {:#018b}",
                    String::from_utf8(vec![trees[y][x]]).unwrap(),
                    current_bit,
                    left_bak
                );
                println!("left after:  {:#018b}", left);
                println!();
            }

            let up = lines.get_mut(3).unwrap().get_mut(x).unwrap();

            // same thing as left, these ones go in reverse
            *up |= current_bit;
            *up = (*up >> current) << current;
        }
    }

    let total: u16 = lines
        .iter()
        .map(|l| l.iter().fold(0, |acc, e| acc + e.count_ones() as u16))
        .sum();

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Total: {total}\nIn {us}us");
}

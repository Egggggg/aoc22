fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let trees: Vec<&[u8]> = file.split(|c| *c == b'\n').collect();
    let height = trees.len();
    let width = trees[0].len();

    // number of trees visible facing left
    // all trees to the right of checked trees are updated until a taller tree is found
    let mut left: Vec<Vec<u8>> = vec![vec![0; height]; width];

    // number of trees visible facing up
    let mut up: Vec<Vec<u8>> = vec![vec![0; height]; width];

    // number of trees visible facing right
    let mut right: Vec<Vec<u8>> = vec![vec![0; height]; width];

    // number of trees visible facing down
    let mut down: Vec<Vec<u8>> = vec![vec![0; height]; width];

    let start_time = std::time::Instant::now();

    for y in 0..height {
        for x in 0..width {
            // turn the ascii numbers into integers
            let tree_height = trees[y][x] - b'0';
            let mut tree_left = tree_height;
            let mut tree_up = tree_height;

            // go right until we hit a taller tree
            for i in (0..=x).rev() {
                if trees[i][y] - b'0' >= tree_left {
                    left[x][y] += 1;
                    tree_left = tree_height;
                }
            }

            // go down until we hit a taller tree
            for i in (0..=y).rev() {
                if trees[x][i] - b'0' >= tree_up {
                    up[x][y] += 1;
                    tree_up = tree_height;
                }
            }

            // go left until we hit a taller tree
            for i in x..width {
                right[i][y] += 1;

                if trees[i][y] - b'0' > tree_height {
                    break;
                }
            }

            // do the same thing going up
            for i in y..height {
                down[x][i] += 1;

                if trees[x][i] - b'0' > tree_height {
                    break;
                }
            }
        }
    }

    let mut max: u32 = 0;

    for y in 0..height {
        for x in 0..width {
            let left = left[x][y] as u32;
            let up = up[x][y] as u32;
            let right = right[x][y] as u32;
            let down = down[x][y] as u32;

            let score = left * up * right * down;

            if score > max {
                dbg!(left, up, right, down);
                dbg!(score, x, y);
                dbg!(trees[x][y] - b'0');
                println!();

                max = score;
            }
        }
    }

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Max: {max}\nIn {us}us");
}

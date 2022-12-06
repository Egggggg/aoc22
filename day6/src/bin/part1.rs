fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let mut current = [0u8; 4];
    let mut found_at: usize = 0;

    let start = std::time::Instant::now();

    for (i, c) in file.iter().enumerate() {
        current.rotate_right(1);
        current[0] = *c;

        let mut existing: Vec<u8> = Vec::new();
        let mut repeat = false;

        for c in current {
            if existing.contains(&c) || c == 0 {
                repeat = true;
                break;
            }

            existing.push(c);
        }

        if !repeat {
            found_at = i + 1; // add one cause 1 indexed
            break;
        }
    }

    let us = start.elapsed().as_nanos() as f64 / 1000.;
    println!("First marker: {found_at}\nIn {us}us");
}

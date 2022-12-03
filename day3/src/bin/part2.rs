fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();
    let mut lines = input.split("\n");
    let mut total = 0;

    while let Some(first) = lines.next() {
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        let common = first
            .chars()
            .find_map(|c| {
                if second.contains(c) && third.contains(c) {
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap();

        let mut priority = common as u16;

        if common >= 'a' {
            // lowercase: subtract 96 (ascii a is 97)
            priority -= 96;
        } else {
            // uppercase: subtract 38 (asci A is 65)
            priority -= 38;
        }

        total += priority;
    }

    println!("Total: {total}");
}

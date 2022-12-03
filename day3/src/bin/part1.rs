fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();
    let lines = input.split("\n");
    let mut total = 0;

    for line in lines {
        let len = line.len();
        let (lhs, rhs) = line.split_at(len / 2);

        let common = lhs
            .chars()
            .find_map(|c| if rhs.contains(c) { Some(c) } else { None })
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

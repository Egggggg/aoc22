fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();
    let lines = input.split("\n");

    let mut total: u16 = 0;

    for line in lines {
        let (first, second) = line.split_once(',').unwrap();

        let first = first.split_once('-').unwrap();
        let second = second.split_once('-').unwrap();

        let first = (pad(first.0), pad(first.1));
        let second = (pad(second.0), pad(second.1));

        if first.0 >= second.0 && first.1 <= second.1 {
            total += 1;
        } else if second.0 >= first.0 && second.1 <= first.1 {
            total += 1;
        }
    }

    println!("Total: {total}");
}

fn pad(input: &str) -> String {
    format!("{:0>2}", input)
}

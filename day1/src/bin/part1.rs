fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();

    let elves = input.split("\n\n");
    let mut max = 0;

    for elf in elves {
        let food = elf.split("\n");
        let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

        if food > max {
            max = food
        }
    }

    println!("Max: {max}");
}

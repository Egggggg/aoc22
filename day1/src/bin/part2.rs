fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();

    let elves = input.split("\n\n");
    let mut max = vec![0, 0, 0];

    for elf in elves {
        let food = elf.split("\n");
        let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

        for i in 0..3 {
            if food > max[i] {
                max.insert(i, food);

                break;
            }
        }
    }

    let total = max[0] + max[1] + max[2];

    println!("Max: {total}");
}

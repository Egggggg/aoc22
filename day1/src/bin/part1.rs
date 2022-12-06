fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();

    let elves = input.split("\n\n");
    let mut max = 0;

    let start_time = std::time::Instant::now();

    for elf in elves {
        let food = elf.split("\n");
        let food = food.fold(0, |total, food| total + food.parse::<u32>().unwrap());

        if food > max {
            max = food
        }
    }

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Max: {max}\nIn {us}us");
}

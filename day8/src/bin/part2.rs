fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();
    let lines: Vec<&str> = input.split('\n').collect();

    let start_time = std::time::Instant::now();

    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Total: {total}\nIn {us}us");
}

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();
    let lines: Vec<&str> = input.split('\n').collect();

    // mapping of directory paths to total sizes
    let mut dirs: HashMap<String, u32> = HashMap::from([("".to_owned(), 0)]);
    let mut current: Vec<String> = Vec::new();
    let start_time = std::time::Instant::now();

    for line in lines {
        let components: Vec<&str> = line.split(' ').collect();

        // i *may* have gotten this idea from betaveros on github
        match &*components {
            ["$", "cd", "/"] => current = Vec::new(),
            ["$", "cd", ".."] => {
                current.pop();
            }
            ["$", "cd", to] => {
                current.push((*to).to_owned());
                dirs.insert(current.join("/"), 0);
            }
            ["$", "ls"] => {}
            ["dir", _] => {}
            [size, _] => {
                let size: u32 = size.parse().unwrap();

                for i in 0..current.len() {
                    let path = current[0..=i].join("/");
                    let path_total = dirs.get_mut(&path).unwrap();

                    *path_total += size;
                }
            }
            _ => unreachable!("invalid input"),
        }
    }

    let total: u32 = dirs.values().filter(|v| **v < 100000u32).sum();
    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Total: {total}\nIn {us}us");
}

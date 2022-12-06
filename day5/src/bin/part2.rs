const NUM_STACKS: usize = 9;

fn main() {
    let file = std::fs::read("input.txt").unwrap();
    let input = String::from_utf8(file).unwrap();
    let (start, input) = input.split_once("\n\n").unwrap();

    let start_time = std::time::Instant::now();

    let mut stacks = {
        // reverse so the top crates are at the end
        let lines = start.lines().rev();

        /*
        let key = lines.pop().unwrap();

        // turn the last line into a list of the positions of ids
        // ids are centered in their stack under the contents of each crate
        let key: Vec<usize> = key
            .chars()
            .enumerate()
            .filter_map(|(i, c)| if c != ' ' { Some(i) } else { None })
            .collect();
        */

        let mut out: [String; NUM_STACKS] = Default::default();

        for line in lines {
            let mut line = line.chars();

            // skip the leading "["
            line.next();

            // there is always a crate in the first stack
            // the id will be the second character
            let first = line.next().unwrap();

            out[0].push(first);

            // index of the stack the crate is on
            let mut idx = 1;

            loop {
                // skip "] ["
                // rows are padded to the same length
                // can use `line.skip(3)` for Reasons i guess
                line.next();
                line.next();
                line.next();

                let Some(next) = line.next() else { break };

                if next != ' ' {
                    out[idx].push(next);
                }

                idx += 1;
            }
        }

        out
    };

    let lines = input.lines();

    for line in lines {
        // skip the move command, which will always be first
        let line = line.chars().skip(5);

        // number goes until a space
        let amount: String = line.clone().take_while(|c| *c != ' ').collect();
        let line = line.skip(amount.len());

        let amount: usize = amount.parse().unwrap();

        // skip " from "
        let mut line = line.skip(6);

        // there are only 9 stacks, so the index of each will only be one digit
        let from = line.next().unwrap().to_digit(10).unwrap() as usize;

        // the stacks in the input are 1 indexed
        let from = from - 1;

        // " to " comes next
        let mut line = line.skip(4);

        let to = line.next().unwrap().to_digit(10).unwrap() as usize;
        let to = to - 1;

        let from_stack = stacks[from].clone();

        let (stay, go) = from_stack.split_at(from_stack.len() - amount);

        stacks[from] = stay.to_owned();
        stacks[to].push_str(go)
    }

    let out: String = stacks.map(|s| s.chars().last().unwrap()).iter().collect();
    let us = start_time.elapsed().as_nanos() as f64 / 1000.;

    println!("Top crates: {out}\nIn {us}us");
}

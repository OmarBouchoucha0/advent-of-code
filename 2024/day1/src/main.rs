use std::fs;

fn main() {
    let content = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(e) => {
            eprint!("Error reading file {e}");
            return;
        }
    };

    let mut diff: Vec<u64> = Vec::new();
    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    for line in content.lines() {
        let id_string: Vec<&str> = line.split_whitespace().collect();

        let ids = [
            id_string[0].parse::<u64>().expect("Invalid number"),
            id_string[1].parse::<u64>().expect("Invalid number"),
        ];
        left.push(ids[0]);
        right.push(ids[1]);
    }
    left.sort();
    right.sort();
    for i in 0..left.len() {
        diff.push(left[i].abs_diff(right[i]));
    }
    let sum: u128 = diff.iter().map(|&x| x as u128).sum();
    println!("the sum : {sum}");
}

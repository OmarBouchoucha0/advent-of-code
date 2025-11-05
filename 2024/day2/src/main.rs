use std::fs;

fn main() {
    let content = match fs::read_to_string("input.txt") {
        Ok(text) => text,
        Err(e) => {
            eprintln!("Error : {e}");
            return;
        }
    };
    let mut unsafe_count = 0;
    for line in content.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .map(|num| num.parse::<i64>().expect("Invalid number"))
            .collect();
        let diff = numbers[1] - numbers[0];

        if 3 < diff.abs() || diff.abs() < 1 {
            unsafe_count += 1;
            continue;
        }

        for i in 2..numbers.len() {
            if diff < 0 {
                if numbers[i] - numbers[i - 1] > -1 || numbers[i] - numbers[i - 1] < -3 {
                    println!("{:?}", line);
                    unsafe_count += 1;
                    break;
                }
            } else {
                if numbers[i] - numbers[i - 1] < 1 || numbers[i] - numbers[i - 1] > 3 {
                    println!("{:?}", line);
                    unsafe_count += 1;
                    break;
                }
            }
        }
    }
    let safe_count = 1000 - unsafe_count;
    println!("safe_count : {safe_count}");
}

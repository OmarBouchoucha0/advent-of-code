use std::fs;

fn largest_jolt(bank: &str) -> u64 {
    let mut max = 0;
    for i in 0..(bank.len() - 1) {
        for j in (i + 1)..bank.len() {
            let mut curr_jolt_str = String::from(bank.chars().nth(i).unwrap());
            curr_jolt_str.push(bank.chars().nth(j).unwrap());
            let curr_jolt = curr_jolt_str.parse().unwrap();
            if max < curr_jolt {
                max = curr_jolt;
            }
        }
    }
    max
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for bank in content.lines() {
        println!("Bank : {bank}");
        println!("Largest Jolt : {}", largest_jolt(bank));
        sum += largest_jolt(bank);
    }
    println!("THE SUM : {sum}");
}

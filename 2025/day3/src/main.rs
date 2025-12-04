use std::usize;

fn largest_single_joltage(bank: &str) -> (usize, char) {
    let mut max = '0';
    let mut index = 0;
    for (i, jolt) in bank.chars().enumerate() {
        if jolt > max {
            max = jolt;
            index = i;
        }
    }
    (index, max)
}

fn join_jolts(i1: usize, i2: usize, j1: char, j2: char) -> String {
    let mut result = String::new();
    if i1 > i2 {
        result.push(j2);
        result.push(j1);
    } else {
        result.push(j1);
        result.push(j2);
    }
    result
}

fn largest_joltage(bank: &str) -> u64 {
    let (i1, jolt1) = largest_single_joltage(bank);

    let (_, modified_bank) = bank.split_at(i1 + 1);

    let (i2, jolt2) = largest_single_joltage(modified_bank);
    let result = join_jolts(i1, i2, jolt1, jolt2);
    result.parse().unwrap()
}
fn main() {
    let content = "987654321111111
811111111111119
234234234234278
818181911112111";
    let mut sum = 0;
    for bank in content.lines() {
        println!("Bank : {bank}");
        println!("Jolt: {}", largest_joltage(bank));
        sum += largest_joltage(bank);
    }
    println!("THE SUM : {sum}");
}

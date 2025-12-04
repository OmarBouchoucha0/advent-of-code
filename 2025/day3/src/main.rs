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

fn largest_joltage(bank: &mut str) -> u64 {
    let (i1, jolt1) = largest_single_joltage(bank);
    bank[i1] = 0;
    let (i2, jolt2) = largest_single_joltage(bank);
    let result = join_jolts(i1, i2, jolt1, jolt2);
    result.parse().unwrap()
}

fn main() {
    let content = "987654321111111
811111111111119
234234234234278
818181911112111";
    for banks in content.lines() {
        println!("Banks : {banks}");
    }
}
